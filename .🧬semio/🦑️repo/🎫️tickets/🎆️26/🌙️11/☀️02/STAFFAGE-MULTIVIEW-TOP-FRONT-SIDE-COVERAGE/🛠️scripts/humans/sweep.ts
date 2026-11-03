import { readdirSync } from "node:fs";
import { L, load, save } from "./lib.ts";
for (const slug of readdirSync(`${L}/🗂️catalog/👤️humans`).sort()) {
  const d = load(slug); let n = 0;
  const visit = (node: any): void => { if (Array.isArray(node)) node.forEach(visit); else if (node && typeof node === "object") { if (node.kind === "extruded" && node.sweep === undefined && "z0" in node) { node.sweep = "w"; n++; } Object.values(node).forEach(visit); } };
  visit(d.asset.representations); visit(d.asset.symbols);
  if (n) { const o = d.asset.representations; for (const r of Object.values<any>(o)) for (const v of ["elevation","front","plan"]) if (r.geometry) for (const s of r.geometry[v]) if (s.solid?.kind==="extruded") { const { kind, sweep, z0, z1, ...rest } = s.solid; s.solid = { kind, sweep, z0, z1, ...rest }; } save(slug, d); }
  console.log(slug, "extruded solids given sweep w:", n);
}
