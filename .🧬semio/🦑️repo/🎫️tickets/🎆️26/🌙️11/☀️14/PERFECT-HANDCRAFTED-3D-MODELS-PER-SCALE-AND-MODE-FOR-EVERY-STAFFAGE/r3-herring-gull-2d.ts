/** ✏️ Round 3 herring gull, ruling E1: redraws the three 2D viewpoints of every authored scale as the orthographic view of that scale's body in the default pose. The table below states by hand which solids each shape outlines, its bone, role, tone, region and its layer per view (true visibility: what is nearer to the viewer lies above); detail strokes are stated by hand. Wing pieces leave the paper plane in the spread poses and carry the `solid` hint the validator demands (elevation: the side profile swept over its lateral span in height bands; front: the same bands; plan: the plan outline as a thin sheet swept along the bone normal). Writes `r3-herring-gull-2d.json` (representations, dimensions, anchors) for `r3-herring-gull-apply.py`.
 * @see ./r3-project2d.ts
 * @see ./r3-herring-gull-apply.py */
import { writeFileSync } from "node:fs";
import { localGeometry, projectWorld, simplifyRing, area, type P2, type SolidMesh, type View } from "./volumes-lib.ts";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { Bench, smooth, type Json, type V3 } from "./r3-herring-gull-kit.ts";
import { HULL_50, footAt, topOf } from "./r3-herring-gull-body.ts";
import { REPRESENTATION_DECISIONS, UNAVAILABLE_500 } from "./r3-herring-gull-decisions.ts";

type Row = { id: string; bone: string; solids: string[]; region: string | string[]; tone: number; z: [number, number, number]; role?: string; views?: View[]; bands?: number };
type Stroke = { id: string; bone: string; role: string; tone: number; z: [number, number, number]; closed: boolean; pts: V3[]; region?: string; penMm?: number; views?: View[] };
const VIEWS: View[] = ["elevation", "front", "plan"];
const E: View[] = ["elevation"];
const HIDDEN = /(propatagium|arm-web|hand-web)$/;
const near = (n: string): boolean => n === "near";

const leg = (n: "near" | "far", level: 50 | 100): Row[] => {
  const b = near(n) ? "n" : "f";
  const zE = near(n) ? 4.5 : 1.1;
  const tone = near(n) ? 0.68 : 0.58;
  const feather = near(n) ? 0.95 : 0.85;
  const webs = level === 50 ? ["web-inner", "web-outer"] : ["web"];
  const toes = level === 50 ? ["toe-inner", "toe-outer", "toe-middle"] : ["toe-middle"];
  return [
    ...(level === 50 ? [{ id: `${n}-thigh`, bone: `thigh-${b}`, solids: [`${n}-thigh`], region: "body-plumage", tone: feather, z: [near(n) ? 2.5 : 0.9, 1.5, 0.6] } as Row] : []),
    { id: `${n}-drumstick`, bone: `shank-${b}`, solids: [`${n}-drumstick`], region: "body-plumage", tone: feather, z: [near(n) ? 2.6 : 0.95, 1.9, 0.55] },
    { id: `${n}-shank`, bone: `shank-${b}`, solids: [`${n}-shank`], region: "legs-skin", tone, z: [near(n) ? 2.95 : zE, 2, 0.5] },
    { id: `${n}-tarsus`, bone: `tarsus-${b}`, solids: [`${n}-tarsus`], region: "legs-skin", tone, z: [zE + 0.01, 2.1, 0.4] },
    { id: `${n}-foot`, bone: `foot-${b}`, solids: webs.map((q) => `${n}-${q}`), region: "web-feet", tone: tone + 0.06, z: [zE + 0.02, 2.2, 0.3] },
    ...toes.map((q, i): Row => ({ id: `${n}-${q}`, bone: `foot-${b}`, solids: [`${n}-${q}`], region: "web-feet", tone, z: [zE + 0.03 + i / 1000, 2.3 + i / 1000, 0.32 + i / 1000] })),
  ];
};
const wing = (n: "near" | "far", level: 50 | 100 | 200): Row[] => {
  const b = near(n) ? "n" : "f";
  const k = near(n) ? 0 : -3.2;
  const d = near(n) ? 0 : -0.06;
  return [
    { id: `${n}-primaries`, bone: `hand-${b}`, solids: [`${n}-primaries`], region: "wing-plumage", tone: 0.1, z: [4.6 + k, 1, 3.4], bands: 1 },
    ...(level === 200 ? [] : [{ id: `${n}-secondaries`, bone: `forearm-${b}`, solids: [`${n}-secondaries`], region: "wing-plumage", tone: 0.64 + d, z: [4.7 + k, 1.2, 3.5], bands: 2 } as Row]),
    { id: `${n}-coverts`, bone: `humerus-${b}`, solids: level === 50 ? [`${n}-coverts`, `${n}-tertials`] : [`${n}-coverts`], region: "wing-plumage", tone: 0.64 + d, z: [4.8 + k, 1.3, 3.6], bands: 2 },
  ];
};
const head = (level: 50 | 100 | 200): Row[] => [
  { id: "bill", bone: "head", solids: ["bill"], region: "bill", tone: 0.8, z: [3.9, 4.1, 3.95] },
  { id: "head", bone: "head", solids: ["head"], region: "head-plumage", tone: 0.95, z: [4, 4, 4] },
  ...(level === 50
    ? [
        { id: "eye", bone: "head", solids: ["eye"], region: "head-plumage", tone: 0.5, z: [5, 4.3, 4.3], role: "detail" } as Row,
        { id: "eye-f", bone: "head", solids: ["eye-f"], region: "head-plumage", tone: 0.5, z: [5, 4.3, 4.3], role: "detail", views: ["front", "plan"] } as Row,
        { id: "bill-spot", bone: "head", solids: ["bill-spot"], region: "bill", tone: 0.32, z: [5.1, 4.35, 4.0], role: "detail", views: ["elevation", "front"] } as Row,
      ]
    : []),
];
const ROWS: Record<number, Row[]> = {
  50: [
    ...leg("far", 50), ...wing("far", 50),
    { id: "tail", bone: "tail", solids: ["tail"], region: "tail-plumage", tone: 0.95, z: [2, 0.5, 1] },
    { id: "body", bone: "trunk", solids: ["body"], region: "body-plumage", tone: 0.95, z: [3, 3, 3] },
    { id: "mantle", bone: "trunk", solids: ["mantle"], region: "body-plumage", tone: 0.66, z: [3.05, 3.05, 3.05] },
    { id: "neck-1", bone: "neck-1", solids: ["neck-1"], region: "neck-plumage", tone: 0.95, z: [2.85, 2.85, 2.85] },
    { id: "neck-2", bone: "neck-2", solids: ["neck-2"], region: "neck-plumage", tone: 0.95, z: [2.8, 2.8, 3.8] },
    ...head(50), ...wing("near", 50),
    { id: "near-skirt", bone: "forearm-n", solids: ["near-skirt"], region: "wing-plumage", tone: 0.9, z: [4.75, 1.25, 3.55], role: "detail", views: E },
    { id: "tertials", bone: "humerus-n", solids: ["near-tertials"], region: "wing-plumage", tone: 0.86, z: [4.85, 1.35, 3.65], role: "detail", views: E },
    { id: "near-mirror", bone: "hand-n", solids: ["near-mirror"], region: "wing-plumage", tone: 0.92, z: [4.65, 1.05, 3.45], role: "detail", views: ["elevation", "plan"] },
    { id: "far-mirror", bone: "hand-f", solids: ["far-mirror"], region: "wing-plumage", tone: 0.92, z: [1.45, 1.05, 3.45], role: "detail", views: ["plan"] },
    ...leg("near", 50),
  ],
  100: [
    ...leg("far", 100), ...wing("far", 100),
    { id: "tail", bone: "tail", solids: ["tail"], region: "tail-plumage", tone: 0.95, z: [2, 0.5, 1] },
    { id: "body", bone: "trunk", solids: ["body"], region: "body-plumage", tone: 0.95, z: [3, 3, 3] },
    { id: "mantle", bone: "trunk", solids: ["mantle"], region: "body-plumage", tone: 0.66, z: [3.05, 3.05, 3.05] },
    { id: "neck-1", bone: "neck-1", solids: ["neck-1"], region: "neck-plumage", tone: 0.95, z: [2.85, 2.85, 2.85] },
    { id: "neck-2", bone: "neck-2", solids: ["neck-2"], region: "neck-plumage", tone: 0.95, z: [2.8, 2.8, 3.8] },
    ...head(100), ...wing("near", 100), ...leg("near", 100),
  ],
  200: [
    ...wing("far", 200),
    { id: "tail", bone: "tail", solids: ["tail"], region: "tail-plumage", tone: 0.95, z: [2, 0.5, 1] },
    { id: "body", bone: "trunk", solids: ["body"], region: "body-plumage", tone: 0.95, z: [3, 3, 3] },
    { id: "neck-1", bone: "neck-1", solids: ["neck-1"], region: "neck-plumage", tone: 0.95, z: [2.85, 2.85, 2.85] },
    { id: "neck-2", bone: "neck-2", solids: ["neck-2"], region: "neck-plumage", tone: 0.95, z: [2.8, 2.8, 3.8] },
    ...head(200), ...wing("near", 200),
  ],
};

function strokes(S: Bench, scale: number): Stroke[] {
  if (scale === 200) {
    const out: Stroke[] = [];
    for (const [n, b, tone, zE] of [["far", "f", 0.58, 1.1], ["near", "n", 0.68, 4.5]] as const) {
      const [knee, ankle, mtp] = [S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`foot-${b}`)];
      const exit: V3 = [(knee[0] + ankle[0]) / 2, (knee[1] + ankle[1]) / 2, knee[2]];
      out.push({ id: `${n}-shank`, bone: `shank-${b}`, role: "body", tone, z: [n === "near" ? 2.95 : zE, 2, 0.5], closed: false, pts: [exit, ankle], region: "legs-skin", penMm: 0.15, views: VIEWS });
      out.push({ id: `${n}-tarsus`, bone: `tarsus-${b}`, role: "body", tone, z: [zE, 2.1, 0.4], closed: false, pts: [ankle, mtp], region: "legs-skin", penMm: 0.15, views: VIEWS });
      out.push({ id: `${n}-foot`, bone: `foot-${b}`, role: "body", tone, z: [zE + 0.01, 2.2, 0.3], closed: false, pts: [[mtp[0] - 0.002, mtp[1] - 0.0015, mtp[2]], [mtp[0] + 0.056, mtp[1] - 0.0035, mtp[2]]], region: "web-feet", penMm: 0.15, views: VIEWS });
    }
    return out;
  }
  if (scale !== 50) return [];
  const w = (s: number, t: number): V3 => S.T(s, t, -0.07);
  return [
    { id: "primary-lines", bone: "hand-n", role: "detail", tone: 0.5, z: [5.5, 0, 0], closed: false, pts: [w(-0.06, 0.004), w(-0.15, 0.03), w(-0.24, 0.044)], region: "wing-plumage" },
    { id: "covert-line", bone: "humerus-n", role: "detail", tone: 0.4, z: [5.5, 0, 0], closed: false, pts: [w(0.09, 0.006), w(0.03, 0.0), w(-0.03, 0.006)], region: "wing-plumage" },
    { id: "toe-line", bone: "foot-n", role: "detail", tone: 0.3, z: [5.6, 0, 0], closed: false, pts: [footAt(S, -1, 0.004, -0.0004), footAt(S, -1, 0.03, -0.0024), footAt(S, -1, 0.054, -0.0038)], region: "web-feet" },
    { id: "gape-line", bone: "head", role: "detail", tone: 0.3, z: [5.6, 0, 0], closed: false, pts: [S.J("head", 0.062, -0.006, -0.02), S.J("head", 0.09, -0.0052, -0.02), S.J("head", 0.118, -0.0075, -0.02)], region: "bill" },
  ];
}

const r5 = (x: number): number => Math.round(x * 1e5) / 1e5;
const f0 = (scale: number): number => 0.00015 * scale;

/** 🤏️ The ring moved inwards by `d` (mitred at each corner): the traced outline lies up to one raster cell outside the true edge, and a drawing must never be wider than its body. */
function inset(ring: readonly P2[], d: number): P2[] {
  const n = ring.length;
  const sign = area(ring as P2[]) > 0 ? 1 : -1;
  return ring.map((p, i) => {
    const [a, b] = [ring[(i + n - 1) % n]!, ring[(i + 1) % n]!];
    const [e1x, e1y] = [p.x - a.x, p.y - a.y];
    const [e2x, e2y] = [b.x - p.x, b.y - p.y];
    const [l1, l2] = [Math.hypot(e1x, e1y) || 1, Math.hypot(e2x, e2y) || 1];
    const [n1x, n1y, n2x, n2y] = [(-e1y / l1) * sign, (e1x / l1) * sign, (-e2y / l2) * sign, (e2x / l2) * sign];
    const [mx, my] = [n1x + n2x, n1y + n2y];
    const k = Math.min(3, 2 / Math.max(0.2, mx * mx + my * my));
    return { x: p.x + mx * d * k, y: p.y + my * d * k } as P2;
  });
}
const out: Json = { representations: {}, anchors: {} };
let worst = 0;
for (const scale of [50, 100, 200]) {
  const S = new Bench(scale);
  const c = S.c;
  const asset: Json = JSON.parse(JSON.stringify(c.m));
  const rep = asset.representations[String(scale)];
  const meshes: SolidMesh[] = (c.vol.body as Json[]).map((solid) => {
    const fr = S.frame(solid.bone);
    const g = localGeometry(solid, { chordM: f0(scale) / 30, minSegments: 24, maxSegments: 96 });
    const world = new Float64Array(g.positions.length);
    for (let i = 0; i < g.positions.length; i += 3) for (let k = 0; k < 3; k++) world[i + k] = (fr.o as V3)[k]! + g.positions[i]! * (fr.u as V3)[k]! + g.positions[i + 1]! * (fr.v as V3)[k]! + g.positions[i + 2]! * (fr.w as V3)[k]!;
    return { id: solid.id, role: solid.role, world, indices: g.indices } as SolidMesh;
  });
  const f = c.feature;
  const lines = strokes(S, scale);
  const used = new Set(ROWS[scale]!.flatMap((r) => r.solids));
  const missing = (c.vol.body as Json[]).filter((s) => !used.has(s.id) && s.slender !== true && s.texture !== true && !HIDDEN.test(s.id)).map((s) => s.id);
  if (missing.length > 0) throw new Error(`1:${scale}: body solids without a drawing: ${missing.join(", ")}`);
  const banded: { row: string; shape: Json }[] = [];
  VIEWS.forEach((view, vi) => {
    const jobs: { meta: Json; target: P2[]; closed: boolean; span?: [number, number, number, number] }[] = [];
    const swept = (bone: string): boolean => /^(humerus|forearm|hand)-/.test(bone);
    const spanOf = (bone: string, solids: readonly string[], y0 = -Infinity, y1 = Infinity): [number, number, number, number] => {
      const fr = S.frame(bone);
      const [o, u] = [fr.o as V3, fr.u as V3];
      const len = (c.m.rig.bones as Json[]).find((q) => q.id === bone)!.lengthM as number;
      const r = [Infinity, -Infinity, Infinity, -Infinity];
      const all = [Infinity, -Infinity];
      for (const m of meshes.filter((q) => solids.includes(q.id))) for (let i = 0; i < m.world.length; i += 3) {
        const along = (m.world[i]! - o[0]) * u[0] + (m.world[i + 1]! - o[1]) * u[1];
        const z = m.world[i + 2]!;
        if (m.world[i + 1]! < y0 || m.world[i + 1]! > y1) continue;
        [all[0], all[1]] = [Math.min(all[0]!, z), Math.max(all[1]!, z)];
        if (along <= 0.2 * len) [r[0], r[1]] = [Math.min(r[0]!, z), Math.max(r[1]!, z)];
        if (along >= 0.8 * len) [r[2], r[3]] = [Math.min(r[2]!, z), Math.max(r[3]!, z)];
      }
      if (!Number.isFinite(all[0]!)) return spanOf(bone, solids);
      if (!Number.isFinite(r[0]!)) [r[0], r[1]] = all as [number, number];
      if (!Number.isFinite(r[2]!)) [r[2], r[3]] = all as [number, number];
      for (const k of [0, 2]) if (r[k + 1]! - r[k]! < 0.002) [r[k], r[k + 1]] = [(r[k]! + r[k + 1]!) / 2 - 0.001, (r[k]! + r[k + 1]!) / 2 + 0.001];
      return r.map(r5) as [number, number, number, number];
    };
    const copies: Json[] = [];
    for (const row of ROWS[scale]!) {
      if (row.views !== undefined && !row.views.includes(view)) continue;
      if (swept(row.bone) && view === "plan") {
        const list = meshes.filter((m) => row.solids.includes(m.id));
        const mask = projectMask("plan", list, f / 40, 4)!;
        const ring = inset(simplifyRing(contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r) => area(r) > 0).sort((p, q) => area(q) - area(p))[0]!, f / 10), f / 30);
        const fr = S.frame(row.bone);
        const [o, u, v] = [fr.o as V3, fr.u as V3, fr.v as V3];
        let [lo, hi] = [Infinity, -Infinity];
        for (const m of list) for (let i = 0; i < m.world.length; i += 3) {
          const t = (m.world[i]! - o[0]) * v[0] + (m.world[i + 1]! - o[1]) * v[1];
          [lo, hi] = [Math.min(lo, t), Math.max(hi, t)];
        }
        const cv = (lo + hi) / 2;
        const [ex, ez] = [projectWorld("plan", [1, 0, 0]), projectWorld("plan", [0, 0, 1])];
        if (Math.abs(ex.x - 1) > 1e-9 || Math.abs(Math.abs(ez.y) - 1) > 1e-9) throw new Error("plan frame is not (x, z)");
        copies.push({ id: row.id, bone: row.bone, role: row.role ?? "body", z: row.z[vi], filled: true, tone: row.tone, region: row.region, kind: "poly", points: ring.map((q) => [r5((q.x - o[0] - cv * v[0]) / u[0]), r5(q.y / ez.y - o[2])]), closed: true, solid: { kind: "extruded", sweep: "v", z0: r5(cv - 0.001), z1: r5(cv + 0.001) } });
        continue;
      }
      if (swept(row.bone) && view === "front") {
        copies.push(...banded.filter((q) => q.row === row.id).map((q) => ({ ...q.shape, z: row.z[vi] })));
        continue;
      }
      const list = meshes.filter((m) => row.solids.includes(m.id));
      if (list.length !== row.solids.length) throw new Error(`1:${scale} ${row.id}: solids ${row.solids.join(", ")} not all in the body`);
      const mask = projectMask(view, list, f / 40, 4);
      if (mask === null) throw new Error(`nothing projected for ${row.id}`);
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r) => area(r) > 0).sort((a, b) => area(b) - area(a));
      const outer = rings[0]!;
      const xs = outer.map((p) => p.x);
      const ys = outer.map((p) => p.y);
      const thin = Math.min(Math.max(...xs) - Math.min(...xs), Math.max(...ys) - Math.min(...ys));
      const target = inset(simplifyRing(outer, Math.max(f / 60, Math.min(f / 10, thin / 14))), f / 30);
      jobs.push({ meta: { id: row.id, bone: row.bone, role: row.role ?? "body", z: row.z[vi], filled: true, tone: row.tone, ...(Array.isArray(row.region) ? { regions: row.region } : { region: row.region }) }, target, closed: true, ...(swept(row.bone) ? { span: spanOf(row.bone, row.solids) } : {}) });
    }
    for (const s of lines) {
      if (!(s.views ?? E).includes(view)) continue;
      jobs.push({ meta: { id: s.id, bone: s.bone, role: s.role, z: s.z[vi], filled: false, tone: s.tone, ...(s.region === undefined ? {} : { region: s.region }), ...(s.penMm === undefined ? {} : { penMm: s.penMm }) }, target: s.pts.map((p) => projectWorld(view, p)), closed: s.closed, ...(swept(s.bone) ? { span: spanOf(s.bone, [`near-${s.bone.startsWith("hand") ? "primaries" : "coverts"}`]) } : {}) });
    }
    if (view === "elevation") {
      const bandJobs: { row: string; meta: Json; target: P2[]; span: [number, number, number, number] }[] = [];
      for (const row of ROWS[scale]!.filter((q) => swept(q.bone) && (q.views === undefined || q.views.includes("front")))) {
        const list = meshes.filter((m) => row.solids.includes(m.id));
        const whole = projectMask("elevation", list, f / 40, 4)!;
        const rowsY = Array.from({ length: whole.height }, (_, y) => y).filter((y) => whole.cells.subarray(y * whole.width, (y + 1) * whole.width).some((q) => q === 1));
        const [lo, hi] = [rowsY[0]!, rowsY[rowsY.length - 1]! + 1];
        const nb = row.bands ?? 1;
        for (let k = 0; k < nb; k++) {
          const [a, b] = [Math.round(lo + ((hi - lo) * k) / nb), Math.round(lo + ((hi - lo) * (k + 1)) / nb)];
          const cells = new Uint8Array(whole.cells.length);
          cells.set(whole.cells.subarray(a * whole.width, b * whole.width), a * whole.width);
          const ring = contours(cells, whole.width, whole.height, whole.box, whole.cell).filter((r) => area(r) > 0).sort((p, q) => area(q) - area(p))[0]!;
          const [y0, y1] = [whole.box.minY + a * whole.cell, whole.box.minY + b * whole.cell];
          bandJobs.push({ row: row.id, meta: { id: k === 0 ? row.id : `${row.id}-${k + 1}`, bone: row.bone, role: "body", z: 0, filled: true, tone: row.tone, region: row.region }, target: inset(simplifyRing(ring, f / 10), f / 30), span: spanOf(row.bone, row.solids, y0, y1) });
        }
      }
      inverse(asset, scale, "elevation", bandJobs).forEach((q, k) => banded.push({ row: bandJobs[k]!.row, shape: { ...bandJobs[k]!.meta, kind: "poly", points: q.points.map((p) => [r5(p.x), r5(p.y)]), closed: true, solid: { kind: "extruded", sweep: "w", z0: bandJobs[k]!.span[0], z1: bandJobs[k]!.span[1], zTip: [bandJobs[k]!.span[2], bandJobs[k]!.span[3]] } } }));
    }
    const solved = inverse(asset, scale, view, jobs);
    const shapes = jobs.map((j, k) => {
      worst = Math.max(worst, solved[k]!.residualM);
      if (solved[k]!.residualM > 2e-5) console.log(`residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm at 1:${scale} ${view} ${j.meta.id}${solved[k]!.edgeOn ? " (edge-on)" : ""}`);
      return { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r5(p.x), r5(p.y)]), closed: j.closed, ...(j.span === undefined ? {} : { solid: { kind: "extruded", sweep: "w", z0: j.span[0], z1: j.span[1], zTip: [j.span[2], j.span[3]] } }) };
    });
    shapes.push(...(copies as typeof shapes));
    shapes.sort((a, b) => a.z - b.z);
    rep.geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  });
  if (scale === 50) rep.pens = { ...rep.pens, detail: 0.13 };
  rep.decisions = { ...rep.decisions, ...REPRESENTATION_DECISIONS[scale] };
  out.representations[String(scale)] = rep;
  if (scale === 50) {
    let [x0, x1, y1, z0, z1] = [Infinity, -Infinity, -Infinity, Infinity, -Infinity];
    for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) {
      x0 = Math.min(x0, m.world[i]!); x1 = Math.max(x1, m.world[i]!); y1 = Math.max(y1, m.world[i + 1]!); z0 = Math.min(z0, m.world[i + 2]!); z1 = Math.max(z1, m.world[i + 2]!);
    }
    const mm = (v: number): number => Math.round(v * 1000) / 1000;
    out.dimensions = { ...c.m.dimensions, heightM: mm(y1), widthM: mm(x1 - x0), depthM: mm(z1 - z0) };
    S.hull = smooth(HULL_50, 3);
    const plume = S.local("trunk", S.T(0.045, topOf(S, 0.045) + 0.001));
    out.anchors = { "back-plume": { x: plume[0], y: plume[1] } };
    console.log("dimensions", JSON.stringify(out.dimensions), "back-plume", plume.join(" "));
  }
}
out.representations["500"] = { ...new Bench(50).c.m.representations["500"], ...UNAVAILABLE_500 };
writeFileSync(new URL("./r3-herring-gull-2d.json", import.meta.url), JSON.stringify(out));
console.log(`worst residual ${(worst * 1000).toFixed(4)} mm`);
