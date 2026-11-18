/** 🔎 Drawn shapes of one view that cover a world point: bun vehicles-who.ts <asset> <scale> <view> <x> <y> [slack-m] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";

const [name, scaleArg, view, xs, ys, slack] = process.argv.slice(2) as [string, string, string, string, string, string | undefined];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(scaleArg) } }) as any;
const num = (v: string): number => Number(v.replace(/^m/, "-"));
const [x, y, s] = [num(xs), num(ys), Number(slack ?? 0.01)];
const pen = (asset.representations as any)[scaleArg].pens;
for (const sh of state.shapes[view]) {
  const pts = sh.rings.flat();
  if (pts.length === 0) continue;
  const [x0, x1, y0, y1] = [Math.min(...pts.map((p: any) => p.x)), Math.max(...pts.map((p: any) => p.x)), Math.min(...pts.map((p: any) => p.y)), Math.max(...pts.map((p: any) => p.y))];
  if (x < x0 - s || x > x1 + s || y < y0 - s || y > y1 + s) continue;
  const near = sh.rings.some((r: any) => r.some((p: any, i: number) => { const q = r[(i + 1) % r.length]; const dx = q.x - p.x, dy = q.y - p.y; const l2 = dx * dx + dy * dy || 1; const t = Math.max(0, Math.min(1, ((x - p.x) * dx + (y - p.y) * dy) / l2)); return Math.hypot(p.x + t * dx - x, p.y + t * dy - y) <= s + (sh.shape.filled === false ? ((sh.shape.penMm ?? pen[sh.shape.role] ?? 0) * Number(scaleArg)) / 2000 : 0); }));
  const inside = sh.rings.reduce((c: number, r: any) => c + r.reduce((k: number, p: any, i: number) => { const q = r[(i + 1) % r.length]; return k + ((p.y > y) !== (q.y > y) && x < ((q.x - p.x) * (y - p.y)) / (q.y - p.y) + p.x ? 1 : 0); }, 0), 0) % 2 === 1;
  console.log(String(sh.shape.id).padEnd(24), sh.shape.role.padEnd(8), sh.shape.filled === false ? "stroke" : "filled", near ? "EDGE-NEAR" : inside ? "INSIDE" : "bbox-only", `x ${x0.toFixed(3)}..${x1.toFixed(3)} y ${y0.toFixed(3)}..${y1.toFixed(3)}`, sh.shape.penMm ?? "");
}
