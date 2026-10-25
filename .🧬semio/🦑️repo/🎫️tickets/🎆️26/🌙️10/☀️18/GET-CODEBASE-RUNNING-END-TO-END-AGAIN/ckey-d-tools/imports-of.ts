import {readFileSync} from "node:fs";
import {dirname,resolve,relative} from "node:path";
import {closure} from "./closure-of.ts";
const t=new Bun.Transpiler({loader:"ts"});
const file=resolve(process.argv[2]!);
const imports=t.scanImports(readFileSync(file,"utf8"));
const rows=[...new Set(imports.filter(r=>r.path.startsWith(".")).map(r=>r.path))].map(p=>{const f=resolve(dirname(file),p);return {p,size:closure([f]).size};});
console.log("total",closure([file]).size);
for(const r of rows.sort((a,b)=>b.size-a.size))console.log(String(r.size).padStart(5),r.p.replace(/^(\.\.\/)+/u,"../").slice(-100));
