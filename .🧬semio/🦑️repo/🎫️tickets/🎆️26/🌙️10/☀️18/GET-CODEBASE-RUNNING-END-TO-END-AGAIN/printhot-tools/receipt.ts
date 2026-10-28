/** 🧾️ Read-only anatomy of a Bazel plan receipt: `bun receipt.ts <root> <workspace> [verify|journal]` prints the receipt size, witness rows per kind, the git/ignore/runtime rows, the parse cost and (with `verify`) the cost of re-checking every stat-like row without the journal, (with `journal`) the cost of the journal-vouched check from the stored cursor. Nothing is written. */
import {readFileSync} from "node:fs";
import {join} from "node:path";
const [root,workspace,mode]=process.argv.slice(2) as [string,string,string|undefined];
const L=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const witness=await import(join(L,"🔭️witness/🟨️.mjs"));
const mb=()=>Math.round(process.memoryUsage().rss/1048576);
let t=performance.now();const text=readFileSync(join(workspace,"plan.receipt.json"),"utf8"),readMs=performance.now()-t;
t=performance.now();const receipt=JSON.parse(text),parseMs=performance.now()-t;
const rows=receipt.witness.observations as [string,string,string][],kinds:Record<string,number>={};
for(const [kind] of rows)kinds[kind]=(kinds[kind]??0)+1;
console.log(JSON.stringify({bytes:text.length,readMs:Math.round(readMs),parseMs:Math.round(parseMs),rssMb:mb(),recordedAtMs:receipt.recordedAtMs,outputs:receipt.outputs?.length,rows:rows.length,kinds}));
for(const [kind,subject,expected] of rows)if("gix".includes(kind))console.log(JSON.stringify({kind,subject:subject.slice(0,200),expected:expected.slice(0,16)}));
if(mode==="verify"){
 const statRows=rows.filter(([kind])=>"lLsSdfkKrpa".includes(kind[0]!));
 t=performance.now();const c=process.cpuUsage();const verdict=await witness.verifyWorldWitnessV1({...receipt.witness,observations:statRows},{signal:new AbortController().signal});const cpu=process.cpuUsage(c);
 console.log(JSON.stringify({verifyRows:statRows.length,verdict,wallMs:Math.round(performance.now()-t),cpuMs:Math.round((cpu.user+cpu.system)/1000),rssMb:mb()}));
}
if(mode==="journal"){
 const [{connectChangeJournalV1},{checkWorldViaJournalV1},{parseChangeCursorV1},endpoint]=await Promise.all([import(join(L,"🏡️resident/📜️journal/🟦️.ts")),import(join(L,"🏡️resident/📜️journal/🔭️world/🟦️.ts")),import(join(root,"🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts")),import(join(L,"🏡️resident/📨️protocol/📍️endpoint/🟨️.mjs"))]);
 const state=JSON.parse(readFileSync(join(workspace,"plan.cursor.json"),"utf8")),statRows=rows.filter(([kind])=>"lLsSdfkKrpa".includes(kind[0]!));
 let restatted=0;t=performance.now();const c=process.cpuUsage();
 const answer=await checkWorldViaJournalV1({client:connectChangeJournalV1(endpoint.canonicalRootV1(root)),cursor:parseChangeCursorV1(state.cursor),workspace:root,rows:statRows,linked:state.linked,stat:async(subset:any[])=>{restatted+=subset.length;return witness.verifyWorldWitnessV1({...receipt.witness,observations:subset},{signal:new AbortController().signal});}});
 const cpu=process.cpuUsage(c);
 console.log(JSON.stringify({journalRows:statRows.length,restatted,verdict:answer.verdict,wallMs:Math.round(performance.now()-t),cpuMs:Math.round((cpu.user+cpu.system)/1000),rssMb:mb()}));
}
