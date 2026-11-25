/** 🌳️ Draws the front and the plan of every foliage mass as ONE real body, as the last step of the chain of a plant: `bun loft.ts <id|file.json> [--save ops.json | --ops ops.json] [--out file] [--write] [--scales 50,100] [--only bone,bone] [--slack 0.04] [--step 16] [--no-fill]`.
 *
 * A foliage mass has a body already: the ellipsoids of its bone in the volume of each scale (`volumes.<scale>.body`), from whose silhouette its three outlines were drawn. The flat front and plan outline of a mass is one sheet at one depth and foreshortens when the wind turns its bone; the depth plates `mvc-*` and the sheets `-mvs` of `../MV-1/carry.ts` mend the extents but show cuts, notches and slivers. This tool replaces the closed body outline of the bone in the front and in the plan by one `loft` shape (same id, paint order, role, tone, region and material) whose body is the union of those ellipsoids, so the engine turns the body itself and every view is its true projection at every pose and clip instant, as one outline:
 * - an ellipsoid is a bead of parallel elliptic sections across the viewing direction (`loftsOf`): its equator, one section at the latitude the contour reaches at the largest turn of the bone over every subject of the matrix (poses, clip instants, transitions; further sections every `step` degrees when the bone turns more) on either side, and a point at each pole, so a bead costs the engine two to four convex hulls; ellipsoids that no turn of the bone shows beyond the others (by `slack` millimetres of paper) are left out (`needed`);
 * - a gap that the ellipsoids of a mass leave open inside its silhouette at some turn (the authored outline of the mass is drawn without such holes) is closed by a filling ellipsoid `fillers` puts behind it, as large as the gap and never seen beyond the silhouette of the mass at any turn; a hole the authored outline draws itself stays open;
 * - the beads of a mass hang on one thread of 0.6 micrometres, which the engine leaves out of the union. Ellipsoids that touch no other of their mass are a second loft `<id>-<n>`, which the depiction merges with the first (same bone and region);
 * - every other shape of the bone (open strokes) and the side elevation stay as authored: the elevation outline turns rigidly in its own plane, which is the true side view of the same body.
 * A bone is converted when it has ellipsoids at the scale and exactly one closed flat body outline in the view. The run prints, per scale, view and bone, how far the silhouette of the body is from the authored outline at rest (extent and largest boundary distance) and the time of one evaluation. `--save` writes the shapes as absolute values to an ops file; `--ops` applies such a file: it removes every `mvc-*` plate and every suffix `-mvs`, then sets the listed shapes in the place of the shape of the same bone and id, so the step is idempotent and runs after `../MV-1/make.sh` whatever that step wrote.
 * @see ../MV-1/carry.ts — the plates this tool replaces
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts — `loftRings`, the sections the engine draws
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { loftShapeIssues } = await import(join(S, "🔨️modules", "🗂️catalog", "🧊️volume", "🟦️.ts"));
const { evaluate, flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { unionPolygons } = await import(join(S, "🔨️modules", "🔺️polygon", "🟦️.ts"));
const { project, lookingOf } = await import(join(S, "🔨️modules", "🎨️depiction", "🔭️views", "🟦️.ts"));

type View = "front" | "plan";
type V3 = [number, number, number];
type Ellipsoid = { id: string; c: V3; a: [V3, V3, V3] };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const only = flag("only")?.split(",");
const scales = flag("scales")?.split(",");
const SLACK = Number(flag("slack") ?? 0.04);
const tolerance: number = engine.toleranceM(asset);
const PLATE = "mvc-";
const SHEET = "-mvs";
const COMMON = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const VIEWS: readonly View[] = ["front", "plan"];
const SEGMENTS = 32;
const THREAD = 3e-7;
const TURN = 2e-5;
const STEP = Number(flag("step") ?? 16);
const FILL = !args.includes("--no-fill");
const FILL_ROUNDS = 6;
const FACETS = 48;
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const rest = restFrames(asset.rig) as Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const mm = (m: number): string => (m * 1000).toFixed(0);
const baseId = (id: string): string => (id.endsWith(SHEET) ? id.slice(0, -SHEET.length) : id);
const sheetOf = (shape: any): boolean => shape.kind !== "loft" && shape.kind !== "member" && shape.solid === undefined && shape.role === "body" && shape.filled === true && flattenGeometry(shape).closed === true;

/** 🧹️ The geometry without the depth plates and without the sheet suffix. */
function strip(geometry: any): void {
  for (const view of VIEWS) {
    geometry[view] = geometry[view].filter((shape: any) => !String(shape.id).startsWith(PLATE));
    for (const shape of geometry[view]) if (String(shape.id).endsWith(SHEET)) shape.id = baseId(shape.id);
  }
}

/** 🧭️ The unit quaternion of the rotation with the columns `x`, `y`, `z`. */
function quaternion(x: V3, y: V3, z: V3): number[] {
  const [m00, m10, m20] = x, [m01, m11, m21] = y, [m02, m12, m22] = z;
  const trace = m00 + m11 + m22;
  let q: number[];
  if (trace > 0) { const s = 2 * Math.sqrt(trace + 1); q = [(m21 - m12) / s, (m02 - m20) / s, (m10 - m01) / s, s / 4]; }
  else if (m00 > m11 && m00 > m22) { const s = 2 * Math.sqrt(1 + m00 - m11 - m22); q = [s / 4, (m01 + m10) / s, (m02 + m20) / s, (m21 - m12) / s]; }
  else if (m11 > m22) { const s = 2 * Math.sqrt(1 + m11 - m00 - m22); q = [(m01 + m10) / s, s / 4, (m12 + m21) / s, (m02 - m20) / s]; }
  else { const s = 2 * Math.sqrt(1 + m22 - m00 - m11); q = [(m02 + m20) / s, (m12 + m21) / s, s / 4, (m10 - m01) / s]; }
  const n = Math.hypot(...q);
  return q.map((c) => Math.round((c / n) * 1e9) / 1e9);
}

/** 🥚️ An ellipsoid of the volume in the frame of its bone: its centre and the columns of its axes times its radii. */
function ellipsoidOf(solid: any): Ellipsoid {
  const [x, y, z, w] = (solid.transform?.rotation ?? [0, 0, 0, 1]) as [number, number, number, number];
  const scale = (solid.transform?.scale ?? [1, 1, 1]) as V3;
  const rotate = (p: V3): V3 => [
    (1 - 2 * (y * y + z * z)) * p[0] + 2 * (x * y - z * w) * p[1] + 2 * (x * z + y * w) * p[2],
    2 * (x * y + z * w) * p[0] + (1 - 2 * (x * x + z * z)) * p[1] + 2 * (y * z - x * w) * p[2],
    2 * (x * z - y * w) * p[0] + 2 * (y * z + x * w) * p[1] + (1 - 2 * (x * x + y * y)) * p[2],
  ];
  const [r0, r1, r2] = solid.radii as V3;
  return { id: solid.id, c: (solid.transform?.translation ?? [0, 0, 0]) as V3, a: [rotate([r0 * scale[0], 0, 0]), rotate([0, r1 * scale[1], 0]), rotate([0, 0, r2 * scale[2]])] };
}

const dot = (p: V3, q: V3): number => p[0] * q[0] + p[1] * q[1] + p[2] * q[2];
const plus = (p: V3, q: V3, t = 1): V3 => [p[0] + q[0] * t, p[1] + q[1] * t, p[2] + q[2] * t];
const minus = (p: V3, q: V3): V3 => plus(p, q, -1);
const times = (p: V3, t: number): V3 => [p[0] * t, p[1] * t, p[2] * t];
const unit = (p: V3): V3 => times(p, 1 / Math.hypot(...p));
const cross = (p: V3, q: V3): V3 => [p[1] * q[2] - p[2] * q[1], p[2] * q[0] - p[0] * q[2], p[0] * q[1] - p[1] * q[0]];

/** 🎯️ How deep a point lies in an ellipsoid: 1 at the centre, 0 on the surface, negative outside. */
function depthIn(e: Ellipsoid, p: V3): number {
  const d = minus(p, e.c);
  return 1 - Math.hypot(...e.a.map((axis) => dot(d, axis) / dot(axis, axis)));
}

/** 🤝️ The point that lies deepest inside two ellipsoids (a walk that shrinks its step from the line between the centres), or nothing when they do not overlap. */
function meeting(a: Ellipsoid, b: Ellipsoid): V3 | undefined {
  const depth = (p: V3): number => Math.min(depthIn(a, p), depthIn(b, p));
  let best: V3 = a.c, value = depth(best);
  for (let i = 0; i <= 40; i++) { const p = plus(a.c, minus(b.c, a.c), i / 40); if (depth(p) > value) { best = p; value = depth(p); } }
  const directions: V3[] = [...a.a, ...b.a].flatMap((axis) => [axis, times(axis, -1)]);
  for (let step = 0.3; step > 0.004; ) {
    const better = directions.map((d) => plus(best, d, step)).find((p) => depth(p) > value + 1e-9);
    if (better === undefined) step /= 2;
    else { best = better; value = depth(better); }
  }
  return value > 0.01 ? best : undefined;
}

type Conic = { x: number; y: number; xx: number; xy: number; yy: number };

/** 🔦️ The ellipse an ellipsoid is seen as, after a turn of `turn` radians about the lateral axis, in the directions `across` and `up` of the view (both in the frame of the bone, `u` and `v` the sagittal plane). */
function seen(e: Ellipsoid, across: V3, up: V3, turn: number): Conic {
  const [c, s] = [Math.cos(turn), Math.sin(turn)];
  const back = (p: V3): V3 => [c * p[0] + s * p[1], -s * p[0] + c * p[1], p[2]];
  const [x, y] = [back(across), back(up)];
  let xx = 0, xy = 0, yy = 0;
  for (const axis of e.a) { const p = dot(axis, x), q = dot(axis, y); xx += p * p; xy += p * q; yy += q * q; }
  return { x: dot(e.c, x), y: dot(e.c, y), xx, xy, yy };
}

/** 🕳️ The ellipsoids a mass needs to be seen as it is: the smallest first, an ellipsoid is dropped when at every turn of the bone its whole outline and its middle lie in the outlines of the kept ones, `slack` metres allowed. */
function needed(list: readonly Ellipsoid[], across: V3, up: V3, turns: readonly number[], slack: number): Ellipsoid[] {
  const conics = new Map(list.map((e) => [e, turns.map((turn) => seen(e, across, up, turn))]));
  const size = (e: Ellipsoid): number => { const k = conics.get(e)![0]!; return k.xx * k.yy - k.xy * k.xy; };
  const holds = (k: Conic, x: number, y: number): boolean => {
    const det = k.xx * k.yy - k.xy * k.xy, dx = x - k.x, dy = y - k.y;
    const value = Math.sqrt((k.yy * dx * dx - 2 * k.xy * dx * dy + k.xx * dy * dy) / det);
    return value <= 1 + slack / Math.sqrt(Math.min(k.xx, k.yy));
  };
  const kept = new Set(list);
  for (const e of [...list].sort((p, q) => size(p) - size(q))) {
    const rest = [...kept].filter((other) => other !== e);
    const hidden = conics.get(e)!.every((k, n) => {
      const l00 = Math.sqrt(k.xx), l10 = k.xy / l00, l11 = Math.sqrt(Math.max(k.yy - l10 * l10, 0));
      for (const radius of [1, 0.7, 0.35, 0]) for (let i = 0; i < 40; i++) {
        const [p, q] = [radius * Math.cos((i * Math.PI) / 20), radius * Math.sin((i * Math.PI) / 20)];
        if (!rest.some((other) => holds(conics.get(other)![n]!, k.x + l00 * p, k.y + l10 * p + l11 * q))) return false;
      }
      return true;
    });
    if (hidden) kept.delete(e);
  }
  return list.filter((e) => kept.has(e));
}

type P2 = { x: number; y: number };
const areaOf = (ring: readonly P2[]): number => ring.reduce((sum, p, i) => sum + p.x * ring[(i + 1) % ring.length]!.y - ring[(i + 1) % ring.length]!.x * p.y, 0) / 2;
const inside = (p: P2, ring: readonly P2[]): boolean => {
  let hit = false;
  for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
    const [a, b] = [ring[i]!, ring[j]!];
    if (a.y > p.y !== b.y > p.y && p.x < ((b.x - a.x) * (p.y - a.y)) / (b.y - a.y) + a.x) hit = !hit;
  }
  return hit;
};
const middle = (ring: readonly P2[]): P2 => ({ x: ring.reduce((sum, p) => sum + p.x, 0) / ring.length, y: ring.reduce((sum, p) => sum + p.y, 0) / ring.length });

/** 🍩️ The rings of a union sorted into outlines and holes: a ring is a hole when one of its points lies inside an odd number of the other rings. */
function sorted(rings: readonly P2[][]): { outlines: P2[][]; holes: P2[][] } {
  const outlines: P2[][] = [], holes: P2[][] = [];
  for (const ring of rings) (rings.filter((other) => other !== ring && inside(ring[0]!, other)).length % 2 === 1 ? holes : outlines).push(ring);
  return { outlines, holes };
}

/** ⭕️ The outline an ellipsoid is seen as: `FACETS` points of its {@link seen} ellipse. */
function facets(k: Conic): P2[] {
  const l00 = Math.sqrt(k.xx), l10 = k.xy / l00, l11 = Math.sqrt(Math.max(k.yy - l10 * l10, 0));
  return Array.from({ length: FACETS }, (_, i) => { const [p, q] = [Math.cos((2 * Math.PI * i) / FACETS), Math.sin((2 * Math.PI * i) / FACETS)]; return { x: k.x + l00 * p, y: k.y + l10 * p + l11 * q }; });
}

/** 🩹️ The filling ellipsoids of a mass in one view: at every turn of the bone the silhouette of the ellipsoids is united; a hole of it (not one the authored outline draws: `open` tells for a point of the rest drawing) gets an ellipsoid at the depth of the ellipsoids around it, as wide as the hole plus a tenth along its two principal directions and as deep as it is narrow. A filling that would be seen beyond the silhouette of the mass at any turn is shrunk by a sixth up to five times and then given up; the rounds end when no turn shows a hole. Returns the fillings and the number of holes left. */
function fillers(list: readonly Ellipsoid[], across: V3, up: V3, n: V3, turns: readonly number[], pad: number, open: (x: number, y: number) => boolean): { fills: Ellipsoid[]; left: number } {
  const axes = (turn: number): [V3, V3, V3] => { const [c, s] = [Math.cos(turn), Math.sin(turn)]; const back = (p: V3): V3 => [c * p[0] + s * p[1], -s * p[0] + c * p[1], p[2]]; return [back(across), back(up), back(n)]; };
  const outer = turns.map((turn) => sorted(unionPolygons(list.map((e) => facets(seen(e, across, up, turn))))).outlines);
  const shown = (e: Ellipsoid): boolean => turns.some((turn, t) => facets(seen(e, across, up, turn)).some((p) => !outer[t]!.some((ring) => inside(p, ring))));
  const fills: Ellipsoid[] = [];
  const given = new Set<string>();
  let left = 0;
  for (let round = 0; round < FILL_ROUNDS; round++) {
    left = 0;
    let added = 0;
    for (const [t, turn] of turns.entries()) {
      const all = [...list, ...fills];
      const conics = all.map((e) => seen(e, across, up, turn));
      const [x, y, z] = axes(turn);
      for (const hole of sorted(unionPolygons(conics.map(facets))).holes) {
        const m = middle(hole);
        const key = `${t}:${m.x.toFixed(3)}:${m.y.toFixed(3)}`;
        if (given.has(key)) { left++; continue; }
        let sxx = 0, sxy = 0, syy = 0;
        for (const p of hole) { sxx += (p.x - m.x) ** 2; sxy += (p.x - m.x) * (p.y - m.y); syy += (p.y - m.y) ** 2; }
        const angle = Math.atan2(2 * sxy, sxx - syy) / 2;
        const [d1, d2] = [{ x: Math.cos(angle), y: Math.sin(angle) }, { x: -Math.sin(angle), y: Math.cos(angle) }];
        const reach = (d: P2): number => Math.max(...hole.map((p) => Math.abs((p.x - m.x) * d.x + (p.y - m.y) * d.y))) * 1.1 + pad;
        const [h1, h2] = [reach(d1), reach(d2)];
        const around = all.filter((_, i) => { const k = conics[i]!; const det = k.xx * k.yy - k.xy * k.xy; return hole.some((p) => Math.abs(Math.sqrt((k.yy * (p.x - k.x) ** 2 - 2 * k.xy * (p.x - k.x) * (p.y - k.y) + k.xx * (p.y - k.y) ** 2) / det) - 1) < 0.04); });
        const depth = (around.length > 0 ? around : all).reduce((sum, e, _, group) => sum + dot(e.c, z) / group.length, 0);
        const c = plus(plus(times(x, m.x), y, m.y), z, depth);
        const make = (f: number): Ellipsoid => ({ id: `fill-${fills.length + 1}`, c, a: [times(plus(times(x, d1.x), y, d1.y), h1 * f), times(plus(times(x, d2.x), y, d2.y), h2 * f), times(z, Math.min(h1, h2) * f)] });
        const rest0 = { x: dot(c, across), y: dot(c, up) };
        let fill: Ellipsoid | undefined;
        if (!open(rest0.x, rest0.y)) for (let f = 1, tries = 0; tries < 6 && fill === undefined; f *= 5 / 6, tries++) if (!shown(make(f))) fill = make(f);
        if (fill === undefined) { given.add(key); left++; continue; }
        fills.push(fill);
        added++;
      }
    }
    if (added === 0) break;
  }
  left = turns.reduce((sum, turn) => sum + sorted(unionPolygons([...list, ...fills].map((e) => facets(seen(e, across, up, turn))))).holes.length, 0);
  return { fills, left };
}

/** 🧵️ The lofts of the ellipsoids of one mass in one view, `n` the viewing direction in the frame of the bone: every ellipsoid is a bead of parallel elliptic sections across the viewing direction: the equator, sections up to the latitude its contour reaches at the largest of the `turns` of the bone (three degrees more, at least four, a section every `STEP` degrees) and a point at each pole; the beads of a mass hang on one thread of 0.6 micrometres, which the union of the engine leaves out, through a point common to every two beads that touch. Every turn of the thread is at most a right angle between two of its points, so no section folds over the next. Beads that touch no other bead of their mass are a loft of their own. */
function loftsOf(ellipsoids: readonly Ellipsoid[], n: V3, turns: readonly number[]): any[][] {
  const bead = (e: Ellipsoid): any[] => {
    const m0 = e.a.map((axis) => dot(axis, n)) as V3;
    const reach = Math.hypot(...m0);
    const m = times(m0, 1 / reach);
    const out = (y: V3): V3 => plus(plus(times(e.a[0], y[0]), e.a[1], y[1]), e.a[2], y[2]);
    const e1 = unit(Math.abs(m[0]) < 0.8 ? cross(m, [1, 0, 0]) : cross(m, [0, 1, 0]));
    const [p, q] = [out(e1), out(cross(m, e1))];
    const t = Math.atan2(2 * dot(p, q), dot(p, p) - dot(q, q)) / 2;
    const [major, minor] = [plus(times(p, Math.cos(t)), q, Math.sin(t)), plus(times(p, -Math.sin(t)), q, Math.cos(t))];
    const [x, y] = [unit(major), unit(minor)];
    const rotation = quaternion(x, y, cross(x, y));
    const grow = 2 / (1 + Math.cos(Math.PI / SEGMENTS));
    const shift = out(m);
    const reached = Math.max(...turns.map((turn) => { const [c, s] = [Math.cos(turn), Math.sin(turn)]; const mt = unit(e.a.map((axis) => dot(axis, [c * n[0] + s * n[1], -s * n[0] + c * n[1], n[2]])) as V3); return (Math.acos(Math.max(-1, Math.min(1, dot(m, mt)))) * 180) / Math.PI; }));
    const widest = Math.min(80, Math.max(4, reached + 3));
    const count = Math.ceil(widest / STEP);
    const rings = [-90, ...Array.from({ length: 2 * count + 1 }, (_, i) => (widest * (i - count)) / count), 90];
    return rings.map((degrees) => {
      const phi = (degrees * Math.PI) / 180;
      const around = Math.abs(degrees) >= 90 ? undefined : Math.cos(phi) * grow;
      return { at: plus(e.c, shift, Math.sin(phi)).map(round), rx: around === undefined ? THREAD : round(Math.hypot(...major) * around), ry: around === undefined ? THREAD : round(Math.hypot(...minor) * around), rotation };
    });
  };
  const fine = (p: V3): number[] => p.map((value) => Math.round(value * 1e7) / 1e7);
  const aside = (d: V3): V3 => unit(Math.abs(dot(d, [0, 0, 1])) < 0.9 ? cross(d, [0, 0, 1]) : cross(d, [1, 0, 0]));
  const turned = (from: V3, d1: V3, d2: V3): V3[] => {
    const angle = Math.acos(Math.max(-1, Math.min(1, dot(d1, d2))));
    const steps = Math.max(1, Math.ceil(angle / (Math.PI / 3)));
    const side = Math.PI - angle < 0.02 ? aside(d1) : angle < 0.02 ? d1 : unit(minus(d2, times(d1, dot(d1, d2))));
    const list: V3[] = [];
    let p = from;
    for (let i = 0; i <= steps; i++) { p = plus(p, plus(times(d1, Math.cos((angle * i) / steps)), side, Math.sin((angle * i) / steps)), TURN); list.push(p); }
    return list;
  };
  const left = new Set(ellipsoids.map((_, index) => index));
  const lofts: any[][] = [];
  const weight = (e: Ellipsoid): number => Math.abs(dot(e.a[0], cross(e.a[1], e.a[2])));
  while (left.size > 0) {
    const root = [...left].sort((p, q) => weight(ellipsoids[q]!) - weight(ellipsoids[p]!))[0]!;
    const sections: any[] = [];
    let heading: V3 = n;
    const last = (): V3 => sections[sections.length - 1]!.at as V3;
    const thread = (p: V3): void => { sections.push({ at: fine(p), rx: THREAD, ry: THREAD }); };
    const walk = (to: V3, then: V3): void => {
      for (const p of turned(last(), heading, unit(minus(to, last())))) thread(p);
      heading = unit(minus(to, last()));
      thread(plus(to, heading, -10 * TURN));
      for (const p of turned(last(), heading, then)) thread(p);
      heading = then;
    };
    const visit = (index: number): void => {
      left.delete(index);
      const e = ellipsoids[index]!;
      const run = bead(e);
      if (sections.length > 0) walk(plus(run[0]!.at, n, -40 * TURN), n);
      sections.push(...run);
      heading = n;
      const links = [...left].map((other) => ({ other, p: meeting(e, ellipsoids[other]!) })).filter((link) => link.p !== undefined);
      for (const { other, p } of links) {
        if (!left.has(other)) continue;
        const aim = unit(minus(p!, last()));
        walk(p!, aim);
        visit(other);
        if ([...left].length === 0) continue;
        walk(p!, unit(minus(p!, last())));
      }
    };
    visit(root);
    while (sections.length > 0 && sections[sections.length - 1]!.rotation === undefined) sections.pop();
    lofts.push(sections);
  }
  return lofts;
}

/** 🔧️ The ops applied to the asset: plates and suffixes removed, the listed shapes set in the place of the shape of their bone and id (further lofts of a mass after its first). */
function applyOps(target: any, ops: any): number {
  let count = 0;
  for (const key of Object.keys(target.representations)) {
    const geometry = target.representations[key].geometry;
    if (geometry === undefined) continue;
    strip(geometry);
    for (const view of VIEWS) {
      const wanted: any[] = ops[key]?.[view] ?? [];
      for (const shape of wanted) {
        const at = geometry[view].findIndex((known: any) => known.bone === shape.bone && known.id === shape.id);
        const after = geometry[view].findLastIndex((known: any) => known.bone === shape.bone && known.kind === "loft");
        if (at >= 0) geometry[view][at] = structuredClone(shape);
        else if (after >= 0) geometry[view].splice(after + 1, 0, structuredClone(shape));
        else throw new Error(`${target.id} 1:${key} ${view}: no shape "${shape.id}" on bone "${shape.bone}" to set`);
        count++;
      }
    }
  }
  return count;
}

const opsFile = flag("ops");
const coarsest = authored.map(Number).sort((a, b) => b - a)[0]!;
const states: any[] = opsFile !== undefined ? [] : engine.subjects(asset, coarsest).map((subject: any) => evaluate(asset, subject.query, engine.scaleSelector(coarsest)));
let result: any;
if (opsFile !== undefined) {
  result = structuredClone(asset);
  console.log(`${asset.id}: ${applyOps(result, JSON.parse(readFileSync(opsFile, "utf8")))} shapes set from ${opsFile}`);
} else {
  result = structuredClone(asset);
  const ops: any = {};
  const turns = new Map<string, number[]>();
  for (const bone of asset.rig.bones) {
    const r = rest[bone.id]!.frame;
    let low = 0, high = 0, askew = 0;
    for (const state of states) {
      const { u, w } = state.bones[bone.id].frame;
      const angle = Math.atan2(u.x * r.v[0] + u.y * r.v[1] + u.z * r.v[2], u.x * r.u[0] + u.y * r.u[1] + u.z * r.u[2]);
      low = Math.min(low, angle);
      high = Math.max(high, angle);
      askew = Math.max(askew, Math.acos(Math.min(1, w.x * r.w[0] + w.y * r.w[1] + w.z * r.w[2])));
    }
    const steps = Math.max(1, Math.ceil((high - low) / 0.05));
    turns.set(bone.id, [0, ...Array.from({ length: steps + 1 }, (_, i) => low + ((high - low) * i) / steps)]);
    if (bone.id.startsWith("fol")) console.log(`turn ${bone.id}: ${((low * 180) / Math.PI).toFixed(1)} to ${((high * 180) / Math.PI).toFixed(1)} degrees over ${states.length} subjects, lateral axis off by at most ${((askew * 180) / Math.PI).toFixed(2)} degrees`);
  }
  const made: { key: string; view: View; bone: string; id: string; sheet: any }[] = [];
  for (const key of authored) {
    const geometry = result.representations[key].geometry;
    strip(geometry);
    const body: any[] = asset.volumes?.[key]?.body ?? [];
    const slack = (SLACK / 1000) * Number(key);
    for (const view of VIEWS) {
      for (const bone of [...new Set<string>(geometry[view].map((shape: any) => shape.bone))]) {
        if (only !== undefined && !only.includes(bone)) continue;
        const list: any[] = geometry[view];
        const ellipsoids = body.filter((solid) => solid.bone === bone && solid.kind === "ellipsoid" && solid.weights === undefined);
        const sheets = list.filter((shape) => shape.bone === bone && sheetOf(shape));
        const lofted = list.filter((shape) => shape.bone === bone && shape.kind === "loft");
        if (ellipsoids.length === 0 || body.some((solid) => solid.bone === bone && solid.kind !== "ellipsoid" && solid.role === "body") || (sheets.length !== 1 && lofted.length === 0)) continue;
        const f = rest[bone]!.frame;
        const local = (d: V3): V3 => [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
        const paper = [{ x: 1, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 0, y: 0, z: 1 }].map((axis) => project(view, axis));
        const looking = lookingOf(view);
        const [n, across, up] = [[looking.x, looking.y, looking.z], paper.map((p) => p.x), paper.map((p) => p.y)].map((d) => local(d as V3)) as [V3, V3, V3];
        const all = ellipsoids.map(ellipsoidOf);
        const beads = needed(all, across, up, turns.get(bone)!, slack);
        const first = sheets[0] ?? lofted[0]!;
        const origin = project(view, { x: f.o[0], y: f.o[1], z: f.o[2] });
        const drawn: P2[][] = sheets.length === 1 ? flattenGeometry(sheets[0]).rings : [];
        const open = (x: number, y: number): boolean => sorted(drawn).holes.some((ring) => inside({ x: x + origin.x, y: y + origin.y }, ring));
        const filled = FILL ? fillers(beads, across, up, n, turns.get(bone)!, slack, open) : { fills: [], left: 0 };
        const lofts = loftsOf([...beads, ...filled.fills], n, turns.get(bone)!);
        const shapes = lofts.map((sections, n) => ({ ...Object.fromEntries(Object.keys(first).filter((name) => COMMON.includes(name)).map((name) => [name, first[name]])), id: n === 0 ? first.id : `${first.id}-${n + 1}`, kind: "loft", sections }));
        for (const shape of shapes) loftShapeIssues(shape as any, `${key} ${view} ${bone}`, (path: string, code: string, en: string) => console.log(`ISSUE ${code} ${path} ${en}`));
        const kept = list.filter((shape) => shape !== first && !lofted.includes(shape));
        kept.splice(list.slice(0, list.indexOf(first)).filter((shape) => kept.includes(shape)).length, 0, ...shapes);
        geometry[view] = kept;
        ((ops[key] ??= {})[view] ??= []).push(...shapes);
        if (sheets.length === 1) made.push({ key, view, bone, id: first.id, sheet: sheets[0] });
        console.log(`${key} ${view} ${bone}: ${ellipsoids.length} ellipsoids, ${beads.length} drawn + ${filled.fills.length} filling${filled.left > 0 ? `, ${filled.left} HOLES LEFT` : ""} -> ${lofts.length} loft${lofts.length > 1 ? "s" : ""} "${first.id}" with ${lofts.map((sections) => sections.length).join(" + ")} sections`);
      }
    }
  }
  for (const key of authored) {
    const selector = engine.scaleSelector(Number(key));
    const pose = { kind: "pose", pose: asset.presentation.defaultPose };
    const t0 = performance.now();
    const state = evaluate(result, pose, selector);
    const shapes = { front: state.shapes.front, plan: state.shapes.plan };
    const t1 = performance.now();
    const timed = asset.poses.map((entry: any) => { const t = performance.now(); const posed = evaluate(result, { kind: "pose", pose: entry.id }, selector); const rings = [...posed.shapes.front, ...posed.shapes.plan].filter((shape: any) => shape.shape.kind === "loft").map((shape: any) => shape.rings.length); return `${entry.id} ${(performance.now() - t).toFixed(0)} ms, ${rings.reduce((a: number, b: number) => a + b, 0) - rings.length} holes or parts`; });
    console.log(`1:${key}: ${timed.join("; ")}`);
    let worst = { extent: 0, boundary: 0, at: "" };
    for (const item of made.filter((entry) => entry.key === key)) {
      const rings: { x: number; y: number }[][] = shapes[item.view].filter((shape: any) => shape.shape.bone === item.bone && shape.shape.kind === "loft").flatMap((shape: any) => shape.rings);
      const drawn: { x: number; y: number }[][] = flattenGeometry(item.sheet).rings;
      const box = (list: { x: number; y: number }[][]): number[] => { const p = list.flat(); return [Math.min(...p.map((q) => q.x)), Math.max(...p.map((q) => q.x)), Math.min(...p.map((q) => q.y)), Math.max(...p.map((q) => q.y))]; };
      const [a, b] = [box(rings), box(drawn)];
      const extent = Math.max(...a.map((value, i) => Math.abs(value - b[i]!)));
      const distance = (p: { x: number; y: number }, list: { x: number; y: number }[][]): number => {
        let best = Infinity;
        for (const ring of list) for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
          const [u, v] = [ring[i]!, ring[j]!];
          const [dx, dy] = [v.x - u.x, v.y - u.y];
          const t = dx * dx + dy * dy === 0 ? 0 : Math.max(0, Math.min(1, ((p.x - u.x) * dx + (p.y - u.y) * dy) / (dx * dx + dy * dy)));
          best = Math.min(best, Math.hypot(p.x - u.x - t * dx, p.y - u.y - t * dy));
        }
        return best;
      };
      const boundary = Math.max(...drawn.flat().map((p) => distance(p, rings)), ...rings.flat().map((p) => distance(p, drawn)));
      console.log(`rest ${key} ${item.view} ${item.bone}: silhouette of the body against the authored outline: extent ${mm(extent)} mm, boundary ${mm(boundary)} mm, ${rings.length} ring${rings.length === 1 ? "" : "s"} (authored ${drawn.length})`);
      if (boundary > worst.boundary) worst = { extent, boundary, at: `${item.view} ${item.bone}` };
    }
    console.log(`1:${key}: one evaluation of front and plan ${(t1 - t0).toFixed(0)} ms; worst boundary distance at rest ${mm(worst.boundary)} mm (${worst.at})`);
  }
  const save = flag("save");
  if (save !== undefined) {
    writeFileSync(save, `${JSON.stringify(ops)}\n`);
    console.log(`${asset.id}: ops saved to ${save}`);
  }
}
let plates = 0, sheets = 0, lofts = 0;
for (const key of Object.keys(result.representations)) for (const view of VIEWS) for (const shape of result.representations[key].geometry?.[view] ?? []) { if (String(shape.id).startsWith(PLATE)) plates++; else if (String(shape.id).endsWith(SHEET)) sheets++; else if (shape.kind === "loft") lofts++; }
console.log(`${asset.id}: tolerance ${mm(tolerance)} mm; ${plates} depth plates, ${sheets} sheets carried by the silhouette, ${lofts} lofts`);
next.asset = result;
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
