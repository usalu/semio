import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
const a = JSON.parse(readFileSync(path(process.argv[2]),"utf8")).asset; const sc=process.argv[3];
const r=a.representations[sc]; console.log(JSON.stringify(r.pens), JSON.stringify(r.validRange), JSON.stringify(r.hatches));
for (const v of ["elevation","front","plan"]) console.log(v, r.geometry[v].map((s:any)=>`${s.id}[${s.bone},${s.role[0]},${s.filled?"F":"o"},${s.kind[0]}]`).join(" "));
console.log(JSON.stringify(r.decisions).slice(0,1500));
