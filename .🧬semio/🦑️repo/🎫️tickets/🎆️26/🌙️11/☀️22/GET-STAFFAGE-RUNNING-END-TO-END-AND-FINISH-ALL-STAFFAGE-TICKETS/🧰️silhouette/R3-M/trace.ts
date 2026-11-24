/** ✍️ Draws volume solids as they project: the outline of the union of the named solids of one scale in one view (default pose) exactly as the guard projects them (a slender solid as the stroke the guard compares), traced on a fine raster (a twenty-fourth of the smallest feature at most), simplified (`--tol`, default a twelfth of a feature and at least one and a half cells) and carried back into the drawing frame of a bone, becomes a filled shape; the op is merged into `ops/<id>.json` (earlier ops of that file are applied first, so a repaired volume is traced as repaired).
 *  bun trace.ts <id> <scale> <view> <solid,solid…; a trailing * matches a prefix> --id <shape id> --bone <bone> --like <shape id whose role, z, tone, region and material it takes> [--replace <shape id it replaces> | --after <shape id>] [--z N] [--material id] [--tone 0…1] [--role role] [--tol metres] [--grow metres] [--holes] [--dry]
 *  `--replace` with another id than `--id` removes that shape and adds the new one in its place. `--holes` keeps the inner loops (openings); without it only outer loops are drawn. */
import { parseArgs } from "node:util";
import type { Asset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, drawnFor, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { fileOf } from "./files.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";

type P = { x: number; y: number };
const { values, positionals } = parseArgs({ allowPositionals: true, options: { id: { type: "string" }, bone: { type: "string" }, like: { type: "string" }, replace: { type: "string" }, after: { type: "string" }, z: { type: "string" }, material: { type: "string" }, tone: { type: "string" }, role: { type: "string" }, tol: { type: "string" }, grow: { type: "string" }, holes: { type: "boolean" }, dry: { type: "boolean" } } });
const [name, sc, view, solidArg] = positionals as [string, string, string, string];
const scale = Number(sc), feature = featureOf(scale);
const opsFile = `${import.meta.dir}/ops/${name}.json`;
const { asset } = (await Bun.file(fileOf(name)).json()) as { asset: Asset };
const existing = await readOps(opsFile);
applyOps(asset, existing);
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const pose = asset.presentation.defaultPose;
const state: any = evaluate(asset, { kind: "pose", pose }, sel);
const wanted = solidArg.split(",");
const ids = new Set((asset as any).volumes[sc].body.map((s: any) => s.id as string).filter((i: string) => wanted.some((w) => (w.endsWith("*") ? i.startsWith(w.slice(0, -1)) : i === w))));
for (const w of wanted) if (![...ids].some((i) => (w.endsWith("*") ? i.startsWith(w.slice(0, -1)) : i === w))) throw new Error(`solid "${w}" not in volumes[${sc}].body`);
const worlds = worldSolids(asset, state.bones as never, scale).filter((w) => ids.has(w.id));
const groups = referenceOf(view as never, worlds, undefined, drawnFor(asset, state, view as never, scale)).cut;
let [minX, minY, maxX, maxY] = [Infinity, Infinity, -Infinity, -Infinity];
for (const g of groups) for (const ring of g) for (const p of ring) { minX = Math.min(minX, p.x); minY = Math.min(minY, p.y); maxX = Math.max(maxX, p.x); maxY = Math.max(maxY, p.y); }
if (!(maxX > minX)) throw new Error("nothing to trace");
const cell = Math.max(Math.min(feature / 24, Math.max(maxX - minX, maxY - minY) / 400), Math.max(maxX - minX, maxY - minY) / 5000);
const mask = new Mask({ minX: minX - 2 * cell, minY: minY - 2 * cell, maxX: maxX + 2 * cell, maxY: maxY + 2 * cell }, cell);
for (const g of groups) mask.fill(g, true);
mask.seal();
const [x0, y0, W, H, g] = [minX - 2 * cell, minY - 2 * cell, mask.width, mask.height, mask.cells];
const at = (i: number, j: number): number => (i < 0 || j < 0 || i >= W || j >= H ? 0 : g[j * W + i]!);
const next = new Map<number, number[]>();
const key = (i: number, j: number): number => j * (W + 1) + i;
const add = (a: number, b: number): void => { const l = next.get(a); if (l) l.push(b); else next.set(a, [b]); };
for (let j = 0; j < H; j++) for (let i = 0; i < W; i++) if (at(i, j)) {
  if (!at(i, j - 1)) add(key(i, j), key(i + 1, j));
  if (!at(i + 1, j)) add(key(i + 1, j), key(i + 1, j + 1));
  if (!at(i, j + 1)) add(key(i + 1, j + 1), key(i, j + 1));
  if (!at(i - 1, j)) add(key(i, j + 1), key(i, j));
}
const cross = (p: P, q: P, r: P): number => (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
const dist = (p: P, q: P, r: P): number => Math.abs(cross(q, r, p)) / (Math.hypot(r.x - q.x, r.y - q.y) || 1);
const rdp = (l: P[], t: number): P[] => { if (l.length < 3) return l; let mi = 0, md = 0; for (let i = 1; i < l.length - 1; i++) { const d = dist(l[i]!, l[0]!, l[l.length - 1]!); if (d > md) { md = d; mi = i; } } return md > t ? [...rdp(l.slice(0, mi + 1), t).slice(0, -1), ...rdp(l.slice(mi), t)] : [l[0]!, l[l.length - 1]!]; };
const rdpRing = (r: P[], t: number): P[] => { let a = 0, b = 0; for (let i = 0; i < r.length; i++) { if (r[i]!.x < r[a]!.x) a = i; if (r[i]!.x > r[b]!.x) b = i; } if (a === b) return r; const rot = [...r.slice(a), ...r.slice(0, a)]; const k = rot.indexOf(r[b]!); return [...rdp(rot.slice(0, k + 1), t).slice(0, -1), ...rdp([...rot.slice(k), rot[0]!], t).slice(0, -1)]; };
const grow = Number(values.grow ?? 0);
const offsetRing = (r: P[], d: number): P[] => d === 0 ? r : r.map((p, i) => { const a = r[(i + r.length - 1) % r.length]!, b = r[(i + 1) % r.length]!; const n1 = { x: p.y - a.y, y: a.x - p.x }, n2 = { x: b.y - p.y, y: p.x - b.x }; const l1 = Math.hypot(n1.x, n1.y) || 1, l2 = Math.hypot(n2.x, n2.y) || 1; const nx = n1.x / l1 + n2.x / l2, ny = n1.y / l1 + n2.y / l2; const nl = Math.hypot(nx, ny) || 1; return { x: p.x + (nx / nl) * d, y: p.y + (ny / nl) * d }; });
const tol = Number(values.tol ?? Math.max(feature / 12, 1.5 * cell));
const loops: P[][] = [];
while (next.size > 0) {
  const [start] = next.keys();
  let cur = start!;
  const loop: number[] = [];
  for (;;) { loop.push(cur); const l = next.get(cur)!; const n = l.pop()!; if (l.length === 0) next.delete(cur); cur = n; if (cur === start || !next.has(cur)) break; }
  const ring = loop.map((k) => ({ x: x0 + (k % (W + 1)) * cell, y: y0 + Math.floor(k / (W + 1)) * cell }));
  let area = 0;
  for (let i = 0; i < ring.length; i++) { const p = ring[i]!, q = ring[(i + 1) % ring.length]!; area += p.x * q.y - q.x * p.y; }
  const outer = area > 0;
  if (!outer && !values.holes) continue;
  if (Math.abs(area) / 2 < (outer ? 0.02 : 1) * feature * feature) continue;
  loops.push(offsetRing(rdpRing(ring, tol), outer ? grow : -grow));
}
if (loops.length === 0) throw new Error("nothing traced");
const geo = (a: any): any[] => a.representations[sc].geometry[view];
const forward = (bone: string, points: P[]): P[] => {
  const probe: any = structuredClone(asset);
  geo(probe).push({ kind: "poly", points: points.map((p) => [p.x, p.y]), closed: true, id: "trace-probe", bone, role: "body", z: 1, filled: true, tone: 0.5 });
  const ps: any = evaluate(probe, { kind: "pose", pose }, sel);
  return ps.shapes[view].find((s: any) => s.shape.id === "trace-probe").rings[0].slice(0, points.length);
};
const bone = values.bone!;
const [o, a, b] = forward(bone, [{ x: 0, y: 0 }, { x: 0.05, y: 0 }, { x: 0, y: 0.05 }]) as [P, P, P];
const m = { a: (a.x - o.x) / 0.05, b: (a.y - o.y) / 0.05, c: (b.x - o.x) / 0.05, d: (b.y - o.y) / 0.05 };
const det = m.a * m.d - m.b * m.c;
if (Math.abs(det) < 0.15) throw new Error(`bone ${bone} is edge-on in ${view}: no drawing frame`);
const inv = (p: P): P => { const dx = p.x - o.x, dy = p.y - o.y; return { x: (m.d * dx - m.c * dy) / det, y: (-m.b * dx + m.a * dy) / det }; };
const locals = loops.map((targets) => {
  let local = targets.map(inv);
  for (let it = 0; it < 8; it++) {
    const got = forward(bone, local);
    const err = targets.map((t, i) => ({ x: t.x - got[i]!.x, y: t.y - got[i]!.y }));
    if (Math.max(...err.map((e) => Math.hypot(e.x, e.y))) < 1e-4) return local;
    local = local.map((l, i) => { const d = inv({ x: o.x + err[i]!.x, y: o.y + err[i]!.y }); return { x: l.x + d.x, y: l.y + d.y }; });
  }
  throw new Error(`the drawing frame of ${bone} in ${view} does not converge`);
});
const like = geo(asset).find((s: any) => s.id === values.like);
if (like === undefined) throw new Error(`shape "${values.like}" not in ${view} of 1:${sc}`);
const keep = Object.fromEntries(["role", "z", "tone", "region", "regions", "material"].filter((k) => like[k] !== undefined).map((k) => [k, like[k]]));
const digits = scale >= 200 ? 3 : 4;
const fmt = (v: number): number => Number(v.toFixed(digits));
const id = values.id!;
const shape: any = { id, bone, ...keep, ...(values.z === undefined ? {} : { z: Number(values.z) }), ...(values.material === undefined ? {} : { material: values.material }), ...(values.tone === undefined ? {} : { tone: Number(values.tone) }), ...(values.role === undefined ? {} : { role: values.role }), filled: true, ...(locals.length === 1 ? { kind: "poly", points: locals[0]!.map((p) => [fmt(p.x), fmt(p.y)]), closed: true } : { kind: "path", d: locals.map((l) => "M " + l.map((p) => `${fmt(p.x)} ${fmt(p.y)}`).join(" L ") + " Z").join(" ") }) };
const has = (x: string | undefined): boolean => geo(asset).some((s: any) => s.id === x);
const added: Op[] = values.replace !== undefined && values.replace !== id
  ? [{ op: "shape-add", scale: sc, view, after: values.replace, shape }, { op: "shape-remove", scale: sc, view, id: values.replace }]
  : has(id) ? [{ op: "shape", scale: sc, view, id, shape }] : [{ op: "shape-add", scale: sc, view, after: values.after ?? values.like, shape }];
console.log(`${name} 1:${sc} ${view} ${id}[${bone}]: ${locals.length} loops, ${locals.reduce((n, l) => n + l.length, 0)} points, cell ${(cell * 1000).toFixed(2)} mm${values.dry ? " (dry)" : ""}`);
if (!values.dry) await writeOps(opsFile, mergeOps(existing, added));
