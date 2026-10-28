/** 🪦️ Laws of the end of a change journal service: a service whose workspace was deleted stops itself (before, it kept running until a two hour idle stop that never came, because the deleted journal directory made it look busy), a service stops when its owner is gone, and the order of reasons equals an independent Python model; a real service process proves both. */
import{expect,test}from"bun:test";
import{spawn,spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{CHANGE_JOURNAL_POLICY_V1}from"../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts";
import{changeJournalStopReasonV1,runChangeJournalServiceV1,type ChangeJournalFactsV1}from"../../🟦️.ts";

const here=(file:string)=>fileURLToPath(new URL(file,import.meta.url)),policy={...CHANGE_JOURNAL_POLICY_V1,heartbeatMilliseconds:40};
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const sandbox=()=>realpathSync(mkdtempSync(join(tmpdir(),"root-gone-")));
const supervisor=()=>({start:async()=>({backend:"fsevents",trust:"exact"}),stop:async()=>{}});
const race=<T>(promise:Promise<T>,milliseconds:number)=>Promise.race([promise,sleep(milliseconds).then(()=>"timeout" as const)]);
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};

test("the stop reason equals the independent Python model on a grid of facts",()=>{
 const rows:ChangeJournalFactsV1[]=[];
 for(const rootExists of[true,false])for(const directoryExists of[true,false])for(const ownerAlive of[true,false,undefined])for(const idleMilliseconds of[0,policy.idleShutdownMilliseconds,policy.idleShutdownMilliseconds+1])for(const residentBytes of[0,policy.maximumResidentBytes,policy.maximumResidentBytes+1])rows.push({rootExists,directoryExists,ownerAlive,idleMilliseconds,residentBytes});
 const oracle=JSON.parse(spawnSync("python3",["-I",here("./🐍️.py"),String(policy.idleShutdownMilliseconds),String(policy.maximumResidentBytes)],{input:JSON.stringify(rows),encoding:"utf8"}).stdout);
 expect(rows.map(row=>changeJournalStopReasonV1(row,policy)??null)).toEqual(oracle);
});

test("a service whose workspace is deleted stops itself with root-gone",async()=>{
 const base=sandbox();
 try{
  const root=join(base,"checkout"),directory=join(root,".cache","journal");mkdirSync(root);
  const service=await runChangeJournalServiceV1({root,directory,fence:".fence",exclude:{rooted:[],names:[]},watcherPolicy:{},policy,residentBytes:()=>0,createSupervisor:supervisor});
  expect(await race(service.closed,300),"alive while the workspace exists").toBe("timeout");
  rmSync(root,{recursive:true,force:true});
  expect(await race(service.closed,3000)).toBe("root-gone");
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("a service stops when its owner process is gone, and not before",async()=>{
 const base=sandbox(),owner=spawn("sleep",["60"],{stdio:"ignore"});
 try{
  const root=join(base,"w");mkdirSync(root);
  const service=await runChangeJournalServiceV1({root,directory:join(base,"j"),fence:".fence",exclude:{rooted:[],names:[]},watcherPolicy:{},policy,residentBytes:()=>0,owner:owner.pid!,createSupervisor:supervisor});
  expect(await race(service.closed,300),"alive while the owner lives").toBe("timeout");
  owner.kill("SIGKILL");
  expect(await race(service.closed,3000)).toBe("owner-gone");
 }finally{owner.kill("SIGKILL");rmSync(base,{recursive:true,force:true});}
});

test("a real service process ends when its workspace is deleted and when its owner is gone, and keeps running while both exist",async()=>{
 const base=sandbox(),owner=spawn("sleep",["60"],{stdio:"ignore"});
 const start=(name:string,withOwner:boolean)=>{
  const root=join(base,name),child=spawn(process.execPath,[here("./👶️service/📜️script.ts"),root,join(root,".journal"),...(withOwner?[String(owner.pid)]:[])],{stdio:["ignore","pipe","inherit"]});
  let output="";child.stdout.on("data",chunk=>{output+=String(chunk);});
  return{root,child,output:()=>output,ready:async()=>{const end=Date.now()+8000;while(!output.includes("READY")&&Date.now()<end)await sleep(50);return output.includes("READY");}};
 };
 const gone=start("gone",false),kept=start("kept",true);
 try{
  expect(await gone.ready()).toBe(true);expect(await kept.ready()).toBe(true);
  await sleep(300);expect(alive(gone.child.pid!)&&alive(kept.child.pid!),"both run while workspace and owner exist").toBe(true);
  rmSync(gone.root,{recursive:true,force:true});
  const end=Date.now()+6000;while(alive(gone.child.pid!)&&Date.now()<end)await sleep(50);
  expect(alive(gone.child.pid!),"the process of the deleted workspace ended").toBe(false);expect(gone.output()).toContain("CLOSED root-gone");
  expect(alive(kept.child.pid!),"the other keeps running").toBe(true);
  owner.kill("SIGKILL");
  const second=Date.now()+6000;while(alive(kept.child.pid!)&&Date.now()<second)await sleep(50);
  expect(alive(kept.child.pid!),"the process of the dead owner ended").toBe(false);expect(kept.output()).toContain("CLOSED owner-gone");
 }finally{for(const row of[gone,kept])try{row.child.kill("SIGKILL");}catch{}owner.kill("SIGKILL");rmSync(base,{recursive:true,force:true});}
},60000);
