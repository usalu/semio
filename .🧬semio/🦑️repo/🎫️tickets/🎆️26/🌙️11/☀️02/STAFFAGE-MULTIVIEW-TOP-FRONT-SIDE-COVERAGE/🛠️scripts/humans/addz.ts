import { load, save } from "./lib.ts";
const slugs = process.argv.slice(2);
for (const slug of slugs) {
  const d = load(slug); const a = d.asset;
  const plan = a.representations["50"]?.geometry?.plan ?? a.representations["100"].geometry.plan;
  for (const b of a.rig.bones) {
    if (!/^(arm|upper-arm|thigh)-[lr]$/.test(b.id) || b.z) continue;
    const c = plan.find((x: any) => x.kind === "capsule" && x.bone === b.id);
    const side = b.id.endsWith("-l") ? 1 : -1;
    const z = c ? c.y0 : side * (b.id.startsWith("thigh") ? 0.09 : 0.18);
    b.z = +z.toFixed(4); console.log(slug, b.id, "z", b.z);
  }
  let conv = 0;
  for (const r of Object.values<any>(a.representations)) {
    if (r.status !== "authored") continue;
    for (const v of ["front", "plan"]) for (const s of r.geometry[v]) {
      if (s.solid?.kind !== "revolved" || !/^(upper-arm|arm|forearm|hand|thigh|shank|foot|toe)-[lr]$/.test(s.bone)) continue;
      const rad = Math.max(0.02, ...((s.points ?? []).map((p: number[]) => Math.abs(p[1]))));
      const z = s.solid.axisZ ?? 0;
      s.solid = { kind: "extruded", z0: +(z - rad).toFixed(4), z1: +(z + rad).toFixed(4) }; conv++;
    }
  }
  console.log(slug, "converted revolved limbs:", conv);
  save(slug, d);
}
