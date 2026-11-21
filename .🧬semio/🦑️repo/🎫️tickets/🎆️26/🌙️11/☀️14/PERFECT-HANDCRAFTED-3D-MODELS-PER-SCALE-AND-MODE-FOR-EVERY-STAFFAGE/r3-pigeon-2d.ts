/** ✏️ Round 3 pigeon, ruling E1: redraws the three 2D viewpoints of every authored scale as the orthographic view of that scale's body in the default pose. The table below states by hand which solids each shape outlines, its bone, role, tone, region and its layer per view (true visibility: what is nearer to the viewer lies above); detail and texture strokes are stated by hand in world metres. Writes `r3-pigeon-2d.json` (representations, dimensions, anchors) for `r3-pigeon-apply.py`.
 * @see ./r3-project2d.ts
 * @see ./r3-pigeon-apply.py */
import { writeFileSync } from "node:fs";
import { localGeometry, projectWorld, simplifyRing, area, type P2, type SolidMesh, type View } from "./volumes-lib.ts";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { Bench, type Json, type V3 } from "./r3-pigeon-kit.ts";
import { REPRESENTATION_DECISIONS } from "./r3-pigeon-decisions.ts";

type Row = { id: string; bone: string; solids: string[]; region: string | string[]; tone: number; z: [number, number, number]; role?: string; views?: View[]; bands?: number };
type Stroke = { id: string; bone: string; role: string; tone: number; z: [number, number, number]; closed: boolean; pts: V3[]; region?: string; penMm?: number; views?: View[] };
const VIEWS: View[] = ["elevation", "front", "plan"];
const E: View[] = ["elevation"];

const leg = (n: "near" | "far", toes: string[]): Row[] => {
  const b = n === "near" ? "n" : "f";
  const zE = n === "near" ? 5 : 1;
  return [
    { id: `${n}-thigh`, bone: `thigh-${b}`, solids: [`${n}-thigh`], region: "plumage-body", tone: n === "near" ? 0.72 : 0.55, z: [n === "near" ? 2.9 : 0.9, 1.5, 0.6] },
    { id: `${n}-drumstick`, bone: `shank-${b}`, solids: [`${n}-drumstick`], region: "plumage-body", tone: n === "near" ? 0.72 : 0.55, z: [n === "near" ? 2.95 : 0.95, 1.9, 0.55] },
    { id: `${n}-shank`, bone: `shank-${b}`, solids: [`${n}-shank`], region: "legs-feet", tone: n === "near" ? 0.72 : 0.55, z: [zE, 2, 0.5] },
    { id: `${n}-tarsus`, bone: `tarsus-${b}`, solids: [`${n}-tarsus`], region: "legs-feet", tone: n === "near" ? 0.72 : 0.55, z: [zE + 0.01, 2.1, 0.4] },
    ...toes.map((t, i): Row => ({ id: `${n}-${t}`, bone: `toes-${b}`, solids: [`${n}-${t}`], region: "legs-feet", tone: n === "near" ? 0.72 : 0.55, z: [zE + 0.02 + i / 1000, 2.2 + i / 1000, 0.3 + i / 1000] })),
  ];
};
const wing = (n: "near" | "far"): Row[] => {
  const b = n === "near" ? "n" : "f";
  const k = n === "near" ? 0 : -3.1;
  return [
    { id: `${n}-primaries`, bone: `hand-${b}`, solids: [`${n}-primaries`], region: "plumage-wings", tone: n === "near" ? 0.55 : 0.45, z: [4.6 + k, 1, 3.4], bands: 2 },
    { id: `${n}-coverts`, bone: `forearm-${b}`, solids: [`${n}-coverts`], region: "plumage-wings", tone: n === "near" ? 0.6 : 0.5, z: [4.7 + k, 1.2, 3.5], bands: 6 },
    { id: `${n}-shoulder`, bone: `humerus-${b}`, solids: [`${n}-shoulder`], region: "plumage-wings", tone: n === "near" ? 0.6 : 0.5, z: [4.8 + k, 1.3, 3.6], bands: 3 },
  ];
};
const wing200 = (n: "near" | "far"): Row => {
  const b = n === "near" ? "n" : "f";
  return { id: `${n}-wing`, bone: `forearm-${b}`, solids: [`${n}-coverts`, `${n}-shoulder`, `${n}-primaries`], region: "plumage-wings", tone: n === "near" ? 0.6 : 0.5, z: [n === "near" ? 4.7 : 1.6, 1.2, 3.5], bands: 6, ...(n === "far" ? { views: ["front", "plan"] as View[] } : {}) };
};
/** 🔵️ 1:200 head seen from ahead and from above: the half widths of the 1:200 head loft (with the bill) along the head axis, revolved about the head bone, so the front and the plan show the round head in every pitch of the head (pecking from ahead shows its length, not a foreshortened sliver). */
const HEAD_200_RADII: readonly [number, number][] = [[-0.0205, 0.0074], [-0.015, 0.0136], [-0.006, 0.0163], [0.006, 0.0164], [0.018, 0.0154], [0.027, 0.0128], [0.033, 0.008], [0.041, 0.0048], [0.05, 0.0032], [0.0545, 0.0014]];
const RANGES: Record<number, { minDenominator: number; maxDenominator: number }> = { 50: { minDenominator: 20, maxDenominator: 55 }, 100: { minDenominator: 55, maxDenominator: 150 }, 200: { minDenominator: 150, maxDenominator: 350 } };
const tails: Row[] = [
  { id: "far-tail", bone: "tail-f", solids: ["far-tail"], region: "plumage-tail", tone: 0.55, z: [1, 0.5, 0.8] },
  { id: "tail", bone: "tail", solids: ["tail"], region: "plumage-tail", tone: 0.66, z: [2.8, 0.6, 1] },
];
const necks: Row[] = [1, 2, 3].map((i): Row => ({ id: `neck-${i}`, bone: `neck-${i}`, solids: [`neck-${i}`], region: "plumage-neck-head", tone: 0.72, z: [3.3 + i / 100, 3.3 + i / 100, 3.7 + i / 100] }));
const face: Row[] = [
  { id: "head", bone: "head", solids: ["head"], region: "plumage-neck-head", tone: 0.72, z: [4, 4, 4] },
  { id: "jaw", bone: "jaw", solids: ["jaw"], region: "beak", tone: 0.3, z: [3.9, 4.05, 3.95] },
  { id: "beak", bone: "head", solids: ["beak"], region: "beak", tone: 0.3, z: [4.1, 4.1, 4.1] },
  { id: "cere", bone: "head", solids: ["cere"], region: "cere", tone: 0.9, z: [4.2, 4.2, 4.2] },
  { id: "eye", bone: "head", solids: ["eye"], region: "eye", tone: 0.04, z: [5, 4.3, 4.3], role: "detail" },
  { id: "eye-f", bone: "head", solids: ["eye-f"], region: "eye", tone: 0.04, z: [5, 4.3, 4.3], role: "detail", views: ["front"] },
];
const ROWS: Record<number, Row[]> = {
  50: [...leg("far", ["toes", "toe-inner", "toe-outer", "hallux"]), ...wing("far"), ...tails, { id: "body", bone: "trunk", solids: ["body", "undertail"], region: "plumage-body", tone: 0.72, z: [3, 3, 3] }, ...necks, ...face, ...wing("near"), ...leg("near", ["toes", "toe-inner", "toe-outer", "hallux"])],
  100: [...leg("far", ["toes", "toe-spread"]), ...wing("far"), ...tails, { id: "body", bone: "trunk", solids: ["body"], region: "plumage-body", tone: 0.72, z: [3, 3, 3] }, ...necks, { id: "head", bone: "head", solids: ["head", "eye", "eye-f"], region: ["plumage-neck-head", "eye"], tone: 0.72, z: [4, 4, 4] }, ...face.filter((r) => r.id !== "head" && !r.id.startsWith("eye")), ...wing("near"), ...leg("near", ["toes", "toe-spread"])],
  200: [
    wing200("far"),
    ...tails,
    { id: "body", bone: "trunk", solids: ["body"], region: "plumage-body", tone: 0.72, z: [3, 3, 3] },
    { id: "neck", bone: "neck-2", solids: ["neck"], region: "plumage-neck-head", tone: 0.72, z: [3.3, 3.3, 3.7] },
    { id: "head", bone: "head", solids: ["head"], region: ["plumage-neck-head", "beak", "cere", "eye"], tone: 0.72, z: [4, 4, 4] },
    wing200("near"),
  ],
};

function strokes(S: Bench, scale: number): Stroke[] {
  if (scale === 200) {
    const out: Stroke[] = [];
    for (const [n, b, tone, zE] of [["far", "f", 0.55, 1], ["near", "n", 0.72, 5]] as const) {
      const [knee, ankle, mtp] = [S.J(`shank-${b}`), S.J(`tarsus-${b}`), S.J(`toes-${b}`)];
      out.push({ id: `${n}-leg`, bone: `shank-${b}`, role: "body", tone, z: [zE, 2, 0.5], closed: false, pts: [[knee[0], knee[1] - 0.012, knee[2]], ankle], region: "legs-feet", penMm: 0.15, views: VIEWS });
      out.push({ id: `${n}-foot`, bone: `tarsus-${b}`, role: "body", tone, z: [zE, 2.1, 0.4], closed: false, pts: [ankle, [mtp[0], 0.002, mtp[2]], [mtp[0] + 0.034, 0.0015, mtp[2]]], region: "legs-feet", penMm: 0.15, views: VIEWS });
    }
    return out;
  }
  if (scale !== 50) return [];
  const eye = S.J("head", 0.022, 0.006, -0.0128);
  const ring: V3[] = Array.from({ length: 16 }, (_, i): V3 => [eye[0] + 0.0058 * Math.cos((i * Math.PI) / 8), eye[1] + 0.0058 * Math.sin((i * Math.PI) / 8), eye[2]]);
  const wrist = S.J("hand-n");
  const d = S.frame("hand-n").u as V3;
  const quill = (q: number, c: number): V3 => [wrist[0] + d[0] * q + d[1] * c, wrist[1] + d[1] * q - d[0] * c, -0.04];
  const tail = (q: number, t: number): V3 => S.J("tail", q, t);
  return [
    { id: "eye-ring", bone: "head", role: "detail", tone: 0.2, z: [5.1, 0, 0], closed: true, pts: ring },
    { id: "neck-sheen", bone: "neck-2", role: "detail", tone: 0.2, z: [5.5, 0, 0], closed: true, pts: Array.from({ length: 14 }, (_, i): V3 => { const a = (i * Math.PI) / 7; return [0.0855 + 0.0125 * Math.cos(a) * 0.985 + 0.007 * Math.sin(a) * 0.17, 0.171 - 0.0125 * Math.cos(a) * 0.17 + 0.007 * Math.sin(a) * 0.985, -0.02]; }) },
    { id: "wing-bar", bone: "forearm-n", role: "detail", tone: 0.25, z: [5.5, 0, 0], closed: true, pts: [[-0.03, 0.0965, -0.04], [-0.0235, 0.099, -0.04], [-0.04, 0.0715, -0.04], [-0.0465, 0.0705, -0.04]] },
    { id: "primary-1", bone: "hand-n", role: "texture", tone: 0.35, z: [5.6, 0, 0], closed: false, pts: [quill(0.075, 0.003), quill(0.186, 0.0005)] },
    { id: "primary-2", bone: "hand-n", role: "texture", tone: 0.35, z: [5.6, 0, 0], closed: false, pts: [quill(0.065, -0.003), quill(0.165, -0.003)] },
    { id: "primary-3", bone: "hand-n", role: "texture", tone: 0.35, z: [5.6, 0, 0], closed: false, pts: [quill(0.055, 0.007), quill(0.135, 0.005)] },
    { id: "breast-feather-1", bone: "trunk", role: "texture", tone: 0.35, z: [5.5, 0, 0], closed: false, pts: [[0.078, 0.11, -0.03], [0.0805, 0.1035, -0.03], [0.0865, 0.1005, -0.03], [0.093, 0.103, -0.03], [0.096, 0.109, -0.03]] },
    { id: "breast-feather-2", bone: "trunk", role: "texture", tone: 0.35, z: [5.5, 0, 0], closed: false, pts: [[0.066, 0.083, -0.03], [0.068, 0.0765, -0.03], [0.074, 0.073, -0.03], [0.0805, 0.0755, -0.03], [0.0835, 0.0815, -0.03]] },
    { id: "tail-band", bone: "tail", role: "texture", tone: 0.35, z: [5.5, 0, 0], closed: false, pts: [tail(0.084, -0.0055), tail(0.09, 0.002)] },
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
  const missing = (c.vol.body as Json[]).filter((s) => !used.has(s.id) && s.slender !== true).map((s) => s.id);
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
        copies.push({ id: row.id, bone: row.bone, role: "body", z: row.z[vi], filled: true, tone: row.tone, ...(Array.isArray(row.region) ? { regions: row.region } : { region: row.region }), kind: "poly", points: ring.map((q) => [r5((q.x - o[0] - cv * v[0]) / u[0]), r5(q.y / ez.y - o[2])]), closed: true, solid: { kind: "extruded", sweep: "v", z0: r5(scale === 200 ? lo : cv - 0.001), z1: r5(scale === 200 ? hi : cv + 0.001) } });
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
      jobs.push({ meta: { id: s.id, bone: s.bone, role: s.role, z: s.z[vi], filled: false, tone: s.tone, ...(s.region === undefined ? {} : { region: s.region }), ...(s.penMm === undefined ? {} : { penMm: s.penMm }) }, target: s.pts.map((p) => projectWorld(view, p)), closed: s.closed, ...(swept(s.bone) ? { span: spanOf(s.bone, [s.bone.startsWith("hand") ? "near-primaries" : "near-coverts"]) } : {}) });
    }
    if (view === "elevation") {
      const bandJobs: { row: string; meta: Json; target: P2[]; span: [number, number, number, number] }[] = [];
      for (const row of ROWS[scale]!.filter((q) => swept(q.bone))) {
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
          bandJobs.push({ row: row.id, meta: { id: k === 0 ? row.id : `${row.id}-${k + 1}`, bone: row.bone, role: "body", z: 0, filled: true, tone: row.tone, ...(Array.isArray(row.region) ? { regions: row.region } : { region: row.region }) }, target: inset(simplifyRing(ring, f / 10), f / 30), span: spanOf(row.bone, row.solids, y0, y1) });
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
    if (scale === 200 && view !== "elevation") {
      const k = shapes.findIndex((q) => q.id === "head");
      shapes[k] = { ...shapes[k]!, points: [...HEAD_200_RADII.map(([a]) => [a, 0]).reverse(), ...HEAD_200_RADII], closed: true, solid: { kind: "revolved", axisBone: "head", axis: "u", axisX: 0, axisZ: 0, mirror: true } } as (typeof shapes)[number];
    }
    shapes.sort((a, b) => a.z - b.z);
    rep.geometry[view] = shapes;
    console.log(`1:${scale} ${view}: ${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  });
  rep.decisions = { ...rep.decisions, ...REPRESENTATION_DECISIONS[scale] };
  rep.validRange = RANGES[scale];
  out.representations[String(scale)] = rep;
  if (scale === 50) {
    let [x0, x1, y1, z0, z1] = [Infinity, -Infinity, -Infinity, Infinity, -Infinity];
    for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) {
      x0 = Math.min(x0, m.world[i]!); x1 = Math.max(x1, m.world[i]!); y1 = Math.max(y1, m.world[i + 1]!); z0 = Math.min(z0, m.world[i + 2]!); z1 = Math.max(z1, m.world[i + 2]!);
    }
    const mm = (v: number): number => Math.round(v * 1000) / 1000;
    out.dimensions = { ...c.m.dimensions, heightM: mm(y1), widthM: mm(x1 - x0), depthM: mm(z1 - z0) };
    const plume = S.local("trunk", S.T(0.085, 0.0113));
    out.anchors = { eye: { x: 0.022, y: 0.006 }, "back-plume": { x: plume[0], y: plume[1] } };
    console.log("dimensions", JSON.stringify(out.dimensions), "back-plume", plume.join(" "));
  }
}
out.representations["500"] = new Bench(50).c.m.representations["500"];
writeFileSync(new URL("./r3-pigeon-2d.json", import.meta.url), JSON.stringify(out));
console.log(`worst residual ${(worst * 1000).toFixed(4)} mm`);
