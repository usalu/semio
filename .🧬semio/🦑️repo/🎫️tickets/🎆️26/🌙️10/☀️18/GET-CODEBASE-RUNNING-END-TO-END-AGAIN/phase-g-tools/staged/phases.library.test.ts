import {afterAll,expect,test} from "bun:test";
import {mkdtempSync,readFileSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {setTimeout as sleep} from "node:timers/promises";
import buildPolicy from "../../../🔣️policy.json";
import {hostFloorMegabytesV1,readHostCapacityPolicyV1,readHostEstimateStoreV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import protocolSchema from "../../../../../../../../🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🧬️schema/🔣️.json";
import {runCapacityCommandV1} from "../../⌨️command/🟦️.ts";
import {buildRebuildSlotsV1,withBuildCapacityV1} from "../../🟦️.ts";
import {markBuildCapacityPhaseV1} from "../../🚏️marks/🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hq-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:30000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10});
const strip=(args:readonly string[])=>args.filter(argument=>!argument.startsWith("--tool")&&!argument.startsWith("--workers"));
const learning=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0,heartbeatMilliseconds:20}});
const args=["run","@x/y:typecheck","--parallel=1"];

test("Phases close the loop: the graph build of some runs teaches the global rebuild class and never the class of the task, so the next request of that class reserves its execution while a run that never marks phases keeps the contaminated size",async()=>{
 const phased=fresh(),plain=fresh(),reservedPhased:number[]=[],reservedPlain:number[]=[],lines:string[]=[];
 const scenario=async(directory:string,marked:boolean,index:number,reserved:number[],level:{value:number})=>{
  const environment:Record<string,string|undefined>={SEMIO_HOST_CAPACITY_DIRECTORY:directory},rebuild=index%3===0;
  await withBuildCapacityV1(args,environment,async granted=>{
   reserved.push(granted.handle!.reservedMegabytes);
   const mark=async(phase:"plan"|"rebuild"|"execution")=>{if(marked){await markBuildCapacityPhaseV1(phase,environment);await sleep(50);}};
   await mark("plan");level.value=900;await sleep(70);
   if(rebuild){await mark("rebuild");level.value=6500;await sleep(110);await mark("plan");level.value=700;await sleep(70);}
   await mark("execution");level.value=1100;await sleep(110);
  },{directory,policy:learning,strip,write:line=>{if(marked)lines.push(line);},sample:async()=>({host:host(),swappedOutMegabytes:0}),observe:async()=>level.value});
 };
 const levelA={value:400},levelB={value:400};
 for(let index=0;index<10;index++){levelA.value=400;levelB.value=400;await scenario(phased,true,index,reservedPhased,levelA);await scenario(plain,false,index,reservedPlain,levelB);}
 expect(reservedPhased[0],"the first request of an unknown class reserves its default").toBe(reservedPlain[0]);
 expect(reservedPlain.at(-1)!,"the whole-tree peak carries the graph build into the class of the task").toBeGreaterThan(reservedPhased.at(-1)!*2);
 expect(reservedPhased.at(-1)!,"a covered run needs about its execution peak").toBeLessThan(2600);expect(reservedPhased.at(-1)!).toBeGreaterThanOrEqual(learning.estimates.minimumMegabytes);
 const store=readHostEstimateStoreV1(phased);
 expect(Object.keys(store.classes).filter(key=>key.startsWith("phase:")).sort()).toEqual(["phase:assembly","phase:plan","phase:rebuild"]);
 expect(Object.keys(store.classes).filter(key=>key.startsWith("run")).every(key=>Math.max(...store.classes[key]!.samples.map(row=>row.ratioBasisPoints))<10000),"no sample of the task classes carries the 6500 MB build").toBe(true);
 expect(Math.max(...store.classes["phase:rebuild"]!.samples.map(row=>row.ratioBasisPoints))*1024/10000).toBeGreaterThan(6000);
 expect(lines.length>0).toBe(true);
},120000);

test("repo capacity status names the phase a grant is in, in English, in German and in the JSON snapshot",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],json:string[]=[],sample=async()=>({host:host(),swappedOutMegabytes:0}),context=(environment:NodeJS.ProcessEnv,write:(line:string)=>void)=>({environment,write,directory,policy:learning,sample});
 await withBuildCapacityV1(args,{},async()=>{
  const file=join(directory,"grants",readdirSync(join(directory,"grants")).find(name=>name.endsWith(".json"))!);writeFileSync(file,JSON.stringify({...JSON.parse(readFileSync(file,"utf8")),phase:"rebuild"}));
  await runCapacityCommandV1(["status"],context({LANG:"en_US.UTF-8"},line=>english.push(line)));await runCapacityCommandV1(["status"],context({LANG:"de_DE.UTF-8"},line=>german.push(line)));await runCapacityCommandV1(["status","--json"],context({},line=>json.push(line)));
 },{directory,policy:learning,strip,sample,pid:process.pid});
 expect(english[1]).toMatch(/pid \d+, since \d+s \[rebuild\]$/u);expect(german[1]).toMatch(/\[rebuild\]$/u);expect(JSON.parse(json.join("\n")).grants[0].phase).toBe("rebuild");
});

test("The compute slots of a rebuild follow the learned rebuild size, a forced count is not touched by it, and a missing marks switch costs nothing",async()=>{
 const directory=fresh();
 expect(buildRebuildSlotsV1({},{directory,totalMegabytes:32768,policy:learning})).toBe(3);
 expect(buildRebuildSlotsV1({},{directory,totalMegabytes:16384,policy:learning})).toBe(1);
 expect(buildRebuildSlotsV1({},{directory,totalMegabytes:32768,policy:learning,now:()=>1})).toBe(Math.max(1,Math.min(learning.estimates.maximumRebuildSlots,Math.floor(Math.floor((32768-hostFloorMegabytesV1(learning,{totalMegabytes:32768}))*learning.estimates.rebuildShareBasisPoints/10000)/learning.estimates.rebuildPriorMegabytes))));
 expect(protocolSchema.definitions.Protocol.const.marksEnvironment,"the names the light marks module spells are the schema names").toBe("SEMIO_HOST_CAPACITY_PHASE_MARKS");expect(protocolSchema.definitions.Protocol.const.grantEnvironment).toBe("SEMIO_HOST_CAPACITY_GRANT");
 expect(await markBuildCapacityPhaseV1("plan",{SEMIO_HOST_CAPACITY_GRANT:"aaaaaaaa1",SEMIO_HOST_CAPACITY_PHASE_MARKS:"off",SEMIO_HOST_CAPACITY_DIRECTORY:directory})).toBe(false);
 expect(await markBuildCapacityPhaseV1("plan",{})).toBe(false);
 expect(await markBuildCapacityPhaseV1("plan",{SEMIO_HOST_CAPACITY_GRANT:"aaaaaaaa1",SEMIO_HOST_CAPACITY_DIRECTORY:directory})).toBe(false);
});
