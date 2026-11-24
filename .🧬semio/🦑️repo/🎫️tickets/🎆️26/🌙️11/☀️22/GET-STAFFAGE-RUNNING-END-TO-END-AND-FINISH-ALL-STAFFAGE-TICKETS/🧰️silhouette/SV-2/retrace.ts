/** ✂️ Brings the drawing of one view of a plant onto the outline of its volume where the guard measures a boundary distance beyond the limit: a filling shape that reaches outside the volume there is cut to the volume, a stroke loses the segments that leave it, and where the volume has no drawing the main filled shape of the bone of the solid (same material first) is joined with the outline of the solids of that bone. Every change is an idempotent op of `repair/<slug>.json` (whole shapes by id); the catalog file is written by `apply.ts`.
 *  bun retrace.ts <slug> <scale> <view>… [--rounds 6] [--eps 0.5] [--limit <m>] [--dry] [--all]
 *  `--patch x,y,r,<stroke id>` (one view): first adds the volume within r metres of (x, y) as a filled shape `<stroke id>-joint` on the bone of that stroke (where two strokes meet in a cell that only the sealed volume raster covers).
 *  `--patch x,y,r` (one view): first joins the volume within r metres of (x, y) with the main filled shape of the bone of the nearest solid; `--restroke`: first redraws the stroke of every slender solid from its centre line.
 *  `--eps`: simplification of a traced outline in smallest features (a shape that is cut a second time is traced three times finer); `--limit`: a tighter largest distance than two smallest features; `--all`: cut every filling shape that leaves the volume and fill every bone, not only at the clusters. */
import { parseArgs } from "node:util";
import { Mask, boundaryOf, distanceField, drawnFor, fillSmallHoles, project, strokeDrawings, strokesOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { centreLineOf, clusterPoints } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧵️centreline/🟦️.ts";
import { validateAsset } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";
import { areaOf, drawnMask, grown, placed, inTriangle, load, opsFile, simplify, touches, trace, trianglesOf, viewOf, volumeMask, withRings, type Box, type Entry, type P, type Tri } from "./lib.ts";
import { applyOp, mergeOps, writeOps, type Op } from "./ops.ts";

const { values, positionals } = parseArgs({ allowPositionals: true, options: { rounds: { type: "string" }, eps: { type: "string" }, limit: { type: "string" }, dry: { type: "boolean" }, all: { type: "boolean" }, restroke: { type: "boolean" }, patch: { type: "string" }, nudge: { type: "boolean" }, novalidate: { type: "boolean" } } });
const [slug, scaleArg, ...views] = positionals as string[];
const scale = Number(scaleArg), sc = String(scale);
const { asset, ops: startOps } = await load(slug!);
const made: Op[] = [];
const log = (s: string): void => console.log(`[DEBUG] ${s}`);
const keyOf = (i: any): string => JSON.stringify([i.code, i.path, i.data?.bone, [...(i.data?.shapes ?? [])].sort()]);
const baseKeys = values.novalidate ? new Set<string>() : new Set<string>((validateAsset(structuredClone(asset) as never) as any[]).map(keyOf));
const geo = (view: string): any[] => asset.representations[sc].geometry[view];
const TILE = 512;

type Cluster = { side: string; x: number; y: number; d: number; kind: "extra" | "missing" | "none"; qx: number; qy: number };

/** The boundary cells of the drawing and of the volume that lie further apart than the limit, tile by tile, each with a witness cell nearby: drawn outside the volume (`extra`) or volume without drawing (`missing`). */
function scan(view: string, v: ReturnType<typeof viewOf>, limit: number): { clusters: Cluster[]; worst: number } {
  const { entries, groups, frame, feature, cell } = v;
  const margin = Math.ceil((6 * feature) / cell);
  const holes = Math.round((4 * feature / cell) ** 2);
  const f: Box = grown(frame, 2 * cell);
  const banded = Math.max(f.maxX - f.minX, f.maxY - f.minY) / cell > TILE;
  const side = banded ? TILE * cell : Infinity;
  const [nx, ny] = banded ? [Math.ceil((f.maxX - f.minX) / side), Math.ceil((f.maxY - f.minY) / side)] : [1, 1];
  const clusters: Cluster[] = [];
  let worst = 0;
  const grid = Math.max(6 * feature, 0.05);
  const seen = new Set<string>();
  for (let tj = 0; tj < ny; tj++) for (let ti = 0; ti < nx; ti++) {
    const [minX, minY] = banded ? [f.minX + ti * side, f.minY + tj * side] : [f.minX, f.minY];
    const core: Box = banded ? { minX, minY, maxX: minX + Math.ceil((Math.min(f.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(f.maxY, minY + side) - minY) / cell) * cell } : f;
    const box = banded ? grown(core, margin * cell) : f;
    const pad = banded ? margin : 0;
    if (!entries.some((e) => touches(e.box, box)) && !groups.some((g) => touches(g.box, box))) continue;
    const D = drawnMask(entries, box, cell), V = volumeMask(groups, box, cell);
    const [w, h] = [V.width, V.height];
    const bd = boundaryOf(fillSmallHoles(D.cells, w, h, holes), w, h), bv = boundaryOf(fillSmallHoles(V.cells, w, h, holes), w, h);
    let any = false;
    for (let k = 0; k < bd.length && !any; k++) any = bd[k] === 1 || bv[k] === 1;
    if (!any) continue;
    const toV = distanceField(bv, w, h), toD = distanceField(bd, w, h);
    const list: { i: number; j: number; d: number; side: string }[] = [];
    for (const [side, from, field] of [["drawn-only", bd, toV], ["volume-only", bv, toD]] as const)
      for (let j = pad; j < Math.min(h, pad + Math.round((core.maxY - core.minY) / cell)); j++) for (let i = pad; i < Math.min(w, pad + Math.round((core.maxX - core.minX) / cell)); i++) {
        const k = j * w + i;
        if (!from[k]) continue;
        const d = (Number.isFinite(field[k]!) && field[k]! < 1e19 ? Math.sqrt(field[k]!) : banded ? margin + 1 : Math.hypot(w, h)) * cell;
        if (d > worst) worst = d;
        if (d > limit) list.push({ i, j, d, side });
      }
    list.sort((a, b) => b.d - a.d);
    for (const c of list) {
      const x = box.minX + (c.i + 0.5) * cell, y = box.minY + (c.j + 0.5) * cell;
      const key = `${c.side}|${Math.round(x / grid)}|${Math.round(y / grid)}`;
      if (seen.has(key)) continue;
      seen.add(key);
      let hit: Cluster = { side: c.side, x, y, d: c.d, kind: "none", qx: x, qy: y }, best = Infinity;
      for (let dj = -16; dj <= 16; dj++) for (let di = -16; di <= 16; di++) {
        const [i, j] = [c.i + di, c.j + dj];
        if (i < 0 || j < 0 || i >= w || j >= h) continue;
        const k = j * w + i, r = di * di + dj * dj;
        if (r >= best || D.cells[k] === V.cells[k]) continue;
        best = r;
        hit = { ...hit, kind: D.cells[k] ? "extra" : "missing", qx: box.minX + (i + 0.5) * cell, qy: box.minY + (j + 0.5) * cell };
      }
      clusters.push(hit);
    }
  }
  clusters.sort((a, b) => b.d - a.d);
  return { clusters, worst };
}

const inRings = (rings: readonly (readonly P[])[], x: number, y: number): boolean => {
  let wind = 0, cross = 0;
  for (const ring of rings) for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
    const a = ring[k]!, b = ring[i]!;
    if (a.y <= y !== b.y <= y && x < a.x + ((y - a.y) / (b.y - a.y)) * (b.x - a.x)) { cross++; wind += b.y > a.y ? 1 : -1; }
  }
  return rings.length === 1 ? wind !== 0 : cross % 2 === 1;
};

const attempts = new Map<string, number>();
const epsFor = (view: string, id: string, feature: number): number => {
  const n = attempts.get(`${view}|${id}`) ?? 0;
  attempts.set(`${view}|${id}`, n + 1);
  return (Number(values.eps ?? 0.5) * feature) / 3 ** Math.min(n, 3);
};

/** The rings of a raster in metres of the view: holes below (1.5 features)^2 and islands of a few cells are dropped, the rest simplified; a shape that is traced for the third time (below 0.1 feature) keeps every ring. */
const ringsOf = (m: Mask, eps: number, feature: number): P[][] => {
  const every = eps < 0.1 * feature;
  const out: P[][] = [];
  for (const ring of trace(m)) {
    const a = areaOf(ring);
    if (!every && (a > 0 ? a < 6 * m.cell * m.cell : -a < (1.5 * feature) ** 2 && -a < 40 * m.cell * m.cell)) continue;
    const s = simplify(ring, eps);
    if (s.length >= 3) out.push(s);
  }
  return out;
};
const note = (r: { residual: number }): string => r.residual > 3e-4 ? `, placed within ${(r.residual * 1000).toFixed(1)} mm` : "";

const put = (view: string, op: Op): void => { applyOp(asset, op); made.push(op); };

/** Cuts a filling shape to the volume; true when it changed. */
function clip(view: string, v: ReturnType<typeof viewOf>, e: Entry): boolean {
  const source = geo(view).find((s) => s.id === e.shape.id);
  if (source.solid !== undefined) { log(`${view} ${e.shape.id}: not a flat shape (solid ${JSON.stringify(e.shape.solid)?.slice(0, 80)}), left`); return false; }
  const box = v.snap(grown(e.box, 3 * v.cell));
  const L = new Mask(box, v.cell);
  L.fill(e.rings);
  const V = volumeMask(v.groups, box, v.cell);
  let out = 0, kept = 0;
  for (let k = 0; k < L.cells.length; k++) if (L.cells[k]) { if (V.cells[k]) kept++; else { out++; L.cells[k] = 0; } }
  if (out === 0) return false;
  const rings = kept === 0 ? [] : ringsOf(L, epsFor(view, e.shape.id, v.feature), v.feature);
  if (rings.length === 0) { put(view, { op: "shape-remove", scale: sc, view, id: e.shape.id }); log(`${view} ${e.shape.id}: nothing of it inside the volume, removed`); return true; }
  const fit = placed(asset, scale, view, source, rings);
  if (fit === undefined) { log(`${view} ${e.shape.id}: cannot be placed, left`); return false; }
  put(view, { op: "shape", scale: sc, view, id: e.shape.id, shape: fit.shape });
  log(`${view} ${e.shape.id}: cut to the volume (${out} cells outside, ${rings.length} rings, ${rings.reduce((n, r) => n + r.length, 0)} points${note(fit)})`);
  return true;
}

/** Drops the segments of a stroke that leave the volume; true when it changed. */
function clipStroke(view: string, v: ReturnType<typeof viewOf>, e: Entry): boolean {
  const source = geo(view).find((s) => s.id === e.shape.id);
  if (source.solid !== undefined) { log(`${view} ${e.shape.id}: stroke with a solid, left`); return false; }
  const V = volumeMask(v.groups, v.snap(grown(e.box, 3 * v.cell)), v.cell);
  const lines: P[][] = [];
  let dropped = 0, q = 0;
  for (const ring of e.rings) {
    const r = e.closed ? [...ring, ring[0]!] : ring;
    let run: P[] = [];
    for (let i = 0; i + 1 < r.length; i++, q++) {
      const quad = e.quads[q]!;
      const m = new Mask(v.snap(grown({ minX: Math.min(...quad.map((p) => p.x)), minY: Math.min(...quad.map((p) => p.y)), maxX: Math.max(...quad.map((p) => p.x)), maxY: Math.max(...quad.map((p) => p.y)) }, v.cell)), v.cell);
      m.fill([quad]);
      let outside = false;
      for (let j = 0; j < m.height && !outside; j++) for (let i2 = 0; i2 < m.width; i2++) if (m.cells[j * m.width + i2] && !V.has(m.box.minX + (i2 + 0.5) * v.cell, m.box.minY + (j + 0.5) * v.cell)) { outside = true; break; }
      if (outside) { dropped++; if (run.length >= 2) lines.push(run); run = []; } else { if (run.length === 0) run.push(r[i]!); run.push(r[i + 1]!); }
    }
    if (run.length >= 2) lines.push(run);
  }
  if (dropped === 0) return false;
  if (lines.length === 0) { put(view, { op: "shape-remove", scale: sc, view, id: e.shape.id }); log(`${view} ${e.shape.id}: stroke outside the volume, removed`); return true; }
  const fit = placed(asset, scale, view, source, lines, true);
  if (fit === undefined) { log(`${view} ${e.shape.id}: stroke cannot be placed, left`); return false; }
  put(view, { op: "shape", scale: sc, view, id: e.shape.id, shape: fit.shape });
  log(`${view} ${e.shape.id}: ${dropped} stroke segments outside the volume dropped, ${lines.length} lines kept${note(fit)}`);
  return true;
}

/** The centre line of a slender solid in the view as open lines (the chains of the edges of its centre line, a straight principal axis where the mesh gives none). */
function centreOf(view: string, solid: any): P[][] {
  const per = solid.copyVertices ?? solid.positions.length / 3;
  if (Math.round(solid.positions.length / 3 / per) !== 1) return [];
  const line = centreLineOf(solid.positions, solid.positions, solid.indices);
  if (line === undefined || line.edges.length === 0) {
    const pts: P[] = [];
    for (let i = 0; i < solid.positions.length; i += 3) pts.push(project(view as never, solid.positions[i], solid.positions[i + 1], solid.positions[i + 2]));
    const mx = pts.reduce((s, q) => s + q.x, 0) / pts.length, my = pts.reduce((s, q) => s + q.y, 0) / pts.length;
    let sxx = 0, sxy = 0, syy = 0;
    for (const q of pts) { sxx += (q.x - mx) ** 2; sxy += (q.x - mx) * (q.y - my); syy += (q.y - my) ** 2; }
    const angle = 0.5 * Math.atan2(2 * sxy, sxx - syy), ux = Math.cos(angle), uy = Math.sin(angle);
    let lo = Infinity, hi = -Infinity, left = Infinity, right = -Infinity;
    for (const q of pts) { const s = (q.x - mx) * ux + (q.y - my) * uy, n = -(q.x - mx) * uy + (q.y - my) * ux; lo = Math.min(lo, s); hi = Math.max(hi, s); left = Math.min(left, n); right = Math.max(right, n); }
    const cx = mx - ((left + right) / 2) * uy, cy = my + ((left + right) / 2) * ux;
    return [[{ x: cx + ux * lo, y: cy + uy * lo }, { x: cx + ux * hi, y: cy + uy * hi }]];
  }
  const centre = clusterPoints(line, solid.positions, 0).map((q) => project(view as never, q[0], q[1], q[2]));
  const chains: P[][] = [];
  let last = -1;
  for (const [i, j] of line.edges) { if (i === last) chains[chains.length - 1]!.push(centre[j]!); else chains.push([centre[i]!, centre[j]!]); last = j; }
  return chains;
}

/** Draws a slender solid as the stroke of its centre line: the stroke of the same id is redrawn, or a stroke is added below the filled shapes; true when it changed. */
function restroke(view: string, v: ReturnType<typeof viewOf>, solid: any, done: Set<string>, pull?: { x: number; y: number; sign: number }): boolean {
  let lines = centreOf(view, solid);
  const tries = attempts.get(`${view}|stroke|${solid.id}`) ?? 0;
  attempts.set(`${view}|stroke|${solid.id}`, tries + 1);
  if (values.nudge && pull !== undefined && tries > 0 && lines.length > 0) {
    let near: P | undefined, best = Infinity;
    for (const l of lines) for (const q of l) { const d = Math.hypot(q.x - pull.x, q.y - pull.y); if (d < best) { best = d; near = q; } }
    const step = (pull.sign * Math.min(tries, 4) * 0.00025) / (best || 1);
    const [dx, dy] = [(pull.x - near!.x) * step, (pull.y - near!.y) * step];
    lines = lines.map((l) => l.map((q) => ({ x: q.x + dx, y: q.y + dy })));
    log(`${view} ${solid.id}: stroke shifted by ${(Math.hypot(dx, dy) * 1000).toFixed(2)} mm ${pull.sign > 0 ? "towards" : "away from"} (${pull.x.toFixed(3)}, ${pull.y.toFixed(3)})`);
  }
  if (lines.length === 0) { log(`${view} slender solid ${solid.id}: no centre line (a repeat), left`); return false; }
  const def = (asset.volumes[sc].body as any[]).find((x) => x.id === solid.id);
  const list = geo(view);
  let source = list.find((s) => s.id === solid.id);
  let id = solid.id as string;
  if (source !== undefined && (source.filled !== false || source.bone !== solid.bone)) { id = `${solid.id}-line`; source = list.find((s) => s.id === id); }
  if (done.has(id)) return false;
  done.add(id);
  const added = source === undefined;
  if (source === undefined) {
    const like = list.find((s) => s.material === def.material && s.region !== undefined) ?? list.find((s) => s.bone === solid.bone && s.region !== undefined);
    const pens = asset.representations[sc].pens as Record<string, number | null>;
    const role = pens.detail != null ? "detail" : "body";
    const thin = Math.min(...list.map((s) => s.penMm ?? pens[s.role] ?? Infinity).filter((x) => x > 0));
    source = { id, bone: solid.bone, role, ...(role === "body" && Number.isFinite(thin) && thin < (pens.body ?? 0) ? { penMm: thin } : {}), z: Math.min(...list.map((s) => s.z ?? 0)), filled: false, tone: def.tone ?? 0.3, ...(like?.region === undefined ? {} : { region: like.region }), ...(def.material === undefined ? {} : { material: def.material }), kind: "path", d: "" };
  }
  const fit = placed(asset, scale, view, source, lines, true);
  if (fit === undefined) { log(`${view} ${id}: stroke cannot be placed, left`); return false; }
  if (added) put(view, { op: "shape-add", scale: sc, view, shape: fit.shape });
  else put(view, { op: "shape", scale: sc, view, id, shape: fit.shape });
  log(`${view} ${id}: ${added ? "added as" : "redrawn as"} the centre line of the slender solid ${solid.id} (${lines.length} lines, ${lines.reduce((n, l) => n + l.length, 0)} points${note(fit)})`);
  return true;
}

/** Joins the main filled shape of the bone of a solid with the outline of the solids of that bone (same material); a bone that draws no filled shape in the view is joined with the shape of that material on its nearest ancestor bone (a new flat shape on such a bone is what `lateral-ambiguous` reports), and only where no ancestor draws one a shape is added; true when it changed. */
function fill(view: string, v: ReturnType<typeof viewOf>, tris: readonly Tri[], solid: any, done: Set<string>, patch?: P & { r?: number }): boolean {
  const defs = new Map<string, any>((asset.volumes[sc].body as any[]).map((x) => [x.id, x]));
  const material = defs.get(solid.id)?.material;
  const fills = v.entries.filter((e) => e.fills && e.shape.role === "body" && e.shape.filled !== false && e.shape.solid === undefined);
  const size = (e: Entry): number => Math.abs(e.rings.reduce((n, r) => n + areaOf(r), 0));
  const pick = (l: Entry[]): Entry | undefined => l.sort((a, b) => size(b) - size(a))[0];
  const own = fills.filter((e) => e.shape.bone === solid.bone);
  const parents = new Map<string, string | null>((asset.rig.bones as any[]).map((b) => [b.id, b.parent]));
  let above: Entry | undefined;
  if (own.length === 0) for (let b = parents.get(solid.bone); b != null && above === undefined; b = parents.get(b)) above = pick(fills.filter((e) => e.shape.bone === b && e.shape.material === material));
  const main = pick(own.filter((e) => e.shape.material === material)) ?? pick(own) ?? above;
  const sameMaterial = main !== undefined && main.shape.material === material;
  const mine = tris.filter((t) => t.solid.bone === solid.bone && t.solid.slender !== true && (main !== above || defs.get(t.solid.id)?.material === material) && (!sameMaterial || defs.get(t.solid.id)?.material === material || !own.some((e) => e.shape.material === defs.get(t.solid.id)?.material)));
  if (mine.length === 0) { log(`${view} solid ${solid.id}: nothing to trace on bone ${solid.bone}`); return false; }
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  for (const t of mine) { b.minX = Math.min(b.minX, t.ax, t.bx, t.cx); b.maxX = Math.max(b.maxX, t.ax, t.bx, t.cx); b.minY = Math.min(b.minY, t.ay, t.by, t.cy); b.maxY = Math.max(b.maxY, t.ay, t.by, t.cy); }
  if (main !== undefined) { b.minX = Math.min(b.minX, main.box.minX); b.maxX = Math.max(b.maxX, main.box.maxX); b.minY = Math.min(b.minY, main.box.minY); b.maxY = Math.max(b.maxY, main.box.maxY); }
  if (patch !== undefined) { const pr = (patch.r ?? 4 * v.cell) + v.cell; b.minX = Math.min(b.minX, patch.x - pr); b.maxX = Math.max(b.maxX, patch.x + pr); b.minY = Math.min(b.minY, patch.y - pr); b.maxY = Math.max(b.maxY, patch.y + pr); }
  const box = v.snap(grown(b, 3 * v.cell));
  const M = new Mask(box, v.cell);
  for (const t of mine) M.fill([[{ x: t.ax, y: t.ay }, { x: t.bx, y: t.by }, { x: t.cx, y: t.cy }]], true);
  M.seal();
  const V = volumeMask(v.groups, box, v.cell);
  if (main !== undefined) { const L = new Mask(box, v.cell); L.fill(main.rings); for (let k = 0; k < L.cells.length; k++) if (L.cells[k] && V.cells[k]) M.cells[k] = 1; }
  for (let k = 0; k < M.cells.length; k++) if (M.cells[k] && !V.cells[k]) M.cells[k] = 0;
  if (patch !== undefined) for (let j = 0; j < M.height; j++) for (let i = 0; i < M.width; i++) if (V.cells[j * M.width + i] && Math.hypot(box.minX + (i + 0.5) * v.cell - patch.x, box.minY + (j + 0.5) * v.cell - patch.y) <= (patch.r ?? 4 * v.cell)) M.cells[j * M.width + i] = 1;
  const template = main?.shape ?? pick(fills.filter((e) => e.shape.material === material))?.shape ?? pick(fills.slice())?.shape;
  if (template === undefined) { log(`${view} solid ${solid.id}: no filled shape to copy for bone ${solid.bone}`); return false; }
  const id = main?.shape.id ?? `${solid.bone}-body`;
  const source = main !== undefined ? geo(view).find((s) => s.id === id) : { ...geo(view).find((s) => s.id === template.id), id, bone: solid.bone };
  if (done.has(id)) return false;
  done.add(id);
  const rings = ringsOf(M, epsFor(view, id, v.feature), v.feature);
  if (rings.length === 0) return false;
  const fit = placed(asset, scale, view, source, rings);
  if (fit === undefined) { log(`${view} ${id}: cannot be placed, left`); return false; }
  const shape = fit.shape;
  if (main !== undefined) put(view, { op: "shape", scale: sc, view, id, shape });
  else {
    if (geo(view).some((s) => s.id === id)) { log(`${view} ${id}: the id is taken by a shape that does not fill, left`); return false; }
    put(view, { op: "shape-add", scale: sc, view, after: template.id, shape });
  }
  log(`${view} ${id}: ${main !== undefined ? "joined with" : "added as"} the outline of the solids of bone ${solid.bone}${sameMaterial ? ` (${material})` : ""} (${rings.length} rings, ${rings.reduce((n, r) => n + r.length, 0)} points${note(fit)})`);
  return true;
}

for (const view of views) {
  const limit0 = 2 * (await import("./lib.ts")).featureOf(scale);
  const limit = Number(values.limit ?? 0.85 * limit0);
  let last = Infinity;
  for (let round = 1; round <= Number(values.rounds ?? 8); round++) {
    const v = viewOf(asset, scale, view);
    const { clusters, worst } = scan(view, v, limit);
    log(`${slug} 1:${scale} ${view} round ${round}: largest distance ${worst.toFixed(4)} m (limit ${limit0}), ${clusters.length} clusters`);
    if (clusters.length === 0) break;
    if (values.dry) { for (const c of clusters.slice(0, 20)) log(`  ${c.side} (${c.x.toFixed(3)}, ${c.y.toFixed(3)}) ${c.d.toFixed(4)} ${c.kind}`); break; }
    const tris = trianglesOf(v.solids, view);
    const done = new Set<string>();
    let changed = 0;
    const targets = values.all && round === 1 ? "all" : "clusters";
    const followed = strokeDrawings(view as never, v.solids as never, drawnFor(asset, v.state, view as never, scale));
    const slenderAt = (x: number, y: number): any => v.solids.find((s) => s.slender === true && strokesOf(view as never, s, followed).some((q) => inRings([q as P[]], x, y)));
    if (values.patch !== undefined && round === 1) {
      const like = values.patch.split(",")[3];
      const [x, y, r] = values.patch.split(",").slice(0, 3).map(Number) as [number, number, number];
      if (like !== undefined) {
        const from = geo(view).find((s) => s.id === like);
        const box = v.snap({ minX: x - r - 2 * v.cell, minY: y - r - 2 * v.cell, maxX: x + r + 2 * v.cell, maxY: y + r + 2 * v.cell });
        const M = volumeMask(v.groups, box, v.cell);
        for (let j = 0; j < M.height; j++) for (let i = 0; i < M.width; i++) if (Math.hypot(box.minX + (i + 0.5) * v.cell - x, box.minY + (j + 0.5) * v.cell - y) > r) M.cells[j * M.width + i] = 0;
        const rings = trace(M).filter((ring) => areaOf(ring) > 0).map((ring) => simplify(ring, 0.02 * v.feature));
        const { penMm: _pen, d: _d, ...rest } = from;
        const fit = rings.length === 0 ? undefined : placed(asset, scale, view, { ...rest, id: `${like}-joint`, filled: true, kind: "path", d: "" }, rings);
        if (fit !== undefined) { put(view, { op: "shape-add", scale: sc, view, after: like, shape: fit.shape }); changed++; log(`${view} ${like}-joint: added as the volume within ${r} m of (${x}, ${y}), a filled shape on bone ${from.bone} (${rings.length} rings)`); }
      } else {
      let near: any, best = Infinity;
      for (const tr of tris) if (tr.solid.slender !== true) { const d = Math.min(Math.hypot(tr.ax - x, tr.ay - y), Math.hypot(tr.bx - x, tr.by - y), Math.hypot(tr.cx - x, tr.cy - y)); if (d < best) { best = d; near = tr.solid; } }
      if (near !== undefined && fill(view, v, tris, near, done, { x, y, r })) changed++;
      }
    } else if (values.restroke && round === 1) { for (const s of v.solids) if (s.slender === true && restroke(view, v, s, done)) changed++; }
    else if (targets === "all") {
      for (const e of v.entries) if (e.fills) { if (clip(view, v, e)) changed++; } else if (clipStroke(view, v, e)) changed++;
    } else for (const c of clusters.slice(0, 60)) {
      if (c.kind === "none") { log(`  ${c.side} (${c.x.toFixed(3)}, ${c.y.toFixed(3)}) ${c.d.toFixed(4)}: no witness cell`); continue; }
      if (c.kind === "extra") {
        const covering = v.entries.filter((e) => e.fills ? inRings(e.rings, c.qx, c.qy) : e.quads.some((q) => inRings([q], c.qx, c.qy)));
        if (covering.length === 0) log(`  extra at (${c.qx.toFixed(3)}, ${c.qy.toFixed(3)}) ${c.d.toFixed(4)}: no shape covers it`);
        for (const e of covering) {
          const twin = e.fills ? undefined : v.solids.find((s) => s.slender === true && s.bone === e.shape.bone && (s.id === e.shape.id || `${s.id}-line` === e.shape.id));
          if (twin !== undefined) { if (restroke(view, v, twin, done, { x: c.qx, y: c.qy, sign: -1 })) changed++; continue; }
          if (done.has(e.shape.id)) continue;
          done.add(e.shape.id);
          if (e.fills ? clip(view, v, e) : clipStroke(view, v, e)) changed++;
        }
      } else {
        const hit = tris.filter((t) => inTriangle(t, c.qx, c.qy)).map((t) => t.solid);
        const solid = hit.find((s) => s.slender !== true && v.entries.some((e) => e.fills && e.shape.bone === s.bone)) ?? hit.find((s) => s.slender !== true) ?? slenderAt(c.qx, c.qy);
        if (solid === undefined) {
          let near: any, best = Infinity;
          for (const tr of tris) if (tr.solid.slender !== true) { const d = Math.min(Math.hypot(tr.ax - c.qx, tr.ay - c.qy), Math.hypot(tr.bx - c.qx, tr.by - c.qy), Math.hypot(tr.cx - c.qx, tr.cy - c.qy)); if (d < best) { best = d; near = tr.solid; } }
          if (near === undefined || best > 6 * v.cell) { log(`  missing at (${c.qx.toFixed(3)}, ${c.qy.toFixed(3)}) ${c.d.toFixed(4)}: no solid there`); continue; }
          if (fill(view, v, tris, near, done, { x: c.qx, y: c.qy })) changed++;
          continue;
        }
        if (solid.slender === true) { if (restroke(view, v, solid, done, { x: c.qx, y: c.qy, sign: 1 })) changed++; continue; }
        const key = `fill|${solid.bone}|${(asset.volumes[sc].body as any[]).find((x) => x.id === solid.id)?.material}`;
        if (done.has(key)) continue;
        done.add(key);
        if (fill(view, v, tris, solid, done)) changed++;
      }
    }
    if (changed === 0) { log(`${slug} 1:${scale} ${view}: nothing left to change, ${clusters.length} clusters stay (worst ${worst.toFixed(4)})`); for (const c of clusters.slice(0, 8)) log(`  ${c.side} (${c.x.toFixed(3)}, ${c.y.toFixed(3)}) ${c.d.toFixed(4)} ${c.kind}`); break; }
    if (worst >= last && round > 3) log(`${slug} 1:${scale} ${view}: no progress in round ${round}`);
    last = worst;
  }
}
if (!values.novalidate) {
  const fresh = (validateAsset(structuredClone(asset) as never) as any[]).filter((i) => !baseKeys.has(keyOf(i)));
  log(`${slug} 1:${scale}: ${fresh.length} new validateAsset issues`);
  for (const i of fresh.slice(0, 8)) log(`  ${i.code} ${i.path ?? ""}: ${i.message.en}`.slice(0, 300));
}
if (!values.dry && made.length > 0) await writeOps(opsFile(slug!), mergeOps(startOps, made));
log(`${slug} 1:${scale} ${views.join(",")}: ${made.length} ops${values.dry ? " (dry)" : ""}`);
