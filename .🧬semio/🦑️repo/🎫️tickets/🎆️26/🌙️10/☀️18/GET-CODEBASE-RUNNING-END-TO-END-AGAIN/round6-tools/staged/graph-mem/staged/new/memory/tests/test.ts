import{test,expect}from"bun:test";
import{mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{fileURLToPath}from"node:url";
import{join}from"node:path";
import{heapStats}from"bun:jsc";
import Ajv from"ajv";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import fixtureSchema from"./🧬️schema/🔣️.json";
import fixture from"./🔣️.json";
import moduleSchema from"../🧬️schema/🔣️.json";
import*as memory from"../🟨️.mjs";

const MEGABYTE=1048576;
const oraclePath=fileURLToPath(new URL("./🐍️oracle.py",import.meta.url)),python=testPythonV1();

/** 🐍️ Runs the independent Python model and returns its JSON answer. */
function oracle(args:string[]):any{
 const done=Bun.spawnSync([python!,"-I",oraclePath,...args]);
 if(done.exitCode!==0)throw Error("Oracle failed: "+new TextDecoder().decode(done.stderr));
 return JSON.parse(new TextDecoder().decode(done.stdout));
}

/** 📄️ Writes a JSON document the oracle reads. */
function written(directory:string,name:string,value:unknown):string{
 const path=join(directory,name);writeFileSync(path,JSON.stringify(value));return path;
}

test("the fixture is closed, the policy and the texts equal their declaration",()=>{
 const ajv=new Ajv({strict:false});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 expect(fixture.policy).toEqual(moduleSchema.definitions.Policy.const);
 expect(fixture.messages).toEqual(moduleSchema.definitions.Messages.const);
 expect(JSON.parse(JSON.stringify(memory.GRAPH_MEMORY_POLICY_V1))).toEqual(moduleSchema.definitions.Policy.const);
 expect(JSON.parse(JSON.stringify(memory.GRAPH_MEMORY_MESSAGES_V1))).toEqual(moduleSchema.definitions.Messages.const);
 for(const key of Object.keys(moduleSchema.definitions.Messages.const.en))expect(Object.keys(moduleSchema.definitions.Messages.const.de)).toContain(key);
 const holders=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]!).sort();
 for(const key of Object.keys(moduleSchema.definitions.Messages.const.en) as ("waiting"|"serial"|"resumed")[]){
  const english=holders(moduleSchema.definitions.Messages.const.en[key]),german=holders(moduleSchema.definitions.Messages.const.de[key]);
  expect(german,key).toEqual(english);for(const name of english)expect(["budget","used","running"],key).toContain(name);
 }
});

test("the admission decision equals the recorded verdicts and the independent model",()=>{
 const directory=mkdtempSync(join(tmpdir(),"memory-law-"));
 try{
  const expected=oracle(["expect",written(directory,"fixture.json",fixture)]);
  fixture.admission.forEach((row,index)=>{
   const actual=memory.decideMemoryAdmissionV1(memory.GRAPH_MEMORY_POLICY_V1,row.state);
   expect(actual,row.name).toEqual(row.expect as never);
   expect(actual,row.name).toEqual(expected.admission[index]);
  });
  fixture.budgets.forEach((row,index)=>{
   const run=()=>memory.graphMemoryBudgetBytesV1(row.environment);
   if("error" in row.expect){expect(run,row.name).toThrow();expect(expected.budgets[index],row.name).toEqual({error:true});}
   else{expect(run(),row.name).toBe(row.expect.bytes);expect(expected.budgets[index],row.name).toEqual({bytes:row.expect.bytes});}
  });
  fixture.languages.forEach((row,index)=>{expect(memory.selectGraphMemoryLanguagesV1(row.environment),row.name).toEqual(row.expect as ("en"|"de")[]);expect(expected.languages[index],row.name).toEqual(row.expect);});
  fixture.messageCases.forEach((row,index)=>{
   const actual=memory.renderGraphMemoryMessagesV1(row.key as "waiting",row.fields as Record<string,number>,row.environment);
   expect(actual).toEqual(row.expect);expect(actual).toEqual(expected.messages[index]);
  });
  expect(()=>memory.renderGraphMemoryMessagesV1("unknown" as "waiting",{},{LANG:"de_DE"})).toThrow();
  expect(Object.isFrozen(memory.GRAPH_MEMORY_POLICY_V1)&&Object.isFrozen(memory.GRAPH_MEMORY_MESSAGES_V1.de)).toBe(true);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

/** ⏱️ A deterministic clock: timers fire in time order, every firing is followed by a full flush of the pending promise work. */
class FakeTime{
 clock=0;timers:{at:number;sequence:number;fire:()=>void}[]=[];sequence=0;
 now=()=>this.clock;
 sleep=(milliseconds:number,signal?:AbortSignal)=>new Promise<void>((resolve,reject)=>{
  if(signal?.aborted){reject(signal.reason);return;}
  const timer={at:this.clock+milliseconds,sequence:this.sequence++,fire:()=>{signal?.removeEventListener("abort",abort);resolve();}},abort=()=>{this.timers=this.timers.filter(row=>row!==timer);reject(signal!.reason);};
  signal?.addEventListener("abort",abort,{once:true});this.timers.push(timer);
 });
 at(milliseconds:number,action:()=>void){this.timers.push({at:milliseconds,sequence:this.sequence++,fire:action});}
 async settle(){for(let round=0;round<8;round++){await Promise.resolve();await new Promise<void>(done=>setImmediate(done));}}
 async drive(done:()=>boolean){
  await this.settle();
  while(!done()){
   this.timers.sort((a,b)=>a.at-b.at||a.sequence-b.sequence);
   const next=this.timers.shift();if(!next)throw Error("Deadlock: no timer left and the scenario is not finished");
   this.clock=Math.max(this.clock,next.at);next.fire();await this.settle();
  }
 }
}

/** 🧪️ Runs one scenario of the fixture against the real governor over a modeled heap and returns the recorded trace. */
async function run(scenario:typeof fixture.governor[number]){
 const time=new FakeTime(),controller=new AbortController(),language=(scenario as {language?:string}).language,environment:Record<string,string>=language==="none"?{}:{LANG:language==="de"?"de_DE.UTF-8":"en_US.UTF-8"};
 let live=0,garbage=0,running=0,collections=0,finishedCount=0,rejected=0;
 const base=scenario.baseMegabytes*MEGABYTE,sample=()=>base+live+garbage,collect=()=>{garbage=0;collections++;};
 const lines:{at:number;text:string}[]=[],starts:{task:number;at:number;runningBefore:number;usedBytes:number}[]=[],finished:{task:number;at:number}[]=[];
 const governor=memory.createMemoryGovernorV1({budgetBytes:scenario.budgetMegabytes*MEGABYTE,workers:scenario.workers,sample,collect,sleep:time.sleep,now:time.now,signal:controller.signal,report:text=>lines.push({at:time.clock,text}),environment});
 if(scenario.abortAtMilliseconds!==undefined)time.at(scenario.abortAtMilliseconds,()=>controller.abort(Error("scenario abort")));
 const results=scenario.tasks.map((task,index)=>governor.run(async()=>{
  starts.push({task:index,at:time.clock,runningBefore:running,usedBytes:sample()});running++;live+=task.memoryMegabytes*MEGABYTE;
  await time.sleep(task.durationMilliseconds);
  live-=task.memoryMegabytes*MEGABYTE;garbage+=Math.round(task.memoryMegabytes*MEGABYTE*scenario.garbageShare);running--;finished.push({task:index,at:time.clock});finishedCount++;return index;
 }).catch(error=>{rejected++;return error;}));
 await time.drive(()=>finishedCount+rejected>=scenario.tasks.length);
 const settled=await Promise.all(results);
 const keyOf=(text:string)=>(["waiting","serial","resumed"] as const).find(key=>(["en","de"] as const).some(language=>new RegExp("^"+memory.GRAPH_MEMORY_MESSAGES_V1[language][key].replace(/[.*+?^${}()|[\]\\]/gu,"\\$&").replace(/\\\{[a-z]+\\\}/gu,".*")+"$","u").test(text)))??"unknown";
 const all=lines.map(row=>({at:row.at,key:keyOf(row.text),text:row.text})),unique=all.filter((row,index)=>all.findIndex(other=>other.at===row.at&&other.key===row.key)===index);
 return{scenario:scenario.name,starts,finished,completed:settled.filter(value=>typeof value==="number"),rejected,expectRejected:scenario.abortAtMilliseconds===undefined?0:scenario.tasks.length-scenario.workers,collections,durationMilliseconds:time.clock,messages:unique,lines:lines.map(row=>row.text),stats:governor.stats()};
}

test("the governor keeps the laws of every scenario, checked by the independent model",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"memory-trace-"));
 try{
  const fixturePath=written(directory,"fixture.json",fixture);
  for(const scenario of fixture.governor){
   const trace=await run(scenario);
   if(process.env.MEMORY_LAW_TRACE===scenario.name)console.log(JSON.stringify(trace,null,1));
   expect(oracle(["trace",fixturePath,written(directory,"trace.json",trace)]),scenario.name).toEqual([]);
   if(scenario.name==="progress-lines-are-spaced"){
    expect(trace.messages.filter(row=>row.key==="waiting").length).toBeGreaterThanOrEqual(2);
    for(const row of trace.messages)expect(row.text.startsWith("Graph: ")).toBe(true);
   }
   if(scenario.name==="unconstrained")expect(trace.collections).toBe(0);
   if(scenario.name==="unresolved-language-prints-both"){
    expect(trace.messages.length).toBeGreaterThan(0);expect(trace.lines.length).toBe(2*trace.messages.length);
    trace.lines.forEach((line,index)=>expect(line.startsWith(index%2===0?"graph: ":"Graph: "),line).toBe(true));
   }
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the governor refuses a missing worker count or budget and rejects every waiter on abort",async()=>{
 expect(()=>memory.createMemoryGovernorV1({budgetBytes:MEGABYTE,workers:0})).toThrow();
 expect(()=>memory.createMemoryGovernorV1({budgetBytes:0,workers:1})).toThrow();
 const controller=new AbortController();controller.abort(Error("stopped"));
 const governor=memory.createMemoryGovernorV1({budgetBytes:MEGABYTE,workers:1,sample:()=>0,collect:()=>undefined,signal:controller.signal});
 await expect(governor.run(()=>1)).rejects.toThrow("stopped");
});

test("interning keeps the JSON bytes, shares equal long strings and never touches keys or short strings",()=>{
 const interner=memory.createStringInternerV1({minimumLength:8});
 const source={"a-long-key-that-is-never-shared":["/Users/x/semio/path/one.ts","/Users/x/semio/path/one.ts","short"],nested:{list:[{value:"/Users/x/semio/path/one.ts",number:7,flag:true,none:null}],text:"🧰️framework/📦️packages/🟦️typescript"},more:["🧰️framework/📦️packages/🟦️typescript"]};
 const text=JSON.stringify(source),interned=memory.internJsonStringsV1(JSON.parse(text),interner);
 expect(JSON.stringify(interned)).toBe(text);
 expect(interner.stats().hits).toBe(3);expect(interner.stats().entries).toBe(2);
 const bounded=memory.createStringInternerV1({maximumCharacters:40,minimumLength:8});
 for(let index=0;index<10;index++)bounded.share("0123456789".repeat(2)+index);
 expect(bounded.stats().clears).toBeGreaterThan(0);expect(bounded.stats().characters).toBeLessThanOrEqual(40);
 expect(bounded.share("abc")).toBe("abc");
});

/** 📏️ The live managed bytes after a full collection. */
function managed():number{Bun.gc(true);const statistics=heapStats();return statistics.heapSize+statistics.extraMemorySize;}

test("interned results cost a fraction of the memory of the same parsed results",()=>{
 const long="{workspaceRoot}/"+"🧰️framework/📦️packages/🟦️typescript/".repeat(8)+"file.ts",text=JSON.stringify({inputs:Array.from({length:20},(_,index)=>long+"?"+index%5),targets:{build:{inputs:[long]}}});
 const retain=(interner?:ReturnType<typeof memory.createStringInternerV1>)=>Array.from({length:2000},()=>{const value=JSON.parse(text);return interner?memory.internJsonStringsV1(value,interner):value;});
 const before=managed(),plain=retain(),plainBytes=managed()-before;
 const afterPlain=managed(),shared=retain(memory.createStringInternerV1()),sharedBytes=managed()-afterPlain;
 expect(plain.length).toBe(shared.length);
 expect(plainBytes).toBeGreaterThan(20*MEGABYTE);
 expect(sharedBytes).toBeLessThan(plainBytes/4);
});
