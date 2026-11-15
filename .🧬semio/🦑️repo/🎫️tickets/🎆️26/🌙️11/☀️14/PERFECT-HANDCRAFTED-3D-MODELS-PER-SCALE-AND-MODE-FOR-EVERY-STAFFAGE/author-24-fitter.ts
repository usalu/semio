/** 🎯️ Incremental silhouette scorer and per-solid parameter fitter shared by `kit-fit.ts`, `kit-coarsen.ts` and `kit-diagnose.ts`.
 *
 * The scorer keeps, per view, a coverage count per raster cell of the union of body solids and the drawn truth of THAT scale in `presentation.defaultPose` (closed shapes of roles body, detail, glazing). Replacing one solid updates the counts of its own cells only, so a trial costs one tessellation plus one triangle rasterisation per view.
 * @see ./kit.md */
import { Mask, VIEWS, meshesAt, projectWorld, type SolidMesh } from "./volumes-lib.ts";
import { clone, drawn, mean, type Ctx, type Json, type P2, type View } from "./kit-lib.ts";

export type Cells = Map<View, Int32Array>;
type VS = { box: { minX: number; minY: number; maxX: number; maxY: number }; cell: number; truth: Mask; count: Int16Array; both: number; either: number; scratch: Mask; truthCount: number };

export const countsInSilhouette = (s: Json): boolean => s.slender !== true && s.texture !== true && s.role !== "texture";

export class Fitter {
  readonly views = new Map<View, VS>();
  constructor(readonly c: Ctx, cells = 900, minCells = 250) {
    const all = VIEWS.map((v) => drawn(c, v));
    const pts = all.flat().flatMap((s) => s.rings.flat());
    const long0 = Math.max(c.m.dimensions.widthM, c.m.dimensions.heightM, c.m.dimensions.depthM);
    const cell = Math.max(c.feature / 8, long0 / cells);
    void minCells;
    void pts;
    for (const v of VIEWS) {
      const shapes = drawn(c, v);
      const b = { minX: Infinity, minY: Infinity, maxX: -Infinity, maxY: -Infinity };
      for (const s of shapes) for (const r of s.rings) for (const p of r) {
        b.minX = Math.min(b.minX, p.x);
        b.maxX = Math.max(b.maxX, p.x);
        b.minY = Math.min(b.minY, p.y);
        b.maxY = Math.max(b.maxY, p.y);
      }
      if (!Number.isFinite(b.minX)) {
        b.minX = b.minY = -1;
        b.maxX = b.maxY = 1;
      }
      const pad = Math.max(b.maxX - b.minX, b.maxY - b.minY) * 0.25 + 4 * cell;
      const box = { minX: b.minX - pad, minY: b.minY - pad, maxX: b.maxX + pad, maxY: b.maxY + pad };
      const truth = new Mask(box, cell);
      for (const s of shapes) truth.fill(s.rings);
      let n = 0;
      for (let i = 0; i < truth.cells.length; i++) n += truth.cells[i]!;
      this.views.set(v, { box, cell, truth, count: new Int16Array(truth.cells.length), both: 0, either: n, scratch: new Mask(box, cell), truthCount: n });
    }
  }

  mesh(solid: Json): SolidMesh {
    return meshesAt(this.c.m, this.c.scale, [solid])[0]!;
  }

  cellsOf(mesh: SolidMesh): Cells {
    const out: Cells = new Map();
    for (const v of VIEWS) {
      const s = this.views.get(v)!;
      const stamp = new Uint8Array(s.count.length);
      const list: number[] = [];
      const n = mesh.world.length / 3;
      const xy = new Float64Array(n * 2);
      for (let i = 0; i < n; i++) {
        const p = projectWorld(v, [mesh.world[i * 3]!, mesh.world[i * 3 + 1]!, mesh.world[i * 3 + 2]!]);
        xy[i * 2] = p.x;
        xy[i * 2 + 1] = p.y;
      }
      for (let t = 0; t < mesh.indices.length; t += 3) {
        const tri: P2[] = [0, 1, 2].map((k) => ({ x: xy[mesh.indices[t + k]! * 2]!, y: xy[mesh.indices[t + k]! * 2 + 1]! }));
        s.scratch.fill([tri], (i) => {
          if (stamp[i]) return;
          stamp[i] = 1;
          list.push(i);
        });
      }
      out.set(v, Int32Array.from(list));
    }
    return out;
  }

  add(cells: Cells, sign: 1 | -1): void {
    for (const v of VIEWS) {
      const s = this.views.get(v)!;
      for (const i of cells.get(v)!) {
        const before = s.count[i]! > 0;
        s.count[i]! += sign;
        const after = s.count[i]! > 0;
        if (before === after) continue;
        const t = s.truth.cells[i] === 1;
        if (after) t ? s.both++ : s.either++;
        else t ? s.both-- : s.either--;
      }
    }
  }

  iou(v: View): number {
    const s = this.views.get(v)!;
    return s.either === 0 ? 1 : s.both / s.either;
  }

  score(): number {
    return mean(VIEWS.map((v) => this.iou(v)));
  }

  /** 🔎️ Fraction of the cells of `cells` in view `v` that lie inside the drawn truth (precision of one solid). */
  precision(cells: Cells, v: View): number {
    const s = this.views.get(v)!;
    const l = cells.get(v)!;
    if (l.length === 0) return 1;
    let inside = 0;
    for (const i of l) inside += s.truth.cells[i]!;
    return inside / l.length;
  }

  /** 🔎️ Drawn area (cells) of the shapes of one bone in view `v` that no solid covers, and the total, for the recall of a bone. */
  recallOfBone(bone: string, v: View): { missing: number; total: number } {
    const s = this.views.get(v)!;
    const m = new Mask(s.box, s.cell);
    for (const sh of drawn(this.c, v, bone)) m.fill(sh.rings);
    let missing = 0;
    let total = 0;
    for (let i = 0; i < m.cells.length; i++) if (m.cells[i]) {
      total++;
      if (s.count[i]! === 0) missing++;
    }
    return { missing, total };
  }
}

/* ------------------------------------------------------------ parametrisation */

export type Param = { mult: boolean; init: number; step: number; lo: number; hi: number };
export type Model = { params: Param[]; build: (vec: readonly number[]) => Json };
export type ModelOptions = { sections: boolean; shift: boolean; exponent: boolean; kLo: number; kHi: number; floor: number };

const mp = (lo: number, hi: number): Param => ({ mult: true, init: 1, step: 1, lo, hi });
const ap = (limit: number): Param => ({ mult: false, init: 0, step: limit / 6, lo: -limit, hi: limit });
const fl = (x: number, f: number): number => Math.max(x, f);

/** 🧮️ The free parameters of one solid and the function that rebuilds it from them: lofts per section (rx, ry multipliers, optional centre shifts and exponent), ellipsoids (radii), capsules (r0, r1, lateralScale), boxes (size), revolves (radial scale), extrudes (profile u, v and depth scales). */
export function modelOf(orig: Json, o: ModelOptions): Model {
  const { kLo, kHi, floor } = o;
  const M = () => mp(kLo, kHi);
  if (orig.kind === "loft") {
    const n = orig.sections.length;
    const per = o.sections || n <= 8;
    const params: Param[] = [];
    const cnt = per ? n : 1;
    for (let i = 0; i < 2 * cnt; i++) params.push(M());
    const shiftAt = params.length;
    if (o.shift) for (let i = 0; i < n; i++) {
      const r = Math.max(orig.sections[i].rx, orig.sections[i].ry);
      params.push(ap(Math.max(3 * floor, 0.5 * r)), ap(Math.max(3 * floor, 0.5 * r)));
    }
    const expAt = params.length;
    if (o.exponent) params.push(mp(0.6, 2.5));
    return {
      params,
      build: (k) => {
        const s = clone(orig);
        s.sections.forEach((q: Json, i: number) => {
          q.rx = fl(orig.sections[i].rx * k[per ? i : 0]!, floor);
          q.ry = fl(orig.sections[i].ry * k[per ? n + i : 1]!, floor);
          if (o.shift) {
            q.at[1] = orig.sections[i].at[1] + k[shiftAt + 2 * i]!;
            q.at[2] = orig.sections[i].at[2] + k[shiftAt + 2 * i + 1]!;
          }
          if (o.exponent) q.exponent = Math.min(12, Math.max(1.5, (orig.sections[i].exponent ?? 2) * k[expAt]!));
        });
        return s;
      },
    };
  }
  if (orig.kind === "ellipsoid") {
    const params = [M(), M(), M()];
    if (o.shift) for (let i = 0; i < 3; i++) params.push(ap(Math.max(3 * floor, 0.3 * orig.radii[i])));
    return { params, build: (k) => ({ ...clone(orig), radii: orig.radii.map((r: number, i: number) => fl(r * k[i]!, floor)), ...(o.shift ? { transform: { ...(orig.transform ?? {}), translation: [0, 1, 2].map((i) => (orig.transform?.translation?.[i] ?? 0) + k[3 + i]!) } } : {}) }) };
  }
  if (orig.kind === "capsule") {
    const params = [M(), M(), mp(0.5, 3)];
    return { params, build: (k) => ({ ...clone(orig), r0: fl(orig.r0 * k[0]!, floor), r1: fl(orig.r1 * k[1]!, floor), lateralScale: (orig.lateralScale ?? 1) * k[2]! }) };
  }
  if (orig.kind === "box") return { params: [M(), M(), M()], build: (k) => {
    const s = clone(orig);
    s.size = orig.size.map((x: number, i: number) => fl(x * k[i]!, floor));
    if (s.cornerRadius !== undefined) s.cornerRadius = Math.min(s.cornerRadius, 0.45 * Math.min(...s.size));
    return s;
  } };
  if (orig.kind === "revolve") return { params: [M()], build: (k) => ({ ...clone(orig), profile: orig.profile.map(([a, r]: number[]) => [a, r * k[0]!]) }) };
  if (orig.kind === "extrude") {
    return {
      params: [M(), M(), M()],
      build: (k) => {
        const s = clone(orig);
        const pr: number[][] = orig.profile;
        const cx = mean(pr.map((q) => q[0]!));
        const cy = mean(pr.map((q) => q[1]!));
        const f = (q: number[]): number[] => [cx + (q[0]! - cx) * k[0]!, cy + (q[1]! - cy) * k[1]!];
        s.profile = pr.map(f);
        if (orig.holes) s.holes = orig.holes.map((h: number[][]) => h.map(f));
        const mid = (orig.from + orig.to) / 2;
        const h = ((orig.to - orig.from) / 2) * k[2]!;
        s.from = mid - h;
        s.to = mid + h;
        return s;
      },
    };
  }
  return { params: [], build: () => clone(orig) };
}

export type Ext = { min: number[]; max: number[] };
export type FitItem = { id: string; orig: Json; cur: Json; vec: number[]; model: Model; cells: Cells; free: boolean; ext?: Ext };

export type FitOptions = { rounds: number; steps: number[]; ridge: number; log: (m: string) => void; bounds?: { dims: [number, number, number]; tol: number } };

const extOf = (m: SolidMesh): Ext => {
  const min = [Infinity, Infinity, Infinity];
  const max = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) {
    min[k] = Math.min(min[k]!, m.world[i + k]!);
    max[k] = Math.max(max[k]!, m.world[i + k]!);
  }
  return { min, max };
};

/** 🏃️ Coordinate descent on the free items: each parameter is moved up and down by the round's step while the mean IoU of the three views (minus a ridge on the departure from the start) improves. */
export function descend(fit: Fitter, items: FitItem[], o: FitOptions): void {
  const reg = (it: FitItem, v: readonly number[]): number => {
    let r = 0;
    v.forEach((x, i) => {
      const p = it.model.params[i]!;
      r += p.mult ? Math.log(x) ** 2 : (x / (p.hi || 1)) ** 2;
    });
    return o.ridge * r;
  };
  if (o.bounds !== undefined) for (const it of items) if (countsInSilhouette(it.cur)) it.ext = extOf(fit.mesh(it.cur));
  const excess = (swap?: { it: FitItem; ext: Ext }): number => {
    if (o.bounds === undefined) return 0;
    let tot = 0;
    for (let k = 0; k < 3; k++) {
      let lo = Infinity;
      let hi = -Infinity;
      for (const it of items) {
        const e = swap !== undefined && swap.it === it ? swap.ext : it.ext;
        if (e === undefined) continue;
        lo = Math.min(lo, e.min[k]!);
        hi = Math.max(hi, e.max[k]!);
      }
      tot += Math.max(0, hi - lo - o.bounds.dims[k]! - o.bounds.tol) / o.bounds.dims[k]!;
    }
    return tot;
  };
  for (let round = 0; round < o.rounds; round++) {
    const step = o.steps[Math.min(round, o.steps.length - 1)]!;
    for (const it of items) {
      if (!it.free) continue;
      for (let i = 0; i < it.model.params.length; i++) {
        const p = it.model.params[i]!;
        for (const dir of [1, -1]) {
          for (;;) {
            const nv = [...it.vec];
            nv[i] = Math.min(p.hi, Math.max(p.lo, p.mult ? nv[i]! * (1 + dir * step) : nv[i]! + dir * step * p.step * 6));
            if (nv[i] === it.vec[i]) break;
            const before = fit.score() - reg(it, it.vec) - excess();
            let cand: Json;
            let cells: Cells;
            let ext: Ext | undefined;
            try {
              cand = it.model.build(nv);
              const mesh = fit.mesh(cand);
              ext = o.bounds === undefined ? undefined : extOf(mesh);
              cells = fit.cellsOf(mesh);
            } catch {
              break;
            }
            fit.add(it.cells, -1);
            fit.add(cells, 1);
            if (fit.score() - reg(it, nv) - excess(ext === undefined ? undefined : { it, ext }) > before + 2e-5) {
              it.ext = ext;
              it.vec = nv;
              it.cells = cells;
              it.cur = cand;
            } else {
              fit.add(cells, -1);
              fit.add(it.cells, 1);
              break;
            }
          }
        }
      }
    }
    o.log(`round ${round + 1} step ${step} IoU ${fit.score().toFixed(4)} (${VIEWS.map((v) => `${v} ${fit.iou(v).toFixed(3)}`).join(" ")})`);
  }
}
