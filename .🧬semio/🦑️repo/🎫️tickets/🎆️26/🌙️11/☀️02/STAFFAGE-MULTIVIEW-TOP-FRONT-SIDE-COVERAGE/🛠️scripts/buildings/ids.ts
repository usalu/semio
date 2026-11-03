import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
for (const id of process.argv.slice(2)) {
  const a = JSON.parse(readFileSync(path(id),"utf8")).asset;
  const out: string[] = [];
  for (const sc of [50,100,200]) {
    const g = a.representations[sc].geometry;
    const bones = new Map<string, Set<string>>();
    for (const v of ["elevation","front","plan"]) for (const s of g[v]) { if (s.role==="ground"||s.role==="texture") continue; if(!bones.has(s.bone)) bones.set(s.bone,new Set()); bones.get(s.bone)!.add(v); }
    const miss = [...bones].filter(([,v])=>v.size<3).map(([b,v])=>`${b}:${["elevation","front","plan"].filter(x=>!v.has(x)).map(x=>x[0]).join("")}`);
    if (miss.length) out.push(`${sc}: no-view ${miss.join(" ")}`);
  }
  console.log(id.padEnd(26), out.join(" | "));
}
