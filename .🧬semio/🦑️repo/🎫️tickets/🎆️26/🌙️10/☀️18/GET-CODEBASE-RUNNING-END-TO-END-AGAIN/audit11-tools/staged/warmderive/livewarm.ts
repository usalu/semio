/** ⏱️ Read-only, graph-free: the course's derived TypeScript-program question asked by the planner's own deriver on the live checkout, cold and then three times through the warm deriver (guard = the planner's observation check over the previous rows, previous program reused), with wall, CPU and RSS per ask. Usage (code from a mirror holding the batch): bun livewarm.ts <code root> <live root> */
const [code,live]=process.argv.slice(2) as [string,string];
const L=code+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build";
const derive=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🧮️derive/🟦️.ts"),warm=await import(L+"/🧭️invocation/🕸️graph/🧠️memo/🧮️derive/♨️warm/🟦️.ts"),reuse=await import(L+"/🕸️graph/📥️inference/🧭️script-inputs/♨️reuse/🟨️.mjs"),observation=await import(L+"/👁️observation/🟨️.mjs");
const question={kind:"derived",path:JSON.stringify({program:1,workspaceRoot:live,projectRoot:"🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript",projects:["{projectRoot}/tsconfig.site.json"]}),fact:""};
const cpu=()=>{const usage=process.cpuUsage();return(usage.user+usage.system)/1e6;},rss=()=>Math.round(process.memoryUsage().rss/1048576);
const store=reuse.createTypeScriptProgramReuseV1(),memory=new Map(),armed:boolean[]=[];reuse.useTypeScriptProgramReuseV1({...store,begin:(arm:boolean)=>{armed.push(arm);store.begin(arm);}});
let reference:unknown;
for(const round of [0,1,2,3]){
 observation.releaseInferenceMemoryV1();const started=performance.now(),before=cpu();
 const base=await derive.openPlannerDeriverV1(live),opened=performance.now();
 const deriver=warm.warmProgramDeriverV1(base,store,memory,(rows:unknown[],signal?:AbortSignal)=>observation.observationsHold(rows,64,new Set(),new Set(),signal));
 const answer=await deriver(question);reference??=answer.fact;
 console.log("[DEBUG]",JSON.stringify({round,armed:armed.at(-1),same:answer.fact===reference,rows:answer.rows.length,openMs:Math.round(opened-started),ms:Math.round(performance.now()-started),cpu:+(cpu()-before).toFixed(2),rss:rss()}));
}
