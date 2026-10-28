import{readFileSync}from"node:fs";import{createHash}from"node:crypto";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/";
const inference=await import(L+"🕸️graph/📥️inference/🟨️.mjs");await inference.libraryBootstrap;
const obs=await import(L+"👁️observation/🟨️.mjs");const G=await import(L+"🕸️graph/🕰️generations/🟨️.mjs");
const idx=G.unpackFactIndexV1(readFileSync(process.argv[2]),1<<29);
const prepared=inference.prepareSourceAnalysesV1(),byId=new Map(Object.values(prepared).map(p=>[p.id,p]));
const ownersOfRow=new Map();for(const c of idx.classes)for(const o of c.o)for(const r of c.r){let s=ownersOfRow.get(r);if(!s)ownersOfRow.set(r,s=new Set());s.add(o);}
const out={analysisRows:0,contentChanged:0,resultSame:0,resultChanged:0,unregistered:0,missing:0,nestedChanged:0};const staleOwners=new Set(),changedOwners=new Set(),perFile=new Map();
for(let i=0;i<idx.rows.length;i++){const row=idx.rows[i];if(row.kind!=="analysis")continue;out.analysisRows++;
  let bytes;try{bytes=readFileSync(row.path);}catch{out.missing++;for(const o of ownersOfRow.get(i)??[])staleOwners.add(o);continue;}
  const sha=createHash("sha256").update(bytes).digest("hex");if(sha===row.fact.source.sha256)continue;out.contentChanged++;
  const provider=byId.get(row.fact.provider);if(!provider){out.unregistered++;for(const o of ownersOfRow.get(i)??[])staleOwners.add(o);continue;}
  let fact;try{const c=await obs.captureObservedSourceInputsV1(()=>obs.observeInferenceSnapshotV1(()=>obs.observedSourceAnalysisV1(provider,row.path)));fact=c.observations.find(r=>r.kind==="analysis"&&r.path===row.path)?.fact;}catch(e){fact=undefined;}
  if(fact&&fact.resultSha256===row.fact.resultSha256){out.resultSame++;continue;}
  out.resultChanged++;const owners=ownersOfRow.get(i)??new Set();for(const o of owners){staleOwners.add(o);changedOwners.add(o);}
  const key=row.path.replace("/Users/ueli/Documents/semio/","");perFile.set(key,(perFile.get(key)??new Set()));for(const o of owners)perFile.get(key).add(o);
}
console.log(JSON.stringify({...out,ownersStaleByAnalysis:staleOwners.size,totalOwners:idx.owners.length}));
const top=[...perFile].sort((a,b)=>b[1].size-a[1].size).slice(0,12);for(const[p,s]of top)console.log(s.size,p.slice(-120));
console.log("files with changed analysis result:",perFile.size);
