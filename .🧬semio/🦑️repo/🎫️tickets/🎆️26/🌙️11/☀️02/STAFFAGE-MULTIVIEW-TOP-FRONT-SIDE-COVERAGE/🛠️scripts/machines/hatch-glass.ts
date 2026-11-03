import { load, save } from "./lib.ts";
for (const id of process.argv.slice(2)) { const d = load(id); let n = 0;
  for (const r of Object.values<any>(d.asset.representations)) if (r.status === "authored") for (const v of ["elevation", "front", "plan"]) for (const s of r.geometry[v]) if (s.role === "glazing" && s.hatch !== undefined) { delete s.hatch; n++; }
  for (const sy of d.asset.symbols) for (const v of ["elevation", "front", "plan"]) for (const s of sy.geometry[v]) if (s.role === "glazing" && s.hatch !== undefined) { delete s.hatch; n++; }
  if (n) { save(id, d); console.log(id, "removed hatch from", n, "glazing shapes"); } }
