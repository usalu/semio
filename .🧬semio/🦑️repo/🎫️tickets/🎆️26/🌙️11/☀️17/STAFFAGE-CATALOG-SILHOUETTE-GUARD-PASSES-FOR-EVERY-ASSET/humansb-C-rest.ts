/** 🧭 Rest-frame polygon for a wanted current-pose polygon: probes the affine map of a bone in a view and inverts it. bun humansb-C-rest.ts <asset> <scale> <view> <bone> '<[[x,y],...] wanted world points>' [patches] */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { applyPatch } from "./humansb-C-patch.ts";
const [name, sc, view, bone, wanted, patch] = process.argv.slice(2);
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
await applyPatch(asset, patch);
const probe = { kind: "poly", points: [[0, 0], [0.1, 0], [0, 0.1]], closed: true, id: "zz-probe", bone, role: "detail", z: 99, filled: true, tone: 0.5, region: "x" };
(asset as any).representations[sc].geometry[view].push(probe);
const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } }) as any;
const r = state.shapes[view].find((s: any) => s.shape.id === "zz-probe").rings[0];
const o = r[0], ex = { x: (r[1].x - o.x) / 0.1, y: (r[1].y - o.y) / 0.1 }, ey = { x: (r[2].x - o.x) / 0.1, y: (r[2].y - o.y) / 0.1 };
const det = ex.x * ey.y - ex.y * ey.x;
const inv = (p: [number, number]) => { const dx = p[0] - o.x, dy = p[1] - o.y; return [+((dx * ey.y - dy * ey.x) / det).toFixed(5), +((-dx * ex.y + dy * ex.x) / det).toFixed(5)]; };
console.error("origin", o, "ex", ex, "ey", ey);
console.log(JSON.stringify(JSON.parse(wanted).map(inv)));
