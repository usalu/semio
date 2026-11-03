import { L, load, save } from "./lib.ts";
const { evaluate } = await import(`${L}/🔨️modules/🧮️evaluation/🟦️.ts`);
const { extentOf } = await import(`${L}/🧪️tests/🧊️three-views/🧰️.ts`);
const write = process.argv[2] === "write";
const fresh = (a: any) => JSON.parse(JSON.stringify(a));
const snap = (a0: any) => { const a = fresh(a0); const out: any = {}; for (const r of Object.values<any>(a.representations)) { if (r.status !== "authored") continue; for (const pose of a.poses) { const st = evaluate(a, { kind: "pose", pose: pose.id }, { kind: "scale", scale: { denominator: r.denominator } }); for (const v of ["elevation", "front", "plan"]) out[`${r.denominator}/${pose.id}/${v}`] = extentOf(st.shapes[v]); } } return out; };
const diff = (before: any, after: any, tol: number) => { let worst = 0; const rows: string[] = []; for (const k of Object.keys(before)) for (const e of ["minX", "maxX", "minY", "maxY"]) { const dd = Math.abs(before[k][e] - after[k][e]); worst = Math.max(worst, dd); if (dd > tol) rows.push(`${k}.${e}=${dd.toFixed(3)}`); } return { worst, rows }; };
for (const slug of process.argv.slice(3)) {
  const d = load(slug); const a = d.asset; const before = snap(a);
  const plan = a.representations["50"]?.geometry?.plan ?? a.representations["100"].geometry.plan;
  const byId = new Map<string, any>(a.rig.bones.map((b: any) => [b.id, b]));
  const worldZ = (id: string): number => { let z = 0; for (let b = byId.get(id); b; b = b.parent ? byId.get(b.parent) : undefined) z += b.z ?? 0; return z; };
  const oldZ = new Map<string, number>(a.rig.bones.map((b: any) => [b.id, worldZ(b.id)]));
  for (const b of a.rig.bones) {
    if (!/^(arm|upper-arm|thigh)-[lr]$/.test(b.id) || b.z) continue;
    const c = plan.find((x: any) => x.kind === "capsule" && x.bone === b.id);
    b.z = +(c ? c.y0 : (b.id.endsWith("-l") ? 1 : -1) * (b.id.startsWith("thigh") ? 0.09 : 0.18)).toFixed(4);
  }
  let n = 0, m = 0;
  for (const r of Object.values<any>(a.representations)) {
    if (r.status !== "authored") continue;
    for (const v of ["elevation", "front", "plan"]) for (const s of r.geometry[v]) {
      const dz = worldZ(s.bone) - (oldZ.get(s.bone) ?? 0);
      if (s.solid?.kind === "revolved") { const dr = (oldZ.get(s.solid.axisBone) ?? 0) - worldZ(s.solid.axisBone); if (Math.abs(dr) > 1e-9) { s.solid.axisZ = +((s.solid.axisZ ?? 0) + dr).toFixed(5); n++; } }
      if (process.env.CAPS && v === "plan" && s.kind === "capsule" && s.followTip && Math.abs(dz) > 1e-9) { s.y0 = +(s.y0 - dz).toFixed(5); s.y1 = +(s.y1 - dz).toFixed(5); m++; }
    }
  }
  const { worst, rows } = diff(before, snap(a), 0.002);
  console.log(slug, "revolved axisZ", n, "plan capsules", m, "worst change over ALL poses/views (m)", worst.toFixed(4), rows.slice(0, 4).join(" "), write ? "(write if < 2 mm)" : "(dry)");
  if (write && worst < 0.002) { save(slug, d); console.log("  WRITTEN"); }
}
