/** 🧲 Snaps the vertices of drawn polygons that lie on the outer boundary of the drawing onto the nearest boundary of the volume silhouette, and inserts vertices where the volume boundary has none nearby. Prints the resulting `shape-set` ops (JSON) for humans-try.ts and humans-ops.py.
 *  bun humansb-A-snap.ts <asset> <scale> <view> <out.json> [rounds]   env PATCH=ops.json applies a patch first, env ONLY=id,id restricts the shapes, env TOL=0.3 (fraction of the limit), env MAXD=0.12 (m) */
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { assetPathsFromFiles, readStreamedAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/📥️files/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, boundaryOf, fillSmallHoles, measureView, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🧰️support/🌓️silhouette/🟦️.ts";

const [name, scaleArg, view, out, roundsArg] = process.argv.slice(2) as [string, string, "elevation" | "front" | "plan", string, string | undefined];
const scale = Number(scaleArg);
const feature = featureOf(scale);
const limit = 2 * feature;
const tol = Number(process.env.TOL ?? 0.3) * limit;
const maxd = Number(process.env.MAXD ?? 0.12);
const only = process.env.ONLY === undefined ? undefined : new Set(process.env.ONLY.split(","));
const entry = (await assetPathsFromFiles()).find((e) => e.folder === name)!;
const { asset } = (await readStreamedAsset(entry)) as { asset: Asset };
const find = (list: any[], id: string) => list.find((x) => x.id === id);
if (process.env.PATCH) {
  for (const o of JSON.parse(await Bun.file(process.env.PATCH).text())) {
    const shapes = (): any[] => (asset as any).representations[o.scale].geometry[o.view];
    const solids = (): any[] => (asset as any).volumes[o.scale].body;
    const [list, key] = o.op.startsWith("shape") ? [shapes(), "shape"] : [solids(), "solid"];
    if (o.op.endsWith("-add")) list.splice(list.indexOf(find(list, o.after)) + 1, 0, o[key]);
    else if (o.op.endsWith("-set")) Object.assign(find(list, o.id), o.set);
    else list[list.indexOf(find(list, o.id))] = o[key];
  }
}
const geometry = (): any[] => (asset as any).representations[scaleArg].geometry[view];
const touched = new Set<string>();

function affine(rest: number[][], world: { x: number; y: number }[]): { a: number[]; b: number[]; err: number } | undefined {
  const n = rest.length;
  const S = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
  const rx = [0, 0, 0], ry = [0, 0, 0];
  for (let i = 0; i < n; i++) {
    const v = [rest[i]![0]!, rest[i]![1]!, 1];
    for (let r = 0; r < 3; r++) { for (let c = 0; c < 3; c++) S[r]![c]! += v[r]! * v[c]!; rx[r]! += v[r]! * world[i]!.x; ry[r]! += v[r]! * world[i]!.y; }
  }
  const solve = (rhs: number[]): number[] | undefined => {
    const M = S.map((row, i) => [...row, rhs[i]!]);
    for (let c = 0; c < 3; c++) {
      let p = c; for (let r = c + 1; r < 3; r++) if (Math.abs(M[r]![c]!) > Math.abs(M[p]![c]!)) p = r;
      if (Math.abs(M[p]![c]!) < 1e-12) return undefined;
      [M[c], M[p]] = [M[p]!, M[c]!];
      for (let r = 0; r < 3; r++) if (r !== c) { const f = M[r]![c]! / M[c]![c]!; for (let k = c; k < 4; k++) M[r]![k]! -= f * M[c]![k]!; }
    }
    return [M[0]![3]! / M[0]![0]!, M[1]![3]! / M[1]![1]!, M[2]![3]! / M[2]![2]!];
  };
  const a = solve(rx), b = solve(ry);
  if (a === undefined || b === undefined) return undefined;
  let err = 0;
  for (let i = 0; i < n; i++) err = Math.max(err, Math.hypot(a[0]! * rest[i]![0]! + a[1]! * rest[i]![1]! + a[2]! - world[i]!.x, b[0]! * rest[i]![0]! + b[1]! * rest[i]![1]! + b[2]! - world[i]!.y));
  return { a, b, err };
}

const pathPoints = (d: string): number[][] | undefined => {
  if (!/^[MLZ\s\d.,eE+-]+$/.test(d) || (d.match(/M/g) ?? []).length !== 1) return undefined;
  const nums = (d.match(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi) ?? []).map(Number);
  const out: number[][] = [];
  for (let i = 0; i + 1 < nums.length; i += 2) out.push([nums[i]!, nums[i + 1]!]);
  return out;
};
const pointsOf = (s: any): number[][] | undefined => (s.kind === "poly" ? s.points : s.kind === "path" ? pathPoints(s.d) : undefined);
const writePoints = (s: any, pts: number[][]): void => {
  if (s.kind === "poly") s.points = pts;
  else s.d = "M" + pts.map((p) => `${p[0]!.toFixed(5)} ${p[1]!.toFixed(5)}`).join(" L") + " Z";
};
const snapshot = (): Map<string, any> => new Map(geometry().filter((g) => touched.has(g.id)).map((g) => [g.id, g.kind === "poly" ? { points: JSON.parse(JSON.stringify(g.points)) } : { d: g.d }]));
function measure(): { m: any; state: any; solids: any[] } {
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } }) as any;
  const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
  const m = measureView(asset, state, solids, view as never, scale, undefined, feature, Infinity, false);
  return { m, state, solids };
}
function masks(m: any) {
  const drawn = new Mask(m.box, m.cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  const vol = new Mask(m.box, m.cell);
  for (const g of m.reference) vol.fill(g, true);
  if (m.section) for (const g of m.visible) vol.fill(g, true);
  vol.seal();
  const [w, h] = [vol.width, vol.height];
  const holes = Math.round((4 * feature / m.cell) ** 2);
  const bd = boundaryOf(fillSmallHoles(drawn.cells, w, h, holes), w, h);
  const bv = boundaryOf(fillSmallHoles(vol.cells, w, h, holes), w, h);
  const pts = (b: Uint8Array) => { const o: { x: number; y: number }[] = []; for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (b[j * w + i]) o.push({ x: m.box.minX + (i + 0.5) * m.cell, y: m.box.minY + (j + 0.5) * m.cell }); return o; };
  return { bd: pts(bd), bv: pts(bv) };
}
const nearest = (p: { x: number; y: number }, list: { x: number; y: number }[]) => { let best = Infinity, q = list[0]!; for (const c of list) { const d = (c.x - p.x) ** 2 + (c.y - p.y) ** 2; if (d < best) { best = d; q = c; } } return { d: Math.sqrt(best), q }; };

const rounds = Number(roundsArg ?? 3);
let best: { score: number; points: Map<string, any> } | undefined;
for (let round = 0; round <= rounds; round++) {
  const { m, state } = measure();
  console.log(`round ${round}: iou ${m.iou.toFixed(3)} hausdorff ${m.hausdorffM.toFixed(4)} limit ${limit}`);
  const score = m.hausdorffM - (m.iou >= 0.92 ? 0 : -1);
  if (best === undefined || m.hausdorffM < best.score) best = { score: m.hausdorffM, points: snapshot() };
  void score;
  if (round === rounds || m.hausdorffM <= limit * 0.6) break;
  const { bd, bv } = masks(m);
  const evald: any[] = state.shapes[view];
  const rings = new Map<string, { s: any; ring: { x: number; y: number }[]; T: ReturnType<typeof affine> }>();
  for (const e of evald) {
    const s = geometry().find((g) => g.id === e.shape.id);
    const sp = s === undefined ? undefined : pointsOf(s);
    if (s === undefined || sp === undefined || e.rings.length !== 1 || s.followTip === true || (s.kind === "poly" && s.closed !== true) || (s.kind === "path" && !/Z/i.test(s.d))) continue;
    if (!["body", "detail"].includes(s.role)) continue;
    if (only !== undefined && !only.has(s.id)) continue;
    const ring = e.rings[0] as { x: number; y: number }[];
    if (ring.length !== sp.length) continue;
    const T = affine(sp, ring);
    if (T === undefined || T.err > 1e-4) continue;
    rings.set(s.id, { s, ring, T });
  }
  let moved = 0, inserted = 0;
  const newWorld = new Map<string, { x: number; y: number }[]>();
  for (const [id, r] of rings) newWorld.set(id, r.ring.map((p) => ({ ...p })));
  for (const [id, r] of rings) {
    const w = newWorld.get(id)!;
    r.ring.forEach((p, k) => {
      if (nearest(p, bd).d > 2 * m.cell) return;
      const { d, q } = nearest(p, bv);
      if (d > tol && d < maxd) { w[k] = { x: q.x, y: q.y }; moved++; }
    });
  }
  const far = process.env.INS === undefined ? [] : bv.filter((c) => nearest(c, bd).d > tol * 1.5);
  const thinned: { x: number; y: number }[] = [];
  for (const c of far) if (thinned.every((t) => Math.hypot(t.x - c.x, t.y - c.y) > 0.006)) thinned.push(c);
  const inserts = new Map<string, Map<number, { t: number; p: { x: number; y: number } }[]>>();
  for (const c of thinned) {
    let best2: { d: number; id: string; k: number; t: number } | undefined;
    for (const [id, r] of rings) {
      const w = newWorld.get(id)!;
      for (let k = 0; k < w.length; k++) {
        const a = w[k]!, b = w[(k + 1) % w.length]!;
        const dx = b.x - a.x, dy = b.y - a.y, l = dx * dx + dy * dy;
        const t = l === 0 ? 0 : Math.max(0, Math.min(1, ((c.x - a.x) * dx + (c.y - a.y) * dy) / l));
        const d = Math.hypot(c.x - a.x - t * dx, c.y - a.y - t * dy);
        const mid = { x: a.x + t * dx, y: a.y + t * dy };
        if (best2 === undefined || d < best2.d) if (nearest(mid, bd).d <= 2 * m.cell) best2 = { d, id, k, t };
      }
    }
    if (best2 === undefined || best2.d > maxd || best2.d <= tol) continue;
    const mapk = inserts.get(best2.id) ?? new Map();
    inserts.set(best2.id, mapk);
    const arr = mapk.get(best2.k) ?? [];
    arr.push({ t: best2.t, p: { x: c.x, y: c.y } });
    mapk.set(best2.k, arr);
    inserted++;
  }
  for (const [id, r] of rings) {
    const w = newWorld.get(id)!;
    const mapk = inserts.get(id);
    const worldOut: { x: number; y: number }[] = [];
    w.forEach((p, k) => { worldOut.push(p); const arr = mapk?.get(k); if (arr) for (const e of arr.sort((u, v) => u.t - v.t)) worldOut.push(e.p); });
    if (worldOut.length === r.ring.length && worldOut.every((p, k) => p.x === r.ring[k]!.x && p.y === r.ring[k]!.y)) continue;
    const { a, b } = r.T!;
    const det = a[0]! * b[1]! - a[1]! * b[0]!;
    writePoints(r.s, worldOut.map((p) => {
      const X = p.x - a[2]!, Y = p.y - b[2]!;
      return [+((b[1]! * X - a[1]! * Y) / det).toFixed(4), +((-b[0]! * X + a[0]! * Y) / det).toFixed(4)];
    }));
    touched.add(r.s.id);
  }
  console.log(`  moved ${moved} vertices, inserted ${inserted}, shapes touched ${touched.size}`);
}
const ops = [...(best?.points ?? [])].map(([id, set]) => ({ op: "shape-set", scale: scaleArg, view, id, set }));
await Bun.write(out, JSON.stringify(ops));
console.log(`wrote ${ops.length} ops, best hausdorff ${best?.score.toFixed(4)}`);
