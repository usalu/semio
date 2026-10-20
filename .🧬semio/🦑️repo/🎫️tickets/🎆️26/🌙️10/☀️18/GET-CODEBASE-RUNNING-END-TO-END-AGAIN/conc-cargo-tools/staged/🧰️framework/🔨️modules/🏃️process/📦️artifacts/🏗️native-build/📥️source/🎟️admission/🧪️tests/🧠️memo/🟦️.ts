import{test,expect}from"bun:test";
import{admitCargoSourceBoundariesV1,type CargoSourceAdmissionMemoV1}from"../../🟦️.ts";
import{readCargoCompilationGraphV1}from"../../../../🕸️units/🟦️.ts";
import{receiveScriptProcessInvocation,createIssuedResourceControlV1}from"../../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import graphCorpus from"../../../../🕸️units/🧪️tests/🧫️fixtures/🔣️.json"with{type:"json"};
import corpus from"../🔣️.json"with{type:"json"};

/** 🧠️ A unit whose admission was remembered reads no Source, asks for no compiler cfg and takes no durable recheck; its computed fields come back exactly, and only a miss reads again. */
const policy={version:1 as const,maxSourceBytes:4096,maxSourceFiles:8,maxActiveLines:100};
function graphOf(units:number){const raw=graphCorpus.cases[graphCorpus.identicalUnits.case]!.units[graphCorpus.identicalUnits.unit]!;const graph=readCargoCompilationGraphV1({version:1,units:Array.from({length:units},()=>({...structuredClone(raw),dependencies:[]})),roots:Array.from({length:units},(_,index)=>index)},graphCorpus.metadata,"/repo");graph.units.forEach((unit,index)=>{unit.compilerFlags=["--cfg","flag"+index];});return graph;}

test("a remembered unit admission skips reads, cfg probes and durable rechecks and restores the computed fields",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const control=createIssuedResourceControlV1(original,"cargo:memo-source-admission",0),store=new Map<string,Parameters<CargoSourceAdmissionMemoV1["recall"]> extends never?never:ReturnType<CargoSourceAdmissionMemoV1["recall"]>>(),remembered:Array<{probed:readonly string[];startedMs:number}>=[];
 const memo:CargoSourceAdmissionMemoV1={recall:unit=>store.get(unit.id),remember:(unit,_policy,result)=>{remembered.push({probed:result.probed,startedMs:result.startedMs});store.set(unit.id,{inputs:result.inputs.map(input=>({...input})),cfg:[...result.cfg],environment:result.environment});}};
 const counts=()=>({cfg:0,reads:0,units:0,complete:0});
 const run=async(count:ReturnType<typeof counts>,graph:ReturnType<typeof graphOf>)=>admitCargoSourceBoundariesV1(graph,"/repo",policy,control,{async recheck(_control,phase){if(phase==="unit")count.units++;else count.complete++;},async cfg(unit){count.cfg++;return[unit.compilerFlags[1]!];},async read(){count.reads++;return new TextEncoder().encode(corpus.source);}},memo);
 const first=graphOf(3),cold=counts();await run(cold,first);
 expect(cold.cfg).toBe(3);expect(cold.reads).toBeGreaterThanOrEqual(3);expect(cold.units).toBe(3);expect(remembered.length).toBe(3);expect(remembered.every(row=>row.probed.length>0&&row.startedMs>0)).toBe(true);
 const second=graphOf(3),warm=counts();await run(warm,second);
 expect(warm.cfg).toBe(0);expect(warm.reads).toBe(0);expect(warm.units).toBe(0);expect(warm.complete).toBe(1);
 for(const [index,unit] of second.units.entries()){expect(unit.sourceInputs).toEqual(first.units[index]!.sourceInputs);expect(unit.compilerCfg).toEqual(first.units[index]!.compilerCfg);expect(unit.compilerEnvironment).toEqual(first.units[index]!.compilerEnvironment);}
 store.delete(second.units[1]!.id);const partial=counts();await run(partial,graphOf(3));
 expect(partial.cfg).toBe(1);expect(partial.units).toBe(1);
}));
