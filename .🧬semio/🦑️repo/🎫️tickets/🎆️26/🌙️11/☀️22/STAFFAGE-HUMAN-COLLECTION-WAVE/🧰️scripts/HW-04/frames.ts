/** 🗺️ Prints the grounded 3D frames (o, u, v, w) of chosen bones of an asset JSON for pose offsets: bun frames.ts <asset.json> '<offsets json>' '<contacts csv>' bone... */
import { join, resolve } from "node:path";
const S = resolve(import.meta.dir, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage");
const { solveChain } = await import(join(S, "🔨️modules/🦴️rig/🟦️.ts"));
const [path, offs, contacts, ...ids] = process.argv.slice(2);
const asset = (await Bun.file(path!).json()).asset;
const o = JSON.parse(offs!);
const full = Object.fromEntries(Object.entries<any>(o).map(([k, v]) => [k, { angleDeg: v.angleDeg ?? 0, abductionDeg: v.abductionDeg ?? 0, twistDeg: v.twistDeg ?? 0, dx: 0, dy: 0 }]));
const w = solveChain(asset.rig, full, { x: 0, y: 0, angleDeg: 0 });
const pt = (f: any, p: number[]) => [0, 1, 2].map((k) => f.o[k] + p[0]! * f.u[k] + p[1]! * f.v[k] + (p[2] ?? 0) * f.w[k]);
let low = Infinity;
for (const c of contacts!.split(",").filter(Boolean)) { const a = asset.anchors.find((x: any) => x.id === c); low = Math.min(low, pt(w[a.bone].frame, [a.x, a.y, a.z ?? 0])[1]!); }
if (!Number.isFinite(low)) low = 0;
const f3 = (v: number[]) => `(${v.map((x) => x.toFixed(4)).join(", ")})`;
console.log(`ground shift ${(-low).toFixed(4)}`);
for (const id of ids) { const f = w[id].frame; console.log(`${id}: o ${f3([f.o[0], f.o[1] - low, f.o[2]])} u ${f3(f.u)} v ${f3(f.v)} w ${f3(f.w)}`); }
