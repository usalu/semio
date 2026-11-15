import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {listOriginsV1,readEventsV1} from "../../📒️journal/🟦️.ts";
import {consolidateGitV1} from "../🧱️consolidate/🟦️.ts";
import {closeReplicaV1,followOnceV1,joinV1,openReplicaV1,publishOnceV1,runV1,seedV1,type ReplicaEventV1,type ReplicaOptionsV1,type ReplicaV1} from "../🟦️.ts";

type Machine={name:string;options:ReplicaOptionsV1;events:ReplicaEventV1[];replica?:ReplicaV1};
const scratch:string[]=[],fast={pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0,gitCheckMilliseconds:50,rescueMilliseconds:50,rescueAgeMilliseconds:0,attestMilliseconds:50};
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`replica-gate-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const posixTest=process.platform==="win32"||Bun.spawnSync(["git","--version"]).exitCode!==0?test.skip:test;
const environment={...process.env,GIT_AUTHOR_NAME:"t",GIT_AUTHOR_EMAIL:"t@example.test",GIT_COMMITTER_NAME:"t",GIT_COMMITTER_EMAIL:"t@example.test",GIT_CONFIG_GLOBAL:"/dev/null",GIT_CONFIG_NOSYSTEM:"1"};
function cli(who:Machine,args:string[]):string{
 const result=Bun.spawnSync(["git",...args],{cwd:who.options.worktree,env:environment,stdout:"pipe",stderr:"pipe"});
 if(result.exitCode!==0)throw Error(`git ${args.join(" ")}: ${result.stderr.toString()}`);
 return result.stdout.toString();
}
const ok=(who:Machine,args:string[]):boolean=>Bun.spawnSync(["git",...args],{cwd:who.options.worktree,env:environment,stdout:"pipe",stderr:"pipe"}).exitCode===0;
const at=(who:Machine,path:string)=>join(who.options.worktree,...path.split("/"));
function write(who:Machine,path:string,text:string):void{const file=at(who,path);mkdirSync(dirname(file),{recursive:true});writeFileSync(file,text);}
const machine=(name:string,store:string):Machine=>({name,events:[],options:{worktree:temp(name),storeRoot:store,originId:name,groupId:"g1",...fast}});
const open=async(who:Machine):Promise<void>=>{who.replica=await openReplicaV1(who.options,event=>who.events.push(event));};
const shut=async(who:Machine):Promise<void>=>{if(who.replica){await closeReplicaV1(who.replica);who.replica=undefined;}};
const packs=(who:Machine):string[]=>readdirSync(at(who,".git/objects/pack")).filter(name=>name.endsWith(".pack")).sort();
async function converge(machines:readonly Machine[],done:()=>boolean,rounds:number=400):Promise<void>{
 for(let round=0;round<rounds;round++){
  for(const who of machines)await publishOnceV1(who.replica!);
  for(const who of machines)await followOnceV1(who.replica!);
  if(done())return;
  await Bun.sleep(15);
 }
 throw Error("The machines did not converge");
}

async function pair(){
 const store=temp("store"),server=machine("g1-server",store),worker=machine("g1-m01",store);
 write(server,".gitignore",fixture.gitignore);write(server,"a.txt","alpha\n");write(server,"dir/c.txt","charlie\n");
 cli(server,["init","-q","-b","main"]);cli(server,["add","-A"]);cli(server,["commit","-q","-m","initial"]);
 cli(server,["repack","-a","-d","-q"]);
 await seedV1(server.options);await joinV1(worker.options);
 await open(server);await open(worker);
 await converge([server,worker],()=>existsSync(at(worker,".git/objects/pack"))&&packs(worker).length===packs(server).length&&ok(worker,["cat-file","-e","HEAD^{commit}"]));
 return {store,server,worker,fleetPack:packs(server)[0]!};
}

async function packEvents(store:string,origin:string):Promise<string[]>{
 const found:string[]=[];
 for(const name of await listOriginsV1(store))if(name===origin)for(const event of (await readEventsV1(store,name,undefined)).events)if(event.path.startsWith(".git/objects/pack/")||event.path.startsWith(".git/objects/info/"))found.push(`${event.operation}:${event.path}`);
 return found;
}

async function settle(server:Machine,worker:Machine,commit:string,pause:()=>Promise<void>):Promise<void>{
 const controller=new AbortController(),runs=[runV1(server.options,controller.signal,event=>server.events.push(event)),runV1(worker.options,controller.signal,event=>worker.events.push(event))];
 try{
  for(let round=0;round<600&&!ok(server,["cat-file","-e",`${commit}^{commit}`]);round++)await Bun.sleep(50);
  await pause();
 }finally{controller.abort();await Promise.all(runs.map(run=>run.catch(()=>{})));}
}

for(const [label,command] of [["git repack -a -d",["repack","-a","-d","-q"]],["git gc --aggressive --prune=now",["gc","--aggressive","--prune=now","-q"]]] as const)
posixTest(`${label} on a worker publishes no pack byte and no pack delete, the commit made there still reaches the fleet and both repositories pass fsck`,async()=>{
 const {store,server,worker,fleetPack}=await pair();
 try{
  await shut(server);await shut(worker);
  write(worker,"worker.txt",`from the worker ${label}\n`);cli(worker,["add","-A"]);cli(worker,["commit","-q","-m","worker commit"]);
  const commit=cli(worker,["rev-parse","HEAD"]).trim();
  cli(worker,["config","gc.auto","0"]);
  cli(worker,command as unknown as string[]);
  expect(packs(worker).some(name=>name!==fleetPack)).toBe(true);
  await settle(server,worker,commit,async()=>{
   for(let round=0;round<200&&packs(worker).some(name=>name!==fleetPack);round++)await Bun.sleep(50);
  });
  expect(ok(server,["cat-file","-e",`${commit}^{commit}`])).toBe(true);
  expect(await packEvents(store,"g1-m01")).toEqual([]);
  expect((await packEvents(store,"g1-server")).filter(item=>item.startsWith("delete:"))).toEqual([]);
  cli(server,["fsck","--full"]);cli(worker,["fsck","--full"]);
  expect(packs(worker).filter(name=>name!==fleetPack)).toEqual([]);
  expect(statSync(at(server,`.git/objects/pack/${fleetPack}`)).size).toBeGreaterThan(0);
  expect(packs(server).length).toBe(1);
 }finally{await shut(server);await shut(worker);}
},180_000);

posixTest("the consolidator publishes its pack with a keep marker, a worker repack leaves that pack alone, and only authorised stems are retired for the fleet",async()=>{
 const {store,server,worker,fleetPack}=await pair();
 try{
  await shut(server);await shut(worker);
  write(server,"later.txt","later\n");cli(server,["add","-A"]);cli(server,["commit","-q","-m","later"]);cli(server,["repack","-d","-q"]);
  const controller=new AbortController(),run=runV1(server.options,controller.signal,()=>{}),runWorker=runV1(worker.options,controller.signal,()=>{});
  let result;
  try{result=await consolidateGitV1({worktree:server.options.worktree,apply:true,storeRoot:store,waitMilliseconds:60_000,pollMilliseconds:100,needBytes:1,space:async()=>undefined});}
  finally{
   for(let round=0;round<400&&!(existsSync(at(worker,`.git/objects/pack/${result?.pack}.pack`))&&ok(worker,["cat-file","-e",`${cli(server,["rev-parse","HEAD"]).trim()}^{commit}`])&&!existsSync(at(worker,`.git/objects/pack/${fleetPack}`)));round++)await Bun.sleep(50);
   controller.abort();await Promise.all([run.catch(()=>{}),runWorker.catch(()=>{})]);
  }
  expect(result?.outcome).toBe("consolidated");
  const fresh=`${result!.pack}.pack`;
  expect(existsSync(at(worker,`.git/objects/pack/${fresh}`))).toBe(true);
  expect(existsSync(at(worker,`.git/objects/pack/${result!.pack}.keep`))).toBe(true);
  expect(existsSync(at(worker,`.git/objects/pack/${fleetPack}`))).toBe(false);
  cli(worker,["repack","-a","-d","-q"]);
  expect(existsSync(at(worker,`.git/objects/pack/${fresh}`))).toBe(true);
  cli(worker,["fsck","--full"]);cli(server,["fsck","--full"]);
 }finally{await shut(server);await shut(worker);}
},180_000);
