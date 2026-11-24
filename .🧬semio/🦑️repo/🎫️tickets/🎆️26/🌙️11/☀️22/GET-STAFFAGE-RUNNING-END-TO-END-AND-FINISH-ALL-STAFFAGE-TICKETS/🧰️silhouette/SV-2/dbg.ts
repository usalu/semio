/** 🐞️ Probe of one drawn shape of a plant: its frame in the view, how its written points compare with its evaluated rings, and how many of its cells lie outside the volume.
 *  bun dbg.ts <slug> <scale> <view> <shape-id> */
import { Mask } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { frameOf, grown, load, viewOf, volumeMask } from "./lib.ts";
const [slug, scaleArg, view, id] = process.argv.slice(2) as string[];
const scale = Number(scaleArg);
const { asset } = await load(slug!);
console.log(`[DEBUG] pose ${asset.presentation.defaultPose} ${JSON.stringify(asset.poses?.find?.((p: any) => p.id === asset.presentation.defaultPose) ?? asset.poses).slice(0, 400)}`);
const v = viewOf(asset, scale, view!);
const e = v.entries.find((x) => x.shape.id === id)!;
const shape = asset.representations[String(scale)].geometry[view!].find((s: any) => s.id === id);
const f = frameOf(asset, scale, view!, shape)!;
console.log(`[DEBUG] frame o ${JSON.stringify(f.toView({ x: 0, y: 0 }))} x ${JSON.stringify(f.toView({ x: 1, y: 0 }))} y ${JSON.stringify(f.toView({ x: 0, y: 1 }))}`);
const nums = (shape.d as string).match(/-?\d+\.?\d*(?:e-?\d+)?/gu)!.map(Number);
let worst = 0;
const ring = e.rings[0]!;
for (let i = 0; i < Math.min(ring.length, nums.length / 2); i++) { const p = f.toView({ x: nums[2 * i]!, y: nums[2 * i + 1]! }); worst = Math.max(worst, Math.hypot(p.x - ring[i]!.x, p.y - ring[i]!.y)); }
console.log(`[DEBUG] ${id}: ${e.rings.length} rings, ${ring.length} points evaluated, ${nums.length / 2} written, largest difference of frame and evaluation ${worst.toFixed(5)} m; keys ${Object.keys(shape).join(",")}`);
const box = grown(e.box, 3 * v.cell);
const L = new Mask(box, v.cell); L.fill(e.rings);
const V = volumeMask(v.groups, box, v.cell);
let out = 0, n = 0; const b = { minX: Infinity, maxX: -Infinity, minY: Infinity, maxY: -Infinity };
for (let k = 0; k < L.cells.length; k++) if (L.cells[k]) { n++; if (!V.cells[k]) { out++; const x = box.minX + (k % L.width + 0.5) * v.cell, y = box.minY + (Math.floor(k / L.width) + 0.5) * v.cell; b.minX = Math.min(b.minX, x); b.maxX = Math.max(b.maxX, x); b.minY = Math.min(b.minY, y); b.maxY = Math.max(b.maxY, y); } }
console.log(`[DEBUG] ${n} cells, ${out} outside the volume, in ${JSON.stringify(b)}`);
