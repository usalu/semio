import { catalogEntries, loadAsset, frames } from "./volumes-base.ts";
const out: any = {};
for (const e of catalogEntries().filter((e: any) => e.category.endsWith("furniture"))) {
  const a = loadAsset(e.path);
  const f = frames(a);
  out[e.slug] = Object.fromEntries(a.rig.bones.map((b: any) => [b.id, { parent: b.parent, x: b.x, y: b.y, ang: b.angleDeg, len: b.lengthM, frame: f[b.id] }]));
}
await Bun.write(import.meta.dir + "/🗑️generated/author-15/frames.json", JSON.stringify(out));
