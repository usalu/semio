/** 🪟️ Staged dependency funding: scopes are built beside the store with no hold, the exclusive hold covers only the bounded publish, readers pin the generation they opened, retirement waits for pins, and a kill -9 at any step leaves a valid store. */
import{test,expect}from"bun:test";
import{spawn,spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import Ajv from"ajv";
import{renderProgressV1,withProgressSinkV1,type ProgressEventV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts";
import setupSchema from"../../🧬️schema.json";
import{fundingWindowPortsV1,openFundingSnapshotV1,publishFundingScopesV1,readFundingPointerV1,recoverFundingStagingV1,retireFundingGenerationsV1,stageFundingScopeV1,type FundingWindowPortsV1}from"../../💰️funding/🟦️.ts";
const protocol=setupSchema.definitions.FundingPublicationWindowProtocol.const,modulePath=join(import.meta.dir,"../../💰️funding/🟦️.ts"),preload=process.env.R5_PRELOAD;
const generation=(index:number)=>createHash("sha1").update("generation-"+index).digest("hex").slice(0,20);
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const alive=(pid:number)=>{try{process.kill(pid,0);return true;}catch{return false;}};
function gate(){let tail:Promise<unknown>=Promise.resolve(),active=0;const seen={entered:0,maximumActive:0,spans:[] as number[]};const run=<T>(body:()=>Promise<T>,signal:AbortSignal):Promise<T>=>{const result=tail.then(async()=>{signal.throwIfAborted();seen.entered++;active++;seen.maximumActive=Math.max(seen.maximumActive,active);const at=performance.now();try{return await body();}finally{active--;seen.spans.push(performance.now()-at);}});tail=result.catch(()=>{});return result;};return{run,seen,idle:()=>active===0};}
function setup(over:Partial<FundingWindowPortsV1>={}){const root=mkdtempSync(join(tmpdir(),"r25-")),exclusive=gate(),clock={skew:0},ports:FundingWindowPortsV1={now:()=>Date.now()+clock.skew,pid:process.pid,startedAtMs:1,alive,exclusive:exclusive.run,say:()=>{},...over};return{root,exclusive,clock,ports,done:()=>rmSync(root,{recursive:true,force:true})};}
const build=(value:string,milliseconds=0)=>async(directory:string)=>{mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"value"),value);for(let index=0;index<40;index++)writeFileSync(join(directory,"f"+index),value);if(milliseconds)await sleep(milliseconds);};
const signal=new AbortController().signal;
async function fund(rig:ReturnType<typeof setup>,rows:ReadonlyArray<readonly[string,number]>,milliseconds=0){
  const staged=(await Promise.all(rows.map(([scope,index])=>stageFundingScopeV1(rig.root,rig.ports,{scope,generation:generation(index),signal},build(generation(index),milliseconds))))).flatMap(row=>row.staged?[row.staged]:[]);
  return publishFundingScopesV1(rig.root,rig.ports,staged,signal);
}
function child(code:string){return[...(preload?["--preload",preload]:[]),"-e",'import{fundingWindowPortsV1,openFundingSnapshotV1,publishFundingScopesV1,recoverFundingStagingV1,stageFundingScopeV1}from'+JSON.stringify(modulePath)+';import{writeFileSync,mkdirSync}from"node:fs";const root=process.argv[1];const ports={now:()=>Date.now(),pid:process.pid,startedAtMs:1,alive:()=>true,exclusive:(body)=>body(),say:()=>{},hook:undefined};const sleep=ms=>new Promise(d=>setTimeout(d,ms));const signal=new AbortController().signal;const gen=n=>n;'+code];}
const g=(index:number)=>generation(index);

test("the schema fixes the window: the exclusive hold is bounded by seconds, shorter than the wait for it, and every file the protocol writes validates against its schema",async()=>{
  expect(protocol.maximumExclusiveMilliseconds).toBeGreaterThan(0);expect(protocol.maximumExclusiveMilliseconds).toBeLessThanOrEqual(30000);expect(protocol.maximumExclusiveMilliseconds).toBeLessThan(protocol.exclusiveWaitMilliseconds);
  const rig=setup(),ajv=new Ajv({strict:false,allErrors:true});await fund(rig,[["a",1],["b",2]]);
  const pointer=JSON.parse(readFileSync(join(rig.root,protocol.pointerName),"utf8")),owner={pid:process.pid,startedAtMs:1};
  expect(ajv.validate({...setupSchema.definitions.FundingPointer},pointer),JSON.stringify(ajv.errors)).toBe(true);expect(ajv.validate({...setupSchema.definitions.FundingStagingOwner},owner)).toBe(true);
  expect(ajv.validate({...setupSchema.definitions.FundingPointer},{...pointer,scopes:{"../x":g(1)}})).toBe(false);expect(ajv.validate({...setupSchema.definitions.FundingPointer},{...pointer,extra:1})).toBe(false);
  rig.done();
});

test("scopes stage concurrently with no exclusive hold at all, and the hold is taken once, for the publish only, well inside the bound",async()=>{
  const rig=setup();const staging=await Promise.all(["a","b","c"].map((scope,index)=>stageFundingScopeV1(rig.root,rig.ports,{scope,generation:g(index),signal},build(g(index),300))));
  expect(rig.exclusive.seen.entered).toBe(0);
  const result=await publishFundingScopesV1(rig.root,rig.ports,staging.map(row=>row.staged!),signal);
  expect(rig.exclusive.seen.entered).toBe(1);expect(rig.exclusive.seen.maximumActive).toBe(1);expect(result.within).toBe(true);expect(result.exclusiveMilliseconds).toBeLessThan(protocol.maximumExclusiveMilliseconds);expect(rig.exclusive.seen.spans[0]!).toBeLessThan(protocol.maximumExclusiveMilliseconds);
  expect(readFundingPointerV1(rig.root).scopes).toEqual({a:g(0),b:g(1),c:g(2)});expect(readdirSync(join(rig.root,protocol.stagingDirectory))).toEqual([]);rig.done();
});

test("two funders of different scopes proceed concurrently: three builds of 400 ms take about 400 ms, not 1200, and both publishes land in the pointer",async()=>{
  const rig=setup(),started=performance.now();
  await Promise.all([fund(rig,[["a",1]],400),fund(rig,[["b",2]],400),fund(rig,[["c",3]],400)]);const wall=performance.now()-started;
  expect(wall).toBeLessThan(1000);expect(rig.exclusive.seen.entered).toBe(3);expect(rig.exclusive.seen.maximumActive).toBe(1);
  const pointer=readFundingPointerV1(rig.root);expect(pointer.scopes).toEqual({a:g(1),b:g(2),c:g(3)});expect(pointer.sequence).toBe(3);rig.done();
});

test("an independent reader of the files (python) sees exactly the published tree and pointer",async()=>{
  const rig=setup();await fund(rig,[["a",1],["b",2]]);
  const run=spawnSync("python3",["-I","-c","import json,os,sys;r=sys.argv[1];p=json.load(open(r+'/current.json'));print(json.dumps({'gen':sorted(os.listdir(r+'/generations')),'scopes':p['scopes'],'seq':p['sequence'],'values':sorted(open(r+'/generations/'+g+'/value').read() for g in os.listdir(r+'/generations')),'staging':os.listdir(r+'/staging')}))",rig.root],{encoding:"utf8"});
  const seen=JSON.parse(run.stdout);expect(seen.gen).toEqual([g(1),g(2)].sort());expect(seen.scopes).toEqual({a:g(1),b:g(2)});expect(seen.seq).toBe(1);expect(seen.values).toEqual([g(1),g(2)].sort());expect(seen.staging).toEqual([]);rig.done();
});

test("a published scope is a hit: the second funding of the same generation builds nothing and does not take the hold",async()=>{
  const rig=setup();await fund(rig,[["a",1]]);let builds=0;const again=await stageFundingScopeV1(rig.root,rig.ports,{scope:"a",generation:g(1),signal},async()=>{builds++;});
  expect(again.origin).toBe("published");expect(builds).toBe(0);expect(rig.exclusive.seen.entered).toBe(1);rig.done();
});

test("cancellation during staging removes the staging directory and takes no hold; cancellation while waiting for the hold leaves a valid store and a complete staging",async()=>{
  const rig=setup(),controller=new AbortController();
  const staging=stageFundingScopeV1(rig.root,rig.ports,{scope:"a",generation:g(1),signal:controller.signal},async()=>{await sleep(200);});setTimeout(()=>controller.abort(),50);
  await expect(staging).rejects.toBeDefined();expect(readdirSync(join(rig.root,protocol.stagingDirectory))).toEqual([]);expect(rig.exclusive.seen.entered).toBe(0);
  const staged=(await stageFundingScopeV1(rig.root,rig.ports,{scope:"a",generation:g(1),signal},build(g(1)))).staged!,stop=new AbortController();stop.abort();
  await expect(publishFundingScopesV1(rig.root,rig.ports,[staged],stop.signal)).rejects.toBeDefined();
  expect(readFundingPointerV1(rig.root).sequence).toBe(0);expect(existsSync(join(rig.root,protocol.stagingDirectory,staged.name,protocol.completeName))).toBe(true);
  await publishFundingScopesV1(rig.root,rig.ports,[staged],signal);expect(readFundingPointerV1(rig.root).scopes.a).toBe(g(1));rig.done();
});

test("an incomplete staging is refused by the publish before the hold is taken",async()=>{
  const rig=setup();mkdirSync(join(rig.root,protocol.stagingDirectory,"a."+g(1)+".x"),{recursive:true});
  await expect(publishFundingScopesV1(rig.root,rig.ports,[{scope:"a",generation:g(1),name:"a."+g(1)+".x"}],signal)).rejects.toThrow("Complete staged funding scope required");expect(rig.exclusive.seen.entered).toBe(0);rig.done();
});

test("retirement waits for pins: a superseded generation stays while a reader pins it and goes after the release and the grace; a pin of a gone process does not hold it",async()=>{
  const rig=setup();await fund(rig,[["a",1]]);const reader=openFundingSnapshotV1(rig.root,rig.ports);await fund(rig,[["a",2]]);
  rig.clock.skew+=protocol.retireGraceMilliseconds+1;expect(retireFundingGenerationsV1(rig.root,rig.ports)).toEqual([]);expect(readFileSync(join(reader.directory("a"),"value"),"utf8")).toBe(g(1));
  reader.release();rig.clock.skew+=0;expect(retireFundingGenerationsV1(rig.root,rig.ports)).toEqual([g(1)]);expect(existsSync(join(rig.root,protocol.generationDirectory,g(1)))).toBe(false);
  await fund(rig,[["a",3]]);mkdirSync(join(rig.root,protocol.pinDirectory,g(2)),{recursive:true});writeFileSync(join(rig.root,protocol.pinDirectory,g(2),"99999999.1.aaaaaaaa"),"");
  rig.clock.skew+=protocol.retireGraceMilliseconds+1;expect(retireFundingGenerationsV1(rig.root,rig.ports)).toEqual([g(2)]);expect(readdirSync(join(rig.root,protocol.generationDirectory))).toEqual([g(3)]);rig.done();
});

test("the grace protects a generation a reader has read from the pointer but not yet pinned",async()=>{
  const rig=setup();await fund(rig,[["a",1]]);await fund(rig,[["a",2]]);
  expect(retireFundingGenerationsV1(rig.root,rig.ports)).toEqual([]);expect(existsSync(join(rig.root,protocol.generationDirectory,g(1)))).toBe(true);rig.done();
});

test("readers started before, during and after continuous publishing and aggressive retirement always see a complete consistent state and are never blocked: real reader process, 1.6 s",async()=>{
  const rig=setup();await fund(rig,[["a",0],["b",1]]);
  const code='const until=Date.now()+1600;let reads=0,bad=0,worst=0;while(Date.now()<until){const at=performance.now();let snap;try{snap=openFundingSnapshotV1(root,ports);}catch(error){bad++;continue;}worst=Math.max(worst,performance.now()-at);try{for(const scope of Object.keys(snap.pointer.scopes)){const want=snap.pointer.scopes[scope];const first=require("node:fs").readFileSync(snap.directory(scope)+"/value","utf8");await sleep(Math.random()*25);const last=require("node:fs").readFileSync(snap.directory(scope)+"/f39","utf8");if(first!==want||last!==want)bad++;}reads++;}catch(error){bad++;}finally{snap.release();}await sleep(Math.random()*10);}console.log(JSON.stringify({reads,bad,worst}));';
  const reader=spawn(process.execPath,[...child(code),rig.root],{stdio:["ignore","pipe","inherit"]});let out="";reader.stdout.on("data",chunk=>out+=chunk);
  const finished=new Promise<number|null>(done=>reader.on("exit",done));
  let retired=0,index=2;const until=Date.now()+1500;
  while(Date.now()<until){await fund(rig,[["a",index],["b",index+1]]);index+=2;rig.clock.skew+=protocol.retireGraceMilliseconds+1;retired+=retireFundingGenerationsV1(rig.root,rig.ports).length;await sleep(20);}
  expect(await finished).toBe(0);const seen=JSON.parse(out.trim().split("\n").at(-1)!);
  expect(seen.bad).toBe(0);expect(seen.reads).toBeGreaterThan(20);expect(seen.worst).toBeLessThan(protocol.maximumExclusiveMilliseconds);expect(retired).toBeGreaterThan(20);console.log("[R25] reader process",JSON.stringify(seen),"published",index-2,"retired",retired);rig.done();
},60000);

test("kill -9 during staging leaves no visible state: the pointer is unchanged, readers are fine, and the next run removes the dead staging and builds again",async()=>{
  const rig=setup();await fund(rig,[["a",1]]);
  const code='await stageFundingScopeV1(root,ports,{scope:"b",generation:'+JSON.stringify(g(2))+',signal},async(directory)=>{mkdirSync(directory,{recursive:true});writeFileSync(directory+"/value","half");process.kill(process.pid,"SIGKILL");await sleep(5000);});';
  const run=spawnSync(process.execPath,[...child(code),rig.root],{encoding:"utf8"});expect(run.signal).toBe("SIGKILL");
  const names=readdirSync(join(rig.root,protocol.stagingDirectory));expect(names).toHaveLength(1);expect(existsSync(join(rig.root,protocol.stagingDirectory,names[0]!,protocol.completeName))).toBe(false);
  const snapshot=openFundingSnapshotV1(rig.root,rig.ports);expect(snapshot.pointer.scopes).toEqual({a:g(1)});expect(readFileSync(join(snapshot.directory("a"),"value"),"utf8")).toBe(g(1));snapshot.release();
  const recovered=recoverFundingStagingV1(rig.root,rig.ports);expect(recovered.removed).toEqual(names);expect(recovered.adoptable).toEqual([]);
  const result=await fund(rig,[["b",2]]);expect(result.within).toBe(true);expect(readFundingPointerV1(rig.root).scopes).toEqual({a:g(1),b:g(2)});rig.done();
},30000);

test("kill -9 after a complete staging: the next run adopts it and builds nothing",async()=>{
  const rig=setup();
  const code='await stageFundingScopeV1(root,ports,{scope:"b",generation:'+JSON.stringify(g(2))+',signal},async(directory)=>{mkdirSync(directory,{recursive:true});writeFileSync(directory+"/value",'+JSON.stringify(g(2))+');});process.kill(process.pid,"SIGKILL");await sleep(5000);';
  const run=spawnSync(process.execPath,[...child(code),rig.root],{encoding:"utf8"});expect(run.signal).toBe("SIGKILL");
  let builds=0;const again=await stageFundingScopeV1(rig.root,rig.ports,{scope:"b",generation:g(2),signal},async()=>{builds++;});
  expect(again.origin).toBe("adopted");expect(builds).toBe(0);await publishFundingScopesV1(rig.root,rig.ports,[again.staged!],signal);
  expect(readFileSync(join(rig.root,protocol.generationDirectory,g(2),"value"),"utf8")).toBe(g(2));rig.done();
},30000);

for(const point of["pointer-written","renamed"]as const)test("kill -9 during the publish, at "+point+": the store is valid, the old state stays published, and the next run completes or cleans up",async()=>{
  const rig=setup();await fund(rig,[["a",1]]);
  const code='const staged=(await stageFundingScopeV1(root,ports,{scope:"a",generation:'+JSON.stringify(g(2))+',signal},async(directory)=>{mkdirSync(directory,{recursive:true});writeFileSync(directory+"/value",'+JSON.stringify(g(2))+');writeFileSync(directory+"/f39",'+JSON.stringify(g(2))+');})).staged;await publishFundingScopesV1(root,{...ports,hook:async(point)=>{if(point===process.env.R25_POINT){writeFileSync(root+"/marker","x");await sleep(60000);}}},[staged],signal);';
  const proc=spawn(process.execPath,[...child(code),rig.root],{env:{...process.env,R25_POINT:point},stdio:"ignore"});
  for(let waited=0;!existsSync(join(rig.root,"marker"))&&waited<20000;waited+=50)await sleep(50);expect(existsSync(join(rig.root,"marker"))).toBe(true);
  const gone=new Promise<void>(done=>proc.on("exit",()=>done()));proc.kill("SIGKILL");await gone;
  const snapshot=openFundingSnapshotV1(rig.root,rig.ports);expect(snapshot.pointer.scopes).toEqual({a:g(1)});expect(readFileSync(join(snapshot.directory("a"),"value"),"utf8")).toBe(g(1));
  const orphan=existsSync(join(rig.root,protocol.generationDirectory,g(2)));expect(orphan).toBe(point==="renamed");
  const recovered=recoverFundingStagingV1(rig.root,rig.ports);expect(readdirSync(rig.root).filter(name=>name.endsWith(".tmp"))).toEqual([]);
  if(point==="pointer-written")expect(recovered.adoptable).toHaveLength(1);
  const retired=retireFundingGenerationsV1(rig.root,rig.ports);expect(retired).toEqual(orphan?[g(2)]:[]);snapshot.release();
  const result=await fund(rig,[["a",2]]);expect(result.within).toBe(true);expect(readFundingPointerV1(rig.root).scopes).toEqual({a:g(2)});
  expect(readFileSync(join(rig.root,protocol.generationDirectory,g(2),"value"),"utf8")).toBe(g(2));rig.done();
},60000);

test("the real ports report the window as `exclusive for N ms` in both languages and compare process start times so a reused pid is a gone process",async()=>{
  const rig=setup(),real=fundingWindowPortsV1(rig.exclusive.run),events:ProgressEventV1[]=[];
  await withProgressSinkV1(event=>events.push(event),()=>fund({...rig,ports:{...real,say:real.say}},[["a",1]]));
  const line=events.find(event=>event.key==="caching.dependency-store.exclusive-window")!;expect(line).toBeDefined();
  expect(renderProgressV1(line,"en")).toMatch(/^Exclusive access to the dependency store for \d+ ms \(limit 20000 ms\): published a$/u);expect(renderProgressV1(line,"de")).toMatch(/^Exklusiver Zugriff auf den Abhängigkeitsspeicher für \d+ ms \(Grenze 20000 ms\): veröffentlicht a$/u);
  const sleeper=spawn("sleep",["30"],{stdio:"ignore"}),startedAt=Date.now();await sleep(300);
  try{expect(real.alive(sleeper.pid!,startedAt)).toBe(true);expect(real.alive(sleeper.pid!,startedAt-3600000)).toBe(false);expect(real.alive(process.pid,1)).toBe(true);}finally{sleeper.kill("SIGKILL");}
  await sleep(200);expect(real.alive(sleeper.pid!,startedAt)).toBe(false);rig.done();
},30000);
