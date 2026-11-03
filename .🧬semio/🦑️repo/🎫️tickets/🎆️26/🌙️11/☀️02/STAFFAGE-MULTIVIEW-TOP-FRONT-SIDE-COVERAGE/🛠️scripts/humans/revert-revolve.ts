import { load, save } from "./lib.ts";
import { readFileSync } from "node:fs";
const bkDir = process.argv[2]!;
for (const slug of process.argv.slice(3)) {
  const d = load(slug); const a = d.asset;
  const o = JSON.parse(readFileSync(`${bkDir}/${slug}.json`, "utf8")).asset;
  let n = 0;
  for (const [k, r] of Object.entries<any>(a.representations)) {
    if (r.status !== "authored") continue;
    for (const v of ["front", "plan"]) for (const s of r.geometry[v]) {
      if (s.solid?.kind !== "extruded") continue;
      const t = o.representations[k]?.geometry?.[v]?.find((x: any) => x.id === s.id && x.bone === s.bone);
      if (t?.solid?.kind === "revolved") { s.solid = JSON.parse(JSON.stringify(t.solid)); n++; }
    }
  }
  for (const b of a.rig.bones) { const ob = o.rig.bones.find((x: any) => x.id === b.id); if (ob && ob.z === undefined && b.z !== undefined) delete b.z; }
  console.log(slug, "restored revolved solids", n); save(slug, d);
}
