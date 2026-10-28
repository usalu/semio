/** 🪆️ Contract: a farm that runs from a captured snapshot starts its workers from that same snapshot (read as descriptor 3, the channel moves to descriptor 4), so a worker reports the captured implementation of its parent and is adopted; a worker of another implementation or another record identity is refused; the shared verb kernel entry serves the worker role without an invocation; a worker recycled right after its job leaves its staged leaves for the parent. Independent oracles: the digest Python computes from the snapshot file, and the process ids and replies of real worker processes. */
import{afterAll,expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{createHash}from"node:crypto";
import{existsSync,fstatSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{basename,dirname,join,relative,sep}from"node:path";
import{fileURLToPath,pathToFileURL}from"node:url";
import{serialize}from"node:v8";
import{testPythonV1}from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{GRAPH_FARM_WORKER_ROLE_V1,graphFarmIdentityDifferenceV1,graphFarmIdentityV1,sameGraphFarmIdentityV1}from"../../🪆️captured/🟨️.mjs";
import{createFarmV1,GRAPH_FARM_POLICY_V1}from"../../🏊️pool/🟨️.mjs";
import{openProjectFarmV1}from"../../🟨️.mjs";
import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";
import{buildSourceClosureStreamLoaderV1}from"../../../../../../📥️source/🔏️closure/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),farmDirectory=join(here,"..",".."),fixtures=join(here,"..","🧫️fixtures"),python=testPythonV1();
const root=(()=>{let at=here;while(basename(at)!=="🧰️framework")at=dirname(at);return dirname(at);})(),rel=(file:string)=>relative(root,file).split(sep).join("/");
const role=join(farmDirectory,"🪆️captured","🟨️.mjs"),worker=join(farmDirectory,"👷️worker","🟨️.mjs"),entry=join(fixtures,"🪆️captured-entry","🟨️.mjs"),service=join(fixtures,"🪆️captured-service","🟨️.mjs"),kernelRole=join(fixtures,"🪆️kernel-role","🟦️.ts"),plainService=pathToFileURL(join(fixtures,"🛰️service","🟨️.mjs")).href;
const areas:string[]=[],area=()=>{const at=mkdtempSync(join(realpathSync(tmpdir()),"graph-farm-captured-"));areas.push(at);return at;};
afterAll(()=>{for(const at of areas)rmSync(at,{recursive:true,force:true});});
const sha=(data:string|Buffer)=>createHash("sha256").update(data).digest("hex");
const pythonDigest=(file:string)=>Bun.spawnSync([python!,"-I","-c","import hashlib,sys;print(hashlib.sha256(open(sys.argv[1],'rb').read()).hexdigest())",file]).stdout.toString().trim();

/** 📦️ Writes the canonical snapshot of the role module, the real worker loop, the stand-in entry and the stand-in service, and answers its file and the digest Python computes for it. */
const snapshot=()=>{
 const row=(file:string,imports:Record<string,string>)=>{const bytes=readFileSync(file);return{path:rel(file),sha256:sha(bytes),base64:bytes.toString("base64"),imports};};
 const files=[row(entry,{"../../../🪆️captured/🟨️.mjs":rel(role),"../../../👷️worker/🟨️.mjs":rel(worker)}),row(service,{"../../../🪆️captured/🟨️.mjs":rel(role)}),row(role,{"../👷️worker/🟨️.mjs":rel(worker)}),row(worker,{})].sort((a,b)=>Buffer.compare(Buffer.from(a.path),Buffer.from(b.path)));
 const file=join(area(),"snapshot.json");writeFileSync(file,JSON.stringify({version:1,root,entry:rel(entry),files}),{mode:0o600});
 return{file,digest:pythonDigest(file)};
};
const capturedPool=(image:{file:string;digest:string},own:ReturnType<typeof graphFarmIdentityV1>,options:Record<string,unknown>={})=>createFarmV1({entry:GRAPH_FARM_WORKER_ROLE_V1,service:pathToFileURL(service).href,session:{records:"law-records"},workers:2,megabytes:100000,environment:{...process.env,BUN_RUNTIME_TRANSPILER_CACHE_PATH:"0"},command:process.execPath,commandArguments:["--eval",buildSourceClosureStreamLoaderV1,"--",image.digest],auxiliaryFile:image.file,accept:(identity:unknown)=>sameGraphFarmIdentityV1(own,identity),...options} as never);

test("the farm identity is the captured implementation and the record identity, and only an equal pair is accepted",()=>{
 const own=graphFarmIdentityV1("r","a".repeat(64));
 expect(own).toEqual({implementation:"a".repeat(64),records:"r"});
 expect(graphFarmIdentityV1("r")).toEqual({implementation:null,records:"r"});
 expect(sameGraphFarmIdentityV1(own,{implementation:"a".repeat(64),records:"r"})).toBe(true);
 for(const offered of[{implementation:"b".repeat(64),records:"r"},{implementation:"a".repeat(64),records:"s"},{implementation:null,records:"r"},"r",null,undefined])expect(sameGraphFarmIdentityV1(own,offered),JSON.stringify(offered)).toBe(false);
 expect([{implementation:"a".repeat(64),records:"r"},{implementation:"b".repeat(64),records:"r"},{implementation:"a".repeat(64),records:"s"},{implementation:null,records:"s"},"r"].map(offered=>graphFarmIdentityDifferenceV1(own,offered))).toEqual(["","implementation","records","implementation, records","unreadable"]);
});

test("workers started from the parent's snapshot through the real loader report its captured implementation, are adopted and run every job in their own processes",async()=>{
 const image=snapshot(),pool=capturedPool(image,graphFarmIdentityV1("law-records",image.digest));
 try{
  expect(await pool.ready).toBe(true);
  const outcomes=await Promise.all([1,2,3,4,5,6].map(n=>pool.run({n})));
  for(const[index,outcome]of outcomes.entries()){const reply=outcome.reply as{n:number;pid:number;implementation:string};expect(reply,JSON.stringify(outcome)).toBeDefined();expect(reply.n).toBe(index+1);expect(reply.implementation).toBe(image.digest);expect(reply.pid).not.toBe(process.pid);}
  expect(new Set(outcomes.map(outcome=>(outcome.reply as{pid:number}).pid)).size).toBeLessThanOrEqual(2);
  expect(pool.stats().disabled).toBeUndefined();
 }finally{await pool.close();}
},120000);

test("a worker of another implementation or another record identity is refused, and so is a worker that loaded its modules from their files",async()=>{
 const image=snapshot();
 for(const own of[graphFarmIdentityV1("law-records","c".repeat(64)),graphFarmIdentityV1("other-records",image.digest)]){
  const reasons:string[]=[],pool=capturedPool(image,own,{report:(event:string,fields:{reason:string})=>{if(event==="disabled")reasons.push(fields.reason);}});
  try{expect(await pool.ready).toBe(false);expect(reasons).toEqual(["identity differs"]);expect((await pool.run({n:1})).lost).toBe("disabled: identity differs");}finally{await pool.close();}
 }
 const reasons:string[]=[],physical=createFarmV1({entry:worker,service:pathToFileURL(service).href,session:{records:"law-records"},workers:1,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>sameGraphFarmIdentityV1(graphFarmIdentityV1("law-records",image.digest),identity),report:(event:string,fields:{reason:string})=>{if(event==="disabled")reasons.push(fields.reason);}} as never);
 try{expect(await physical.ready).toBe(false);expect(reasons).toEqual(["identity differs"]);}finally{await physical.close();}
},120000);

/** 🧪️ A scripted child that announces the identity it is given and answers every job with an empty reply. */
class ScriptedChild extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly identity:unknown){super();}
 send(message:{type:string;id?:number}){
  if(message.type==="init")setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:this.identity}));
  else if(message.type==="run")setImmediate(()=>this.emit("message",{type:"result",id:message.id,reply:{names:[],cpuMilliseconds:1},rssMegabytes:1}));
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

test("a farm in a captured process starts every worker with the snapshot as descriptor 3, the channel as descriptor 4 and the worker role, adopts its implementation, refuses a worker from files and leaves no snapshot behind",async()=>{
 const implementation="d".repeat(64),launch=Object.freeze({command:"/semio-law/runtime",arguments:Object.freeze(["--eval","loader","--",implementation]),environment:Object.freeze({SEMIO_LAW_CAPTURED:"1"}),implementation,text:()=>"captured snapshot text"});
 const plan=JSON.stringify({version:1,id:"x",jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536}),job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
 const images=()=>new Set(readdirSync(realpathSync(tmpdir())).filter(name=>name.startsWith("semio-graph-farm-image-"))),before=images();
 for(const[offered,adopted]of[[graphFarmIdentityV1(inferenceRecordIdentityV1(),implementation),true],[graphFarmIdentityV1(inferenceRecordIdentityV1(),null),false]] as const){
  const spawned:{command:string;args:readonly string[];stdio:unknown[];env:Record<string,string>;text:string;mode:number}[]=[],lines:string[]=[];
  const farm=openProjectFarmV1({workspaceRoot:area(),context:{registry:null},projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_PLAN:plan},processed:()=>24,cores:10,write:(line:string)=>lines.push(line),launch,
   spawnProcess:((command:string,args:readonly string[],options:{stdio:unknown[];env:Record<string,string>})=>{const descriptor=options.stdio[3] as number;spawned.push({command,args,stdio:options.stdio,env:options.env,text:readFileSync(descriptor,"utf8"),mode:fstatSync(descriptor).mode&0o777});return new ScriptedChild(offered);}) as never,
   request:(()=>({granted:Promise.resolve(undefined),release:()=>undefined})) as never})!;
  for(let index=0;index<GRAPH_FARM_POLICY_V1.minimumProjects;index++)await farm.prepare(job("miss"+index));
  expect(spawned).toHaveLength(1);
  const[start]=spawned;
  expect(start!.command).toBe(launch.command);expect(start!.args).toEqual([...launch.arguments,GRAPH_FARM_WORKER_ROLE_V1]);
  expect(typeof start!.stdio[3]).toBe("number");expect(start!.stdio[4]).toBe("ipc");expect(start!.stdio).toHaveLength(5);
  expect(start!.text).toBe("captured snapshot text");if(process.platform!=="win32")expect(start!.mode).toBe(0o600);
  expect(start!.env.SEMIO_LAW_CAPTURED).toBe("1");expect(start!.env.SEMIO_GRAPH_WORKER).toBe("1");
  if(adopted){expect(await new Promise<number>(done=>farm.onReady(done))).toBe(GRAPH_FARM_POLICY_V1.loopsPerWorker);expect(await farm.prepare(job("first"))).toBe(false);}
  else{await Bun.sleep(50);expect(lines.some(line=>line.includes("(identity differs: implementation)"))).toBe(true);expect(await farm.prepare(job("refused"))).toBe(false);}
  await farm.close();
 }
 expect([...images()].filter(name=>!before.has(name))).toEqual([]);
},60000);

test("a worker that exits right after its job leaves its staged leaves to the parent: the real service never removes its staging, the parent does once the process is gone",async()=>{
 const base=area(),session={version:2,workspaceRoot:base,registry:null,commandInputs:null,lateClosures:false,projectsByRoot:[],componentRoots:[],stagingBase:base},gone:string[]=[];
 const pool=createFarmV1({entry:worker,service:pathToFileURL(join(farmDirectory,"👷️service","🟨️.mjs")).href,session,workers:1,megabytes:100000,environment:{...process.env},accept:()=>true} as never);
 expect(await pool.ready).toBe(true);await pool.close();await Bun.sleep(200);
 expect(existsSync(join(base,"w0-g1"))).toBe(true);
 const owned=createFarmV1({entry:worker,service:pathToFileURL(join(farmDirectory,"👷️service","🟨️.mjs")).href,session,workers:1,megabytes:100000,environment:{...process.env},accept:()=>true,retired:(slot:number,generation:number)=>{gone.push(slot+":"+generation);rmSync(join(base,"w"+slot+"-g"+generation),{recursive:true,force:true});}} as never);
 expect(await owned.ready).toBe(true);await owned.close();await Bun.sleep(200);
 expect(gone).toEqual(["0:1"]);expect(existsSync(join(base,"w0-g1"))).toBe(false);
},120000);

test("the shared verb kernel entry serves the graph farm worker role without an invocation and the worker answers over its channel",async()=>{
 const digest="e".repeat(64),messages:unknown[]=[],environment:Record<string,string|undefined>={...process.env};delete environment.SEMIO_SCRIPT_PROCESS_INVOCATION;
 const child=Bun.spawn([process.execPath,kernelRole,root,digest],{stdio:["ignore","inherit","inherit"],env:environment,ipc:message=>{messages.push(message);}});
 const session=join(area(),"session.bin");writeFileSync(session,serialize({identity:"kernel-role"}));
 child.send({type:"init",service:plainService,sessionFile:session,slot:0,generation:1});
 const started=Date.now();while(!messages.length&&Date.now()-started<60000&&child.exitCode===null)await Bun.sleep(50);
 expect(messages[0]).toEqual({type:"ready",pid:child.pid,identity:"kernel-role"});
 child.send({type:"shutdown"});
 expect(await child.exited).toBe(0);
},90000);
