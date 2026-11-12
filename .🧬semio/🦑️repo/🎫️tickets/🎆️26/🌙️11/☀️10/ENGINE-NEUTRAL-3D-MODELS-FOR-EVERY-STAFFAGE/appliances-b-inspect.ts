import { frames, loadAsset, resolveAsset } from "./volume-lib.ts";
const r = (x: number) => Math.round(x * 1e4) / 1e4;
const a = loadAsset(resolveAsset(process.argv[2]!).path);
const f = frames(a);
console.log("dims", JSON.stringify(a.dimensions));
console.log("materials", a.materials.map((m: any) => m.id).join(","));
console.log("regions", a.thermal.regions.map((m: any) => m.id).join(","));
console.log("BONES");
for (const b of a.rig.bones) {
  const fr = f[b.id]!;
  console.log(` ${b.id} parent=${b.parent} len=${b.lengthM} joint=${JSON.stringify(b.joint)} o=${fr.o.map(r)} u=${fr.u.map(r)} v=${fr.v.map(r)} w=${fr.w.map(r)}`);
}
const reps = Object.keys(a.representations);
console.log("scales", reps.join(","));
for (const d of reps) {
  const g = a.representations[d].geometry;
  if (!g) { console.log("rep", d, JSON.stringify(a.representations[d]).slice(0, 200)); continue; }
  const ids: Record<string, string[]> = {};
  for (const v of ["elevation", "front", "plan"]) ids[v] = (g[v] ?? []).map((s: any) => s.id);
  if (d !== "50") { console.log("rep", d, "ids", JSON.stringify(ids).slice(0, 700)); continue; }
  for (const v of ["elevation", "front", "plan"]) for (const s of g[v] ?? []) {
    const { id, bone, role, z, filled, tone, kind, region, regions, material, ...rest } = s;
    console.log(` [${v}] ${id} b=${bone} ${role} t=${tone} r=${region ?? regions} m=${material} ${kind} ${JSON.stringify(rest).slice(0, 400)}`);
  }
}
console.log("POSES");
for (const p of a.poses) console.log(" ", p.id, JSON.stringify(p.bones));
console.log("CLIPS");
for (const c of a.clips ?? []) console.log(" ", c.id, JSON.stringify(c).slice(0, 500));
