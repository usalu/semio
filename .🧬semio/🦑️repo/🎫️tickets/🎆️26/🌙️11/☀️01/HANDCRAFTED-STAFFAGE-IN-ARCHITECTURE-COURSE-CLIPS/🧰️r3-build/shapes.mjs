import { readFileSync } from "node:fs";
const [f, den] = process.argv.slice(2);
const d = JSON.parse(readFileSync(f, "utf8")).asset;
const r = d.representations[den];
for (const vp of ["elevation", "front", "plan"]) {
  const g = r.geometry[vp];
  if (!g) continue;
  console.log("==", vp, g.length);
  for (const s of g) console.log(s.id, s.bone, s.kind, s.role, s.region ?? "", JSON.stringify({ x: s.x, y: s.y, w: s.w, h: s.h, cx: s.cx, cy: s.cy, rx: s.rx, ry: s.ry, solid: s.solid && s.solid.kind, pts: s.points && s.points.length }));
}
console.log(JSON.stringify(d.rig.bones.map(b=>[b.id,b.parent,b.x,b.y,b.lengthM])));
