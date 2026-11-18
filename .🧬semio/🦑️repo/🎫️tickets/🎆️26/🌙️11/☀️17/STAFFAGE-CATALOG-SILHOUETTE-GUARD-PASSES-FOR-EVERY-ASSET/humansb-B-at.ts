/** 📍 All drawn shapes (with ops applied) whose ring bounding box contains a point of a view. bun humansb-B-at.ts <asset> <scale> <view> <patch|-> <x> <y> [r] */
import { readFileSync } from "node:fs";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, sc, view, patch, xs, ys, rs] = process.argv.slice(2) as [string, string, string, string, string, string, string | undefined];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
if (patch !== "-") {
  const find = (list: any[], id: string) => list.find((x) => x.id === id);
  for (const o of JSON.parse(readFileSync(patch, "utf8"))) {
    if (!o.op.startsWith("shape")) continue;
    const list: any[] = (asset as any).representations[o.scale].geometry[o.view];
    if (o.op === "shape-add") list.splice(list.indexOf(find(list, o.after)) + 1, 0, o.shape);
    else if (o.op === "shape-set") Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o.shape;
  }
}
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as never) as any;
const [x, y, r] = [Number(xs), Number(ys), Number(rs ?? 0.01)];
for (const s of state.shapes[view]) {
  const pts = s.rings.flat();
  if (pts.length === 0) continue;
  const [x0, x1, y0, y1] = [Math.min(...pts.map((p: any) => p.x)), Math.max(...pts.map((p: any) => p.x)), Math.min(...pts.map((p: any) => p.y)), Math.max(...pts.map((p: any) => p.y))];
  if (x >= x0 - r && x <= x1 + r && y >= y0 - r && y <= y1 + r) console.log(String(s.shape.id).padEnd(28), s.shape.role, s.shape.kind, "closed", s.closed, "filled", s.shape.filled, "bone", s.shape.bone, "solid", s.shape.solid?.kind ?? "-", `x ${x0.toFixed(3)} ${x1.toFixed(3)} y ${y0.toFixed(3)} ${y1.toFixed(3)}`);
}
