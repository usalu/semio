import { readFileSync } from "node:fs";
import { solveRig, apply } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const [path, scale] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(path!, "utf8")).asset;
const world = solveRig(asset.rig, {}, { x: 0, y: 0, angleDeg: 0 });
for (const bone of asset.rig.bones) {
  const o = apply(world[bone.id]!, 0, 0);
  const t = apply(world[bone.id]!, bone.lengthM, 0);
  console.log("bone", bone.id.padEnd(18), "origin", o.x.toFixed(3), o.y.toFixed(3), "tip", t.x.toFixed(3), t.y.toFixed(3));
}
const rep = asset.representations[scale ?? "50"];
for (const s of rep.geometry.elevation) console.log("shape", s.id.padEnd(22), s.bone.padEnd(14), s.role, s.kind, JSON.stringify({ ...s, id: undefined, bone: undefined, role: undefined, kind: undefined, z: undefined, tone: undefined, material: undefined, region: undefined }).slice(0, 150));
