import{afterEach,expect,test}from"bun:test";
import{EventEmitter}from"node:events";
import{closeProjectFarmsV1,openProjectFarmV1}from"../../🟨️.mjs";
import{inferenceRecordIdentityV1}from"../../../../../../👁️observation/🟨️.mjs";
import{graphFarmIdentityV1}from"../../🪆️captured/🟨️.mjs";

/** 🪶️ Contract: on a declared small machine the project farm is lean. One child is enough (two cores no longer mean no farm), the farm engages at the first cold project, every project is inferred by a child that is replaced after a few projects, and the parent infers nothing while a child can be had.
 * @see ../../../../../../🧠️memory/🪶️lean/🧬️schema/🔣️.json */
afterEach(async()=>{await closeProjectFarmsV1();});
class Child extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly log:string[]){super();setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));}
 send(message:{type:string;id?:number}){
  if(message.type==="run"){this.log.push("run");setImmediate(()=>this.emit("message",{type:"result",id:message.id,reply:{names:["x"],cpuMilliseconds:1},rssMegabytes:100}));}
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}
const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
const open=(host:string,cores:number,mode?:string)=>{
 const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=host;
 const log:string[]=[],children:Child[]=[],asked:{preferredJobs:number;megabytes:number}[]=[],lines:string[]=[];
 try{
  const farm=openProjectFarmV1({workspaceRoot:"/nonexistent",context:{registry:null},contracts:{},launch:null,projects:300,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",...mode?{SEMIO_SLICE_LEAN:mode}:{}},processed:()=>0,cores,write:line=>lines.push(line),
   adopt:(()=>({published:1,present:0,declined:false})) as never,
   spawnProcess:(()=>{const child=new Child(log);children.push(child);return child;}) as never,
   request:(((input:{preferredJobs:number;megabytes:number})=>{asked.push(input);return{granted:Promise.resolve({jobs:1,megabytes:input.megabytes}),release:()=>undefined};}) as never)});
  return{farm,log,children,asked,lines};
 }finally{if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}
};

test("8 GB and two cores: the farm opens with one 1700 MB child, engages at once, every project is inferred by a child and the parent infers none; children are replaced after four projects",async()=>{
 const run=open("8192",2);
 expect(run.farm,"a lean farm exists where the declared farm needs two workers").toBeDefined();
 const farm=run.farm!;
 await farm.prepare(job("first"));
 await new Promise<number>(done=>farm.onReady(done));
 const results:boolean[]=[];
 for(let at=0;at<20;at++)results.push(await farm.prepare(job("p"+at)));
 expect(results.every(Boolean),"the parent inferred a project").toBe(true);
 expect(run.log.length).toBeGreaterThanOrEqual(20);
 expect(run.children.length,"a child is replaced after four projects").toBeGreaterThanOrEqual(5);
 expect(run.asked[0]!.megabytes).toBe(1700);
 await farm.close();
});

test("16 GB lean read grants more children than one; a serving host and lean off keep the declared farm",async()=>{
 const sixteen=open("16384",8);expect(sixteen.farm).toBeDefined();expect(sixteen.asked[0]!.megabytes).toBe(1700);await sixteen.farm!.close();await closeProjectFarmsV1();
 const off=open("8192",2,"off");expect(off.farm,"lean off at two cores: no farm, as before").toBeUndefined();await closeProjectFarmsV1();
 const serve=open("65536",2);expect(serve.farm,"a serving host at two cores: no farm, as before").toBeUndefined();
});
