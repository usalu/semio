import {join} from "node:path";
const root="/Users/ueli/Documents/semio";
const B=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const S=process.argv[3];
const {readAdmissionMemoV1}=await import(join(B,"🕸️graph/📥️load/🧠️admission-memo/🟦️.ts"));
const {buildGraphSourceFactsHoldV1}=await import(join(B,"🕸️graph/📁️workspace/🟦️.ts"));
const live=await import(join(B,"🕸️graph/📥️load/🗞️journal-clean/🟦️.ts"));
const next=await import(join(S,"🕸️graph/📥️load/🗞️journal-clean/🟦️.ts"));
const {connectChangeJournalV1}=await import(join(B,"🏡️resident/📜️journal/🟦️.ts"));
const file=process.argv[2],scope=file.replace(/\.admission\.json$/,""),memoPath=join(root,".cache/b/g/m/selected",file);
const lent=await readAdmissionMemoV1(memoPath,scope,()=>true);
const rows=lent.observations as any[];await lent.close();
const kinds:Record<string,number>={};for(const r of rows)kinds[r.kind]=(kinds[r.kind]??0)+1;console.log("rows",rows.length,JSON.stringify(kinds));
const feed=connectChangeJournalV1(root) as any;
for(const [name,mod] of [["live",live],["new",next]] as const){
  const att=join(process.argv[4],name+".json");
  const ports={feed,root,read:()=>mod.readJournalAttestationV1(att),write:(v:any)=>mod.writeJournalAttestationV1(att,v)};
  const j1=await mod.journalCleanRowsV1(ports,"k",rows);await j1.seal("k");
  await Bun.sleep(1500);
  let t=performance.now();const j2=await mod.journalCleanRowsV1(ports,"k",rows);const setup=Math.round(performance.now()-t);
  const by:Record<string,number>={};let skipped=0;t=performance.now();for(const r of rows)if(j2.clean(r)){skipped++;by[r.kind]=(by[r.kind]??0)+1;}
  console.log(name,"setup ms",setup,"vouched",skipped,JSON.stringify(by),"pass ms",Math.round(performance.now()-t));
  if(name==="new"){
    t=performance.now();const stale:string[]=[];const hold=await buildGraphSourceFactsHoldV1(rows,undefined,(r:any)=>stale.push(r.kind+":"+r.path),undefined,false,true,j2.clean as never);
    console.log("hold with vouching",hold,"stale",stale.length,Math.round(performance.now()-t),"ms");
    const resolveStale=stale.filter(s=>s.startsWith("resolve:"));console.log("stale resolve rows",resolveStale.length,resolveStale.slice(0,3));
  }
}
process.exit(0);
