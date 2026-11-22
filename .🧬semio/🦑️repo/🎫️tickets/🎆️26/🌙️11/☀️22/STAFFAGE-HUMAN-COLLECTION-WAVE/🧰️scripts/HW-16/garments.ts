/** 🧥️ Garment operations shared by the HW-16 builders: inflating authored lofts (a down fill over a coat), adding new lofted garments per scale and drawing every changed garment again as the projection of its own solids in the three views of each scale.
 *
 * @see ./lib.ts
 */
import { SCALES, VIEWS, assignSolids, r5, retrace, shapesFrom, type Json } from "./lib.ts";

/** 🎈️ Grows the cross-sections of loft solids by `d` metres (half-width and half-depth), every listed solid of every scale. */
export function inflate(asset: Json, ids: (id: string, scale: string) => number): void {
  for (const sc of SCALES) {
    const v = asset.volumes[sc];
    if (v?.status !== "authored") continue;
    for (const list of ["body", "thermal"])
      for (const s of v[list]) {
        if ("of" in s || s.kind !== "loft") continue;
        const d = ids(s.id, sc);
        if (!d) continue;
        for (const sec of s.sections) (sec.rx = r5(sec.rx + d)), (sec.ry = r5(sec.ry + d));
      }
  }
}

/** ✏️ Redraws, at every scale and view, each filled shape picked by `pick` as the outline of the candidate solids it stands for (centroid assignment); shapes that stand for no candidate stay as they are. */
export function redraw(asset: Json, pick: (s: Json, scale: string, view: string) => boolean, candidates: (scale: string) => string[], tolMm = 0.03): void {
  for (const sc of SCALES) {
    const r = asset.representations[sc];
    if (r?.status !== "authored") continue;
    const ids = candidates(sc).filter((id) => asset.volumes[sc].body.some((b: Json) => b.id === id));
    for (const view of VIEWS) {
      const shapes = r.geometry[view].filter((s: Json) => s.filled && s.kind === "poly" && pick(s, sc, view));
      if (shapes.length === 0) continue;
      const map = assignSolids(asset, sc, view, shapes, ids);
      const out: Json[] = [];
      for (const s of r.geometry[view]) {
        const mine = map.get(s);
        if (!mine || mine.length === 0) out.push(s);
        else out.push(...retrace(asset, sc, view, s, mine, tolMm));
      }
      r.geometry[view] = out;
    }
  }
}

/** ➕️ Adds new body solids per scale and their drawings in the three views (one filled poly per view, bound to the solid's bone, inserted after the shape `after` or at the end). */
export function addGarment(asset: Json, spec: { scales: readonly string[]; solids: (scale: string) => Json[]; shape: { id: string; bone: string; region: string; material: string; tone: number }; z: Record<string, number>; thermal?: (scale: string, solid: Json) => Json | null; tolMm?: number }): void {
  for (const sc of spec.scales) {
    const v = asset.volumes[sc];
    const solids = spec.solids(sc);
    v.body.push(...solids);
    for (const s of solids) {
      const th = spec.thermal?.(sc, s);
      if (th) v.thermal.push(th);
    }
    const r = asset.representations[sc];
    for (const view of VIEWS) {
      const made = shapesFrom(asset, sc, view, solids.map((s) => s.id), { ...spec.shape, z: spec.z[view]! }, spec.tolMm ?? 0.03);
      const list = r.geometry[view] as Json[];
      list.push(...made);
      list.sort((a, b) => a.z - b.z);
    }
  }
}

/** 🧶️ Loft sections enclosing the union of two lofts on the same bone (sampled where both exist, by u) and grown by `d`: a knitted cap over head and hair. */
export function envelopeSections(a: Json, b: Json, from: number, to: number, n: number, d: number, exponent = 2.2): Json[] {
  const at = (loft: Json, u: number): { c: number[]; rx: number; ry: number } | null => {
    const s = loft.sections;
    for (let i = 0; i + 1 < s.length; i++) {
      const [p, q] = [s[i], s[i + 1]];
      if ((p.at[0] - u) * (q.at[0] - u) <= 0 && p.at[0] !== q.at[0]) {
        const f = (u - p.at[0]) / (q.at[0] - p.at[0]);
        return { c: [0, 1, 2].map((k) => p.at[k] + f * (q.at[k] - p.at[k])), rx: p.rx + f * (q.rx - p.rx), ry: p.ry + f * (q.ry - p.ry) };
      }
    }
    return null;
  };
  const rot = a.sections[0].rotation;
  const out: Json[] = [];
  for (let i = 0; i < n; i++) {
    const u = from + ((to - from) * i) / (n - 1);
    const ps = [at(a, u), at(b, u)].filter(Boolean) as { c: number[]; rx: number; ry: number }[];
    if (ps.length === 0) continue;
    const vmin = Math.min(...ps.map((p) => p.c[1]! - p.ry));
    const vmax = Math.max(...ps.map((p) => p.c[1]! + p.ry));
    const wmin = Math.min(...ps.map((p) => p.c[2]! - p.rx));
    const wmax = Math.max(...ps.map((p) => p.c[2]! + p.rx));
    out.push({ at: [r5(u), r5((vmin + vmax) / 2), r5((wmin + wmax) / 2)], rx: r5((wmax - wmin) / 2 + d), ry: r5((vmax - vmin) / 2 + d), exponent, rotation: rot });
  }
  return out;
}

/** 🦾️ Drawings of a limb segment that stay exact in every pose: the solids of the bone are cut into `pieces` lengths along the bone, each drawn as an extruded solid swept along the bone (`sweep: "u"`) whose (v, w) profile is the superellipse that bounds the cross-sections of that length (tessellated solids at rest, in the bone frame), so the front and plan views of a flexed forearm or a hand in a pocket are the true projections of the segment and not a carried flat outline. */
export function limbShapes(asset: Json, scale: string, bone: string, ids: readonly string[], base: { id: string; region: string; material: string; tone: number; z: number; role?: string }, pieces: number, exponent = 2.4): Json[] {
  const { worldSolids } = require("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
  const { rest, restBones } = require("./lib.ts");
  const f = rest(asset)[bone];
  const set = new Set(ids);
  const pts: number[][] = [];
  for (const w of worldSolids(asset, restBones(asset), Number(scale), "body")) {
    if (!set.has(w.id)) continue;
    for (let i = 0; i < w.positions.length; i += 3) {
      const d = [w.positions[i] - f.o[0], w.positions[i + 1] - f.o[1], w.positions[i + 2] - f.o[2]];
      pts.push([0, 1, 2].map((k) => d[0] * [f.u, f.v, f.w][k][0] + d[1] * [f.u, f.v, f.w][k][1] + d[2] * [f.u, f.v, f.w][k][2]));
    }
  }
  if (pts.length === 0) return [];
  const u0 = Math.min(...pts.map((p) => p[0]!));
  const u1 = Math.max(...pts.map((p) => p[0]!));
  const out: Json[] = [];
  for (let k = 0; k < pieces; k++) {
    const [a, b] = [u0 + ((u1 - u0) * k) / pieces, u0 + ((u1 - u0) * (k + 1)) / pieces];
    const inside = pts.filter((p) => p[0]! >= a - 1e-9 && p[0]! <= b + 1e-9);
    if (inside.length < 3) continue;
    const [v0, v1] = [Math.min(...inside.map((p) => p[1]!)), Math.max(...inside.map((p) => p[1]!))];
    const [w0, w1] = [Math.min(...inside.map((p) => p[2]!)), Math.max(...inside.map((p) => p[2]!))];
    const [cv, cw, hv, hw] = [(v0 + v1) / 2, (w0 + w1) / 2, (v1 - v0) / 2, (w1 - w0) / 2];
    const ring = Array.from({ length: 28 }, (_, i) => {
      const th = (2 * Math.PI * i) / 28;
      const [c, s] = [Math.cos(th), Math.sin(th)];
      return [r5(cv + hv * Math.sign(c) * Math.abs(c) ** (2 / exponent)), r5(cw + hw * Math.sign(s) * Math.abs(s) ** (2 / exponent))];
    });
    const overlap = k + 1 < pieces ? 0.002 : 0;
    out.push({ id: pieces === 1 ? base.id : `${base.id}-${k + 1}`, bone, role: base.role ?? "body", z: base.z, filled: true, tone: base.tone, region: base.region, material: base.material, kind: "poly", points: ring, closed: true, solid: { kind: "extruded", sweep: "u", z0: r5(a - (k > 0 ? 0.002 : 0)), z1: r5(b + overlap) } });
  }
  return out;
}

/** 🦿️ Replaces, in the listed views of every scale, the flat drawings of the given limb bones by {@link limbShapes} built from all body solids of that bone (a hand with its fingers; the first drawing of the bone lends id, region, material, tone and paint order). */
export function solidLimbs(asset: Json, bones: readonly string[], views: readonly string[], pieces: (scale: string, bone: string) => number): void {
  for (const sc of SCALES) {
    const r = asset.representations[sc];
    const vol = asset.volumes[sc];
    for (const view of views)
      for (const bone of bones) {
        const mine = r.geometry[view].filter((s: Json) => s.bone === bone && s.filled);
        if (mine.length === 0) continue;
        const first = mine.sort((p: Json, q: Json) => q.z - p.z).find((s: Json) => s.role === "body") ?? mine[0];
        const { subtree } = require("./lib.ts");
        const carried: Set<string> = bone.startsWith("hand-") ? subtree(asset, [bone]) : new Set([bone]);
        const ids = vol.body.filter((s: Json) => carried.has(s.bone)).map((s: Json) => s.id);
        const made = limbShapes(asset, sc, bone, ids, { id: first.id, region: first.region, material: first.material, tone: first.tone, z: first.z }, pieces(sc, bone));
        if (made.length === 0) continue;
        const at = r.geometry[view].findIndex((s: Json) => s.bone === bone);
        r.geometry[view] = r.geometry[view].filter((s: Json) => s.bone !== bone);
        r.geometry[view].splice(at, 0, ...made);
      }
  }
}

/** 🌀️ The drawing of a round limb segment as a body of revolution about its own centre line (`revolved`, `axis: "u"`, `mirror: true`): the profile radius at each length is the half-extent of the tessellated solids of the bone across that length that the view sees at rest (the 95th percentile of the distances from the centre of that length along v in the side elevation, along w in the front elevation, the smaller of the two in the plan) (bone frame at rest), the axis runs through their mean centre; one outline that is the true projection in every view and pose. */
export function revolvedShape(asset: Json, scale: string, bone: string, ids: readonly string[], base: { id: string; region: string; material: string; tone: number; z: number; role?: string }, samples: number, radius: "v" | "w" | "min" = "min"): Json | null {
  const { worldSolids } = require("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
  const { rest, restBones } = require("./lib.ts");
  const f = rest(asset)[bone];
  const set = new Set(ids);
  const pts: number[][] = [];
  for (const w of worldSolids(asset, restBones(asset), Number(scale), "body")) {
    if (!set.has(w.id)) continue;
    for (let i = 0; i < w.positions.length; i += 3) {
      const d = [w.positions[i] - f.o[0], w.positions[i + 1] - f.o[1], w.positions[i + 2] - f.o[2]];
      pts.push([f.u, f.v, f.w].map((ax: number[]) => d[0] * ax[0]! + d[1] * ax[1]! + d[2] * ax[2]!));
    }
  }
  if (pts.length === 0) return null;
  const u0 = Math.min(...pts.map((p) => p[0]!));
  const u1 = Math.max(...pts.map((p) => p[0]!));
  const slabs = Array.from({ length: samples }, (_, k) => {
    const [a, b] = [u0 + ((u1 - u0) * k) / samples, u0 + ((u1 - u0) * (k + 1)) / samples];
    const q = pts.filter((p) => p[0]! >= a && p[0]! <= b);
    if (q.length === 0) return null;
    const [v0, v1, w0, w1] = [Math.min(...q.map((p) => p[1]!)), Math.max(...q.map((p) => p[1]!)), Math.min(...q.map((p) => p[2]!)), Math.max(...q.map((p) => p[2]!))];
    const [cv, cw] = [(v0 + v1) / 2, (w0 + w1) / 2];
    const spread = (k: 1 | 2, c: number) => {
      const d = q.map((p) => Math.abs(p[k]! - c)).sort((x, y) => x - y);
      return d[Math.floor(0.95 * (d.length - 1))]!;
    };
    const [hv, hw] = [spread(1, cv), spread(2, cw)];
    return { a, b, cv, cw, r: radius === "v" ? hv : radius === "w" ? hw : Math.min(hv, hw) };
  }).filter(Boolean) as { a: number; b: number; cv: number; cw: number; r: number }[];
  const cv = slabs.reduce((s, x) => s + x.cv, 0) / slabs.length;
  const cw = slabs.reduce((s, x) => s + x.cw, 0) / slabs.length;
  const off = [0, 1, 2].map((k) => cv * f.v[k]! + cw * f.w[k]!);
  const [axisX, axisZ] = [off[0]!, off[2]!];
  const dot = (p: number[], q: number[]) => p[0]! * q[0]! + p[1]! * q[1]! + p[2]! * q[2]!;
  const perp = (x: number[]) => {
    const h = dot(x, f.u);
    return [0, 1, 2].map((k) => x[k]! - h * f.u[k]!);
  };
  const V = perp(f.v);
  const C = perp([0, 1, 2].map((k) => axisZ * f.w[k]! - (k === 0 ? axisX : k === 2 ? axisZ : 0)));
  const bOf = (r: number) => {
    const [vv, vc, cc] = [dot(V, V), dot(V, C), dot(C, C)];
    return (-vc + Math.sqrt(Math.max(0, vc * vc - vv * (cc - r * r)))) / vv;
  };
  const prof: number[][] = [[u0, bOf(0)]];
  for (const s of slabs) {
    const dev = Math.hypot(s.cv - cv, s.cw - cw);
    prof.push([(s.a + s.b) / 2, bOf(s.r + dev * 0.15)]);
  }
  prof.push([u1, bOf(0)]);
  const pts2 = prof.filter((p, i) => i === 0 || Math.hypot(p[0]! - prof[i - 1]![0]!, p[1]! - prof[i - 1]![1]!) > 1e-5).map((p) => p.map(r5));
  return { id: base.id, bone, role: base.role ?? "body", z: base.z, filled: true, tone: base.tone, region: base.region, material: base.material, kind: "poly", points: pts2, closed: true, solid: { kind: "revolved", axisBone: bone, axis: "u", axisX: r5(axisX), axisZ: r5(axisZ), mirror: true } };
}

/** 🦾️ Replaces, in the listed views of every scale, the drawings of round limb bones by one {@link revolvedShape} each and of flat hands (with their fingers) by {@link handPrism}; the first body drawing of a bone lends id, region, material, tone and paint order. */
export function roundLimbs(asset: Json, bones: readonly string[], views: readonly string[], samples: (scale: string) => number): void {
  const { subtree } = require("./lib.ts");
  for (const sc of SCALES) {
    const r = asset.representations[sc];
    const vol = asset.volumes[sc];
    for (const view of views)
      for (const bone of bones) {
        const mine = r.geometry[view].filter((s: Json) => s.bone === bone && s.filled);
        if (mine.length === 0) continue;
        const { ringArea } = require("./lib.ts");
        const size = (s: Json) => (s.kind === "poly" ? Math.abs(ringArea(s.points)) : 0);
        const first = [...mine].filter((s: Json) => s.role === "body").sort((p: Json, q: Json) => size(q) - size(p))[0] ?? mine[0];
        const hand = bone.startsWith("hand-");
        const carried: Set<string> = hand ? subtree(asset, [bone]) : new Set([bone]);
        const ids = vol.body.filter((s: Json) => carried.has(s.bone)).map((s: Json) => s.id);
        const meta = { id: first.id, region: first.region, material: first.material, tone: first.tone, z: first.z };
        const made = [hand ? handPrism(asset, sc, bone, ids, meta) : revolvedShape(asset, sc, bone, ids, meta, samples(sc), view === "elevation" ? "v" : view === "front" ? "w" : "min")].filter(Boolean);
        if (made.length === 0) continue;
        const at = r.geometry[view].findIndex((s: Json) => s.bone === bone);
        r.geometry[view] = r.geometry[view].filter((s: Json) => s.bone !== bone);
        r.geometry[view].splice(at, 0, ...made);
      }
  }
}

/** ✋️ The drawing of a flat hand with its fingers as one prism (`extruded`, `sweep: "w"`): the profile is the projected outline of the hand solids in the bone plane at rest, swept across the lateral extent they occupy in the rest pose, so the hand keeps its true outline in the side view and its true thickness and width in the front and plan views in every pose. */
export function handPrism(asset: Json, scale: string, bone: string, ids: readonly string[], base: { id: string; region: string; material: string; tone: number; z: number; role?: string }): Json | null {
  const { outlineOf, simplify, toShapeFrame, ringArea } = require("./lib.ts");
  const { worldSolids } = require("../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts");
  const { restBones } = require("./lib.ts");
  const rings = outlineOf(asset, scale, new Set(ids), "elevation").sort((p: Json, q: Json) => Math.abs(ringArea(q)) - Math.abs(ringArea(p)));
  if (rings.length === 0) return null;
  const ring = toShapeFrame(asset, bone, "elevation", simplify(rings[0], (0.03 * Number(scale)) / 1000));
  let [z0, z1] = [Infinity, -Infinity];
  for (const w of worldSolids(asset, restBones(asset), Number(scale), "body")) if (ids.includes(w.id)) for (let i = 2; i < w.positions.length; i += 3) (z0 = Math.min(z0, w.positions[i])), (z1 = Math.max(z1, w.positions[i]));
  return { id: base.id, bone, role: base.role ?? "body", z: base.z, filled: true, tone: base.tone, region: base.region, material: base.material, kind: "poly", points: ring.map((p: number[]) => p.map(r5)), closed: true, solid: { kind: "extruded", sweep: "w", z0: r5(z0 + 0.0005), z1: r5(z1 - 0.0005) } };
}

/** 🧵️ Redraws a garment bone by bone at every scale and view: the candidate solids are grouped by their bone and each group is drawn as the outline of its own solids on its own bone, so a coat skirt swings with its thigh in the drawing as in the volume. A group reuses the picked drawing of its bone (the one named like one of its solids, else the largest; id, paint order, tone) or else takes those of the picked drawing nearest in the rig, with the id of its first solid and a paint order offset by `dz(bone)`; picked drawings whose bone has no candidate any more are removed. */
export function redrawByBone(asset: Json, pick: (s: Json) => boolean, candidates: (scale: string) => string[], tolMm = 0.03, dz: (bone: string) => number = () => 0): void {
  const { treeDistance } = require("./lib.ts");
  for (const sc of SCALES) {
    const r = asset.representations[sc];
    const body = asset.volumes[sc].body;
    const ids = candidates(sc).filter((id) => body.some((b: Json) => b.id === id));
    const groups = new Map<string, string[]>();
    for (const id of ids) {
      const bone = body.find((b: Json) => b.id === id).bone;
      groups.set(bone, [...(groups.get(bone) ?? []), id]);
    }
    for (const view of VIEWS) {
      const picked = r.geometry[view].filter((s: Json) => s.filled && s.kind === "poly" && pick(s));
      if (picked.length === 0) continue;
      let list: Json[] = r.geometry[view].filter((s: Json) => !picked.includes(s));
      for (const [bone, solids] of groups) {
        const { ringArea } = require("./lib.ts");
        const mine = picked.filter((s: Json) => s.bone === bone);
        const own = mine.find((s: Json) => solids.includes(s.id)) ?? mine.sort((p: Json, q: Json) => Math.abs(ringArea(q.points)) - Math.abs(ringArea(p.points)))[0];
        const meta = own ?? [...picked].sort((p: Json, q: Json) => treeDistance(asset, p.bone, bone) - treeDistance(asset, q.bone, bone))[0];
        const made = shapesFrom(asset, sc, view, solids, { id: own ? own.id : solids[0]!, bone, region: meta.region, material: meta.material, tone: meta.tone, z: own ? own.z : Math.round((meta.z + dz(bone)) * 1000) / 1000, role: "body" }, tolMm);
        list.push(...made);
      }
      r.geometry[view] = list.sort((p: Json, q: Json) => p.z - q.z);
    }
  }
}
