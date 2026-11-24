/** 🧱️ Draws the bars of the body that a view leaves out or draws elsewhere: every straight bar solid (a tube of revolution of constant radius, a box along one axis of its bone; each copy of a `repeat`) whose projection the drawing leaves uncovered by at least a fraction and a patch of 1.5 smallest features squared, or whose member of the same name (or name-1) lies beside its axis by more than 2 mm or has another section, gets a `member` shape with the end points and the section of the solid, in the style of the nearest member of its bone in that view (a shape of the same id is replaced, keeping its style). Greedy, largest gap first, so bars that project onto each other are drawn once. Writes `shape` and `shape-add` ops; lists what stays uncovered and the drawn shapes that lie beside the body.
 *  bun cover.ts <id|file.json> <scale> <view> <ops-in.json|-> <ops-out.json|-> [fraction=0.04] [min cells: a square of 1.5 smallest features]   (SKIP=<regex> leaves solids out) */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, bodyOutline, comparisonCell, project, strokeQuad, worldSolids, type Box } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { applyOps, mergeOps, readOps, writeOps, type Op } from "./ops.ts";
import { fileOf } from "./people.ts";
type P = { x: number; y: number };
const [name, sc, view, opsIn, opsOut, fracArg, minArg] = process.argv.slice(2) as string[];
const asset = (await Bun.file(fileOf(name!)).json()).asset;
if (opsIn && opsIn !== "-") applyOps(asset, await readOps(opsIn));
const scale = Number(sc), feature = featureOf(scale);
const fraction = Number(fracArg ?? 0.04);
const skip = process.env.SKIP === undefined ? undefined : new RegExp(process.env.SKIP);
const sel = { kind: "scale", scale: { denominator: scale } } as const;
const pose = { kind: "pose", pose: asset.presentation.defaultPose } as const;
const shapes = asset.representations[sc!].geometry[view!] as any[];
const rep = asset.representations[sc!];
const r5 = (n: number): number => Math.round(n * 1e5) / 1e5;

const candidates: { id: string; solid: string; moved: boolean; shape: any }[] = [];
for (const s of asset.volumes[sc!].body as any[]) {
  if (s.texture === true || (skip !== undefined && skip.test(s.id))) continue;
  let bar: { from: number[]; to: number[]; section: any; d: number } | undefined;
  if (s.kind === "revolve" && s.profile.length === 2 && s.profile[0][1] === s.profile[1][1] && s.transform === undefined) {
    const [l0, r] = s.profile[0], l1 = s.profile[1][0];
    if (l1 - l0 >= 6 * r) bar = { from: s.origin.map((o: number, k: number) => r5(o + s.axis[k] * l0)), to: s.origin.map((o: number, k: number) => r5(o + s.axis[k] * l1)), section: { kind: "circle", d: r5(2 * r) }, d: 2 * r };
  } else if (s.kind === "box" && (s.transform === undefined || (s.transform.rotation === undefined && s.transform.scale === undefined))) {
    const t = s.transform?.translation ?? [0, 0, 0];
    const k = s.size.indexOf(Math.max(...s.size));
    const others = [0, 1, 2].filter((i) => i !== k);
    if (s.size[k] >= 3 * Math.max(s.size[others[0]!], s.size[others[1]!])) {
      const end = (sign: number): number[] => t.map((v: number, i: number) => r5(v + (i === k ? (sign * s.size[k]) / 2 : 0)));
      const [w, h] = k === 0 ? [s.size[1], s.size[2]] : k === 1 ? [s.size[2], s.size[0]] : [s.size[1], s.size[0]];
      bar = { from: end(-1), to: end(1), section: { kind: "rect", w, h }, d: Math.min(w, h) };
    }
  }
  if (bar === undefined) continue;
  const copies = s.repeat?.count ?? 1;
  if (s.repeat !== undefined && s.repeat.rotationRad !== undefined) continue;
  for (let c = 0; c < copies; c++) {
    const step = (s.repeat?.translation ?? [0, 0, 0]).map((v: number) => v * c);
    const pt = (q: number[]) => ({ x: r5(q[0]! + step[0]), y: r5(q[1]! + step[1]), z: r5(q[2]! + step[2]) });
    const id = s.repeat === undefined ? s.id : `${s.id}-${c + 1}`;
    const members = shapes.filter((x) => x.kind === "member" && x.bone === s.bone);
    const pool = members.length > 0 ? members : shapes.filter((x) => x.kind === "member");
    const size = (x: any): number => (x.section.kind === "circle" ? x.section.d : Math.min(x.section.w, x.section.h));
    const own = shapes.find((x) => x.kind === "member" && (x.id === id || (s.repeat === undefined && x.id === `${id}-1`)));
    const like = own ?? pool.slice().sort((a, b) => Math.abs(size(a) - bar!.d) - Math.abs(size(b) - bar!.d))[0];
    if (like === undefined) continue;
    const [a, b] = [pt(bar.from), pt(bar.to)];
    const lateral = (q: { x: number; y: number; z: number }): number => { const [dx, dy, dz] = [b.x - a.x, b.y - a.y, b.z - a.z]; const l = Math.hypot(dx, dy, dz); const [px, py, pz] = [q.x - a.x, q.y - a.y, q.z - a.z]; const k = (px * dx + py * dy + pz * dz) / l; return Math.hypot(px - (k * dx) / l, py - (k * dy) / l, pz - (k * dz) / l); };
    const sizes = (x: any): number[] => (x.section.kind === "circle" ? [x.section.d, x.section.d] : [x.section.w, x.section.h]);
    const moved = own !== undefined && (Math.max(lateral(own.from), lateral(own.to)) > 0.002 || own.section.kind !== bar.section.kind || sizes(own).some((v, k) => Math.abs(v - sizes({ section: bar!.section })[k]!) > 0.001));
    const shape: any = { kind: "member", from: a, to: b, section: bar.section, id: own?.id ?? id, bone: s.bone, role: like.role, z: like.z, filled: like.filled, tone: like.tone };
    if (like.region !== undefined) shape.region = like.region;
    shape.material = s.material ?? like.material;
    if (like.penMm !== undefined) shape.penMm = own !== undefined && size(own) === bar.d ? own.penMm : like.filled === false ? Math.max(0.13, Math.round(((bar.d * 1000) / scale) * 1000) / 1000) : like.penMm;
    candidates.push({ id, solid: s.id, moved, shape: { ...shape, after: like.id } });
  }
}
const probe = structuredClone(asset);
const probeShapes = probe.representations[sc!].geometry[view!] as any[];
for (const c of candidates) { const { after: _a, ...shape } = c.shape; probeShapes.push({ ...shape, id: `cover~${c.id}` }); }
const probed: any = evaluate(probe, pose, sel);
const ringsOf = new Map<string, P[][]>();
for (const s of probed.shapes[view!]) if (String(s.shape.id).startsWith("cover~")) ringsOf.set(String(s.shape.id).slice(6), s.rings);

const byId = new Map(candidates.map((c) => [c.id, c]));
const state: any = evaluate(asset, pose, sel);
const solids = (worldSolids(asset, state.bones as never, scale) as any[]).filter((w) => w.texture !== true);
const boxOf = (groups: P[][][]): Box => { const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity }; for (const g of groups) for (const r of g) for (const p of r) { if (p.x < b.minX) b.minX = p.x; if (p.x > b.maxX) b.maxX = p.x; if (p.y < b.minY) b.minY = p.y; if (p.y > b.maxY) b.maxY = p.y; } return b; };
type Part = { label: string; id: string; copy?: number; groups: P[][][]; box: Box };
const vols: Part[] = [], draws: Part[] = [];
for (const w of solids) {
  const per = w.copyVertices ?? w.positions.length / 3;
  const copies = Math.max(1, Math.round(w.positions.length / 3 / per));
  const tris = w.indices.length / 3 / copies;
  const q: P[] = [];
  for (let i = 0; i < w.positions.length; i += 3) q.push(project(view as never, w.positions[i], w.positions[i + 1], w.positions[i + 2]));
  for (let c = 0; c < copies; c++) {
    const groups: P[][][] = [];
    for (let t = c * tris; t < (c + 1) * tris; t++) groups.push([[q[w.indices[t * 3]]!, q[w.indices[t * 3 + 1]]!, q[w.indices[t * 3 + 2]]!]]);
    vols.push({ label: `${w.id}${copies > 1 ? `-${c + 1}` : ""}[${w.bone}]`, id: copies > 1 ? `${w.id}-${c + 1}` : w.id, groups, box: boxOf(groups) });
  }
}
const drawnGroups = (s: any): P[][][] => {
  const fills = s.closed && (s.shape.filled !== false || bodyOutline(view as never, solids as never, s.rings));
  if (fills) return [s.rings];
  const pen = Math.max(((s.shape.penMm ?? rep.pens[s.shape.role] ?? 0) * scale) / 1000, 1e-4);
  const groups: P[][][] = [];
  for (const ring of s.rings) { const r = s.closed ? [...ring, ring[0]] : ring; for (let i = 0; i + 1 < r.length; i++) groups.push([strokeQuad(r[i], r[i + 1], pen) as P[]]); }
  return groups;
};
for (const s of state.shapes[view!]) if (s.rings.length > 0 && ["body", "detail", "glazing"].includes(s.shape.role)) { const groups = drawnGroups(s); draws.push({ label: `${s.shape.id}[${s.shape.bone}] ${s.shape.kind}`, id: s.shape.id, groups, box: boxOf(groups) }); }
const all = boxOf([...vols, ...draws].map((p) => [[{ x: p.box.minX, y: p.box.minY }, { x: p.box.maxX, y: p.box.maxY }]]));
const long = Math.max(all.maxX - all.minX, all.maxY - all.minY);
const cell = Math.max(comparisonCell(feature, long), long / 14000);
const win: Box = { minX: all.minX - 2 * cell, minY: all.minY - 2 * cell, maxX: all.maxX + 2 * cell, maxY: all.maxY + 2 * cell };
const minCells = Number(minArg ?? Math.max(6, Math.round(((1.5 * feature) / cell) ** 2)));
const drawn = new Mask(win, cell), vol = new Mask(win, cell);
for (const p of vols) for (const g of p.groups) vol.fill(g);
for (const p of draws) for (const g of p.groups) drawn.fill(g);
const tally = (p: Part, other: Mask): { n: number; of: number } => {
  const [i0, j0] = [Math.max(0, Math.floor((p.box.minX - win.minX) / cell) - 1), Math.max(0, Math.floor((p.box.minY - win.minY) / cell) - 1)];
  const [i1, j1] = [Math.min(other.width, Math.ceil((p.box.maxX - win.minX) / cell) + 1), Math.min(other.height, Math.ceil((p.box.maxY - win.minY) / cell) + 1)];
  if (i1 <= i0 || j1 <= j0) return { n: 0, of: 0 };
  const one = new Mask({ minX: win.minX + i0 * cell, minY: win.minY + j0 * cell, maxX: win.minX + i1 * cell - cell / 2, maxY: win.minY + j1 * cell - cell / 2 }, cell);
  for (const g of p.groups) one.fill(g);
  let [n, of] = [0, 0];
  for (let j = 0; j < one.height; j++) for (let i = 0; i < one.width; i++) if (one.cells[j * one.width + i]) { of++; if (!other.cells[(j + j0) * other.width + i + i0]) n++; }
  return { n, of };
};
const first = vols.map((p) => ({ p, ...tally(p, drawn) })).filter((x) => x.n > 0 || byId.get(x.p.id)?.moved === true).sort((a, b) => b.n - a.n);
const ops: Op[] = [];
const left: string[] = [];
for (const x of first) {
  const c = byId.get(x.p.id);
  const now = tally(x.p, drawn);
  if (c?.moved !== true && (now.n < minCells || now.n / now.of < fraction)) continue;
  const rings = c === undefined ? undefined : ringsOf.get(c.id);
  if (c === undefined || rings === undefined || rings.length === 0) { left.push(`${x.p.label} ${now.n}/${now.of}`); continue; }
  const b = boxOf([rings]);
  const off = Math.max(Math.abs(b.minX - x.p.box.minX), Math.abs(b.maxX - x.p.box.maxX), Math.abs(b.minY - x.p.box.minY), Math.abs(b.maxY - x.p.box.maxY));
  if (off > Math.max(0.002, feature / 4)) { left.push(`${x.p.label} ${now.n}/${now.of} (member off ${(off * 1000).toFixed(1)} mm)`); continue; }
  drawn.fill(rings);
  const { after, ...shape } = c.shape;
  const exists = shapes.some((s) => s.id === shape.id);
  ops.push(exists ? { op: "shape", scale: sc, view, id: shape.id, shape } : { op: "shape-add", scale: sc, view, after, shape });
  console.log(`  ${exists ? (c.moved ? "moved  " : "redrawn") : "added  "} ${shape.id} [${shape.bone}] ${shape.section.kind} ${shape.section.d ?? `${shape.section.w}x${shape.section.h}`} uncovered ${now.n}/${now.of}`);
}
console.log(`== ${asset.id} 1:${scale} ${view} cell ${(cell * 1000).toFixed(2)} mm: ${ops.length} members`);
if (left.length > 0) console.log(`  uncovered, no bar: ${left.slice(0, 40).join("; ")}`);
const beside = draws.map((p) => ({ p, ...tally(p, vol) })).filter((x) => x.n >= minCells && x.n / x.of >= fraction && !ops.some((o) => o.op === "shape" && o.id === x.p.id)).sort((a, b) => b.n - a.n);
if (beside.length > 0) console.log(`  drawn beside the body: ${beside.slice(0, 40).map((x) => `${x.p.label} ${x.n}/${x.of}`).join("; ")}`);
if (opsOut !== undefined && opsOut !== "-") await writeOps(opsOut, mergeOps(await readOps(opsOut), ops));
