/** 🏊️ Contract: closing the farm releases the grant of every pool, also when the close of an earlier pool fails; the failure is not swallowed, it is raised once every pool has been closed, released and removed. The oracle is the list of grants returned to the host and the staging directories left on disk. */
import{expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{pathToFileURL}from"node:url";
import{openProjectFarmV1}from"../../🟨️.mjs";
import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";
import{graphFarmIdentityV1}from"../../🪆️captured/🟨️.mjs";

class Child extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly hostile:boolean){super();}
 send(message:{type:string}){
  if(message.type==="init")setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 removeAllListeners(event?:string){if(this.hostile)throw Error("the pool cannot be closed");return super.removeAllListeners(event);}
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

test("a pool whose close fails does not keep the grant of a later pool: every grant is returned, the failure is raised afterwards",async()=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"farm-close-"))),children:Child[]=[],released:string[]=[],lines:string[]=[];
 let grant!:(value:{jobs:number;megabytes:number}|undefined)=>void;
 const before=new Set(readdirSync(realpathSync(tmpdir())).filter(name=>name.startsWith("semio-graph-farm-")));
 try{
  const plan=JSON.stringify({version:1,id:"x",jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536});
  const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_PLAN:plan},processed:()=>24,cores:10,write:(line:string)=>lines.push(line),
   spawnProcess:(()=>{const child=new Child(children.length===0);children.push(child);return child;}) as never,
   request:(()=>({granted:new Promise<{jobs:number;megabytes:number}|undefined>(done=>{grant=done;}),release:()=>{released.push("host");}})) as never})!;
  const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
  for(let index=0;index<24;index++)await farm.prepare(job("miss"+index));
  await new Promise<number>(done=>farm.onReady(done));
  grant({jobs:1,megabytes:3584});await Bun.sleep(50);
  await new Promise<number>(done=>farm.onReady((total:number)=>{if(total>0&&children.length===2)done(total);}));
  expect(children.length).toBe(2);expect(children[0]!.hostile).toBe(true);
  await expect(farm.close()).rejects.toThrow("the pool cannot be closed");
  expect(released,"the grant of the second pool is returned although the first pool failed to close").toEqual(["host"]);
  expect(children[1]!.exitCode).toBe(0);
  const left=readdirSync(realpathSync(tmpdir())).filter(name=>name.startsWith("semio-graph-farm-")&&!before.has(name)&&existsSync(join(realpathSync(tmpdir()),name)));
  expect(left,"no staging directory of the farm is left").toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

/** 🧟️ Runs one farm in a process of its own that starts its pool from a captured snapshot and then either closes it or is killed before it can, and answers the farm directories it created. */
const farmProcess=(temporary:string,killed:boolean):string[]=>{
 const plan=JSON.stringify({version:1,id:"x",jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536}),farm=pathToFileURL(join(import.meta.dir,"..","..","🟨️.mjs")).href;
 const code=`const{EventEmitter}=await import("node:events"),fs=await import("node:fs"),{openProjectFarmV1}=await import(${JSON.stringify(farm)}),temporary=${JSON.stringify(temporary)},before=new Set(fs.readdirSync(temporary));class Child extends EventEmitter{connected=true;exitCode=null;signalCode=null;send(message){if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}return true;}kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}}const farm=openProjectFarmV1({workspaceRoot:temporary,context:{registry:null},contracts:{},projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY_PLAN:${JSON.stringify(plan)}},processed:()=>24,cores:10,write:()=>undefined,launch:{command:"/semio-law/runtime",arguments:[],environment:{},implementation:"d".repeat(64),text:()=>"captured snapshot text"},spawnProcess:()=>new Child(),request:()=>({granted:Promise.resolve(undefined),release:()=>undefined})});for(let index=0;index<24;index++)await farm.prepare({name:"m"+index,root:"m",json:{},native:false,pipeline:false,nativeRoot:"m",selectedInputs:[],head:[],cacheKey:"0".repeat(24)});fs.writeSync(1,JSON.stringify(fs.readdirSync(temporary).filter(name=>name.startsWith("semio-")&&!before.has(name))));if(${killed})process.kill(process.pid,"SIGKILL");await farm.close();`;
 const run=Bun.spawnSync([process.execPath,"-e",code],{env:{...process.env,TMPDIR:temporary},stderr:"pipe"});
 if(!killed)expect(run.exitCode,run.stderr.toString().slice(-2000)).toBe(0);
 return JSON.parse(run.stdout.toString()||"[]") as string[];
};

test("a farm removes the snapshot copy, staging and session directories a killed predecessor left behind, and keeps those of a running process and every name it does not write",()=>{
 const temporary=realpathSync(mkdtempSync(join(tmpdir(),"farm-reclaim-"))),running=`semio-graph-farm-image-${process.pid}-a7live`,foreign="semio-graph-farm-image-1-short",kept=[running,foreign];
 for(const name of kept)mkdirSync(join(temporary,name),{recursive:true});
 const left=farmProcess(temporary,true);
 try{
  expect(left.length,"the killed farm left its directories").toBeGreaterThanOrEqual(3);
  expect(left.every(name=>existsSync(join(temporary,name)))).toBe(true);
  farmProcess(temporary,false);
  expect(left.filter(name=>existsSync(join(temporary,name))),"the successor removed what the killed farm left").toEqual([]);
  expect(kept.filter(name=>existsSync(join(temporary,name))),"directories of running processes and foreign names stay").toEqual(kept);
 }finally{rmSync(temporary,{recursive:true,force:true});}
},60000);
