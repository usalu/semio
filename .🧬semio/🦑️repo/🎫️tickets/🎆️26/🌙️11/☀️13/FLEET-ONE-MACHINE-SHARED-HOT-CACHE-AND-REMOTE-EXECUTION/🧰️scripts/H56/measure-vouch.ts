import {join} from "node:path";
const root="/Users/ueli/Documents/semio";
const B=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const {readAdmissionMemoV1}=await import(join(B,"🕸️graph/📥️load/🧠️admission-memo/🟦️.ts"));
const {buildGraphSourceFactsHoldV1}=await import(join(B,"🕸️graph/📁️workspace/🟦️.ts"));
const {journalCleanRowsV1,readJournalAttestationV1,writeJournalAttestationV1}=await import(join(B,"🕸️graph/📥️load/🗞️journal-clean/🟦️.ts"));
const {connectChangeJournalV1}=await import(join(B,"🏡️resident/📜️journal/🟦️.ts"));
const file=process.argv[2],scope=file.replace(/\.admission\.json$/,""),memoPath=join(root,".cache/b/g/m/selected",file),EXTRA=new Set(["analysis","typescriptText","field"]);
const lent=await readAdmissionMemoV1(memoPath,scope,()=>true);if(!lent){console.log("memo unreadable");process.exit(0);}
const rows=lent.observations as any[];await lent.close();
const lift=(row:any)=>EXTRA.has(row.kind)&&!(row.kind==="field"&&row.fact?.paths?.length)?{...row,kind:"file"}:row;
const att=join("/private/tmp/claude-501/h56/m","att.json"),feed=connectChangeJournalV1(root) as any;
const ports={feed,root,read:()=>readJournalAttestationV1(att),write:(v:any)=>writeJournalAttestationV1(att,v)};
const stale:string[]=[];let t=performance.now();
const full=await buildGraphSourceFactsHoldV1(rows,undefined,(r:any)=>stale.push(r.kind+":"+r.path),undefined,false,true);
console.log("rows",rows.length,"full hold",full,"stale",stale.length,Math.round(performance.now()-t),"ms");
const j1=await journalCleanRowsV1(ports,"k",rows.map(lift));await j1.seal("k",rows.map(lift));
await Bun.sleep(3000);
for(const [name,mapper] of [["old",(r:any)=>r],["new",lift]] as const){
 const j=await journalCleanRowsV1(ports,"k",rows.map(mapper));
 const stale2:string[]=[];t=performance.now();
 const held=await buildGraphSourceFactsHoldV1(rows,undefined,(r:any)=>stale2.push(r.kind+":"+r.path),undefined,false,true,((r:any)=>j.clean(mapper(r))) as never);
 console.log(name,"hold",held,"stale",stale2.length,"counts",JSON.stringify(j.counts()),Math.round(performance.now()-t),"ms","sameStale",JSON.stringify([...stale2].sort())===JSON.stringify([...stale].sort()));
}
process.exit(0);
