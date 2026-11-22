/** 🔍️ Writes an SVG overlay of the drawing (red) and the volume projection (blue) of one asset, scale and view in the default pose (or POSE), the two sides the silhouette guard compares: bun overlay.ts <slug> <scale> <view> */
import { join } from "node:path";
import { measureAsset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { featureOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { GENERATED, HUMANS } from "./lib.ts";
const [slug, scale, view] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan"];
const asset = (await Bun.file(join(HUMANS, slug, "🔣️.json")).json()).asset;
const m = measureAsset(asset, Number(scale), process.env.POSE ?? undefined, undefined, featureOf(Number(scale) as never));
const v = m.views[view];
const all = [...v.filled, ...v.reference].flat(2);
const [x0, x1, y0, y1] = [Math.min(...all.map((p) => p.x)), Math.max(...all.map((p) => p.x)), Math.min(...all.map((p) => p.y)), Math.max(...all.map((p) => p.y))];
const k = 800 / Math.max(x1 - x0, y1 - y0);
const path = (groups: readonly (readonly (readonly { x: number; y: number }[])[])[], fill: string) => groups.map((g) => `<path d="${g.map((r) => `M${r.map((p) => `${((p.x - x0) * k + 10).toFixed(1)},${((y1 - p.y) * k + 10).toFixed(1)}`).join("L")}Z`).join("")}" fill="${fill}" fill-opacity="0.4" fill-rule="evenodd"/>`).join("");
const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="${(x1 - x0) * k + 20}" height="${(y1 - y0) * k + 20}"><rect width="100%" height="100%" fill="white"/>${path(v.reference, "blue")}${path(v.filled, "red")}${path(v.outlines, "green")}</svg>`;
const out = join(GENERATED, slug, `overlay-${scale}-${view}.svg`);
await Bun.write(out, svg);
console.log(out, v.reference.length, v.filled.length, v.outlines.length, `iou ${v.iou.toFixed(3)} H ${(v.hausdorffM * 1000).toFixed(0)}mm`);
