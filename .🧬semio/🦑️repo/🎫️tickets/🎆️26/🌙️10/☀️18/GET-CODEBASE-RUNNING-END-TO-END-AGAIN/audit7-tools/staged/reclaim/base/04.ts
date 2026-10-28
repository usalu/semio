/** 🏊️ Contract: closing the farm releases the grant of every pool, also when the close of an earlier pool fails; the failure is not swallowed, it is raised once every pool has been closed, released and removed. The oracle is the list of grants returned to the host and the staging directories left on disk. */
import{expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{existsSync,mkdtempSync,readdirSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
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
