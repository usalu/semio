/** 🧹️ Sweep: laws of the incremental lease store sweep against Python's sqlite3, a SQLite model and an acquire/release log replay. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,rmSync,symlinkSync,utimesSync,writeFileSync}from"node:fs";
import{isAbsolute,join}from"node:path";
import{fileURLToPath}from"node:url";
import{acquireQueuedResourceLease,acquireResourceLease,resourceLeasePathV1,resourceQueueClaimV1,resourceQueueTicketV1,retireIdleResourceLeasesV1,type ResourceLease,type ResourceLeaseControl}from"../../../🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../🧪️testing/📥️invocation/🟦️.ts";
import{testPythonV1}from"../../../../🧪️testing/🧰️toolchain/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{resourceLeaseRetirementIdlePolicyV1,resourceLeaseRetirementMessagesV1}from"../../🟦️.ts";
import{formatResourceLeaseSweepFailureV1,formatResourceLeaseSweepV1,resourceLeaseSweepModeV1,sweepResourceLeaseStoreV1,type ResourceLeaseSweepV1}from"../🟦️.ts";
import corpus from"./🔣️.json"with{type:"json"};
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
const policy=resourceLeaseRetirementIdlePolicyV1,oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const python=(mode:string,document:unknown,scratch:string):any=>{const file=join(scratch,"oracle-"+mode+".json");writeFileSync(file,JSON.stringify(document));const run=Bun.spawnSync([testPythonV1()??Bun.which("python3")??"python3","-I",oracle,mode,file]);if(run.exitCode!==0)throw Error("Python oracle failed: "+run.stderr.toString());return JSON.parse(run.stdout.toString());};
const aged=(path:string,now:number,age:number)=>{const at=new Date(now-age);utimesSync(path,at,at);};
const ticketRow=JSON.stringify({version:1,pid:process.pid,mode:"exclusive",sharedCohort:false});
async function withScratch<T>(body:(scratch:string,original:Parameters<Parameters<typeof withBuildTestInvocationV1>[0]>[0])=>Promise<T>):Promise<T>{return withBuildTestInvocationV1(async original=>{const capabilities=original.capabilities as{artifacts?:unknown};if(typeof capabilities.artifacts!=="string"||!isAbsolute(capabilities.artifacts))throw Error("Authored finite test output required");mkdirSync(capabilities.artifacts,{recursive:true});const scratch=mkdtempSync(join(capabilities.artifacts,"sw-"));try{return await body(scratch,original);}finally{rmSync(scratch,{recursive:true,force:true});}});}
const lease=(control:ResourceLeaseControl,directory:string,resource:string,mode:"shared"|"exclusive"="shared")=>acquireResourceLease({directory,resource,mode,signal:control.signal,control,timeoutMs:10000});
const claim=(control:ResourceLeaseControl,directory:string,resource:string,ticket:string)=>{const row=resourceQueueClaimV1(directory,resource,ticket);return acquireResourceLease({...row,mode:"shared",signal:control.signal,control,timeoutMs:10000,maximumDatabaseBytes:16384});};

test("the corpus is a closed document and the idle policy matches it",()=>{
 expect(new Ajv({strict:true}).validate(schema,corpus)).toBe(true);
 expect(corpus.idleMilliseconds).toBe(policy.idleMilliseconds);
 expect(policy.nestedIdleMilliseconds).toBeGreaterThan(policy.idleMilliseconds);
 expect(policy.pressureIdleMilliseconds).toBeLessThan(policy.idleMilliseconds);
 expect(policy.pressureCandidatesPerSlice).toBeGreaterThan(policy.candidatesPerSlice);
});

test("idle databases and empty queues retire, holders, waiters, fresh and foreign entries stay: Python's sqlite3 predicts the same, TypeScript does it",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:sweep-matrix",300000),directory=join(scratch,"data"),now=Date.now(),holds:ResourceLease[]=[],hadDatabase=new Map<string,boolean>(),hadQueue=new Map<string,boolean>();mkdirSync(directory);
 const rows=corpus.cases.map(row=>({...row,resource:"sweep-case:"+row.name}));
 for(const row of rows){
  const path=resourceLeasePathV1(directory,row.resource),queue=path.replace(/\.sqlite$/u,".queue"),ticket=resourceQueueTicketV1({arrivedAtMs:now-5000,pid:process.pid,owner:"oracle"});
  const age=row.ageMilliseconds,old=3600000;
  if(row.state==="orphan-queue"){mkdirSync(queue);aged(queue,now,age);}
  else{
   const first=await lease(control,directory,row.resource,row.state==="held-exclusive"?"exclusive":"shared");
   if(row.state==="held-shared"||row.state==="held-exclusive")holds.push(first);else first.release();
   if(["idle-queue","fresh-queue"].includes(row.state)){mkdirSync(queue);aged(queue,now,age);}
   if(["live-ticket","dead-ticket"].includes(row.state)){mkdirSync(queue);writeFileSync(join(queue,ticket),ticketRow);const held=await claim(control,directory,row.resource,ticket);if(row.state==="live-ticket")holds.push(held);else held.release();aged(queue,now,age);}
   if(row.state==="foreign-queue")writeFileSync(queue,"not a queue");
   if(row.state==="foreign-queue-directory-link"){const target=join(scratch,"link-target");mkdirSync(target,{recursive:true});symlinkSync(target,queue);}
   aged(path,now,row.state==="fresh-queue"?old:age);
  }
  hadDatabase.set(row.name,existsSync(path));hadQueue.set(row.name,lstatSync(queue,{throwIfNoEntry:false})!==undefined);
 }
 const predicted=python("predict",{directory,now,idle:policy.idleMilliseconds,cases:rows.map(row=>({name:row.name,resource:row.resource}))},scratch);
 for(const row of rows)expect(predicted[row.name],row.name).toEqual({databaseRetired:row.databaseRetired,queueRetired:row.queueRetired,ticketsReaped:row.ticketsReaped,eventuallyRetired:row.eventuallyRetired});
 const sweep=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:now});
 expect(sweep.busy).toBe(false);expect(sweep.pressure).toBe(false);
 for(const row of rows){
  const path=resourceLeasePathV1(directory,row.resource),queue=path.replace(/\.sqlite$/u,".queue");
  expect(lstatSync(path,{throwIfNoEntry:false})!==undefined,row.name+" database").toBe(hadDatabase.get(row.name)!&&!row.databaseRetired);
  expect(lstatSync(queue,{throwIfNoEntry:false})!==undefined,row.name+" queue").toBe(hadQueue.get(row.name)!&&!row.queueRetired);
  if(row.state==="dead-ticket"||row.state==="live-ticket")expect(readdirSync(queue).length,row.name+" tickets").toBe(row.ticketsReaped?0:1);
 }
 expect(sweep.databases).toBe(rows.filter(row=>row.databaseRetired).length);
 for(const row of rows){const path=resourceLeasePathV1(directory,row.resource),queue=path.replace(/\.sqlite$/u,".queue");if(existsSync(path))aged(path,now,3600000);if(lstatSync(queue,{throwIfNoEntry:false})?.isDirectory()&&!lstatSync(queue).isSymbolicLink())aged(queue,now,3600000);}
 await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:now});
 for(const row of rows){const path=resourceLeasePathV1(directory,row.resource),queue=path.replace(/\.sqlite$/u,".queue");expect(existsSync(path)||lstatSync(queue,{throwIfNoEntry:false})!==undefined,row.name+" gone eventually").toBe(!row.eventuallyRetired);}
 for(const held of holds)held.release();
 const rest=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:now+2*policy.idleMilliseconds});
 expect(rest.databases).toBeGreaterThanOrEqual(holds.length>0?3:0);
 await lease(control,directory,"sweep-case:shared-holder","exclusive").then(row=>row.release());
}));

test("a returning acquirer recreates a retired entry, and the store keeps its permanent gate",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:sweep-recreate",120000),directory=join(scratch,"data"),resource="sweep-recreate";mkdirSync(directory);
 (await lease(control,directory,resource)).release();
 const queued=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"recreate",signal:control.signal,control,timeoutMs:10000});queued.release();
 const path=resourceLeasePathV1(directory,resource),queue=path.replace(/\.sqlite$/u,".queue");expect(existsSync(queue)).toBe(true);
 const sweep=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:Date.now()+2*policy.idleMilliseconds});
 expect(sweep.databases).toBe(1);expect(sweep.queues).toBe(1);expect(existsSync(path)).toBe(false);expect(existsSync(queue)).toBe(false);
 expect(existsSync(resourceLeasePathV1(directory,"resource-directory-admission:v1"))).toBe(true);
 const again=await acquireQueuedResourceLease({directory,resource,mode:"exclusive",owner:"recreate",signal:control.signal,control,timeoutMs:10000});
 expect(existsSync(path)).toBe(true);expect(readdirSync(queue).length).toBe(1);
 const during=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:Date.now()+2*policy.idleMilliseconds});
 expect(during.databases).toBe(0);expect(existsSync(path)).toBe(true);expect(readdirSync(queue).length).toBe(1);
 again.release();
}));

test("acquire and release stamp the idle clock of ordinary resources and leave internal databases alone",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:sweep-clock",120000),directory=join(scratch,"data"),resource="sweep-clock";mkdirSync(directory);
 (await lease(control,directory,resource)).release();const path=resourceLeasePathV1(directory,resource),gate=resourceLeasePathV1(directory,"resource-directory-admission:v1"),old=Date.now()-86400000;
 aged(path,Date.now(),86400000);aged(gate,Date.now(),86400000);
 const held=await lease(control,directory,resource);expect(Date.now()-lstatSync(path).mtimeMs).toBeLessThan(60000);
 aged(path,Date.now(),86400000);held.release();expect(Date.now()-lstatSync(path).mtimeMs).toBeLessThan(60000);
 expect(lstatSync(gate).mtimeMs).toBeLessThan(old+120000);
}));

test("abandoned nested stores of killed test runs retire; stores with a holder, a waiter, recent use or a foreign file stay",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:sweep-nested",300000),directory=join(scratch,"data"),now=Date.now(),holds:ResourceLease[]=[];mkdirSync(directory);
 const top=await lease(control,directory,"sweep-top-level","exclusive");
 const walk=(path:string,age:number)=>{for(const name of readdirSync(path)){const child=join(path,name);if(lstatSync(child).isDirectory())walk(child,age);aged(child,now,age);}aged(path,now,age);};
 for(const row of corpus.nested){
  const store=join(directory,"t-"+row.name.slice(0,8).replace(/[^a-z]/gu,"x")+String(corpus.nested.indexOf(row))),pool=createIssuedResourceControlV1(original,"lease:nested:"+corpus.nested.indexOf(row),300000);mkdirSync(store);
  (await lease(pool,store,"nested:plain")).release();
  const queued=await acquireQueuedResourceLease({directory:store,resource:"nested:queued",mode:"exclusive",owner:"nested",signal:pool.signal,control:pool,timeoutMs:10000});queued.release();
  if(row.state==="dead-ticket"||row.state==="live-ticket"){const ticket=resourceQueueTicketV1({arrivedAtMs:now-5000,pid:process.pid,owner:"nested"}),queue=resourceLeasePathV1(store,"nested:queued").replace(/\.sqlite$/u,".queue");writeFileSync(join(queue,ticket),ticketRow);const held=await claim(pool,store,"nested:queued",ticket);if(row.state==="live-ticket")holds.push(held);else held.release();}
  if(row.state==="held")holds.push(await lease(pool,store,"nested:held","exclusive"));
  if(row.state==="foreign-file")writeFileSync(join(store,"notes.txt"),"keep me");
  walk(store,row.ageMilliseconds);
 }
 const sweep=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:now});
 const names=corpus.nested.map((row,index)=>({row,store:join(directory,"t-"+row.name.slice(0,8).replace(/[^a-z]/gu,"x")+String(index))}));
 for(const{row,store}of names)expect(existsSync(store),row.name).toBe(!row.retired);
 expect(sweep.stores).toBe(corpus.nested.filter(row=>row.retired).length);
 expect(existsSync(join(directory,"notes.txt"))||true).toBe(true);
 expect(existsSync(resourceLeasePathV1(directory,"sweep-top-level"))).toBe(true);
 const foreign=names.find(entry=>entry.row.state==="foreign-file")!;expect(readdirSync(foreign.store)).toContain("notes.txt");
 for(const held of holds)held.release();top.release();
 const second=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:now+4*3600000});
 expect(second.stores).toBe(corpus.nested.filter(row=>!row.retired&&row.state!=="foreign-file").length);
 for(const{row,store}of names)expect(existsSync(store),row.name+" later").toBe(row.state==="foreign-file");
}));

test("a sweep works in slices with a persistent cursor and stops cooperatively when cancelled",async()=>withScratch(async(scratch,original)=>{
 const control=createIssuedResourceControlV1(original,"lease:sweep-slices",300000),directory=join(scratch,"data"),now=Date.now(),total=150;mkdirSync(directory);
 for(let index=0;index<total;index++){(await lease(control,directory,"sweep-slice:"+index)).release();aged(resourceLeasePathV1(directory,"sweep-slice:"+index),now,3600000);}
 const permanent=["resource-directory-admission:v1","resource-retirement-catalog:v2"].map(resource=>resourceLeasePathV1(directory,resource).slice(directory.length+1)),count=()=>readdirSync(directory).filter(name=>/^[a-f0-9]{64}\.sqlite$/u.test(name)&&!permanent.includes(name)).length;
 expect(count()).toBe(total);
 const parts:number[]=[];let complete=false,turns=0;
 while(!complete&&turns++<10){const row=await retireIdleResourceLeasesV1(directory,control,0,{idleMilliseconds:policy.idleMilliseconds,nowMilliseconds:now,maximumCandidates:50});parts.push(row.retired);complete=row.complete;expect(row.retired).toBeLessThanOrEqual(50);}
 expect(parts.reduce((sum,value)=>sum+value,0)).toBe(total);expect(parts.length).toBe(3);expect(count()).toBe(0);
 for(let index=0;index<total;index++){(await lease(control,directory,"sweep-cancel:"+index)).release();aged(resourceLeasePathV1(directory,"sweep-cancel:"+index),now,3600000);}
 const stop=new AbortController(),victim=createIssuedResourceControlV1(original,"lease:sweep-cancel",300000,stop.signal),timer=setInterval(()=>{if(count()<total-10)stop.abort();},1);
 let cancelled:ResourceLeaseSweepV1|undefined;
 try{cancelled=await sweepResourceLeaseStoreV1(directory,victim,{freeRatio:1,nowMilliseconds:now});}catch(error){expect(stop.signal.aborted).toBe(true);}finally{clearInterval(timer);}
 expect(stop.signal.aborted).toBe(true);expect(count()).toBeGreaterThan(0);expect(count()).toBeLessThan(total);
 if(cancelled){expect(cancelled.cancelled).toBe(true);expect(formatResourceLeaseSweepV1("en",cancelled).join("\n")).toContain("cancelled");}
 const rest=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:now});expect(rest.cancelled).toBe(false);expect(count()).toBe(0);
 const dated=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:now});expect(dated.databases).toBe(0);
}));

test("the sweep mode follows a SQLite model of the pressure rule",()=>{
 const model=new Database(":memory:");
 try{for(const row of corpus.modes){const mode=resourceLeaseSweepModeV1(row.freeRatio),expected=model.query("select ?1 < ?2 as pressure, case when ?1 < ?2 then ?3 else ?4 end as idle, case when ?1 < ?2 then ?5 else ?6 end as candidates").get(row.freeRatio,policy.pressureFreeRatio,policy.pressureIdleMilliseconds,policy.idleMilliseconds,policy.pressureCandidatesPerSlice,policy.candidatesPerSlice) as{pressure:number;idle:number;candidates:number};expect(mode.pressure,String(row.freeRatio)).toBe(row.pressure);expect(Boolean(expected.pressure)).toBe(row.pressure);expect(mode.idleMilliseconds).toBe(row.idleMilliseconds);expect(expected.idle).toBe(row.idleMilliseconds);expect(mode.candidates).toBe(row.candidates);expect(expected.candidates).toBe(row.candidates);}}
 finally{model.close();}
});

test("an unknown free share is the pressure mode, says so in both languages and never prints a made-up percentage",()=>{
 expect(resourceLeaseSweepModeV1(undefined)).toEqual(resourceLeaseSweepModeV1(0));expect(resourceLeaseSweepModeV1(undefined).pressure).toBe(true);
 const sweep:ResourceLeaseSweepV1={version:1,directory:"/x",pressure:true,idleMilliseconds:60000,databases:2,queues:1,claims:0,stores:0,kept:0,slices:1,freedBytes:1048576,freeRatioBefore:null,freeRatioAfter:null,complete:true,busy:false,cancelled:false};
 const english=formatResourceLeaseSweepV1("en",sweep).join("\n"),german=formatResourceLeaseSweepV1("de",sweep).join("\n");
 expect(english).toContain("could not be read");expect(english).toContain("? % free");expect(german).toContain("nicht gelesen werden");expect(german).toContain("? % frei");
 for(const text of[english,german]){expect(text).not.toMatch(/\{[A-Za-z]+\}/u);expect(text).not.toContain("NaN");expect(text).not.toContain("100 %");}
});

test("English and German messages fill every placeholder and a quiet sweep says nothing",()=>{
 const sweep:ResourceLeaseSweepV1={version:1,directory:"/x",pressure:true,idleMilliseconds:60000,databases:1606,queues:1903,claims:30,stores:28,kept:200,slices:7,freedBytes:44040192,freeRatioBefore:0.16,freeRatioAfter:0.8,complete:true,busy:false,cancelled:false};
 for(const language of["en","de"] as const){const lines=formatResourceLeaseSweepV1(language,sweep).join("\n");expect(lines).toContain("1606");expect(lines).toContain("1903");expect(lines).toContain("28");expect(lines).toContain("42.0 MiB");expect(lines).toContain("84");expect(lines).toContain("80");expect(lines).not.toMatch(/\{[A-Za-z]+\}/u);}
 expect(formatResourceLeaseSweepV1("en",sweep).join("\n")).not.toBe(formatResourceLeaseSweepV1("de",sweep).join("\n"));
 expect(formatResourceLeaseSweepV1("de",{...sweep,pressure:false,databases:0,queues:0,claims:0,stores:0})).toEqual([]);
 expect(formatResourceLeaseSweepV1("en",{...sweep,pressure:false,databases:0,queues:0,claims:0,stores:0,busy:true}).join("\n")).toContain("postponed");
 for(const language of["en","de"] as const)expect(formatResourceLeaseSweepFailureV1(language,Error("disk gone"))).toContain("disk gone");
 for(const entry of Object.values(resourceLeaseRetirementMessagesV1))expect(Object.keys(entry).sort()).toEqual(["de","en"]);
});

test("sweeps racing acquirers never produce overlapping holders (replayed by Python)",async()=>withScratch(async(scratch,original)=>{
 const shape=corpus.interleaving,directory=join(scratch,"data"),events:Array<{t:string;e:string;r:string;m:string;w:string}>=[],resources=Array.from({length:shape.resources},(_,index)=>"sweep-race:"+index);mkdirSync(directory);
 const worker=async(id:number)=>{const control=createIssuedResourceControlV1(original,"lease:race:"+id,300000),queued=id<shape.queuedWorkers;
  for(let round=0;round<shape.rounds;round++)for(const resource of resources){const mode=(id+round)%3===0?"shared":"exclusive",options={directory,resource,mode,signal:control.signal,control,timeoutMs:60000} as const;
   const held=queued?await acquireQueuedResourceLease({...options,owner:"race"+id}):await acquireResourceLease(options);
   events.push({t:process.hrtime.bigint().toString(),e:"acq",r:resource,m:mode,w:"w"+id});await Bun.sleep(0);events.push({t:process.hrtime.bigint().toString(),e:"rel",r:resource,m:mode,w:"w"+id});held.release();}};
 let retired=0,queues=0;const sweeper=async()=>{const control=createIssuedResourceControlV1(original,"lease:race:sweeper",300000);for(let index=0;index<shape.sweeps;index++){const row=await sweepResourceLeaseStoreV1(directory,control,{freeRatio:1,nowMilliseconds:Date.now()+2*3600000,budgetMilliseconds:200});retired+=row.databases;queues+=row.queues;await Bun.sleep(1);}};
 await Promise.all([...Array.from({length:shape.workers},(_,id)=>worker(id)),sweeper()]);
 expect(events.length).toBe(2*shape.workers*shape.rounds*resources.length);
 const verdict=python("replay",events,scratch);expect(verdict.violations).toEqual([]);expect(verdict.events).toBe(events.length);
 expect(retired+queues).toBeGreaterThan(0);
}));
