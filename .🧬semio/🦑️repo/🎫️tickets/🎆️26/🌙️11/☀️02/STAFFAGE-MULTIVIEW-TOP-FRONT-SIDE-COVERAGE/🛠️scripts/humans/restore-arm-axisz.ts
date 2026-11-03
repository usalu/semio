import { load, save } from "./lib.ts";
for (const slug of process.argv.slice(2)) {
  const d = load(slug); let n = 0;
  for (const r of Object.values<any>(d.asset.representations)) {
    if (r.status !== "authored") continue;
    for (const s of r.geometry.front) {
      const m = /^(upper-arm|arm|forearm|hand)-([lr])$/.exec(s.bone);
      if (!m || s.solid?.kind !== "revolved") continue;
      const plan = r.geometry.plan.find((x: any) => x.bone === s.bone && x.kind === "capsule") ?? r.geometry.plan.find((x: any) => /^(upper-arm|arm)-[lr]$/.test(x.bone) && x.bone.endsWith("-" + m[2]) && x.kind === "capsule");
      if (!plan) continue;
      const target = +(plan.y0).toFixed(5);
      if (Math.abs((s.solid.axisZ ?? 0) - target) > 1e-4) { s.solid.axisZ = target; n++; }
    }
  }
  console.log(slug, "front arm axisZ restored", n); save(slug, d);
}
