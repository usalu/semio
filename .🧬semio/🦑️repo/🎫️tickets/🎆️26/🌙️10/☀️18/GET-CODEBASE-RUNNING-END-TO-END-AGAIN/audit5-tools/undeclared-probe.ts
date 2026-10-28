const root="/Users/ueli/Documents/semio";
const L=root+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/";
const led=await import(L+"⚡️caching/💾️storage/🧾️ledger/🟦️.ts");
const run=await import(L+"🏗️build/🧹️cache-gc/🏃️runner/🟦️.ts");
const ledger=led.cacheLedgerV1();
const now=Date.now();
const ports={signal:new AbortController().signal,now:()=>Date.now(),progress:async()=>{},publish:async()=>{},yieldContinuation:async()=>{},alive:(p:number)=>{try{process.kill(p,0);return true}catch(e:any){return e.code==="EPERM"}},sqliteFree:async()=>true};
for(const id of ["repository","native"] as const){
  const r=ledger.roots[id];
  const base=led.cacheRootDirectoryV1(root,ledger,id);
  const names=await run.undeclaredEntriesV1(root,ledger,id);
  console.log(`[DEBUG] root ${id} dir=${r.directory} unusedAge=${r.undeclaredUnusedAgeMs/86400000}d quiet=${r.undeclaredQuietMs/60000}min undeclared=${names.length}`);
  for(const n of names){const m=await run.measureCacheTreeV1(base+"/"+n,ports as any,200000);console.log(`[DEBUG]   ${n} bytes=${(m.bytes/1048576).toFixed(1)}MB entries=${m.entries} ageDays=${((now-m.recencyMs)/86400000).toFixed(1)} partial=${m.partial} sqlite=${m.sqlite.length} queues=${m.queues.length}`);}
}
for(const r of ledger.retirements){console.log("[DEBUG] retired",r.root,r.directory,"quiet",r.quietMs/60000,"min");}
