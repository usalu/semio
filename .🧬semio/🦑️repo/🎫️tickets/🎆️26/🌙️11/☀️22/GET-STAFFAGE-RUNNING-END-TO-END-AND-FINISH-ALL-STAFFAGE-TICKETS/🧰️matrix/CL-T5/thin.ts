/** 🪒️ Thins the facets of polyhedra of the body of a tree without moving its outline in the three views of the silhouette guard, for masses that are not convex (`crown.ts` is for convex lobes): per scale every body solid in the default pose is drawn into a coverage count per view (cells of `--cell` mm of paper). Solid after solid, inner ones first, a corner point is merged into a neighbour (the facets around it are spanned from that neighbour, no corner point moves and none is added) when (1) the shell stays a closed shell that the kernel accepts as a polyhedron (one outward shell whose facets do not cut each other) and no facet turns by more than 60 degrees, (2) every corner point taken away so far stays within the tolerance of the new facets (`--error` mm of paper, at most `--share` of three times volume by area of the solid, which is the radius of a ball and 1.5 radii of a tube, so a twig is never flattened) and the solid keeps at least `--volume` (default 0.8) of its volume, and (3) in each view either the facets around the corner point all face the same way before and after, so that they cover exactly the same patch of the view, or every facet before and after lies, with a margin of one cell, where the other solids as they are at that moment cover the view. So no step changes the union of the outlines. A corner point within 0.2 m beyond the reach of an outlet of the thermal list (`--emitters`) stays. At the end the coverage of every view is drawn again from the thinned solids and compared cell by cell. With `--list structure` the solids of the structure are thinned by rules (1) and (2) alone, because no view of the guard shows them. thin.ts <asset id | file.json> --out <ops.json> [--match <regular expression>] [--emitters <regular expression>] [--error 1] [--share 0.1] [--cell 0.06] [--volume 0.8] [--scales 50,100,200,500] [--list structure] writes the `thin` ops of `facet.ts` and prints the counts.
 * @see ./crown.ts
 * @see ./facet.ts */
import { writeFileSync } from "node:fs";
import { parseArgs } from "node:util";
import { polyhedronFault, tessellateSolid } from "../../../../../../../../../🧰️framework/🔨️modules/🧊️3d/🗿️model/🟦️.ts";
import { worldSolids } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";
import { evaluate } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { qualityOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧊️volume/⏱️job/🟦️.ts";
import { readDoc } from "../MR/lib.ts";

type W = { id: string; positions: Float64Array; indices: Uint32Array };
type V3 = [number, number, number];
const { values, positionals } = parseArgs({ options: { out: { type: "string" }, match: { type: "string" }, emitters: { type: "string" }, error: { type: "string" }, share: { type: "string" }, cell: { type: "string" }, scales: { type: "string" }, list: { type: "string" }, volume: { type: "string" } }, allowPositionals: true });
const asset = readDoc(positionals[0]!).doc.asset;
const quality = qualityOf(asset);
const pattern = new RegExp(values.match ?? ".", "u");
const outlets = new RegExp(values.emitters ?? "^em-", "u");
const errorMm = Number(values.error ?? 1);
const share = Number(values.share ?? 0.1);
const cellMm = Number(values.cell ?? 0.06);
const floor = Number(values.volume ?? 0.8);
const list = (values.list ?? "body") as "body" | "structure";
const AXES: [number, number][] = [[0, 1], [2, 1], [0, 2]];
const sub = (a: V3, b: V3): V3 => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const cross = (a: V3, b: V3): V3 => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];

/** 📍️ The distance of a point from a triangle in space. */
function distance(p: V3, a: V3, b: V3, c: V3): number {
  const ab = sub(b, a), ac = sub(c, a), ap = sub(p, a);
  const d1 = dot(ab, ap), d2 = dot(ac, ap);
  if (d1 <= 0 && d2 <= 0) return Math.hypot(...ap);
  const bp = sub(p, b), d3 = dot(ab, bp), d4 = dot(ac, bp);
  if (d3 >= 0 && d4 <= d3) return Math.hypot(...bp);
  const vc = d1 * d4 - d3 * d2;
  if (vc <= 0 && d1 >= 0 && d3 <= 0) { const t = d1 / (d1 - d3); return Math.hypot(ap[0] - t * ab[0], ap[1] - t * ab[1], ap[2] - t * ab[2]); }
  const cp = sub(p, c), d5 = dot(ab, cp), d6 = dot(ac, cp);
  if (d6 >= 0 && d5 <= d6) return Math.hypot(...cp);
  const vb = d5 * d2 - d1 * d6;
  if (vb <= 0 && d2 >= 0 && d6 <= 0) { const t = d2 / (d2 - d6); return Math.hypot(ap[0] - t * ac[0], ap[1] - t * ac[1], ap[2] - t * ac[2]); }
  const va = d3 * d6 - d5 * d4;
  if (va <= 0 && d4 - d3 >= 0 && d5 - d6 >= 0) { const t = (d4 - d3) / (d4 - d3 + (d5 - d6)); return Math.hypot(bp[0] - t * (c[0] - b[0]), bp[1] - t * (c[1] - b[1]), bp[2] - t * (c[2] - b[2])); }
  const n = cross(ab, ac);
  return Math.abs(dot(n, ap)) / Math.hypot(...n);
}

const ops: unknown[] = [];
for (const scale of (values.scales ?? "50,100,200,500").split(",").map(Number)) {
  const started = Date.now();
  const cell = (cellMm / 1000) * scale;
  const state = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const authored = new Map((asset.volumes[`${scale}`][list] as Record<string, unknown>[]).map((s) => [s.id as string, s]));
  const worlds = worldSolids(asset, state.bones as never, scale, list) as unknown as W[];
  const emitters = (worldSolids(asset, state.bones as never, scale, "thermal") as unknown as W[]).filter((w) => outlets.test(w.id)).map((w) => {
    const c: V3 = [0, 0, 0];
    for (let i = 0; i < w.positions.length; i++) c[i % 3]! += w.positions[i]! / (w.positions.length / 3);
    let r = 0;
    for (let i = 0; i < w.positions.length; i += 3) r = Math.max(r, Math.hypot(w.positions[i]! - c[0], w.positions[i + 1]! - c[1], w.positions[i + 2]! - c[2]));
    return { c, r: r + 0.2 };
  });
  const lo = [Infinity, Infinity, Infinity], hi = [-Infinity, -Infinity, -Infinity];
  for (const w of worlds) for (let i = 0; i < w.positions.length; i++) { lo[i % 3] = Math.min(lo[i % 3]!, w.positions[i]!); hi[i % 3] = Math.max(hi[i % 3]!, w.positions[i]!); }
  const grids = AXES.map(([ax, ay]) => { const x0 = lo[ax]! - 4 * cell, y0 = lo[ay]! - 4 * cell; const nx = Math.ceil((hi[ax]! - x0) / cell) + 5, ny = Math.ceil((hi[ay]! - y0) / cell) + 5; return { ax, ay, x0, y0, nx, ny, count: new Uint16Array(nx * ny) }; });

  /** 🖌️ Adds `by` to the coverage count of every cell of a view whose centre lies in a facet of the mesh, each cell once. */
  const paint = (view: number, positions: ArrayLike<number>, triangles: ArrayLike<number>, by: number): void => {
    const g = grids[view]!;
    let bx0 = Infinity, by0 = Infinity, bx1 = -Infinity, by1 = -Infinity;
    for (let i = 0; i < positions.length; i += 3) { const x = positions[i + g.ax]!, y = positions[i + g.ay]!; if (x < bx0) bx0 = x; if (x > bx1) bx1 = x; if (y < by0) by0 = y; if (y > by1) by1 = y; }
    const i0 = Math.max(0, Math.floor((bx0 - g.x0) / cell) - 1), j0 = Math.max(0, Math.floor((by0 - g.y0) / cell) - 1);
    const w = Math.min(g.nx - 1, Math.ceil((bx1 - g.x0) / cell) + 1) - i0 + 1, h = Math.min(g.ny - 1, Math.ceil((by1 - g.y0) / cell) + 1) - j0 + 1;
    const seen = new Uint8Array(w * h);
    for (let t = 0; t < triangles.length; t += 3) {
      const [a, b, c] = [triangles[t]! * 3, triangles[t + 1]! * 3, triangles[t + 2]! * 3];
      const [xa, ya, xb, yb, xc, yc] = [positions[a + g.ax]!, positions[a + g.ay]!, positions[b + g.ax]!, positions[b + g.ay]!, positions[c + g.ax]!, positions[c + g.ay]!];
      const area = (xb - xa) * (yc - ya) - (yb - ya) * (xc - xa);
      if (area === 0) continue;
      const s = area > 0 ? 1 : -1;
      const ia = Math.max(i0, Math.ceil((Math.min(xa, xb, xc) - g.x0) / cell - 0.5)), ib = Math.min(i0 + w - 1, Math.floor((Math.max(xa, xb, xc) - g.x0) / cell - 0.5));
      const ja = Math.max(j0, Math.ceil((Math.min(ya, yb, yc) - g.y0) / cell - 0.5)), jb = Math.min(j0 + h - 1, Math.floor((Math.max(ya, yb, yc) - g.y0) / cell - 0.5));
      for (let j = ja; j <= jb; j++) {
        const y = g.y0 + (j + 0.5) * cell;
        for (let i = ia; i <= ib; i++) {
          const x = g.x0 + (i + 0.5) * cell;
          if (s * ((xb - xa) * (y - ya) - (yb - ya) * (x - xa)) >= 0 && s * ((xc - xb) * (y - yb) - (yc - yb) * (x - xb)) >= 0 && s * ((xa - xc) * (y - yc) - (ya - yc) * (x - xc)) >= 0) seen[(j - j0) * w + (i - i0)] = 1;
        }
      }
    }
    for (let j = 0; j < h; j++) for (let i = 0; i < w; i++) if (seen[j * w + i] === 1) g.count[(j + j0) * g.nx + i + i0]! += by;
  };

  /** 🫥️ Whether a facet lies in a view, with a margin of one cell, where other solids cover the view: every cell whose centre is within one cell of the facet and its eight neighbours are covered. */
  const hidden = (view: number, p: V3, q: V3, r: V3): boolean => {
    const g = grids[view]!;
    const [xa, ya, xb, yb, xc, yc] = [p[g.ax]!, p[g.ay]!, q[g.ax]!, q[g.ay]!, r[g.ax]!, r[g.ay]!];
    const area = (xb - xa) * (yc - ya) - (yb - ya) * (xc - xa);
    const s = area >= 0 ? 1 : -1;
    const [la, lb, lc] = [Math.hypot(xb - xa, yb - ya) || 1, Math.hypot(xc - xb, yc - yb) || 1, Math.hypot(xa - xc, ya - yc) || 1];
    const flat = Math.abs(area) < cell * Math.max(la, lb, lc) * 0.5;
    const ia = Math.max(1, Math.floor((Math.min(xa, xb, xc) - g.x0) / cell - 1.5)), ib = Math.min(g.nx - 2, Math.ceil((Math.max(xa, xb, xc) - g.x0) / cell + 0.5));
    const ja = Math.max(1, Math.floor((Math.min(ya, yb, yc) - g.y0) / cell - 1.5)), jb = Math.min(g.ny - 2, Math.ceil((Math.max(ya, yb, yc) - g.y0) / cell + 0.5));
    for (let j = ja; j <= jb; j++) {
      const y = g.y0 + (j + 0.5) * cell;
      for (let i = ia; i <= ib; i++) {
        const x = g.x0 + (i + 0.5) * cell;
        if (!flat && !((s * ((xb - xa) * (y - ya) - (yb - ya) * (x - xa))) / la >= -cell && (s * ((xc - xb) * (y - yb) - (yc - yb) * (x - xb))) / lb >= -cell && (s * ((xa - xc) * (y - yc) - (ya - yc) * (x - xc))) / lc >= -cell)) continue;
        const at = j * g.nx + i;
        if (g.count[at] === 0 || g.count[at - 1] === 0 || g.count[at + 1] === 0 || g.count[at - g.nx] === 0 || g.count[at + g.nx] === 0 || g.count[at - g.nx - 1] === 0 || g.count[at - g.nx + 1] === 0 || g.count[at + g.nx - 1] === 0 || g.count[at + g.nx + 1] === 0) return false;
      }
    }
    return true;
  };

  for (const w of worlds) for (let v = 0; v < 3; v++) paint(v, w.positions, w.indices, 1);
  const was = grids.map((g) => { const m = new Uint8Array(g.count.length); for (let i = 0; i < m.length; i++) m[i] = g.count[i]! > 0 ? 1 : 0; return m; });
  const middle = [0, 1, 2].map((k) => (lo[k]! + hi[k]!) / 2);
  const centreOf = (w: W): number => { const c = [0, 0, 0]; for (let i = 0; i < w.positions.length; i++) c[i % 3]! += w.positions[i]! / (w.positions.length / 3); return Math.hypot(c[0]! - middle[0]!, c[1]! - middle[1]!, c[2]! - middle[2]!); };
  const chosen = worlds.filter((w) => { const s = authored.get(w.id); return pattern.test(w.id) && s?.kind === "polyhedron" && s.repeat === undefined; }).sort((a, b) => centreOf(a) - centreOf(b));
  const thin: Record<string, { keep: number[]; triangles: number[][] }> = {};
  const after = new Map<string, { positions: Float64Array; indices: Uint32Array }>();
  let before = 0, left = 0, exact = 0, covered = 0, worst = 0, facetsBefore = 0, facetsAfter = 0;
  for (const w of chosen) {
    const solid = authored.get(w.id)!;
    const vertices = solid.vertices as number[][];
    const local = tessellateSolid({ id: "lobe", node: "n", material: "m", kind: "polyhedron", vertices, triangles: solid.triangles } as never, quality).positions;
    if (local.length !== w.positions.length) throw new Error(`${w.id}: ${local.length / 3} local and ${w.positions.length / 3} posed vertices`);
    const index = new Map(vertices.map((p, k) => [p.join(","), k]));
    const P: V3[] = new Array(vertices.length);
    for (let i = 0; i < local.length; i += 3) { let k = index.get(`${local[i]},${local[i + 1]},${local[i + 2]}`); if (k === undefined) { let d = Infinity; vertices.forEach((p, at) => { const x = (p[0]! - local[i]!) ** 2 + (p[1]! - local[i + 1]!) ** 2 + (p[2]! - local[i + 2]!) ** 2; if (x < d) { d = x; k = at; } }); if (d > 1e-12) throw new Error(`${w.id}: a posed vertex is no authored corner point`); } P[k!] = [w.positions[i]!, w.positions[i + 1]!, w.positions[i + 2]!]; }
    const tris: (number[] | undefined)[] = (solid.triangles as number[][]).map((t) => [...t]);
    if (P.some((p, k) => p === undefined && tris.some((t) => t!.includes(k)))) throw new Error(`${w.id}: a corner point of a facet is not in the posed mesh`);
    const star: Set<number>[] = vertices.map(() => new Set());
    tris.forEach((t, at) => { for (const k of t!) star[k]!.add(at); });
    const lost: number[][] = tris.map(() => []);
    let volume = 0, area = 0;
    const origin = P[tris[0]![0]!]!;
    const wedge = (t: number[]): number => dot(sub(P[t[0]!]!, origin), cross(sub(P[t[1]!]!, origin), sub(P[t[2]!]!, origin))) / 6;
    for (const t of tris) { const [a, b, c] = [P[t![0]!]!, P[t![1]!]!, P[t![2]!]!]; volume += wedge(t!); area += Math.hypot(...cross(sub(b, a), sub(c, a))) / 2; }
    const least = floor * Math.abs(volume);
    let now = Math.abs(volume);
    const turn = volume < 0 ? -1 : 1;
    const tolerance = Math.min((errorMm / 1000) * scale, (share * 3 * Math.abs(volume)) / area);
    const fixed = vertices.map((_, k) => P[k] !== undefined && emitters.some((e) => Math.hypot(P[k]![0] - e.c[0], P[k]![1] - e.c[1], P[k]![2] - e.c[2]) < e.r));
    for (let v = 0; v < 3; v++) paint(v, w.positions, w.indices, -1);
    const sign = (view: number, t: number[]): number => { const [ax, ay] = AXES[view]!; const [a, b, c] = [P[t[0]!]!, P[t[1]!]!, P[t[2]!]!]; const s = (b[ax]! - a[ax]!) * (c[ay]! - a[ay]!) - (b[ay]! - a[ay]!) * (c[ax]! - a[ax]!); return s > 1e-12 ? 1 : s < -1e-12 ? -1 : 0; };
    const around = (u: number): Set<number> => { const n = new Set<number>(); for (const at of star[u]!) for (const k of tris[at]!) if (k !== u) n.add(k); return n; };

    /** ⚖️ The error of merging corner point u into its neighbour v and how the three views allow it (2 exact, 1 covered), or nothing when a rule forbids it. */
    const merge = (u: number, v: number): { error: number; how: number; change: number } | undefined => {
      const mine = around(u), theirs = around(v);
      let shared = 0;
      for (const k of mine) if (theirs.has(k)) shared += 1;
      if (shared !== 2) return undefined;
      const old = [...star[u]!].map((at) => tris[at]!);
      const kept = old.filter((t) => !t.includes(v));
      if (old.length - kept.length !== 2 || kept.length === 0) return undefined;
      const next = kept.map((t) => t.map((k) => (k === u ? v : k)));
      const there = new Set([...star[v]!].map((at) => [...tris[at]!].sort((x, y) => x - y).join(",")));
      if (next.some((t) => there.has([...t].sort((x, y) => x - y).join(",")))) return undefined;
      for (let i = 0; i < kept.length; i++) {
        const n0 = cross(sub(P[kept[i]![1]!]!, P[kept[i]![0]!]!), sub(P[kept[i]![2]!]!, P[kept[i]![0]!]!)), n1 = cross(sub(P[next[i]![1]!]!, P[next[i]![0]!]!), sub(P[next[i]![2]!]!, P[next[i]![0]!]!));
        const l0 = Math.hypot(...n0), l1 = Math.hypot(...n1);
        if (!(l1 > 1e-10) || dot(n0, n1) < 0.5 * l0 * l1) return undefined;
      }
      const change = turn * (next.reduce((sum, t) => sum + wedge(t), 0) - old.reduce((sum, t) => sum + wedge(t), 0));
      if (now + change < least) return undefined;
      let error = 0;
      const points = [u];
      for (const at of star[u]!) for (const k of lost[at]!) points.push(k);
      for (const k of points) {
        let d = Infinity;
        for (const t of next) d = Math.min(d, distance(P[k]!, P[t[0]!]!, P[t[1]!]!, P[t[2]!]!));
        if (d > error) error = d;
        if (error > tolerance) return undefined;
      }
      let how = 2;
      for (let view = 0; view < 3 && list === "body"; view++) {
        const s = sign(view, old[0]!);
        if (s !== 0 && old.every((t) => sign(view, t) === s) && next.every((t) => sign(view, t) === s)) continue;
        how = 1;
        for (const t of [...old, ...next]) if (!hidden(view, P[t[0]!]!, P[t[1]!]!, P[t[2]!]!)) return undefined;
      }
      return { error, how, change };
    };
    const banned = new Set<number>();
    const best: ({ to: number; error: number; how: number; change: number } | undefined)[] = vertices.map(() => undefined);
    const rate = (u: number): void => {
      best[u] = undefined;
      if (fixed[u] || star[u]!.size === 0) return;
      for (const v of around(u)) { if (banned.has(u * vertices.length + v)) continue; const m = merge(u, v); if (m !== undefined && (best[u] === undefined || m.error < best[u]!.error)) best[u] = { to: v, ...m }; }
    };
    let alive = 0;
    for (let u = 0; u < vertices.length; u++) if (star[u]!.size > 0) { alive += 1; rate(u); }
    before += alive;
    facetsBefore += tris.length;
    while (alive > 8) {
      let u = -1;
      for (let k = 0; k < best.length; k++) if (best[k] !== undefined && (u < 0 || best[k]!.error < best[u]!.error)) u = k;
      if (u < 0) break;
      const rated = best[u]!;
      rate(u);
      if (best[u] === undefined || best[u]!.to !== rated.to || best[u]!.error > rated.error) continue;
      const { to: v, error, how, change } = best[u]!;
      const used = new Set<number>();
      const trial = tris.filter((t) => t !== undefined && !(t.includes(u) && t.includes(v))).map((t) => t!.map((k) => { const x = k === u ? v : k; used.add(x); return x; }));
      const order = new Map([...used].map((k, i) => [k, i]));
      if (polyhedronFault([...used].map((k) => vertices[k]!), trial.map((t) => t.map((k) => order.get(k)!))) !== null) { banned.add(u * vertices.length + v); rate(u); continue; }
      const ring = around(u);
      const points = [u];
      const changed: number[] = [];
      for (const at of [...star[u]!]) {
        points.push(...lost[at]!);
        lost[at] = [];
        if (tris[at]!.includes(v)) { for (const k of tris[at]!) star[k]!.delete(at); tris[at] = undefined; } else { tris[at] = tris[at]!.map((k) => (k === u ? v : k)); star[v]!.add(at); changed.push(at); }
      }
      star[u] = new Set();
      for (const k of points) { let d = Infinity, home = changed[0]!; for (const at of changed) { const t = tris[at]!; const x = distance(P[k]!, P[t[0]!]!, P[t[1]!]!, P[t[2]!]!); if (x < d) { d = x; home = at; } } lost[home]!.push(k); }
      best[u] = undefined;
      now += change;
      alive -= 1;
      if (how === 2) exact += 1; else covered += 1;
      if (error > worst) worst = error;
      for (const k of ring) rate(k);
    }
    left += alive;
    const keep = vertices.map((_, k) => k).filter((k) => star[k]!.size > 0);
    const at = new Map(keep.map((k, i) => [k, i]));
    const faces = tris.filter((t) => t !== undefined).map((t) => t!.map((k) => at.get(k)!));
    const twins = new Set(faces.flatMap(([a, b, c]) => [`${a},${b}`, `${b},${c}`, `${c},${a}`]));
    if (twins.size !== faces.length * 3 || [...twins].some((e) => !twins.has(e.split(",").reverse().join(",")))) throw new Error(`${w.id}: the thinned facets are no closed shell`);
    facetsAfter += faces.length;
    const positions = new Float64Array(keep.length * 3);
    keep.forEach((k, i) => positions.set(P[k]!, i * 3));
    const indices = new Uint32Array(faces.flat());
    after.set(w.id, { positions, indices });
    for (let v = 0; v < 3; v++) paint(v, positions, indices, 1);
    if (keep.length < vertices.length) thin[w.id] = { keep, triangles: faces };
  }
  const differ = grids.map((g, v) => { let n = 0; for (let i = 0; i < g.count.length; i++) if ((g.count[i]! > 0 ? 1 : 0) !== was[v]![i]) n += 1; return n; });
  console.log(`1:${scale}: cell ${(cell * 1000).toFixed(1)} mm, ${worlds.length} solids drawn, ${chosen.length} thinned: ${before} -> ${left} corner points, ${facetsBefore} -> ${facetsAfter} facets; ${exact} merges cover the same patch in every view, ${covered} lie where other solids cover; largest distance of a corner point taken away ${(worst * 1000).toFixed(1)} mm; cells whose coverage differs afterwards: elevation ${differ[0]}, front ${differ[1]}, plan ${differ[2]} (${((Date.now() - started) / 1000).toFixed(0)} s)`);
  if (Object.keys(thin).length > 0) ops.push({ thin, list, scales: [scale] });
}
if (values.out !== undefined) writeFileSync(values.out, JSON.stringify(ops) + "\n");
