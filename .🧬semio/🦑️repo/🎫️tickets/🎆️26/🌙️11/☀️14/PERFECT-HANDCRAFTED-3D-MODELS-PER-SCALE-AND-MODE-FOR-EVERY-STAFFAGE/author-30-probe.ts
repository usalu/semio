import { readFileSync } from "node:fs";
import { measureAsset } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { Mask, entryOf, featureM, loadSource, merged, poseOf } from "./volumes-lib.ts";
const [spec, scaleArg, view, pts] = process.argv.slice(2);
const { entry, asset } = entryOf(spec!);
const source = loadSource(entry);
const m = merged(asset, source);
const scale = Number(scaleArg);
const pm = measureAsset(m as never, scale, poseOf(m), undefined, featureM(scale, m));
const v = (pm.views as any)[view!];
const drawn = new Mask(v.box, v.cell);
const vol = new Mask(v.box, v.cell);
for (const g of v.filled) drawn.fill(g);
for (const g of v.outlines) drawn.fill(g);
for (const g of v.reference) vol.fill(g);
const at = (m: Mask, x: number, y: number) => { const i = Math.floor((x - v.box.minX) / v.cell), j = Math.floor((y - v.box.minY) / v.cell); return m.cells[j * m.width + i]; };
for (const p of pts!.split(";")) { const [x, y] = p.split(",").map(Number); console.log(x, y, "drawn", at(drawn, x!, y!), "3d", at(vol, x!, y!)); }
console.log("filled count", v.filled.length, "outlines", v.outlines.length);
