// fact-regions.mjs <generations-dir> <factIndexDigest>: counts of fact rows by kind x region (workspace-relative top-level, excluded, outside), streaming over openPackedFactIndexV1.
import {readFileSync} from "node:fs";
import {join} from "node:path";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const {openPackedFactIndexV1,GENERATION_POLICY_V1}=await import(G);
const [dir,digest]=process.argv.slice(2);
const bytes=readFileSync(join(dir,"x",digest+".facts.br"));
const index=openPackedFactIndexV1(bytes,1<<30);
const root="/Users/ueli/Documents/semio/";
const counts=new Map();
let n=0;
const bump=(k)=>counts.set(k,(counts.get(k)??0)+1);
const excluded=/(^|\/)(\.git|\.cache|node_modules|target|\.nx|dist|build|out)(\/|$)/;
for(let i=0;i<index.rows.length;i++){
  const row=index.rows[i]; n++;
  let p=row.path, region;
  if(p.startsWith(root)) p=p.slice(root.length);
  if(p.startsWith("/")) region="outside-abs";
  else if(p.startsWith("..")) region="outside-rel";
  else if(excluded.test(p)) region="excluded:"+(p.match(excluded)[2]);
  else region="inside";
  bump(row.kind+" | "+region);
}
console.log("rows",n,"rss MB",Math.round(process.memoryUsage().rss/1048576));
for(const [k,v] of [...counts].sort((a,b)=>b[1]-a[1]).slice(0,40)) console.log(String(v).padStart(8),k);
