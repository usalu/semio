/** 🗺️ Which volume solid covers each cell of a window of one view: bun vehicles-owner.ts <asset> <scale> <view> <cx> <cy> <r> [cols] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, extentOf, referenceOf, worldSolids, drawnFor } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view, cx, cy, r, colsArg] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string, string, string, string | undefined];
const scale = Number(scaleArg);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } });
const solids = worldSolids(asset, state.bones as never, scale).filter((w) => w.texture !== true);
void extentOf;
const drawn = drawnFor(asset, state, view, scale);
const num = (v: string): number => Number(v.replace(/^m/, "-"));
const [x, y, h] = [num(cx), num(cy), num(r)];
const cols = Number(colsArg ?? 100);
const cell = (2 * h) / cols;
const box = { minX: x - h, maxX: x + h, minY: y - h, maxY: y + h };
const masks = solids.map((s) => {
  const m = new Mask(box, cell);
  for (const g of referenceOf(view, [s], undefined, drawn).cut) m.fill(g, true);
  m.seal();
  return m;
});
const letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const used = new Set<number>();
const rows: string[] = [];
const w = masks[0]!.width, hh = masks[0]!.height;
for (let j = hh - 1; j >= 0; j -= 2) {
  let line = "";
  for (let i = 0; i < w; i++) {
    let c = ".";
    const hits: number[] = [];
    for (let k = 0; k < masks.length; k++) if (masks[k]!.cells[j * w + i]) hits.push(k);
    if (hits.length > 0) { c = letters[hits[hits.length - 1]! % letters.length]!; hits.forEach((k) => used.add(k)); }
    line += c;
  }
  rows.push(line);
}
console.log(rows.join("\n"));
for (const k of used) console.log(letters[k % letters.length], solids[k]!.id, solids[k]!.bone ?? "");
console.log(`window x ${(x - h).toFixed(3)}..${(x + h).toFixed(3)} y ${(y - h).toFixed(3)}..${(y + h).toFixed(3)}; char ${(cell * 1000).toFixed(1)} mm wide, ${(cell * 2000).toFixed(1)} mm tall`);
