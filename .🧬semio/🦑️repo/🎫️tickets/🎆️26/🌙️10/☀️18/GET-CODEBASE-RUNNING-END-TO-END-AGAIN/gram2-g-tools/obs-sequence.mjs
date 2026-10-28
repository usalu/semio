// [gram2-g] byte-identity oracle of the observation record path: a deterministic fs scenario (reads, listings, probes, repeats, an edit between two reads, memoized replays, persisted project records reused in a second round) is run inside real captures; prints a digest of every observation array (kind, path, fact, order). usage: bun obs-sequence.mjs   (compare the digest with and without the staged edits)
import{createHash}from"node:crypto";import{mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync,utimesSync}from"node:fs";import{tmpdir}from"node:os";import{join}from"node:path";
const L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/",obs=await import(L+"👁️observation/🟨️.mjs"),{observedFs:fs}=obs;
const root=realpathSync(mkdtempSync(join(tmpdir(),"obs-seq-")));const t=1700000000;
const put=(rel,text)=>{const p=join(root,rel);mkdirSync(join(p,".."),{recursive:true});writeFileSync(p,text);utimesSync(p,t,t);return p;};
const digests=[];const h=rows=>createHash("sha256").update(JSON.stringify(rows.map(r=>[r.kind,r.path.replace(root,"@"),typeof r.fact==="object"&&r.fact?.sha256?{...r.fact,mtimeMs:0,ctimeMs:0,ino:0}:r.fact]))).digest("hex");
try{
 for(let i=0;i<40;i++)put(`d${i%7}/f${i}.txt`,"x".repeat(i)+"\n");
 const cap=await obs.captureObservedSourceInputsV1(async()=>obs.observeInferenceSnapshotV1(async()=>{
  for(let round=0;round<2;round++){
   for(let i=0;i<40;i++){fs.readFileSync(join(root,`d${i%7}/f${i}.txt`),"utf8");if(i%3===0)fs.existsSync(join(root,`d${i%7}/missing${i}`));if(i%5===0)fs.lstatSync(join(root,`d${i%7}`));fs.readdirSync(join(root,`d${i%7}`),{withFileTypes:true});fs.statSync(join(root,`d${i%7}/f${i}.txt`));}
   for(let i=0;i<40;i+=4)fs.readdirSync(join(root,`d${i%7}`));
   if(round===0){put("d3/f3.txt","changed\n");}
  }
  const cache=new Map(),a=obs.memoizedObservedComputationV1(cache,"k",()=>{fs.readFileSync(join(root,"d1/f1.txt"));fs.existsSync(join(root,"nothing"));return 1;}),b=obs.memoizedObservedComputationV1(cache,"k",()=>2);
  return [a,b];
 }),undefined,false);
 digests.push(["capture",cap.observations.length,h(cap.observations)]);
 const record=join(root,"np.json");
 for(let round=0;round<2;round++){const value=await obs.observeInferenceSnapshotV1(()=>obs.observeProjectInferenceV1({cachePath:record,key:"k",owner:root,stamps:[]},async()=>{const rows=fs.readdirSync(root);return rows.length;}));
  digests.push(["record",round,value,(obs.projectInferenceObservationsV1(record)??[]).length,h(obs.projectInferenceObservationsV1(record)??[])]);}
 console.log(JSON.stringify(digests));
 console.log("OBS-SEQUENCE",createHash("sha256").update(JSON.stringify(digests)).digest("hex"));
}finally{rmSync(root,{recursive:true,force:true});}
