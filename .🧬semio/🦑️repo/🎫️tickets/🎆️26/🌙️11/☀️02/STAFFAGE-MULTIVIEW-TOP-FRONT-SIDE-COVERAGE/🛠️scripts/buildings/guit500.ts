import { readFileSync } from "node:fs";
import { load, save } from "../humans/lib.ts";
const bk = JSON.parse(readFileSync("../../🗑️generated/buildings/bk/seated-guitarist.json", "utf8")).asset;
const doc = load("seated-guitarist"); const a = doc.asset;
const bone = (id: string) => a.rig.bones.find((b: any) => b.id === id);
const rep = a.representations["500"];
for (const cap of bk.representations["500"].geometry.plan) {
  if (cap.kind !== "capsule" || !/^(upper-arm|forearm)-[lr]$/.test(cap.id)) continue;
  const L = bone(cap.bone).lengthM;
  const r = Math.max(cap.r0, cap.r1);
  const shape: any = { id: cap.id, bone: cap.bone, role: cap.role, z: cap.z, filled: cap.filled, tone: cap.tone };
  for (const k of ["region", "regions", "material", "hatch"]) if (cap[k] !== undefined) shape[k] = cap[k];
  Object.assign(shape, { kind: "poly", points: [[cap.x0, -cap.r0], [L + cap.x1, -cap.r1], [L + cap.x1, cap.r1], [cap.x0, cap.r0]], closed: true, solid: { kind: "extruded", z0: +(cap.y0 - r).toFixed(4), z1: +(cap.y0 + r).toFixed(4) } });
  const i = rep.geometry.plan.findIndex((s: any) => s.id === cap.id);
  rep.geometry.plan[i] = shape;
}
save("seated-guitarist", doc);
