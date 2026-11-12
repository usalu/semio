/** 🧊️ Drafts a `Volume` for one asset or for every catalog asset from the 1:50 representation: every body, detail and glazing shape of the elevation becomes a `VolumeSolid` in the bone-local u, v, w frame, with its lateral span measured from the matching front and plan shapes.
 *
 * Usage: `repo tools exec bun -- volume-draft.ts (--asset <slug|category/slug|file> | --all) [--out <dir>] [--scale 50] [--no-check]`
 * Output: `<out>/<category>/<slug>.json` = `{ volume, report }`; with `--all` also `<out>/index.json`.
 * Progress goes to stderr; Ctrl-C (SIGINT, SIGTERM, SIGHUP/SIGBREAK) stops after the current asset and keeps what was written.
 * @see ./design.md
 * @see ./volume-drafting.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { join, resolve } from "node:path";

import {
  GENERATED,
  analyse,
  area,
  bboxOf,
  catalogEntries,
  cancel,
  checkCancelled,
  cross,
  dirToLocal,
  flattenGeometry,
  frames,
  installCancellation,
  loadAsset,
  median,
  norm,
  pointInRing,
  resolveAsset,
  restState,
  round5,
  simplifyRing,
  toLocal,
  toWorld,
  yieldToLoop,
  CancelledError,
  type CatalogEntry,
  type Evaluated,
  type Frame,
  type Json,
  type P2,
  type Ring,
  type V3,
} from "./volume-lib.ts";

export type Flag = { code: string; severity: "needs-author" | "low-confidence" | "info"; subject: string; message: string };

type Span = { z0: number; z1: number; view: "front" | "plan"; id: string; score: number; via: "id" | "family" | "bone" };
type Cluster = { z0: number; z1: number; spans: Span[]; disagreement: boolean };
type Role = "body" | "detail" | "glazing";
type Quaternion = [number, number, number, number];

const TOLERANCE_M = 0.0003;
const MIN_SPAN_M = 0.002;
const LOFT_STATIONS = 14;
const LOFT_EXPONENT = 8;
const SKIPPED_ROLES = new Set(["texture", "ground"]);
const DRAFT_ROLES = new Set<string>(["body", "detail", "glazing"]);

const qz = (angle: number): Quaternion => [0, 0, Math.sin(angle / 2), Math.cos(angle / 2)];

function qBasis(x: V3, y: V3, z: V3): Quaternion {
  const m = [x[0], y[0], z[0], x[1], y[1], z[1], x[2], y[2], z[2]] as const;
  const trace = m[0] + m[4] + m[8];
  let q: Quaternion;
  if (trace > 0) {
    const s = Math.sqrt(trace + 1) * 2;
    q = [(m[7] - m[5]) / s, (m[2] - m[6]) / s, (m[3] - m[1]) / s, 0.25 * s];
  } else if (m[0] > m[4] && m[0] > m[8]) {
    const s = Math.sqrt(1 + m[0] - m[4] - m[8]) * 2;
    q = [0.25 * s, (m[1] + m[3]) / s, (m[2] + m[6]) / s, (m[7] - m[5]) / s];
  } else if (m[4] > m[8]) {
    const s = Math.sqrt(1 + m[4] - m[0] - m[8]) * 2;
    q = [(m[1] + m[3]) / s, 0.25 * s, (m[5] + m[7]) / s, (m[2] - m[6]) / s];
  } else {
    const s = Math.sqrt(1 + m[8] - m[0] - m[4]) * 2;
    q = [(m[2] + m[6]) / s, (m[5] + m[7]) / s, 0.25 * s, (m[3] - m[1]) / s];
  }
  const l = Math.hypot(...q);
  return q.map((c) => round5(c / l)) as Quaternion;
}

function qFromTo(from: V3, to: V3): Quaternion {
  const a = norm(from);
  const b = norm(to);
  const d = a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
  if (d > 1 - 1e-9) return [0, 0, 0, 1];
  if (d < -1 + 1e-9) {
    const axis = norm(cross(a, Math.abs(a[0]) < 0.9 ? [1, 0, 0] : [0, 1, 0]));
    return [round5(axis[0]), round5(axis[1]), round5(axis[2]), 0];
  }
  const c = cross(a, b);
  const l = Math.hypot(1 + d, c[0], c[1], c[2]);
  return [round5(c[0] / l), round5(c[1] / l), round5(c[2] / l), round5((1 + d) / l)];
}

const overlap1D = (a0: number, a1: number, b0: number, b1: number): number => {
  const inter = Math.min(a1, b1) - Math.max(a0, b0);
  const union = Math.max(a1, b1) - Math.min(a0, b0);
  return union <= 1e-9 ? 1 : Math.max(0, inter) / union;
};

const FAMILY_SUFFIX = /(?:^(?:l|r|n|f|left|right|near|far)-|-(?:l|r|n|f|left|right|near|far|[0-9]+|[a-z])$)/u;
const family = (id: string): string => {
  let out = id;
  for (let i = 0; i < 3; i++) out = out.replace(FAMILY_SUFFIX, "");
  return out;
};
const related = (a: string, b: string): boolean => a.startsWith(`${b}-`) || b.startsWith(`${a}-`) || family(a) === family(b);

type Candidate = { evaluated: Evaluated; id: string; bone: string };

/** 🔗️ The front or plan shapes that depict an elevation shape: the same id; else the same id family on the same bone; else the best vertical (front) or forward (plan) overlap on the same bone. */
function matchView(view: "front" | "plan", elevation: { id: string; bone: string; x0: number; x1: number; y0: number; y1: number }, candidates: readonly Candidate[]): { spans: Span[]; ids: string[] } {
  const scored: { c: Candidate; ov: number; box: ReturnType<typeof bboxOf> }[] = [];
  for (const c of candidates) {
    const box = bboxOf(c.evaluated.rings.flat());
    if (!Number.isFinite(box.minX)) continue;
    const ov = view === "front" ? overlap1D(elevation.y0, elevation.y1, box.minY, box.maxY) : overlap1D(elevation.x0, elevation.x1, box.minX, box.maxX);
    scored.push({ c, ov, box });
  }
  const exact = scored.filter((s) => s.c.id === elevation.id);
  let chosen: typeof scored;
  let via: Span["via"];
  if (exact.length > 0) {
    chosen = exact;
    via = "id";
  } else {
    const kin = scored.filter((s) => s.c.bone === elevation.bone && related(s.c.id, elevation.id) && s.ov >= 0.3);
    if (kin.length > 0) {
      chosen = kin;
      via = "family";
    } else {
      const near = scored.filter((s) => s.c.bone === elevation.bone && s.ov >= 0.5).sort((a, b) => b.ov - a.ov);
      chosen = near.filter((s) => s.ov >= near[0]!.ov - 0.1);
      via = "bone";
    }
  }
  const spans = chosen.map((s): Span => (view === "front" ? { z0: s.box.minX, z1: s.box.maxX, view, id: s.c.id, score: s.ov, via } : { z0: s.box.minY, z1: s.box.maxY, view, id: s.c.id, score: s.ov, via }));
  return { spans, ids: chosen.map((s) => s.c.id) };
}

/** 🧮️ Groups spans of both views that depict the same lateral part (substantial overlap) and settles a disagreement between the views by the better score. */
function cluster(spans: readonly Span[]): Cluster[] {
  const sorted = [...spans].sort((a, b) => a.z0 + a.z1 - (b.z0 + b.z1));
  const groups: Span[][] = [];
  for (const span of sorted) {
    const home = groups.find((g) => g.some((s) => {
      const len = Math.min(s.z1 - s.z0, span.z1 - span.z0);
      const inter = Math.min(s.z1, span.z1) - Math.max(s.z0, span.z0);
      return len <= 0.004 ? Math.abs((s.z0 + s.z1 - span.z0 - span.z1) / 2) < 0.01 : inter / len >= 0.5;
    }));
    if (home === undefined) groups.push([span]);
    else home.push(span);
  }
  return groups.map((g) => {
    const best = [...g].sort((a, b) => b.score - a.score || (a.view === "plan" ? -1 : 1))[0]!;
    const widest = Math.max(...g.map((s) => s.z1 - s.z0), 0.02);
    const disagreement = g.some((s) => Math.max(Math.abs(s.z0 - best.z0), Math.abs(s.z1 - best.z1)) / widest > 0.35);
    return { z0: best.z0, z1: best.z1, spans: g, disagreement };
  });
}

type Drafted = { solid: Json; route: string; flags: Flag[] };

function groupRings(rings: readonly Ring[]): { outer: Ring; holes: Ring[] }[] {
  const items = rings.map((r) => ({ ring: r, a: Math.abs(area(r)) })).filter((r) => r.a > 1e-9 && r.ring.length >= 3).sort((p, q) => q.a - p.a);
  const groups: { outer: Ring; holes: Ring[]; chain: Ring[] }[] = [];
  const parent = new Map<Ring, Ring | null>();
  const depthOf = (ring: Ring): number => {
    let d = 0;
    let at = parent.get(ring) ?? null;
    while (at !== null) {
      d++;
      at = parent.get(at) ?? null;
    }
    return d;
  };
  items.forEach((item, i) => {
    let home: Ring | null = null;
    for (let j = i - 1; j >= 0; j--) if (pointInRing(item.ring[0]!, items[j]!.ring)) {
      home = items[j]!.ring;
      break;
    }
    parent.set(item.ring, home);
  });
  for (const item of items) {
    const depth = depthOf(item.ring);
    if (depth % 2 === 0) groups.push({ outer: item.ring, holes: [], chain: [item.ring] });
    else {
      const owner = parent.get(item.ring)!;
      const group = groups.find((g) => g.outer === owner);
      if (group !== undefined) group.holes.push(item.ring);
    }
  }
  return groups.map(({ outer, holes }) => ({ outer, holes }));
}

const pair = (p: P2, flip = false): [number, number] => (flip ? [round5(p.y), round5(p.x)] : [round5(p.x), round5(p.y)]);

/** 📏️ The interval of a ring along its second coordinate at the first-coordinate station `t`, from the crossings of the vertical line. */
function crossing(ring: Ring, t: number): [number, number] | null {
  let lo = Infinity;
  let hi = -Infinity;
  for (let i = 0, k = ring.length - 1; i < ring.length; k = i++) {
    const a = ring[k]!;
    const b = ring[i]!;
    if (a.x <= t !== b.x <= t) {
      const y = a.y + ((t - a.x) / (b.x - a.x)) * (b.y - a.y);
      if (y < lo) lo = y;
      if (y > hi) hi = y;
    }
  }
  return Number.isFinite(lo) ? [lo, hi] : null;
}

type Common = { id: string; bone: string; material: string; role: Role; tone: number; region?: string; regions?: string[] };

function extrudeOf(common: Common, outer: Ring, holes: Ring[], z0: number, z1: number, sweep: "w" | "v"): Json {
  const flip = sweep === "v";
  const solid: Json = {
    ...common,
    kind: "extrude",
    profile: outer.map((p) => pair(p, flip)),
    plane: flip ? "zx" : "xy",
    from: round5(z0),
    to: round5(z1),
  };
  if (holes.length > 0) solid.holes = holes.map((h) => h.map((p) => pair(p, flip)));
  return solid;
}

/** 🪜️ A tapering extrusion as a loft of rectangle-like superellipse sections along u, because `taperTo` scales the whole profile, not the span along the bone. */
function loftOfTaper(common: Common, outer: Ring, span: (u: number) => [number, number], sweep: "w" | "v"): Json | null {
  const box = bboxOf(outer);
  const length = box.maxX - box.minX;
  if (length <= 1e-6) return null;
  const sections: Json[] = [];
  for (let i = 0; i < LOFT_STATIONS; i++) {
    const t = box.minX + (length * i) / (LOFT_STATIONS - 1);
    const probe = Math.min(box.maxX - length * 0.002, Math.max(box.minX + length * 0.002, t));
    const interval = crossing(outer, probe);
    if (interval === null) continue;
    const [s0, s1] = span(t);
    const profileCentre = (interval[0] + interval[1]) / 2;
    const profileHalf = Math.max((interval[1] - interval[0]) / 2, 1e-4);
    const spanCentre = (s0 + s1) / 2;
    const spanHalf = Math.max((s1 - s0) / 2, 1e-4);
    sections.push(sweep === "w" ? { at: [round5(t), round5(profileCentre), round5(spanCentre)], rx: round5(profileHalf), ry: round5(spanHalf), exponent: LOFT_EXPONENT, rotation: [0.5, 0.5, 0.5, 0.5] } : { at: [round5(t), round5(spanCentre), round5(profileCentre)], rx: round5(spanHalf), ry: round5(profileHalf), exponent: LOFT_EXPONENT, rotation: [0.5, 0.5, 0.5, 0.5] });
  }
  return sections.length < 2 ? null : { ...common, kind: "loft", sections, capStart: true, capEnd: true };
}

const copy = (c: Common): Common => ({ ...c });
const rep = (ctx: Context): Json => ctx.asset.representations[String(ctx.scale)];

type Context = {
  asset: Json;
  scale: number;
  frame: Record<string, Frame>;
  materialOf: (shape: Json, twins: readonly Json[]) => { id: string; guessed: boolean };
};

/** 🌀️ A `solid.revolved` shape as a `revolve` about its hinted axis: the outline measured as [axial, radial] about the axis through the hinted point, built on the canonical x axis and placed by `transform` so `lateralScale` always acts on the lateral direction. */
function revolveOf(ctx: Context, shape: Json, rings: readonly Ring[], common: Common): Drafted {
  const hint = shape.solid;
  const flags: Flag[] = [];
  const own = ctx.frame[shape.bone]!;
  const axisFrame = ctx.frame[hint.axisBone] ?? own;
  const axisWorldDir = axisFrame[hint.axis as "u" | "v" | "w"] as V3;
  const axisPoint: V3 = [axisFrame.o[0] + hint.axisX, axisFrame.o[1], axisFrame.o[2] + (hint.axisZ ?? 0)];
  const a = norm(dirToLocal(own, axisWorldDir));
  const origin = toLocal(own, axisPoint);
  const k = hint.lateralScale ?? 1;
  if (Math.abs(a[2]) > 0.5) return { solid: null as never, route: "revolve", flags: [{ code: "revolve-axis-lateral", severity: "needs-author", subject: shape.id, message: "the hinted axis is lateral in the bone frame; draft the part by hand" }] };
  const normal: V3 = [0, 0, 1];
  const m = norm(cross(normal, a));
  const group = groupRings(rings)[0];
  if (group === undefined) return { solid: null as never, route: "revolve", flags: [{ code: "degenerate-outline", severity: "needs-author", subject: shape.id, message: "revolved outline has no area" }] };
  const sample = group.outer.map((p) => {
    const d: V3 = [p.x - origin[0], p.y - origin[1], 0];
    return { h: d[0] * a[0] + d[1] * a[1], s: d[0] * m[0] + d[1] * m[1] };
  });
  const above = sample.some((p) => p.s > 1e-6);
  const below = sample.some((p) => p.s < -1e-6);
  let profile: [number, number][];
  if (above && below) {
    const hs = sample.map((p) => p.h);
    const lo = Math.min(...hs);
    const hi = Math.max(...hs);
    const rows: [number, number][] = [];
    const ring: Ring = sample.map((p) => ({ x: p.h, y: p.s }));
    for (let i = 0; i <= 24; i++) {
      const t = Math.min(hi - (hi - lo) * 0.002, Math.max(lo + (hi - lo) * 0.002, lo + ((hi - lo) * i) / 24));
      const interval = crossing(ring, t);
      if (interval !== null) rows.push([round5(t), round5(Math.max(Math.abs(interval[0]), Math.abs(interval[1])))]);
    }
    profile = rows;
    flags.push({ code: "revolve-envelope", severity: "info", subject: shape.id, message: "the outline straddles the axis; the profile is its radial envelope" });
  } else {
    const ring = simplifyRing(sample.map((p) => ({ x: p.h, y: Math.abs(p.s) })), TOLERANCE_M);
    profile = ring.map((p) => [round5(p.x), round5(p.y)]);
  }
  if (profile.length < 2) return { solid: null as never, route: "revolve", flags: [{ code: "degenerate-outline", severity: "needs-author", subject: shape.id, message: "revolved profile collapsed" }] };
  const solid: Json = { ...common, kind: "revolve", profile, axis: [1, 0, 0], origin: [0, 0, 0] };
  if (Math.abs(k - 1) > 1e-6) solid.lateralScale = k;
  const angle = Math.atan2(a[1], a[0]);
  const transform: Json = { translation: origin.map(round5) };
  if (Math.abs(angle) > 1e-6) transform.rotation = qz(angle);
  if (Math.abs(angle) <= 1e-6) {
    solid.origin = origin.map(round5);
  } else solid.transform = transform;
  return { solid, route: "revolve", flags };
}

/** 🧱️ One elevation shape to its solids, given the lateral clusters measured from front and plan. */
function draftShape(ctx: Context, shape: Json, evaluated: Evaluated, clusters: Cluster[], fallback: boolean, common: Common): Drafted[] {
  const out: Drafted[] = [];
  const flags: Flag[] = [];
  const bone = ctx.asset.rig.bones.find((b: Json) => b.id === shape.bone);
  const own = ctx.frame[shape.bone]!;
  const rings = flattenGeometry(shape).rings.map((r: Ring) => r);
  const hint = shape.solid;

  if (shape.kind === "member") {
    const end = (e: Json): V3 => toLocal(own, toWorld(ctx.frame[e.bone ?? shape.bone]!, [e.x, e.y, e.z]));
    const from = end(shape.from);
    const to = end(shape.to);
    const d: V3 = [to[0] - from[0], to[1] - from[1], to[2] - from[2]];
    const length = Math.hypot(...d);
    const centre: V3 = [(from[0] + to[0]) / 2, (from[1] + to[1]) / 2, (from[2] + to[2]) / 2];
    if (length < 1e-6) return [{ solid: null as never, route: "member", flags: [{ code: "degenerate-outline", severity: "needs-author", subject: shape.id, message: "member of zero length" }] }];
    const a = norm(d);
    if (shape.section.kind === "circle") {
      const r = round5(shape.section.d / 2);
      const q = qFromTo([0, 0, 1], a);
      const rotation = q;
      const sections = [from, to].map((p) => ({ at: p.map(round5), rx: r, ry: r, rotation }));
      return [{ solid: { ...common, kind: "loft", sections, capStart: true, capEnd: true }, route: "member-loft", flags }];
    }
    const across = cross(a, [0, 0, 1]);
    const width = norm(Math.hypot(...across) > 1e-6 ? across : cross(a, [0, 1, 0]));
    const depth = cross(width, a);
    const solid: Json = { ...common, kind: "box", size: [round5(length), round5(shape.section.h), round5(shape.section.w)], transform: { translation: centre.map(round5), rotation: qBasis(a, depth, width) } };
    return [{ solid, route: "member-box", flags }];
  }

  const box = bboxOf(rings.flat());
  const lateralFlags = (cl: Cluster): Flag[] => {
    const f: Flag[] = [];
    if (cl.disagreement) f.push({ code: "low-confidence-lateral", severity: "low-confidence", subject: shape.id, message: `front and plan disagree on the lateral span (${cl.spans.map((s) => `${s.view}:${s.id} ${s.z0.toFixed(3)}..${s.z1.toFixed(3)}`).join(", ")}); took the better-scored one` });
    if (cl.spans.every((s) => s.via === "bone")) f.push({ code: "low-confidence-lateral", severity: "low-confidence", subject: shape.id, message: `lateral span matched only by bone and view overlap (${cl.spans.map((s) => `${s.view}:${s.id}`).join(", ")})` });
    if (cl.z1 - cl.z0 < MIN_SPAN_M) f.push({ code: "low-confidence-lateral", severity: "low-confidence", subject: shape.id, message: `lateral span ${(1000 * (cl.z1 - cl.z0)).toFixed(1)} mm is below 2 mm; widened to 2 mm` });
    return f;
  };

  if (hint !== undefined && hint.kind === "revolved") {
    const r = revolveOf(ctx, shape, rings, common);
    return r.solid === null ? [r] : [r];
  }

  if (hint !== undefined && hint.kind === "extruded") {
    const sweep: "w" | "v" = hint.sweep === "v" ? "v" : "w";
    const base = sweep === "w" ? own.o[2] : 0;
    const groups = groupRings(rings);
    if (groups.length === 0) return [{ solid: null as never, route: "extrude", flags: [{ code: "degenerate-outline", severity: "needs-author", subject: shape.id, message: "extruded outline has no area" }] }];
    const length = bone?.lengthM ?? 0;
    groups.forEach((g, i) => {
      const id = i === 0 ? common.id : `${common.id}-${i + 1}`;
      const c = { ...copy(common), id };
      const taper = hint.zTip !== undefined && (Math.abs(hint.zTip[0] - hint.z0) > 1e-6 || Math.abs(hint.zTip[1] - hint.z1) > 1e-6);
      if (taper) {
        const span = (u: number): [number, number] => {
          const t = length > 0 ? Math.min(1, Math.max(0, u / length)) : 0;
          return [hint.z0 + (hint.zTip[0] - hint.z0) * t - base, hint.z1 + (hint.zTip[1] - hint.z1) * t - base];
        };
        const loft = loftOfTaper(c, g.outer, span, sweep);
        if (loft !== null) {
          out.push({ solid: loft, route: "extrude-taper-loft", flags: [{ code: "taper-as-loft", severity: "info", subject: shape.id, message: "span taper along the bone drafted as a loft of superellipse sections; holes in the outline are dropped" }] });
          return;
        }
      }
      out.push({ solid: extrudeOf(c, simplifyRing(g.outer, TOLERANCE_M), g.holes.map((h) => simplifyRing(h, TOLERANCE_M)), hint.z0 - base, hint.z1 - base, sweep), route: "extrude-hint", flags: [] });
    });
    return out;
  }

  const solids = (build: (z0: number, z1: number, suffix: string, extra: Flag[]) => Json | null, sizeHint: number): Drafted[] => {
    const results: Drafted[] = [];
    if (clusters.length === 0) {
      const half = sizeHint / 2;
      const note: Flag[] = [{ code: "needs-author", severity: "needs-author", subject: shape.id, message: `no front or plan shape matches; lateral span estimated as ${(sizeHint * 1000).toFixed(0)} mm centred on the symmetry plane` }];
      const solid = build(-half, half, "", note);
      if (solid !== null) results.push({ solid, route: shape.kind, flags: note });
      return results;
    }
    const spanned = clusters.map((cl) => {
      const z0 = cl.z1 - cl.z0 < MIN_SPAN_M ? (cl.z0 + cl.z1) / 2 - MIN_SPAN_M / 2 : cl.z0;
      const z1 = cl.z1 - cl.z0 < MIN_SPAN_M ? (cl.z0 + cl.z1) / 2 + MIN_SPAN_M / 2 : cl.z1;
      return { cl, z0: z0 - own.o[2], z1: z1 - own.o[2] };
    });
    const symmetric = spanned.length === 2 && Math.abs(spanned[0]!.z1 - spanned[0]!.z0 - (spanned[1]!.z1 - spanned[1]!.z0)) <= 0.15 * Math.max(spanned[0]!.z1 - spanned[0]!.z0, spanned[1]!.z1 - spanned[1]!.z0) && spanned[0]!.cl.z0 + spanned[0]!.cl.z1 < spanned[1]!.cl.z0 + spanned[1]!.cl.z1;
    if (symmetric) {
      const [a, b] = spanned as [(typeof spanned)[number], (typeof spanned)[number]];
      const step = (b.z0 + b.z1) / 2 - (a.z0 + a.z1) / 2;
      const f = [...lateralFlags(a.cl), ...lateralFlags(b.cl)];
      const solid = build(a.z0, a.z1, "", f);
      if (solid !== null) {
        solid.repeat = { count: 2, translation: [0, 0, round5(step)] };
        results.push({ solid, route: `${shape.kind}-pair`, flags: [...f, { code: "pair-as-repeat", severity: "info", subject: shape.id, message: `left and right part drafted as one solid with repeat (step ${step.toFixed(3)} m along w)` }] });
      }
      return results;
    }
    spanned.forEach(({ cl, z0, z1 }, i) => {
      const centre = (cl.z0 + cl.z1) / 2;
      const suffix = spanned.length === 1 ? "" : spanned.length === 2 ? (centre >= 0 ? "-l" : "-r") : `-${i + 1}`;
      const f = lateralFlags(cl);
      const solid = build(z0, z1, suffix, f);
      if (solid !== null) results.push({ solid, route: shape.kind, flags: f });
    });
    return results;
  };

  const sagittal = Math.min(box.maxX - box.minX, box.maxY - box.minY);
  const depthM = ctx.asset.dimensions.depthM as number;
  const sizeHint = (() => {
    if (shape.kind === "ellipse") return 2 * Math.min(shape.rx, shape.ry);
    if (shape.kind === "capsule") return 2 * Math.min(shape.r0, shape.r1 || shape.r0);
    const large = (box.maxX - box.minX) >= 0.4 * ctx.asset.dimensions.widthM || (box.maxY - box.minY) >= 0.4 * ctx.asset.dimensions.heightM;
    return Math.min(depthM * 0.9, large ? 0.8 * depthM : Math.max(0.02, 0.6 * sagittal));
  })();

  if (shape.kind === "ellipse") {
    return solids((z0, z1, suffix) => {
      const solid: Json = { ...copy(common), id: `${common.id}${suffix}`, kind: "ellipsoid", radii: [round5(shape.rx), round5(shape.ry), round5(Math.max((z1 - z0) / 2, 1e-4))] };
      const transform: Json = { translation: [round5(shape.cx), round5(shape.cy), round5((z0 + z1) / 2)] };
      if (shape.rotationDeg !== undefined && Math.abs(shape.rotationDeg) > 1e-6) transform.rotation = qz((shape.rotationDeg * Math.PI) / 180);
      solid.transform = transform;
      return solid;
    }, sizeHint);
  }

  if (shape.kind === "capsule") {
    return solids((z0, z1, suffix) => {
      const id = `${common.id}${suffix}`;
      const half = Math.max((z1 - z0) / 2, 1e-4);
      const r = Math.max(shape.r0, shape.r1);
      const k = half / Math.max(r, 1e-6);
      const centre = (z0 + z1) / 2;
      const length = Math.hypot(shape.x1 - shape.x0, shape.y1 - shape.y0);
      if (length < 1e-6) return { ...copy(common), id, kind: "ellipsoid", radii: [round5(r), round5(r), round5(half)], transform: { translation: [round5(shape.x0), round5(shape.y0), round5(centre)] } };
      if (Math.abs(k - 1) <= 0.02) return { ...copy(common), id, kind: "capsule", p0: [round5(shape.x0), round5(shape.y0), round5(centre)], p1: [round5(shape.x1), round5(shape.y1), round5(centre)], r0: round5(shape.r0), r1: round5(shape.r1) };
      const angle = Math.atan2(shape.y1 - shape.y0, shape.x1 - shape.x0);
      const transform: Json = { translation: [round5(shape.x0), round5(shape.y0), round5(centre)] };
      if (Math.abs(angle) > 1e-6) transform.rotation = qz(angle);
      return { ...copy(common), id, kind: "capsule", p0: [0, 0, 0], p1: [round5(length), 0, 0], r0: round5(shape.r0), r1: round5(shape.r1), lateralScale: round5(k), transform };
    }, sizeHint);
  }

  if (shape.kind === "rect") {
    return solids((z0, z1, suffix) => {
      const solid: Json = { ...copy(common), id: `${common.id}${suffix}`, kind: "box", size: [round5(shape.w), round5(shape.h), round5(z1 - z0)], transform: { translation: [round5(shape.x + shape.w / 2), round5(shape.y + shape.h / 2), round5((z0 + z1) / 2)] } };
      if ((shape.r ?? 0) > 1e-6) solid.cornerRadius = round5(Math.min(shape.r, Math.min(shape.w, shape.h) / 2));
      return solid;
    }, sizeHint);
  }

  if (shape.kind === "poly" && shape.closed === false) {
    const line = rings[0] ?? [];
    if (line.length < 2) return [{ solid: null as never, route: "poly", flags: [{ code: "degenerate-outline", severity: "info", subject: shape.id, message: "open polyline with fewer than two points" }] }];
    const pen = shape.penMm ?? rep(ctx).pens?.[shape.role] ?? 0.25;
    const fallbackRadius = Math.max(0.004, (pen * ctx.scale) / 2000);
    const exact = clusters.filter((c) => c.spans.every((sp) => sp.via === "id"));
    const bars = exact.length > 0 ? exact : [null];
    const longest = Math.max(...line.slice(1).map((q, i) => Math.hypot(q.x - line[i]!.x, q.y - line[i]!.y)));
    const cap = Math.max(0.004, 0.2 * longest);
    bars.forEach((cl, ci) => {
      const centre = cl === null ? 0 : (cl.z0 + cl.z1) / 2 - own.o[2];
      const r = round5(Math.min(cap, cl === null ? fallbackRadius : Math.max((cl.z1 - cl.z0) / 2, 0.002)));
      const suffix = bars.length === 1 ? "" : bars.length === 2 ? (centre >= 0 ? "-l" : "-r") : `-${ci + 1}`;
      const f: Flag[] = cl === null ? [{ code: "needs-author", severity: "needs-author", subject: shape.id, message: `thin bar without a lateral twin; section radius ${(r * 1000).toFixed(1)} mm taken from the pen of the role at this scale` }] : lateralFlags(cl);
      for (let i = 0; i + 1 < line.length; i++) {
        const a = line[i]!;
        const b = line[i + 1]!;
        if (Math.hypot(b.x - a.x, b.y - a.y) < 1e-6) continue;
        const id = `${common.id}${suffix}${line.length > 2 ? `-${i + 1}` : ""}`;
        out.push({ solid: { ...copy(common), id, kind: "capsule", p0: [round5(a.x), round5(a.y), round5(centre)], p1: [round5(b.x), round5(b.y), round5(centre)], r0: r, r1: r }, route: "polyline-capsule", flags: i === 0 ? f : [] });
      }
    });
    if (out.length > 0) out[0]!.flags.push({ code: "polyline-as-capsules", severity: "info", subject: shape.id, message: "open polyline drafted as round bars (capsules) of the measured or pen-derived radius" });
    return out;
  }

  const groups = groupRings(rings);
  if (groups.length === 0) return [{ solid: null as never, route: shape.kind, flags: [{ code: "degenerate-outline", severity: "info", subject: shape.id, message: "outline has no area" }] }];
  groups.forEach((g, gi) => {
    const outer = simplifyRing(g.outer, TOLERANCE_M);
    const holes = g.holes.map((h) => simplifyRing(h, TOLERANCE_M)).filter((h) => h.length >= 3);
    const sub = solids((z0, z1, suffix) => extrudeOf({ ...copy(common), id: `${common.id}${gi === 0 ? "" : `-${gi + 1}`}${suffix}` }, outer, holes, z0, z1, "w"), sizeHint);
    out.push(...sub);
  });
  void evaluated;
  void fallback;
  return out;
}

export type Provenance = { solid: string; route: string; source: { view: "elevation"; shape: string; kind: string }; lateral: { z: [number, number] | null; views: string[]; shapes: string[]; confidence: "measured" | "hinted" | "estimated" | "n/a" } };

export type DraftResult = {
  volume: { status: "authored"; solids: Json[]; decisions: never[] };
  report: {
    asset: string;
    category: string;
    scale: number;
    bboxM: { draft: V3 | null; dimensions: V3; relativeError: V3; axes: "[x ahead = widthM, y up = heightM, z lateral = depthM]" };
    counts: { elevationShapes: number; drafted: number; solids: number; skipped: number; needsAuthor: number; lowConfidence: number; unmatchedFront: number; unmatchedPlan: number };
    provenance: Provenance[];
    flags: Flag[];
    unmatched: { view: "front" | "plan"; id: string; bone: string; kind: string; role: string }[];
    skipped: { shape: string; reason: string }[];
    silhouette?: { elevation: Json; front: Json; plan: Json };
  };
};

const commonOf = (ctx: Context, shape: Json, twins: readonly Json[]): Common & { guessed: boolean } => {
  const mat = ctx.materialOf(shape, twins);
  const c: Common & { guessed: boolean } = { id: shape.id, bone: shape.bone, material: mat.id, role: shape.role, tone: shape.tone, guessed: mat.guessed };
  if (shape.region !== undefined) c.region = shape.region;
  if (shape.regions !== undefined) c.regions = shape.regions;
  return c;
};

/** 👁️ A part drawn only in the front and/or plan view (no elevation shape claims it): the front outline (z, y) extruded along x over the forward range of its plan twin, or the plan outline (x, z) along y over the height range of its front twin, built on world axes about the bone origin and rotated into the bone frame by `transform`; the missing extent of a single view is estimated from the other two dimensions of the part. */
function viewOnly(ctx: Context, shape: Json, f: Candidate | undefined, p: Candidate | undefined, twin: Candidate | undefined, elevation: readonly Evaluated[], common: Common & { guessed: boolean }): Drafted[] {
  const own = ctx.frame[shape.bone]!;
  const phi = Math.atan2(own.u[1], own.u[0]);
  const flags: Flag[] = [];
  const { guessed, ...base } = common;
  if (guessed) flags.push({ code: "material-guessed", severity: "info", subject: shape.id, message: `material ${base.material} taken from the region or the bone, not from the shape` });
  const primary = f ?? p!;
  if (!primary.evaluated.closed) return [{ solid: null as never, route: "view-only", flags: [{ code: "skipped", severity: "info", subject: shape.id, message: "open polyline seen in one view only; draft it by hand" }] }];
  const rings = primary.evaluated.rings.map((r) => r);
  const groups = groupRings(rings);
  if (groups.length === 0) return [{ solid: null as never, route: "view-only", flags: [{ code: "degenerate-outline", severity: "info", subject: shape.id, message: "outline has no area" }] }];
  const box = bboxOf(rings.flat());
  const sizeA = box.maxX - box.minX;
  const sizeB = box.maxY - box.minY;
  let lo: number;
  let hi: number;
  const source = f !== undefined ? (p ?? twin) : twin;
  if (source !== undefined) {
    const sb = bboxOf(source.evaluated.rings.flat());
    [lo, hi] = f !== undefined ? [sb.minX, sb.maxX] : [sb.minY, sb.maxY];
  } else {
    const t = Math.max(0.01, Math.min(0.6 * Math.min(sizeA, sizeB), 0.3));
    const near = elevation.filter((e) => e.shape.bone === shape.bone && e.rings.length > 0).flatMap((e) => e.rings.flat());
    const eb = near.length > 0 ? bboxOf(near) : null;
    const centre = f !== undefined ? (eb === null ? own.o[0] : (eb.minX + eb.maxX) / 2) : eb === null ? own.o[1] : (eb.minY + eb.maxY) / 2;
    [lo, hi] = [centre - t / 2, centre + t / 2];
    flags.push({ code: "needs-author", severity: "needs-author", subject: shape.id, message: `part seen in the ${f !== undefined ? "front" : "plan"} view only; its ${f !== undefined ? "forward" : "vertical"} extent is estimated as ${(t * 1000).toFixed(0)} mm` });
  }
  if (hi - lo < MIN_SPAN_M) [lo, hi] = [(lo + hi) / 2 - MIN_SPAN_M / 2, (lo + hi) / 2 + MIN_SPAN_M / 2];
  const out: Drafted[] = [];
  groups.forEach((g, gi) => {
    const outer = simplifyRing(g.outer, TOLERANCE_M);
    const holes = g.holes.map((h) => simplifyRing(h, TOLERANCE_M)).filter((h) => h.length >= 3);
    const id = `${base.id}${gi === 0 ? "" : `-${gi + 1}`}`;
    const solid: Json =
      f !== undefined
        ? { ...base, id, kind: "extrude", profile: outer.map((q) => [round5(q.y - own.o[1]), round5(q.x - own.o[2])]), plane: "yz", from: round5(lo - own.o[0]), to: round5(hi - own.o[0]) }
        : { ...base, id, kind: "extrude", profile: outer.map((q) => [round5(q.y - own.o[2]), round5(q.x - own.o[0])]), plane: "zx", from: round5(lo - own.o[1]), to: round5(hi - own.o[1]) };
    if (holes.length > 0) solid.holes = holes.map((h) => h.map((q) => (f !== undefined ? [round5(q.y - own.o[1]), round5(q.x - own.o[2])] : [round5(q.y - own.o[2]), round5(q.x - own.o[0])])));
    if (Math.abs(phi) > 1e-6) solid.transform = { rotation: qz(-phi) };
    out.push({ solid, route: f !== undefined ? "view-only-front" : "view-only-plan", flags: gi === 0 ? [...flags, { code: "view-only-part", severity: "info", subject: shape.id, message: `part drawn only in the ${f !== undefined ? "front" : "plan"} view${source !== undefined ? " (extent from the other view)" : " (other extent estimated)"}` }] : [] });
  });
  return out;
}

export function draftAsset(asset: Json, scale = 50, check = true): DraftResult {
  const rep = asset.representations[String(scale)];
  if (rep === undefined || rep.status !== "authored") throw new Error(`asset "${asset.id}" has no authored 1:${scale} representation`);
  const frame = frames(asset);
  const state = restState(asset, scale);
  const materialIds = new Set<string>(asset.materials.map((m: Json) => m.id));
  const regionMaterial = new Map<string, string>((asset.thermal?.regions ?? []).map((r: Json) => [r.id, r.material]));
  const boneMaterial = new Map<string, Map<string, number>>();
  for (const v of ["elevation", "front", "plan"] as const) for (const s of rep.geometry[v]) if (s.material !== undefined && materialIds.has(s.material)) {
    const m = boneMaterial.get(s.bone) ?? new Map<string, number>();
    m.set(s.material, (m.get(s.material) ?? 0) + 1);
    boneMaterial.set(s.bone, m);
  }
  const ctx: Context = {
    asset,
    scale,
    frame,
    materialOf: (shape, twins) => {
      for (const s of [shape, ...twins]) if (s.material !== undefined && materialIds.has(s.material)) return { id: s.material, guessed: false };
      for (const s of [shape, ...twins]) {
        const m = s.region !== undefined ? regionMaterial.get(s.region) : s.regions?.map((r: string) => regionMaterial.get(r)).find((x: string | undefined) => x !== undefined);
        if (m !== undefined && materialIds.has(m)) return { id: m, guessed: true };
      }
      const byBone = [...(boneMaterial.get(shape.bone)?.entries() ?? [])].sort((a, b) => b[1] - a[1])[0];
      if (byBone !== undefined) return { id: byBone[0], guessed: true };
      return { id: asset.materials[0].id, guessed: true };
    },
  };
  const used = new Set<string>();
  const unique = (id: string): string => {
    let out = id;
    for (let n = 2; used.has(out); n++) out = `${id}-${n}`;
    used.add(out);
    return out;
  };
  const front: Candidate[] = state.front.filter((e) => !SKIPPED_ROLES.has(e.shape.role)).map((e) => ({ evaluated: e, id: e.shape.id, bone: e.shape.bone }));
  const plan: Candidate[] = state.plan.filter((e) => !SKIPPED_ROLES.has(e.shape.role)).map((e) => ({ evaluated: e, id: e.shape.id, bone: e.shape.bone }));
  const consumed = new Set<string>();
  const solids: Json[] = [];
  const flags: Flag[] = [];
  const provenance: Provenance[] = [];
  const skipped: { shape: string; reason: string }[] = [];
  let drafted = 0;

  for (const evaluated of state.elevation) {
    const shape = evaluated.shape;
    if (!DRAFT_ROLES.has(shape.role)) {
      skipped.push({ shape: shape.id, reason: `role ${shape.role}` });
      continue;
    }
    const points = evaluated.rings.flat();
    const box = bboxOf(points);
    const elevation = { id: shape.id as string, bone: shape.bone as string, x0: box.minX, x1: box.maxX, y0: box.minY, y1: box.maxY };
    const frontMatch = Number.isFinite(box.minX) ? matchView("front", elevation, front) : { spans: [], ids: [] };
    const planMatch = Number.isFinite(box.minX) ? matchView("plan", elevation, plan) : { spans: [], ids: [] };
    const sameId = [...state.front, ...state.plan].filter((e) => e.shape.id === shape.id).map((e) => e.shape);
    for (const id of frontMatch.ids) consumed.add(`front:${id}`);
    for (const id of planMatch.ids) consumed.add(`plan:${id}`);
    if (shape.solid !== undefined) {
      for (const c of front) if (c.id === shape.id || (c.bone === shape.bone && related(c.id, shape.id))) consumed.add(`front:${c.id}`);
      for (const c of plan) if (c.id === shape.id || (c.bone === shape.bone && related(c.id, shape.id))) consumed.add(`plan:${c.id}`);
    }
    const clusters = cluster([...frontMatch.spans, ...planMatch.spans]);
    const mat = ctx.materialOf(shape, sameId);
    const common: Common = { id: shape.id, bone: shape.bone, material: mat.id, role: shape.role, tone: shape.tone };
    if (shape.region !== undefined) common.region = shape.region;
    if (shape.regions !== undefined) common.regions = shape.regions;
    let results: Drafted[];
    try {
      results = draftShape(ctx, shape, evaluated, shape.solid === undefined ? clusters : [], clusters.length === 0, common);
    } catch (error) {
      skipped.push({ shape: shape.id, reason: `error: ${(error as Error).message}` });
      flags.push({ code: "skipped", severity: "needs-author", subject: shape.id, message: `drafting failed: ${(error as Error).message}` });
      continue;
    }
    let any = false;
    for (const result of results) {
      for (const f of result.flags) flags.push(f);
      if (result.solid === null) {
        skipped.push({ shape: shape.id, reason: result.flags[0]?.message ?? "no solid" });
        continue;
      }
      any = true;
      result.solid.id = unique(result.solid.id);
      if (mat.guessed) flags.push({ code: "material-guessed", severity: "info", subject: result.solid.id, message: `material ${mat.id} taken from the region or the bone, not from the shape` });
      solids.push(result.solid);
      const hinted = shape.solid !== undefined;
      provenance.push({
        solid: result.solid.id,
        route: result.route,
        source: { view: "elevation", shape: shape.id, kind: shape.kind },
        lateral: hinted ? { z: null, views: ["hint"], shapes: [], confidence: "hinted" } : shape.kind === "member" ? { z: null, views: [], shapes: [], confidence: "n/a" } : clusters.length === 0 ? { z: null, views: [], shapes: [], confidence: "estimated" } : { z: [round5(Math.min(...clusters.map((c) => c.z0))), round5(Math.max(...clusters.map((c) => c.z1)))], views: [...new Set(clusters.flatMap((c) => c.spans.map((s) => s.view)))], shapes: clusters.flatMap((c) => c.spans.map((s) => `${s.view}:${s.id}`)), confidence: "measured" },
      });
    }
    if (any) drafted++;
  }

  const unmatched: DraftResult["report"]["unmatched"] = [];
  const loose = (view: "front" | "plan", list: readonly Candidate[]) => list.filter((c) => !consumed.has(`${view}:${c.id}`) && DRAFT_ROLES.has(c.evaluated.shape.role) && !state.elevation.some((e) => e.shape.id === c.id));
  const looseFront = loose("front", front);
  const loosePlan = loose("plan", plan);
  const pairs: { f?: Candidate; p?: Candidate }[] = [];
  const takenPlan = new Set<Candidate>();
  for (const f of looseFront) {
    const p = loosePlan.find((c) => !takenPlan.has(c) && c.id === f.id) ?? loosePlan.find((c) => !takenPlan.has(c) && c.bone === f.bone && related(c.id, f.id));
    if (p !== undefined) takenPlan.add(p);
    pairs.push({ f, p });
  }
  for (const p of loosePlan) if (!takenPlan.has(p)) pairs.push({ p });
  for (const { f, p } of pairs) {
    const lead = (f ?? p)!;
    for (const c of [f, p]) if (c !== undefined) {
      const view = c === f ? "front" : "plan";
      unmatched.push({ view, id: c.id, bone: c.bone, kind: c.evaluated.shape.kind, role: c.evaluated.shape.role });
    }
    const twin = f !== undefined ? (p === undefined ? plan.find((c) => c.id === f.id) : undefined) : front.find((c) => c.id === p!.id);
    const results = viewOnly(ctx, lead.evaluated.shape, f, p, twin, state.elevation, commonOf(ctx, lead.evaluated.shape, [f?.evaluated.shape, p?.evaluated.shape].filter((x): x is Json => x !== undefined)));
    for (const r of results) {
      for (const fl of r.flags) flags.push(fl);
      if (r.solid === null) {
        skipped.push({ shape: lead.id, reason: r.flags[0]?.message ?? "no solid" });
        continue;
      }
      r.solid.id = unique(r.solid.id);
      solids.push(r.solid);
      provenance.push({ solid: r.solid.id, route: r.route, source: { view: "elevation", shape: lead.id, kind: lead.evaluated.shape.kind }, lateral: { z: null, views: [f === undefined ? null : "front", p === undefined ? null : "plan"].filter((x): x is string => x !== null), shapes: [f === undefined ? null : `front:${f.id}`, p === undefined ? null : `plan:${p.id}`].filter((x): x is string => x !== null), confidence: f !== undefined && p !== undefined ? "measured" : "estimated" } });
    }
  }

  if (state.degraded !== undefined) flags.push({ code: "silhouette-degraded", severity: "low-confidence", subject: asset.id, message: `staffage evaluation of the authored solids failed (${state.degraded}); matching and IoU used the flat outlines of the shapes without their solid hints` });

  const volume = { status: "authored" as const, solids, decisions: [] as never[] };
  const result: DraftResult = {
    volume,
    report: {
      asset: asset.id,
      category: asset.category,
      scale,
      bboxM: { draft: null, dimensions: [asset.dimensions.widthM, asset.dimensions.heightM, asset.dimensions.depthM], relativeError: [1, 1, 1], axes: "[x ahead = widthM, y up = heightM, z lateral = depthM]" },
      counts: {
        elevationShapes: state.elevation.length,
        drafted,
        solids: solids.length,
        skipped: skipped.length,
        needsAuthor: flags.filter((f) => f.severity === "needs-author").length,
        lowConfidence: flags.filter((f) => f.severity === "low-confidence").length,
        unmatchedFront: unmatched.filter((u) => u.view === "front").length,
        unmatchedPlan: unmatched.filter((u) => u.view === "plan").length,
      },
      provenance,
      flags,
      unmatched,
      skipped,
    },
  };
  if (check) {
    const analysis = analyse(asset, volume, scale, state);
    result.report.bboxM = { ...result.report.bboxM, draft: analysis.bboxM === null ? null : (analysis.bboxM.map(round5) as V3), relativeError: analysis.bboxRelativeError.map((e) => Number(e.toFixed(4))) as V3 };
    result.report.silhouette = analysis.views;
  }
  return result;
}

type Args = { asset?: string; all: boolean; out: string; scale: number; check: boolean };

function parse(argv: readonly string[]): Args {
  const args: Args = { all: false, out: join(GENERATED, "drafts"), scale: 50, check: true };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i]!;
    if (a === "--all") args.all = true;
    else if (a === "--no-check") args.check = false;
    else if (a === "--asset") args.asset = argv[++i];
    else if (a === "--out") args.out = resolve(argv[++i]!);
    else if (a === "--scale") args.scale = Number(argv[++i]);
    else throw new Error(`unknown argument "${a}"; usage: volume-draft.ts (--asset <slug|category/slug|file> | --all) [--out <dir>] [--scale 50] [--no-check]`);
  }
  if (!args.all && args.asset === undefined) throw new Error("give --asset <slug|category/slug|file> or --all");
  return args;
}

export function summaryLine(entry: CatalogEntry, r: DraftResult): string {
  const s = r.report.silhouette;
  const iou = s === undefined ? "n/a" : `${s.elevation.iou.toFixed(2)}/${s.front.iou.toFixed(2)}/${s.plan.iou.toFixed(2)}`;
  const err = r.report.bboxM.relativeError.map((e) => `${(e * 100).toFixed(0)}%`).join("/");
  return `${entry.category}/${entry.slug}: ${r.report.counts.solids} solids, bbox err ${err}, IoU e/f/p ${iou}, needs-author ${r.report.counts.needsAuthor}, low-confidence ${r.report.counts.lowConfidence}, unmatched ${r.report.counts.unmatchedFront}+${r.report.counts.unmatchedPlan}`;
}

async function main(): Promise<void> {
  const args = parse(process.argv.slice(2));
  installCancellation();
  const entries = args.all ? catalogEntries() : [resolveAsset(args.asset!)];
  const index: Json[] = [];
  let done = 0;
  const started = Date.now();
  try {
    for (const entry of entries) {
      checkCancelled();
      process.stderr.write(`[${done}/${entries.length}] ${entry.category}/${entry.slug}\n`);
      const asset = loadAsset(entry.path);
      let result: DraftResult;
      try {
        result = draftAsset(asset, args.scale, args.check);
      } catch (error) {
        if (error instanceof CancelledError) throw error;
        process.stderr.write(`[error] ${entry.category}/${entry.slug}: ${(error as Error).stack ?? error}\n`);
        index.push({ category: entry.category, slug: entry.slug, error: String((error as Error).message) });
        done++;
        continue;
      }
      const dir = join(args.out, asset.category ?? entry.category);
      mkdirSync(dir, { recursive: true });
      const path = join(dir, `${entry.slug}.json`);
      writeFileSync(path, `${JSON.stringify(result)}\n`);
      process.stdout.write(`${summaryLine(entry, result)}\n`);
      index.push({ category: asset.category ?? entry.category, slug: entry.slug, path });
      done++;
      await yieldToLoop();
    }
  } catch (error) {
    if (!(error instanceof CancelledError)) throw error;
  }
  if (args.all) {
    mkdirSync(args.out, { recursive: true });
    writeFileSync(join(args.out, "index.json"), `${JSON.stringify({ scale: args.scale, assets: index }, null, 1)}\n`);
  }
  process.stderr.write(`[${cancel.requested ? "cancelled" : "done"} ${done}/${entries.length}] ${((Date.now() - started) / 1000).toFixed(1)} s\n`);
  if (cancel.requested) process.exit(130);
}

void median;

if (import.meta.main) await main();
