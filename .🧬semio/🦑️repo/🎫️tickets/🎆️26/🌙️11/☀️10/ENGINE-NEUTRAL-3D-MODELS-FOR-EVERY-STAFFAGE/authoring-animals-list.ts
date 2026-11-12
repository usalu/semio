import { frames, loadAsset, resolveAsset, restState, toLocal } from "./volume-lib.ts";
const asset = loadAsset(resolveAsset(process.argv[2]!).path);
const st = restState(asset, 50);
const f = frames(asset);
const r = (x: number) => (Math.round(x * 1000) / 1000).toString();
const lat = (id: string) => { for (const v of ["plan", "front"] as const) { const e = st[v].find((q) => q.shape.id === id); if (e) { const z = e.rings.flat().map((p) => (v === "plan" ? p.y : p.x)); return `${v[0]}:z${r(Math.min(...z))}..${r(Math.max(...z))}`; } } return "-"; };
for (const e of st.elevation) {
  const s = e.shape;
  if (s.role === "texture" || s.role === "ground") continue;
  const loc = e.rings.flat().map((p) => toLocal(f[s.bone]!, [p.x, p.y, 0]));
  if (!loc.length) continue;
  const x = e.rings.flat();
  console.log(`${s.id} | ${s.bone} | ${s.role} ${s.kind}${s.solid ? "/" + s.solid.kind : ""}${e.closed ? "" : " OPEN"} | ${s.region ?? s.regions ?? ""} | world x${r(Math.min(...x.map((p) => p.x)))}..${r(Math.max(...x.map((p) => p.x)))} y${r(Math.min(...x.map((p) => p.y)))}..${r(Math.max(...x.map((p) => p.y)))} | ${lat(s.id)}`);
}
console.log("-- front/plan only:");
const ids = new Set(st.elevation.map((e) => e.shape.id));
for (const v of ["front", "plan"] as const) for (const e of st[v]) { const s = e.shape; if (ids.has(s.id) || s.role === "texture" || s.role === "ground") continue; const x = e.rings.flat(); if (!x.length) continue; ids.add(s.id); console.log(`${v} ${s.id} | ${s.bone} | ${s.role} ${s.kind} | ${s.region ?? s.regions ?? ""} | ${v === "plan" ? `x${r(Math.min(...x.map((p) => p.x)))}..${r(Math.max(...x.map((p) => p.x)))} z${r(Math.min(...x.map((p) => p.y)))}..${r(Math.max(...x.map((p) => p.y)))}` : `z${r(Math.min(...x.map((p) => p.x)))}..${r(Math.max(...x.map((p) => p.x)))} y${r(Math.min(...x.map((p) => p.y)))}..${r(Math.max(...x.map((p) => p.y)))}`}`); }
