import {createHash} from "node:crypto";
import {readFileSync,readdirSync,lstatSync} from "node:fs";
import {homedir} from "node:os";
import {join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
const root=process.cwd(),lib=join(root,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build");
const inference=await import(join(lib,"🕸️graph/📥️inference/🟨️.mjs"));
await inference.libraryBootstrap;
const observation=await import(join(lib,"👁️observation/🟨️.mjs"));
const analysts=await import(join(lib,"🕸️graph/📥️inference/🧬️source-analyses/🟨️.mjs"));
const portable=await import(join(lib,"🪪️graph-state/📦️project-records/🧳️portable/🟦️.ts"));
const rust=analysts.prepareSourceAnalysesV1().rust,host=portable.hostOfRootV1(root);
const ac=join(root,".cache/f/cache-server/v1/main/ac"),cas=join(homedir(),".cache/semio/cas/v1/o");
let entries=0,records=0,rows=0,matched=0,holds=0;
const digests=new Set<string>();
outer:for(const shard of readdirSync(ac).slice(0,8)){
 if(!/^[0-9a-f]{2}$/.test(shard))continue;
 for(const name of readdirSync(join(ac,shard))){
  if(++entries>1000||digests.size>=12)break outer;
  if(!/^[0-9a-f]{64}$/.test(name))continue;
  const entry=join(ac,shard,name),st=lstatSync(entry,{throwIfNoEntry:false});
  if(!st?.isFile()||st.size>65536)continue;
  const buffer=readFileSync(entry),offset=buffer.indexOf('{"version"');
  if(offset<0)continue;
  let metadata;try{metadata=JSON.parse(buffer.subarray(offset).toString());}catch{continue;}
  if(metadata.kind!=="project-inference")continue;
  const doc=metadata.documents?.find((d:any)=>d.name==="record.br");
  if(!doc||!/^[a-f0-9]{64}$/.test(doc.hash)||doc.size>2*1024**2)continue;
  const file=join(cas,doc.hash.slice(0,2),doc.hash.slice(2)),stat=lstatSync(file,{throwIfNoEntry:false});
  if(!stat?.isFile()||stat.size!==doc.size)continue;
  const compressed=readFileSync(file);if(createHash("sha256").update(compressed).digest("hex")!==doc.hash)continue;
  let bundle;try{bundle=portable.unpackRecordV1(JSON.parse(brotliDecompressSync(compressed,{maxOutputLength:12*1024**2}).toString()));}catch{continue;}
  records++;
  for(const row of bundle.observations){
   if(row.kind!=="analysis"||row.fact?.provider!=="build:rust-references:v1:⟦analysis-owner⟧")continue;
   rows++;if(row.fact.revision!==rust.revision)continue;
   const local=portable.localizeRowsV1([row],host)[0];matched++;
   if(await observation.observeInferenceSnapshotV1(()=>observation.observationRowFreshV1(local))===true){holds++;digests.add(doc.hash);break;}
  }
 }
}
console.log(JSON.stringify({entries,records,rows,matched,holds,distinctContentDigests:digests.size,example:[...digests].slice(0,2)}));
if(!holds)process.exitCode=1;
