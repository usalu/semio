import {readFileSync} from "node:fs";
import {join} from "node:path";
const G="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🟨️.mjs";
const {openPackedFactIndexV1}=await import(G);
const [dir,digest]=process.argv.slice(2);
const index=openPackedFactIndexV1(readFileSync(join(dir,"x",digest+".facts.br")),1<<30);
const seen={};
const short=s=>JSON.stringify(s).slice(0,420);
for(let i=0;i<index.rows.length;i++){const r=index.rows[i];seen[r.kind]??=0;if(seen[r.kind]++<2 && ["resolve","field","closure","typescriptText","typescriptRealpath","derived","stamp","digest","registry","stat","realpath","analysis","readdirNames"].includes(r.kind))console.log(r.kind,short(r.path),short(r.fact));}
console.log(Object.keys(index), index.classes.length, index.owners.length);
