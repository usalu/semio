/** 🪜️ Contract: a farm of ONE worker is still a farm. A saturated host that leaves room for a single child (`maximumWorkers` 1) still runs the owners in that cancellable child process, and its first request is essential, instead of leaving every owner to the parent thread that cannot cancel or retry a stuck one.
 * @see ../../🟨️.mjs */
import {afterEach,expect,test} from "bun:test";
import {EventEmitter} from "node:events";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {closeProjectFarmsV1,openProjectFarmV1 as openFarmV1} from "../../🟨️.mjs";
import {GRAPH_FARM_POLICY_V1} from "../../🏊️pool/🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../../👁️observation/🟨️.mjs";
import {graphFarmIdentityV1} from "../../🪆️captured/🟨️.mjs";

const roots:string[]=[];
afterEach(async()=>{await closeProjectFarmsV1();for(const root of roots.splice(0))rmSync(root,{recursive:true,force:true});});
class Scripted extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(){super();setImmediate(()=>this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())}));}
 send(message:{type:string}){if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}return true;}
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}
const open=(maximumWorkers:number,asked:{essential?:boolean;preferredJobs:number}[],lines:string[])=>{
 const root=realpathSync(mkdtempSync(join(tmpdir(),"f30-farm-")));roots.push(root);
 return openFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off"},processed:()=>400,cores:10,maximumWorkers,write:line=>lines.push(line),engage:true,
  adopt:(()=>({published:1,present:0,declined:false})) as never,
  policy:{...GRAPH_FARM_POLICY_V1,grantWaitMilliseconds:150,holdMilliseconds:300},
  spawnProcess:(()=>new Scripted()) as never,
  request:(((input:{essential?:boolean;preferredJobs:number})=>{asked.push({essential:input.essential,preferredJobs:input.preferredJobs});return{granted:new Promise<{jobs:number;megabytes:number}|undefined>(()=>undefined),release:()=>undefined};}) as never)});
};

test("a host with room for one child still opens a farm whose first request is essential",()=>{
 const asked:{essential?:boolean;preferredJobs:number}[]=[],lines:string[]=[];
 const farm=open(1,asked,lines);
 expect(farm).toBeDefined();expect(asked.length).toBe(1);expect(asked[0]!.essential).toBe(true);expect(asked[0]!.preferredJobs).toBe(1);
 expect(lines.join("\n")).not.toContain("in this process (memory)");
});

test("a host with room for no child leaves the owners to the parent and says so",()=>{
 const asked:{essential?:boolean;preferredJobs:number}[]=[],lines:string[]=[];
 const farm=open(0,asked,lines);
 expect(farm).toBeUndefined();expect(asked.length).toBe(0);expect(lines.join("\n")).toContain("in this process (memory)");
});
