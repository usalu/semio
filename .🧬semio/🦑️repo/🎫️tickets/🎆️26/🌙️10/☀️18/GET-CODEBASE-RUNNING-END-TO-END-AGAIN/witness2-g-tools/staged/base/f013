import {afterAll,beforeAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {execFileSync} from "node:child_process";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {CHANGE_JOURNAL_POLICY_V1,readChangeRecordsAfterV1,readChangeStateV1,changeHeartbeatAgeV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import {createChangeJournalClientV1} from "../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts";
import {changeJournalDirectoryV1,runChangeJournalServiceV1,startChangeJournalServiceV1,ChangeJournalRunningErrorV1,type ChangeWatchFactoryV1} from "../🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),policy=CHANGE_JOURNAL_POLICY_V1;
const binary=process.env.SEMIO_REPO_WATCH_BINARY??"";
let sandbox="";
beforeAll(()=>{sandbox=realpathSync(mkdtempSync(join(tmpdir(),"journal-service-")));});
afterAll(()=>rmSync(sandbox,{recursive:true,force:true}));
let counter=0;
const workspace=():{root:string;directory:string}=>{const root=join(sandbox,"w"+(counter++));mkdirSync(root,{recursive:true});return{root,directory:join(sandbox,"j"+counter)};};
const exclude={rooted:[".git"],names:["node_modules"]};
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
const watchers=(root:string):string[]=>{try{return execFileSync("pgrep",["-f","semio-repo-watch.*"+root.replaceAll("/",".")],{encoding:"utf8"}).trim().split("\n").filter(Boolean);}catch{return[];}};
const sleep=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

type Handles={emit:(rows:[string,number,string][])=>Promise<void>;lost:(scope:string,reason:string)=>void;trust:(trust:string,reason?:string)=>void};
function fake(trust="exact",backend="fsevents"){
 const handles:{value?:Handles;stopped:boolean}={stopped:false};
 const factory:ChangeWatchFactoryV1=options=>({
  async start(){
   handles.value={emit:rows=>options.onBatch(rows,Date.now()),lost:(scope,reason)=>options.onLost(scope,reason),trust:(next,reason="")=>options.onTrust({backend,trust:next,reason})};
   return{backend,trust};
  },
  async stop(){handles.stopped=true;},
 });
 return{factory,handles};
}

test("the service writes batches, losses and trust changes into one new epoch and removes its heartbeat and lock when it stops",async()=>{
 const{root,directory}=workspace(),{factory,handles}=fake();
 const service=await runChangeJournalServiceV1({root,directory,fence:".fence",exclude,watcherPolicy:{},createSupervisor:factory,residentBytes:()=>0});
 const state=readChangeStateV1(directory)!;
 expect(state).toMatchObject({epoch:service.epoch,ready:true,trust:"exact",backend:"fsevents",pid:process.pid});
 expect(changeHeartbeatAgeV1(directory)).toBeLessThan(2000);
 await handles.value!.emit([["m",0,"a.ts"]]);handles.value!.lost("src","must-scan");handles.value!.trust("verify","canary");
 await handles.value!.emit(Array.from({length:policy.maximumRowsPerRecord+1},(_,index)=>["m",0,"f"+index] as [string,number,string]));
 const read=readChangeRecordsAfterV1(directory,service.epoch,0);
 expect(read.ok&&read.records.map(record=>record.rows?.[0]?.[2]??record.lost)).toEqual(["a.ts",["must-scan","src"],["trust-canary",""],["overflow",""]]);
 expect(readChangeStateV1(directory)).toMatchObject({trust:"verify",reason:"canary"});
 await expect(runChangeJournalServiceV1({root,directory,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,residentBytes:()=>0})).rejects.toBeInstanceOf(ChangeJournalRunningErrorV1);
 await service.stop("test");
 expect(await service.closed).toBe("test");expect(handles.stopped).toBe(true);
 expect(existsSync(join(directory,"heartbeat"))).toBe(false);expect(existsSync(join(directory,"lock"))).toBe(false);
 expect(readChangeStateV1(directory)).toMatchObject({ready:false});
});

test("a watcher that cannot vouch (portable backend or failed canary) leaves a cool-down note and no service",async()=>{
 for(const[trust,backend]of[["exact","portable"],["verify","fsevents"]] as const){
  const{root,directory}=workspace();
  await expect(runChangeJournalServiceV1({root,directory,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake(trust,backend).factory,residentBytes:()=>0})).rejects.toThrow(/cannot vouch/u);
  expect(JSON.parse(readFileSync(join(directory,"unavailable.json"),"utf8")).reason).toContain(backend==="portable"?"portable":"fsevents");
  expect(existsSync(join(directory,"lock"))).toBe(false);
  const client=createChangeJournalClientV1({directory,root});
  expect(await client.cursor()).toMatchObject({exact:false,reason:"no-service"});
 }
});

test("the service shuts itself down when idle, when over its memory bound and takes over a lock whose owner died",async()=>{
 const tight={...policy,heartbeatMilliseconds:20,idleShutdownMilliseconds:150};
 const first=workspace();
 const idle=await runChangeJournalServiceV1({...first,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,policy:tight,residentBytes:()=>0});
 expect(await idle.closed).toBe("idle");
 const second=workspace();
 const heavy=await runChangeJournalServiceV1({...second,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,policy:{...policy,heartbeatMilliseconds:20},residentBytes:()=>policy.maximumResidentBytes+1});
 expect(await heavy.closed).toBe("memory");
 const third=workspace();
 mkdirSync(join(third.directory,"lock"),{recursive:true});writeFileSync(join(third.directory,"lock","owner.json"),JSON.stringify({pid:2147483646,at:0}));
 const taken=await runChangeJournalServiceV1({...third,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,residentBytes:()=>0});
 expect(readChangeStateV1(third.directory)?.pid).toBe(process.pid);
 await taken.stop("done");
});

test("a service whose workspace vanished stops itself and removes its journal",async()=>{
 const{root,directory}=workspace(),tight={...policy,heartbeatMilliseconds:20};
 const service=await runChangeJournalServiceV1({root,directory,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,policy:tight,residentBytes:()=>0});
 expect(existsSync(directory)).toBe(true);
 rmSync(root,{recursive:true,force:true});
 expect(await service.closed).toBe("root-gone");
 await sleep(100);
 expect(existsSync(directory)).toBe(false);
});

test("a service stops itself when its journal directory vanished, and a watcher whose stop never returns cannot keep it alive",async()=>{
 const tight={...policy,heartbeatMilliseconds:20,stopWaitMilliseconds:100},first=workspace();
 const service=await runChangeJournalServiceV1({...first,fence:".fence",exclude,watcherPolicy:{},createSupervisor:fake().factory,policy:tight,residentBytes:()=>0});
 rmSync(first.directory,{recursive:true,force:true});
 expect(await service.closed).toBe("directory-gone");
 const second=workspace(),hanging:ChangeWatchFactoryV1=()=>({async start(){return{backend:"fsevents",trust:"exact"};},stop:()=>new Promise<void>(()=>{})});
 const stuck=await runChangeJournalServiceV1({...second,fence:".fence",exclude,watcherPolicy:{},createSupervisor:hanging,policy:tight,residentBytes:()=>0});
 const started=performance.now();await stuck.stop("test");
 expect(await stuck.closed).toBe("test");expect(performance.now()-started).toBeLessThan(2000);
});

test("a burst of one million change rows does not stop the service under its declared memory bound, and does stop it under the former 50 MiB bound (negative control)",()=>{
 const run=(bound:number)=>JSON.parse(execFileSync(process.execPath,[join(here,"🌊️burst","🟦️.ts"),"1000","1000",String(bound)],{encoding:"utf8"}).trim().split("\n").pop()!) as {result:string;peakBytes:number};
 const declared=run(policy.maximumResidentBytes);
 expect(declared.result).toBe("alive");expect(declared.peakBytes).toBeLessThan(policy.maximumResidentBytes);
 expect(run(50*1024*1024).result).toBe("memory");
},60000);

test.skipIf(!binary)("the detached process starts on demand with the real watcher, is shared by concurrent starters, stays under its memory bound, and a kill -9 ends in stale, then a new epoch that refuses the old cursor",async()=>{
 const{root}=workspace(),environment={...process.env,SEMIO_REPO_WATCH_BINARY:binary,XDG_RUNTIME_DIR:sandbox,SEMIO_REPO_CHANGES:"on"};
 const directory=changeJournalDirectoryV1(root,environment);
 const startedAt=performance.now();
 await Promise.all([1,2,3].map(()=>startChangeJournalServiceV1({directory,root,environment})));
 const startMs=performance.now()-startedAt,state=readChangeStateV1(directory)!;
 expect(state).toMatchObject({ready:true,trust:"exact",root});
 expect(alive(state.pid)).toBe(true);
 expect(watchers(root).length).toBe(1);
 const rss=Number(execFileSync("ps",["-o","rss=","-p",String(state.pid)],{encoding:"utf8"}).trim())*1024;
 expect(rss).toBeLessThan(policy.maximumResidentBytes);
 const client=createChangeJournalClientV1({directory,root,environment,start:()=>startChangeJournalServiceV1({directory,root,environment})});
 const cursor=await client.cursor() as {epoch:string;seq:number};
 expect(cursor.epoch).toBe(state.epoch);
 writeFileSync(join(root,"x.ts"),"x");
 expect(await client.since(cursor,[""])).toMatchObject({exact:true,rows:[["a",0,"x.ts"]]});
 process.kill(state.pid,"SIGKILL");
 await sleep(policy.heartbeatStaleMilliseconds+500);
 expect(watchers(root)).toEqual([]);
 const after=await createChangeJournalClientV1({directory,root,environment}).since(cursor,["x.ts"]);
 expect(after).toMatchObject({exact:false,reason:"no-service"});
 const restarted=await client.since(cursor,["x.ts"]);
 expect(restarted).toMatchObject({exact:false,reason:"epoch"});
 const fresh=await client.cursor() as {epoch:string};
 expect(fresh.epoch).not.toBe(cursor.epoch);
 process.kill(readChangeStateV1(directory)!.pid,"SIGTERM");
 await sleep(500);
 expect(existsSync(join(directory,"heartbeat"))).toBe(false);
},60000);

