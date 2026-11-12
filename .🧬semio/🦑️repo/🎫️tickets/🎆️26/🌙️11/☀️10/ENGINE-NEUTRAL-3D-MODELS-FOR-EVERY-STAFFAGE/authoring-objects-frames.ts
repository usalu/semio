import { catalogEntries, loadAsset, frames } from "./volume-lib.ts";
const out: any = {};
for (const e of catalogEntries().filter((e) => e.category.endsWith("objects"))) {
  const a = loadAsset(e.path);
  const f = frames(a);
  out[e.slug] = Object.fromEntries(a.rig.bones.map((b: any) => [b.id, { parent: b.parent, x: b.x, y: b.y, ang: b.angleDeg, len: b.lengthM, joint: b.joint?.type, frame: f[b.id] }]));
}
await Bun.write(import.meta.dir + "/🗑️generated/authoring-objects/frames.json", JSON.stringify(out));
