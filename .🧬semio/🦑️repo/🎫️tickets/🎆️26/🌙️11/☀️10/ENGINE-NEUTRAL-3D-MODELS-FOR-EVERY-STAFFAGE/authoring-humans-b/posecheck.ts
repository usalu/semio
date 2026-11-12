/** 🤸️ Poses a volume: for every requested pose (default: the first and a second one, `--all` for every pose) solids are carried by the evaluated bone frames; reports detached neighbours (touching at rest, far apart in the pose), solids that changed side, and the lowest point.
 * Usage: bun posecheck.ts <slug> [--pose id ...] [--all] [--volume file] */
import { readFileSync } from "node:fs";
import { tessellateSolid } from "../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🔺️tessellation/🟦️.ts";
import { evaluate } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { QUALITY_FOR, frames, loadAsset, resolveAsset, toWorld, type V3 } from "../volume-lib.ts";

const argv = process.argv.slice(2);
const slug = argv[0]!;
const poses: string[] = [];
let volumePath: string | undefined;
let all = false;
for (let i = 1; i < argv.length; i++) { if (argv[i] === "--pose") poses.push(argv[++i]!); else if (argv[i] === "--volume") volumePath = argv[++i]; else if (argv[i] === "--all") all = true; }
const asset = loadAsset(resolveAsset(slug).path);
const raw = volumePath ? JSON.parse(readFileSync(volumePath, "utf8")) : { volume: asset.volume };
const vol = raw.volume ?? raw;
const dims = [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM];
const q = QUALITY_FOR(Math.max(...dims) * 2);
const ids: string[] = asset.poses.map((p: any) => p.id);
const chosen = poses.length ? poses : all ? ids : [...new Set([asset.presentation?.defaultPose ?? ids[0], ...ids.slice(0, 2)])];
const local = vol.solids.map((s: any) => { const g = tessellateSolid({ ...s, node: s.bone } as never, q); const pts: V3[] = []; const step = Math.max(1, Math.floor(g.positions.length / 3 / 600)); for (let i = 0; i < g.positions.length / 3; i += step) pts.push([g.positions[i * 3]!, g.positions[i * 3 + 1]!, g.positions[i * 3 + 2]!]); return pts; });
const world = (fr: Record<string, any>) => vol.solids.map((s: any, n: number) => local[n]!.map((p) => toWorld(fr[s.bone], p)));
const frameOf = (state: any) => { const o: Record<string, any> = {}; for (const [id, b] of Object.entries<any>(state.bones)) o[id] = { o: [b.frame.o.x, b.frame.o.y, b.frame.o.z], u: [b.frame.u.x, b.frame.u.y, b.frame.u.z], v: [b.frame.v.x, b.frame.v.y, b.frame.v.z], w: [b.frame.w.x, b.frame.w.y, b.frame.w.z] }; return o; };
const restWorld = world(frames(asset) as any);
const minDist = (a: V3[], b: V3[]): number => { let m = 1e9; for (const p of a) for (const r of b) { const d = (p[0] - r[0]) ** 2 + (p[1] - r[1]) ** 2 + (p[2] - r[2]) ** 2; if (d < m) m = d; } return Math.sqrt(m); };
const box = (pts: V3[]) => { const mn = [1e9, 1e9, 1e9], mx = [-1e9, -1e9, -1e9]; for (const p of pts) for (let k = 0; k < 3; k++) { mn[k] = Math.min(mn[k]!, p[k]!); mx[k] = Math.max(mx[k]!, p[k]!); } return { mn, mx }; };
const parent: Record<string, string | null> = Object.fromEntries(asset.rig.bones.map((x: any) => [x.id, x.parent]));
const joined = (p: string, q: string) => parent[p] === q || parent[q] === p;
const n = vol.solids.length;
const pairs: { a: number; b: number; d0: number }[] = [];
const rb = restWorld.map(box);
for (let a = 0; a < n; a++) for (let b = a + 1; b < n; b++) {
  if (vol.solids[a].bone === vol.solids[b].bone || !joined(vol.solids[a].bone, vol.solids[b].bone)) continue;
  const A = rb[a]!, B = rb[b]!;
  if ([0, 1, 2].some((k) => A.mn[k]! > B.mx[k]! + 0.03 || B.mn[k]! > A.mx[k]! + 0.03)) continue;
  const d0 = minDist(restWorld[a]!, restWorld[b]!);
  if (d0 <= 0.025) pairs.push({ a, b, d0 });
}
const mean = (pts: V3[], k: number) => pts.reduce((s, p) => s + p[k]!, 0) / pts.length;
let bad = 0;
for (const id of chosen) {
  const st: any = evaluate(asset, { kind: "pose", pose: id }, { kind: "scale", scale: { denominator: 50 } } as any);
  const pw = world(frameOf(st));
  const issues: string[] = [];
  const joints = new Map<string, { d0: number; d: number; a: string; b: string }>();
  for (const p of pairs) {
    const d = minDist(pw[p.a]!, pw[p.b]!);
    const A = vol.solids[p.a].bone, B = vol.solids[p.b].bone;
    const key = A < B ? `${A}|${B}` : `${B}|${A}`;
    const cur = joints.get(key);
    if (!cur) joints.set(key, { d0: p.d0, d, a: vol.solids[p.a].id, b: vol.solids[p.b].id });
    else { cur.d0 = Math.min(cur.d0, p.d0); cur.d = Math.min(cur.d, d); }
  }
  for (const [key, j] of joints) if (j.d - j.d0 > 0.03) issues.push(`detached bones ${key}: nearest solids ${(j.d0 * 1000).toFixed(0)} mm at rest, ${(j.d * 1000).toFixed(0)} mm posed`);
  const all2 = pw.flat();
  const zc = mean(all2, 2);
  vol.solids.forEach((s: any, k: number) => {
    const side = /-l$/.test(s.id) ? 1 : /-r$/.test(s.id) ? -1 : 0;
    if (!side) return;
    const zr = mean(restWorld[k]!, 2) - mean(restWorld.flat(), 2), zp = mean(pw[k]!, 2) - zc;
    if (Math.sign(zr) === side && Math.sign(zp) !== side && Math.abs(zp) > 0.03) issues.push(`wrong side ${s.id}: z ${zr.toFixed(3)} at rest, ${zp.toFixed(3)} posed`);
  });
  const low = Math.min(...all2.map((p) => p[1]));
  const bb = box(all2);
  const ext = [0, 1, 2].map((k) => (bb.mx[k]! - bb.mn[k]!).toFixed(3)).join(" x ");
  console.log(`${asset.id} pose ${id}${id === asset.presentation?.defaultPose ? " (DEFAULT)" : ""}: ${pairs.length} touching pairs, lowest y ${low.toFixed(3)} m, bbox ${ext} (dimensions ${dims.join(" x ")}), ${issues.length} issues`);
  for (const i of issues.slice(0, 25)) console.log("   " + i);
  bad += issues.length;
}
process.exit(bad ? 1 : 0);
