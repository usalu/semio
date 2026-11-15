"""R41: a read without a change-journal cursor (the verify epoch) settles its input closures from the facts it has just verified: the rows that no longer hold are the dirty set, so the manifest, the part metas and the vouches of R28 apply and a restarted read resumes instead of repeating."""
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
B=L+"🏗️build/"
G=B+"🕸️graph/"
E=[]
def add(rel,old,new,count=1):E.append((rel,old,new,count))
K=G+"🧮️derived-nodes/🧩️closure-kinds/"
add(K+"🟨️.mjs",'import{dirtySummaryV1,rowPathsV1,rowTouchedV1}from"./🧭️row-paths/🟨️.mjs";','import{dirtySummaryV1,rowPathsV1,rowTouchedV1,staleSummaryV1}from"./🧭️row-paths/🟨️.mjs";')
add(K+"🟨️.mjs",'const runOnce=async({pseudoRows,derive,hold=async()=>true,dirty,signal,progress,vouchStamp,answers:keepAnswers=false})=>{','const runOnce=async({pseudoRows,derive,hold=async()=>true,dirty,staleFacts,signal,progress,vouchStamp,answers:keepAnswers=false})=>{')
add(K+"🟨️.mjs",'const summary=dirty===undefined||dirty.overflow===true?undefined:dirtySummaryV1(dirty);activeSummary=summary;','const summary=dirty!==undefined?dirty.overflow===true?undefined:dirtySummaryV1(dirty):staleFacts===undefined||staleFacts.length===0?undefined:staleSummaryV1(staleFacts,relative,VOLATILE);activeSummary=summary;')
add(K+"🟨️.d.mts",'settle(options:{pseudoRows:ReadonlyMap<string,GenerationRowV1>;','settle(options:{pseudoRows:ReadonlyMap<string,GenerationRowV1>;staleFacts?:readonly GenerationRowV1[];')
add(K+"🧭️row-paths/🟨️.mjs",'const stampMemo=new Map();','''/** 🧮️ The dirty summary of a read that has no change journal: the fact rows that were just verified against the disk and no longer hold are exactly what moved since the generation, so their reach (files, probed directories, bare ancestors, configuration) is the set the closure parts are tested against; a row about a path above the checkout is checked once per pass by the settler itself, a row nothing is known about marks its parts volatile, which are always asked again. A part that holds no stale row can reach none of them, so it is never skipped wrongly.
 * @param {Iterable<{kind:string;path:string;fact?:unknown}>} rows @param {(path:string)=>string} relative @param {string} volatileMarker */
export function staleSummaryV1(rows,relative,volatileMarker){
 const paths=new Set(),directories=new Set();let configs=false,modules=false;
 for(const row of rows){
  const one=rowPathsV1(row,relative,volatileMarker);
  for(const path of one.paths)paths.add(path);
  for(const dir of one.dirs)directories.add(dir);
  for(const dir of one.bare)directories.add(dir);
  if(one.config)configs=true;
  if(one.bare.length)modules=true;
 }
 const summary=dirtySummaryV1({paths:[...paths],directories:[...directories]});
 return{...summary,configs:summary.configs||configs,modules:summary.modules||modules};
}

const stampMemo=new Map();''')
add(G+"🕰️generations/🟨️.mjs",'hold,...journalDirty?{dirty:journalDirty}:{},','hold,...journalDirty?{dirty:journalDirty}:{staleFacts:[...stale].map(id=>index.rows[id])},')
