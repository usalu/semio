import {expect,test} from "bun:test";
import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,existsSync,rmSync} from "node:fs";
import {join,resolve} from "node:path";
import {devNull} from "node:os";
import {Database} from "bun:sqlite";

const source=resolve("🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica"),scratch=process.env.SEMIO_TEST_ARTIFACT_DIR!;
const {seedV1,joinV1,openReplicaV1,closeReplicaV1,publishOnceV1,followOnceV1}=await import(join(source,"🟦️.ts"));
const {readHeadV1,readEventsV1,formatHlcV1}=await import(join(source,"..","📒️journal","🟦️.ts"));
const {INDEX_PREFIX_V1}=await import(join(source,"..","🌿️git","🟦️.ts"));
const {packLocalV1}=await import(join(source,"🧷️packgate","🟦️.ts"));
const git=(cwd:string,args:readonly string[],input?:string)=>{
 const env={...process.env,GIT_AUTHOR_NAME:"Fixture",GIT_AUTHOR_EMAIL:"fixture@example.test",GIT_COMMITTER_NAME:"Fixture",GIT_COMMITTER_EMAIL:"fixture@example.test",GIT_CONFIG_GLOBAL:devNull,GIT_CONFIG_NOSYSTEM:"1"};
 const run=Bun.spawnSync(["git",...args],{cwd,env,stdin:input===undefined?undefined:Buffer.from(input),stdout:"pipe",stderr:"pipe"});
 if(run.exitCode!==0)throw Error(run.stderr.toString());
 return run.stdout.toString().trim();
};
const object=(oid:string)=>".git/objects/"+oid.slice(0,2)+"/"+oid.slice(2);

test("packed-only required staged object is published after its register and the receiver exposes an intermediate index",async()=>{
 const root=mkdtempSync(join(scratch,"po-")),store=join(root,"store"),a=join(root,"a"),b=join(root,"b");
 for(const path of [a,b])mkdirSync(path);
 const options=(worktree:string,originId:string)=>({worktree,storeRoot:store,originId,groupId:"g1",pollMilliseconds:10,scanSeconds:900,debounceMilliseconds:5,persistMilliseconds:0});
 const author=options(a,"g1-server"),member=options(b,"g1-m01");
 const db=new Database(":memory:");
 let server,worker;
 try{
  writeFileSync(join(a,".gitignore"),".cache/\n");
  writeFileSync(join(a,"foundation.txt"),"fixture baseline\n");
  git(a,["init","-q","-b","main"]);git(a,["add","-A"]);git(a,["commit","-q","-m","scratch fixture"]);
  await seedV1(author);await joinV1(member);
  server=await openReplicaV1(author,()=>{});worker=await openReplicaV1(member,()=>{});
  const base=git(b,["write-tree"]),cursor=await readHeadV1(store,author.originId);
  writeFileSync(join(a,"loose.txt"),"loose exact content\n");
  writeFileSync(join(a,"packed.txt"),"packed exact content\n");
  git(a,["add","--","loose.txt","packed.txt"]);
  const loose=git(a,["rev-parse",":loose.txt"]),packed=git(a,["rev-parse",":packed.txt"]);
  const target=git(a,["write-tree"]);
  const stem="pack-"+git(a,["pack-objects",join(a,".git","objects","pack","pack")],packed+"\n");
  rmSync(join(a,object(packed)));
  expect(existsSync(join(a,object(packed)))).toBe(false);
  expect(git(a,["cat-file","blob",packed])).toBe("packed exact content");
  await publishOnceV1(server,undefined,[object(loose),"loose.txt"]);
  await publishOnceV1(server,undefined,[".git/index","packed.txt"]);
  const first=(await readEventsV1(store,author.originId,cursor)).events;
  expect(first.filter((event:{path:string})=>event.path.startsWith(INDEX_PREFIX_V1)).map((event:{path:string})=>event.path.slice(INDEX_PREFIX_V1.length)).sort()).toEqual(["loose.txt","packed.txt"]);
  expect(first.some((event:{path:string})=>event.path===object(packed)||event.path.endsWith(stem+".pack"))).toBe(false);
  const beforeRepair=await followOnceV1(worker);
  const intermediate=git(b,["write-tree"]);
  expect(intermediate).not.toBe(base);expect(intermediate).not.toBe(target);
  expect(git(b,["ls-files","--","loose.txt","packed.txt"])).toBe("loose.txt");
  expect(beforeRepair.queued).toBe(1);
  for(const permit of [false,true]){
   const unchecked=".git/objects/pack/pack-"+(permit?"1":"2").repeat(40)+".pack";
   const id=server.book.intern(unchecked);
   if(permit)server.manifest.set(unchecked,{hlc:formatHlcV1({ms:Date.now(),counter:0},author.originId),operation:"delete"});
   const named=(path:string)=>server.manifest.idOf(path)!==undefined;
   expect(named(unchecked)).toBe(true);
   const expected=Boolean((db.query("select ?1 <> 'put' as local").get(server.manifest.operationAt(id)??"absent") as {local:number}).local);
   expect(expected).toBe(true);
   expect(packLocalV1(unchecked,named,()=>true,a)).toBe(false);
  }
  await publishOnceV1(server,undefined,[".git/objects/pack/"+stem+".pack",".git/objects/pack/"+stem+".idx"]);
  await Bun.sleep(1200);
  const afterRepair=await followOnceV1(worker);
  expect(afterRepair.queued).toBe(0);expect(git(b,["write-tree"])).toBe(target);
  const names=(await readEventsV1(store,author.originId,cursor)).events.map((row:{path:string})=>row.path);
  expect(names.indexOf(INDEX_PREFIX_V1+"packed.txt")).toBeLessThan(names.indexOf(".git/objects/pack/"+stem+".pack"));
  console.log("[DEBUG] exact state",JSON.stringify({pendingBeforePack:beforeRepair.queued,pendingAfterPack:afterRepair.queued,intermediateChanged:intermediate!==base,intermediateEqualsTarget:intermediate===target}));
 }finally{
  if(worker)await closeReplicaV1(worker);
  if(server)await closeReplicaV1(server);
  db.close();rmSync(root,{recursive:true,force:true});
 }
});
