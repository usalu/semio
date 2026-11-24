/** 🧩️ The shared machinery of the R4-P tools: the guard's raster of one view cut into the guard's own tiles, the drawn and the volume mask of a tile as the guard fills them, and the boundary cells whose distance to the other boundary exceeds a limit.
 * @see /Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts — the measure */
import { featureOf } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🧱️rules/🟦️.ts";
import { evaluate } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { Mask, TILE_CELLS, WINDOW_FEATURES, boundaryOf, boundsOf, comparisonCell, distanceField, drawnFor, fillSmallHoles, measureView, referenceOf, worldSolids } from "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts";

export type P = { x: number; y: number };
export type Box = { minX: number; minY: number; maxX: number; maxY: number };
export const VIEWS = ["elevation", "front", "plan"] as const;
export const ROLES = ["body", "detail", "glazing"];

/** 🎬️ One view of the default pose at a scale, prepared as the guard prepares it. */
export function scene(asset: any, scale: number, view: string) {
  const feature = featureOf(scale);
  const state: any = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: scale } } as never);
  const solids = worldSolids(asset, state.bones as never, scale).filter((w: any) => w.texture !== true);
  const section = asset.representations[String(scale)].sections?.[view];
  if (section !== undefined) throw new Error(`${asset.id} 1:${scale} ${view} declares a section: not handled by R4-P`);
  const reference = referenceOf(view as never, solids, undefined, drawnFor(asset, state, view as never, scale)).cut as P[][][];
  const evaluated = (state.shapes[view] as any[]).filter((s) => ROLES.includes(s.shape.role) && s.rings.length > 0);
  const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
  const grow = (q: readonly [number, number, number, number]): void => { b.minX = Math.min(b.minX, q[0]); b.maxX = Math.max(b.maxX, q[1]); b.minY = Math.min(b.minY, q[2]); b.maxY = Math.max(b.maxY, q[3]); };
  for (const g of reference) grow(boundsOf(g));
  for (const s of evaluated) grow(boundsOf(s.rings));
  const cell = comparisonCell(feature, Math.max(b.maxX - b.minX, b.maxY - b.minY));
  const frame: Box = { minX: b.minX - 2 * cell, minY: b.minY - 2 * cell, maxX: b.maxX + 2 * cell, maxY: b.maxY + 2 * cell };
  return { asset, scale, view, feature, state, solids, reference, evaluated, cell, frame };
}
export type Scene = ReturnType<typeof scene>;

/** 🧱️ The tiles of a scene (core and window with the guard's margin) that hold any geometry, with the geometry that touches each window. */
export function tiles(sc: Scene, only?: Box): { core: Box; box: Box; reference: P[][][]; evaluated: any[] }[] {
  const { cell, frame, feature } = sc;
  const side = TILE_CELLS * cell;
  const margin = Math.ceil((feature * WINDOW_FEATURES) / cell) * cell;
  const [nx, ny] = [Math.ceil((frame.maxX - frame.minX) / side), Math.ceil((frame.maxY - frame.minY) / side)];
  const bucket = <T>(items: readonly T[], boxOf: (t: T) => readonly [number, number, number, number]): T[][] => {
    const out: T[][] = Array.from({ length: nx * ny }, () => []);
    for (const t of items) {
      const q = boxOf(t);
      const [i0, i1] = [Math.max(0, Math.floor((q[0] - margin - frame.minX) / side)), Math.min(nx - 1, Math.floor((q[1] + margin - frame.minX) / side))];
      const [j0, j1] = [Math.max(0, Math.floor((q[2] - margin - frame.minY) / side)), Math.min(ny - 1, Math.floor((q[3] + margin - frame.minY) / side))];
      for (let j = j0; j <= j1; j++) for (let i = i0; i <= i1; i++) out[j * nx + i]!.push(t);
    }
    return out;
  };
  const refs = bucket(sc.reference, (g) => boundsOf(g));
  const evs = bucket(sc.evaluated, (s) => { const q = boundsOf(s.rings); const pen = 0.01; return [q[0] - pen, q[1] + pen, q[2] - pen, q[3] + pen] as const; });
  const out: { core: Box; box: Box; reference: P[][][]; evaluated: any[] }[] = [];
  for (let j = 0; j < ny; j++) for (let i = 0; i < nx; i++) {
    const [minX, minY] = [frame.minX + i * side, frame.minY + j * side];
    const core: Box = { minX, minY, maxX: minX + Math.ceil((Math.min(frame.maxX, minX + side) - minX) / cell) * cell, maxY: minY + Math.ceil((Math.min(frame.maxY, minY + side) - minY) / cell) * cell };
    const box: Box = { minX: core.minX - margin, minY: core.minY - margin, maxX: core.maxX + margin, maxY: core.maxY + margin };
    if (only !== undefined && (box.maxX < only.minX || box.minX > only.maxX || box.maxY < only.minY || box.minY > only.maxY)) continue;
    const k = j * nx + i;
    if (refs[k]!.length === 0 && evs[k]!.length === 0) continue;
    out.push({ core, box, reference: refs[k]!, evaluated: evs[k]! });
  }
  return out;
}

/** 🔬️ The guard's measure of one tile and its two masks (drawn, volume) on the tile's raster. */
export function tileMasks(sc: Scene, t: { core: Box; box: Box; reference: P[][][]; evaluated: any[] }) {
  const m: any = measureView(sc.asset, sc.state, sc.solids as never, sc.view as never, sc.scale, undefined, sc.feature, Infinity, false, { core: t.core, box: t.box, cell: sc.cell, prepared: { reference: t.reference, visible: t.reference, full: t.reference, evaluated: t.evaluated } } as never);
  const drawn = new Mask(t.box, sc.cell), vol = new Mask(t.box, sc.cell);
  for (const g of m.filled) drawn.fill(g);
  for (const g of m.outlines) drawn.fill(g);
  for (const g of m.reference) vol.fill(g, true);
  vol.seal();
  return { m, drawn: drawn.cells, vol: vol.cells, width: vol.width, height: vol.height };
}

export type Offence = { x: number; y: number; d: number; side: "drawn" | "volume"; inDrawn: boolean; inVolume: boolean };

/** 🚨️ The boundary cells of the core of a tile that lie further than `limit` from the other boundary, as the guard counts them (small holes filled first). */
export function offences(sc: Scene, t: { core: Box; box: Box }, drawn: Uint8Array, vol: Uint8Array, width: number, height: number, limit: number): Offence[] {
  const { cell, feature } = sc;
  const holes = Math.max(1, Math.round((4 * feature / cell) ** 2));
  const [fd, fv] = [fillSmallHoles(drawn, width, height, holes), fillSmallHoles(vol, width, height, holes)];
  const [bd, bv] = [boundaryOf(fd, width, height), boundaryOf(fv, width, height)];
  const core = { i0: Math.round((t.core.minX - t.box.minX) / cell), i1: Math.round((t.core.maxX - t.box.minX) / cell), j0: Math.round((t.core.minY - t.box.minY) / cell), j1: Math.round((t.core.maxY - t.box.minY) / cell) };
  const out: Offence[] = [];
  for (const [from, to, side] of [[bd, bv, "drawn"], [bv, bd, "volume"]] as const) {
    const any = to.some((x) => x === 1);
    const field = any ? distanceField(to, width, height) : undefined;
    for (let j = core.j0; j < core.j1; j++) for (let i = core.i0; i < core.i1; i++) {
      const k = j * width + i;
      if (!from[k]) continue;
      const d = (field === undefined ? Math.hypot(width, height) : Math.sqrt(field[k]!)) * cell;
      if (d > limit) out.push({ x: t.box.minX + (i + 0.5) * cell, y: t.box.minY + (j + 0.5) * cell, d, side, inDrawn: fd[k] === 1, inVolume: fv[k] === 1 });
    }
  }
  return out;
}

/** 🧲️ Groups offences that lie within `reach` of each other (a hash grid of `reach` cells, flood over neighbouring cells); the worst offence of a group comes first, the group with the worst offence first. */
export function clusters(list: Offence[], reach: number): Offence[][] {
  const grid = new Map<string, Offence[]>();
  const key = (o: Offence): [number, number] => [Math.floor(o.x / reach), Math.floor(o.y / reach)];
  for (const o of list) { const [i, j] = key(o); const k = `${i},${j}`; const l = grid.get(k); if (l === undefined) grid.set(k, [o]); else l.push(o); }
  const out: Offence[][] = [];
  const seen = new Set<string>();
  for (const k of grid.keys()) {
    if (seen.has(k)) continue;
    seen.add(k);
    const stack = [k];
    const group: Offence[] = [];
    while (stack.length > 0) {
      const c = stack.pop()!;
      group.push(...grid.get(c)!);
      const [i, j] = c.split(",").map(Number) as [number, number];
      for (let dj = -1; dj <= 1; dj++) for (let di = -1; di <= 1; di++) { const n = `${i + di},${j + dj}`; if (!seen.has(n) && grid.has(n)) { seen.add(n); stack.push(n); } }
    }
    out.push(group.sort((a, b) => b.d - a.d));
  }
  return out.sort((a, b) => b[0]!.d - a[0]!.d);
}

/** 🎯️ Whether a point lies inside rings by the even-odd rule. */
export function inside(p: P, rings: readonly (readonly P[])[]): boolean {
  let c = false;
  for (const r of rings) for (let i = 0, j = r.length - 1; i < r.length; j = i++) {
    const [a, b] = [r[i]!, r[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) c = !c;
  }
  return c;
}

/** 📏️ The distance of a point to the nearest edge of rings. */
export function edgeDistance(p: P, rings: readonly (readonly P[])[], closed: boolean): number {
  let best = Infinity;
  for (const r of rings) for (let i = 0; i + (closed ? 0 : 1) < r.length; i++) {
    const [a, b] = [r[i]!, r[(i + 1) % r.length]!];
    const [dx, dy] = [b.x - a.x, b.y - a.y];
    const u = Math.max(0, Math.min(1, ((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy || 1)));
    best = Math.min(best, Math.hypot(p.x - a.x - u * dx, p.y - a.y - u * dy));
  }
  return best;
}
