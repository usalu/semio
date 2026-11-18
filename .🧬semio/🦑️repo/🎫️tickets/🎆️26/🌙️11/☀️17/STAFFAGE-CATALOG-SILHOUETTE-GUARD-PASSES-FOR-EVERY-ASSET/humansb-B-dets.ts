/** 🧭 Determinant of the carry of a flat shape per bone in one view. bun humansb-B-dets.ts <asset> <scale> <view> */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
const [name, sc, view] = process.argv.slice(2) as [string, string, string];
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const base = ((await readStreamedAsset(entry)) as { asset: Asset }).asset as any;
const bones: string[] = base.rig.bones.map((b: any) => b.id);
const a = structuredClone(base);
const list = a.representations[sc].geometry[view];
for (const b of bones) list.push({ kind: "poly", points: [[0, 0], [1, 0], [0, 1], [0.1, 0.1], [0.3, 0.1], [0.1, 0.3], [-0.1, -0.1]], closed: true, id: "__p" + b, bone: b, role: "body", z: 9, filled: true, tone: 0.5 });
const st = evaluate(a, { kind: "pose", pose: a.presentation.defaultPose }, { kind: "scale", scale: { denominator: Number(sc) } } as never) as any;
for (const b of bones) { const s = st.shapes[view].find((x: any) => x.shape.id === "__p" + b); const [p0, p1, p2] = s.rings[0]; const det = (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y); const q = s.rings[0]; const e = [3, 4, 5, 6].map((k) => { const pr = [[0.1, 0.1], [0.3, 0.1], [0.1, 0.3], [-0.1, -0.1]][k - 3]!; const px = p0.x + pr[0]! * (p1.x - p0.x) + pr[1]! * (p2.x - p0.x), py = p0.y + pr[0]! * (p1.y - p0.y) + pr[1]! * (p2.y - p0.y); return Math.hypot(q[k].x - px, q[k].y - py); }); console.log(b.padEnd(14), det.toFixed(2), "affine err mm", Math.max(...e.map((v) => v * 1000)).toFixed(1)); }
