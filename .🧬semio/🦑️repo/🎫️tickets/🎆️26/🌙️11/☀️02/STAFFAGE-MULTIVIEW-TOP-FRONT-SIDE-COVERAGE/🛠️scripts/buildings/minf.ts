import { readFileSync } from "node:fs"; import { path } from "./sy.ts";
for (const id of process.argv.slice(2)) {
  const a = JSON.parse(readFileSync(path(id),"utf8")).asset; const g=a.representations["50"].geometry; const out:string[]=[];
  for (const v of ["elevation","front","plan"]) for (const s of g[v]) { if (["ground","texture"].includes(s.role)) continue; let d:number|undefined;
    if (s.kind==="rect") d=Math.min(s.w,s.h); else if (s.kind==="ellipse") d=2*Math.min(s.rx,s.ry); else if (s.kind==="poly"&&s.closed){const xs=s.points.map((p:any)=>p[0]),ys=s.points.map((p:any)=>p[1]); d=Math.min(Math.max(...xs)-Math.min(...xs),Math.max(...ys)-Math.min(...ys));}
    if (d!==undefined&&d>=0.008) out.push(`${d.toFixed(4)} ${v[0]}:${s.id}`); }
  out.sort(); console.log(id.padEnd(22), out.slice(0,5).join(" | "));
}
