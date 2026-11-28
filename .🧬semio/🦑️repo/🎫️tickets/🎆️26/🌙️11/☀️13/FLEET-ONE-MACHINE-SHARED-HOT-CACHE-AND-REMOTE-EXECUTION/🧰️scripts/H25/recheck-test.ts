/** 🔄️ Contract: a unit that waited long for local admission asks the fleet cache once more before it compiles, because another machine may have stored it meanwhile; a short wait costs no extra lookup, a failing lookup never blocks the compile.
 * @see ../../⏳️admit/🟦️.ts (`awaitCompileAdmissionV1`)
 * @see ../../📜️script.ts (`compile`) */
import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {awaitCompileAdmissionV1} from "../../⏳️admit/🟦️.ts";
import {tryAcquireSlotV1} from "../../🔒️lock/🟦️.ts";

const scratch:string[]=[];
const directory=()=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"recheck-")));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sleep=(milliseconds:number)=>new Promise<void>(done=>setTimeout(done,milliseconds));
const NOISE=()=>undefined;
const world=(root:string,extra:object)=>({root,slots:1,crate:"c",since:Date.now(),environment:{},facts:undefined,alone:false,report:NOISE,event:NOISE,...extra});

test("a unit that waited longer than the threshold is served from the fleet cache, gives its slot back and is told not to compile, with an en/de line",async()=>{
 const root=directory(),holder=tryAcquireSlotV1(root,1)!,reports:string[]=[];let asked=0;
 setTimeout(()=>holder.release(),400);
 const admission=await awaitCompileAdmissionV1(world(root,{recheck:async()=>{asked++;return true;},report:(text:string)=>reports.push(text),bounds:{slotMaximumWaitMilliseconds:5000,recheckAfterMilliseconds:200,parkAfterMilliseconds:50}}));
 expect(asked).toBe(1);expect(admission.served).toBe(true);expect(admission.slot).toBeUndefined();
 expect(reports.some(text=>text.includes("installed from the fleet cache")&&text.includes("aus dem Flotten-Cache installiert"))).toBe(true);
 const again=tryAcquireSlotV1(root,1);expect(again,"the slot was given back").toBeDefined();again!.release();
},15000);

test("a wait shorter than the threshold costs no lookup and keeps the slot",async()=>{
 const root=directory();let asked=0;
 const admission=await awaitCompileAdmissionV1(world(root,{recheck:async()=>{asked++;return true;},bounds:{recheckAfterMilliseconds:60000}}));
 expect(asked).toBe(0);expect(admission.served).toBe(false);expect(admission.slot).toBeDefined();admission.slot!.release();
});

test("a miss after a long wait keeps the slot and the compile goes on; a lookup that throws or hangs-then-fails is a miss",async()=>{
 for(const recheck of [async()=>false,async()=>{throw new Error("down");}]){
  const root=directory(),holder=tryAcquireSlotV1(root,1)!;
  setTimeout(()=>holder.release(),300);
  const admission=await awaitCompileAdmissionV1(world(root,{recheck,bounds:{slotMaximumWaitMilliseconds:5000,recheckAfterMilliseconds:100,parkAfterMilliseconds:50}}));
  expect(admission.served).toBe(false);expect(admission.slot).toBeDefined();admission.slot!.release();
 }
},15000);

test("a claim lost to another machine while the unit was parked asks again even after a short wait",async()=>{
 const root=directory(),holder=tryAcquireSlotV1(root,1)!;let asked=0;
 setTimeout(()=>holder.release(),300);
 const hold={park:async()=>undefined,resume:async()=>"lost" as const};
 const admission=await awaitCompileAdmissionV1(world(root,{hold,recheck:async()=>{asked++;return false;},bounds:{slotMaximumWaitMilliseconds:5000,recheckAfterMilliseconds:600000,parkAfterMilliseconds:50}}));
 expect(admission.claim).toBe("lost");expect(asked).toBe(1);admission.slot?.release();
},15000);

test("the wrapper wires it: compile takes a recheck that fetches the fleet unit and the served outcome is installed, never compiled",()=>{
 const text=readFileSync(join(import.meta.dir,"../../📜️script.ts"),"utf8");
 expect(text).toContain("fetchFleetUnitV1(fleet!)===\"hit\"");
 expect(text).toContain("if(admission.served)return{status:0");
 expect(text).toContain("after-admission-wait");
});
