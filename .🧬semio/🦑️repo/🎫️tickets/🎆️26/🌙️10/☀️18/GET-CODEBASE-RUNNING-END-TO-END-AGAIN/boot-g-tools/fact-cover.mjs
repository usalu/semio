// fact-cover.mjs <generations-dir> <factIndexDigest> <frontierDigest>: which fact directories lie outside the frontier directory set (by kind / top-level)
import {readFileSync} from "node:fs";
import {join} from "node:path";
import {brotliDecompressSync} from "node:zlib";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const {openPackedFactIndexV1}=await import(G);
const [dir,digest,fdigest]=process.argv.slice(2);
const index=openPackedFactIndexV1(readFileSync(join(dir,"x",digest+".facts.br")),1<<30);
const frontier=JSON.parse(brotliDecompressSync(readFileSync(join(dir,"f",fdigest+".json.br"))).toString());
const dirs=new Set(frontier.directories.map(d=>d[0]));
const root="/Users/ueli/Documents/semio/";
const excl=/(^|\/)(\.git|\.cache|node_modules|target|\.nx|dist|build|out)(\/|$)/;
const out=new Map(), pathKinds=new Set(["file","analysis","exists","fileExists","directoryExists","lstat","stat","readdir","readdirNames","realpath","typescriptText","typescriptRealpath","digest","field","stamp"]);
let total=0, notIn=0, abs=0, opaque=0;
for(let i=0;i<index.rows.length;i++){
  const r=index.rows[i]; total++;
  if(!pathKinds.has(r.kind)){opaque++;continue;}
  let p=r.path; if(p.startsWith(root)) p=p.slice(root.length); else if(p.startsWith("/")){abs++;continue;}
  if(excl.test(p)) continue;
  const isDirKind=["readdir","readdirNames","directoryExists"].includes(r.kind);
  const d= isDirKind? p : (p.includes("/")? p.slice(0,p.lastIndexOf("/")) : ".");
  if(!dirs.has(d)){ notIn++; const k=r.kind+" | "+d.split("/").slice(0,3).join("/"); out.set(k,(out.get(k)??0)+1);}
}
console.log({total,opaque,abs,notIn,frontierDirs:dirs.size});
for(const [k,v] of [...out].sort((a,b)=>b[1]-a[1]).slice(0,25)) console.log(String(v).padStart(7),k);
