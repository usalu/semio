/** ⏫️ Contract: the children of a read run at the priority class the farm schema declares for reads (normal); a child that inherits a lower priority from its starter is not silently left there: the claim is tried, the refusal of the platform is reported once in both languages. A real child process reports its own priority.
 * @see ../../🏊️pool/🟨️.mjs */
import {afterEach,expect,test} from "bun:test";
import {spawn} from "node:child_process";
import {EventEmitter} from "node:events";
import {getPriority} from "node:os";
import {claimFarmChildPriorityV1,createFarmV1,GRAPH_FARM_POLICY_V1,GRAPH_FARM_MESSAGES_V1} from "../../🏊️pool/🟨️.mjs";

const kids:import("node:child_process").ChildProcess[]=[];
afterEach(()=>{for(const kid of kids.splice(0))kid.kill();});
const report=`process.stdout.write(String(require("node:os").getPriority())+"\\n");setInterval(()=>{},1000);`;
const real=(nice:number)=>{const kid=spawn(nice===0?process.execPath:"nice",nice===0?["-e",report]:["-n",String(nice),process.execPath,"-e",report],{stdio:["ignore","pipe","inherit"]});kids.push(kid);return new Promise<{kid:typeof kid;own:number}>(resolve=>kid.stdout!.once("data",chunk=>resolve({kid,own:Number(String(chunk).trim())})));};

test("the schema declares normal priority for the read farm and a lowered one for fleet pool children",()=>{
 expect(GRAPH_FARM_POLICY_V1.priority.readFarm).toBe(0);
 expect(GRAPH_FARM_POLICY_V1.priority.fleetPool).toBeGreaterThan(0);
 expect(GRAPH_FARM_MESSAGES_V1.en.priority).toContain("{nice}");
 expect(GRAPH_FARM_MESSAGES_V1.de.priority).toContain("{nice}");
});

test("a real child at normal priority is left alone, a real niced child is tried once and its refusal is named",async()=>{
 const normal=await real(0),niced=await real(5);
 expect(normal.own).toBe(0);expect(niced.own).toBe(5);
 const calls:number[]=[];
 const kept=claimFarmChildPriorityV1(normal.kid.pid,{get:getPriority,set:(pid,value)=>{calls.push(value);}});
 expect(kept).toEqual({before:0,after:0,raised:false,inherited:false});expect(calls).toEqual([]);
 const claimed=claimFarmChildPriorityV1(niced.kid.pid);
 expect(claimed.before).toBe(5);
 expect(claimed.raised).toBe(process.platform==="win32"||process.getuid?.()===0);
 expect(claimed.after).toBe(getPriority(niced.kid.pid));
 expect(claimed.inherited).toBe(!claimed.raised);
});

test("a platform that lets the child be raised ends at the declared class",()=>{
 let held=5;
 const claimed=claimFarmChildPriorityV1(1,{get:()=>held,set:(_pid,value)=>{held=value;}});
 expect(claimed).toEqual({before:5,after:0,raised:true,inherited:false});
});

class Scripted extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;
 constructor(readonly pid:number){super();}
 send(message:{type:string}){if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}return true;}
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}
test("a farm whose children inherit a lowered priority says so once, naming niceness and class",async()=>{
 const niced=await real(5),events:[string,Record<string,unknown>][]=[];
 const farm=createFarmV1({entry:"x",service:"s",session:{},workers:2,megabytes:64,environment:{LANG:"en_US.UTF-8"},report:(event:string,fields:Record<string,unknown>)=>{events.push([event,fields]);},priority:{get:getPriority,set:()=>{throw Error("EACCES");}},spawnProcess:(()=>new Scripted(niced.kid.pid!)) as never} as never);
 await new Promise(resolve=>setTimeout(resolve,100));
 const said=events.filter(([event])=>event==="priority");
 expect(said).toEqual([["priority",{nice:5,wanted:0}]]);
 await farm.close();
},20000);
