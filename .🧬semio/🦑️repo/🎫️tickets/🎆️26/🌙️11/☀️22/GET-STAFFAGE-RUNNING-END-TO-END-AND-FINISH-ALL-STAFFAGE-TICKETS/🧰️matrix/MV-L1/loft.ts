/** 🌳️ Draws every foliage mass as ONE real body in the three views, as the last step of the chain of a plant: `bun loft.ts <id|file.json> [--save ops.json | --ops ops.json] [--out file] [--write] [--scales 50,100] [--only bone,bone] [--slack 0.04] [--views elevation,front,plan] [--step 16] [--speck 1] [--no-fill]`.
 *
 * A foliage mass has a body already: the ellipsoids of its bone in the volume of each scale (`volumes.<scale>.body`), from whose silhouette its three outlines were drawn. The flat front and plan outline of a mass is one sheet at one depth and foreshortens when the wind turns its bone; the depth plates `mvc-*` and the sheets `-mvs` of `../MV-1/carry.ts` mend the extents but show cuts, notches and slivers. This tool replaces the closed body outline of the bone in the side elevation, the front and the plan (`--views` for fewer) by one `loft` shape per view (same id, paint order, role, tone, region and material) whose body is the union of those ellipsoids, so the engine turns the body itself and every view is its true projection at every pose and clip instant, as one outline:
 * - an ellipsoid is a bead of parallel elliptic sections across the viewing direction (`loftsOf`): its equator, one section at the latitude the contour reaches at the largest turn of the bone over every subject of the matrix (poses, clip instants, transitions; further sections every `step` degrees when the bone turns more) on either side, and a point at each pole, so a bead costs the engine two to four convex hulls; ellipsoids that no turn of the bone shows beyond the others (by `slack` millimetres of paper) are left out (`needed`);
 * - a gap that the ellipsoids of a mass leave open inside its silhouette at some turn is closed by a filling ellipsoid `fillers` puts into it, as large as the gap and never seen beyond the silhouette of the mass at any turn; a hole the authored outline draws itself stays as drawn in the default pose;
 * - the beads of a mass hang on one thread of 0.6 micrometres, which the engine leaves out of the union. Ellipsoids that touch no other of their mass hang on the same thread (a second loft of the same mass would draw its own outline inside the first);
 * - after that the engine itself draws every subject of the matrix: a hole of a loft smaller than `speck` millimetres of paper (the engine draws 32 points per section, so three ellipsoids that just meet can leave a dot) gets a ball of at least `BALL` millimetres of paper radius (the dot between three ellipsoids that just meet wanders with the turn, a ball of its own size would be needed at every instant; at most `SPECK_MOST` per mass and view; at a depth of the mass where it is not seen beyond the silhouette of the mass at any turn and lies in no authored opening at rest; a pinhole of at most `PIN` millimetres of paper that finds no such place takes a ball of its own size that only touches the edge of an authored opening at rest), up to five passes;
 * - every other shape of the bone (open strokes) stays as authored. The side elevation takes the loft too (README "Loft shapes": the same body stands in the three views; since D66 of 2026-11-25 a loft of the elevation is the depth twin of the flat strokes of its bone): an authored elevation outline is up to the generalisation of the scale away from the body, and the three lofts agree by construction.
 * A bone is converted when it has ellipsoids at the scale and exactly one closed flat body outline in the view; the plates and sheets of every other bone (limbs) stay as they are. The run prints, per scale, the time of one evaluation, the holes the engine still draws how far the extent of the body is from the authored outline in the default pose, and `LOST` for a mass the engine draws smaller than its ellipsoids reach there (it must not appear). `--save` writes the shapes as absolute values to an ops file; `--ops` applies such a file: for every bone it lists it removes the `mvc-*` plates of that bone, then sets the listed lofts in the place of the body outline of the same bone and id (and removes any other loft of the bone), so the step is idempotent and runs after `../MV-1/make.sh` whatever that step wrote.
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

type View = "elevation" | "front" | "plan";
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
const VIEWS: readonly View[] = (flag("views")?.split(",") as View[] | undefined) ?? ["elevation", "front", "plan"];
const SEGMENTS = 32;
const THREAD = 3e-7;
const TURN = 2e-5;
const STEP = Number(flag("step") ?? 16);
const FILL = !args.includes("--no-fill");
const FILL_ROUNDS = 6;
const FILL_MOST = 40;
const FACETS = 48;
const LEAST = 4;
const SPECK_MOST = 40;
const PIN = 0.5;
const BALL = 0.5;
const SPECK = Number(flag("speck") ?? 1);
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const rest = restFrames(asset.rig) as Record<string, { frame: { o: V3; u: V3; v: V3; w: V3 } }>;
const round = (n: number): number => Math.round(n * 1e5) / 1e5;
const mm = (m: number): string => (m * 1000).toFixed(0);
const baseId = (id: string): string => (id.endsWith(SHEET) ? id.slice(0, -SHEET.length) : id);
const sheetOf = (shape: any): boolean => shape.kind !== "loft" && shape.kind !== "member" && shape.solid === undefined && shape.role === "body" && shape.filled === true && flattenGeometry(shape).closed === true;

/** 🧹️ One view of a geometry without the depth plates of a bone; the plates of every other bone (limbs) stay, and so does the suffix `-mvs` of the leaf strokes of the bone: a stroke without a twin of its id is carried at the depth of the elevation silhouette of its bone vertex by vertex, a stroke with one at a single depth (`field-maple` 1:50 `storm`: the strokes of `fol-limb-c1` 355 mm beyond the mass in the plan without the suffix). */
function strip(geometry: any, view: View, bone: string): void {
  geometry[view] = geometry[view].filter((shape: any) => !(shape.bone === bone && String(shape.id).startsWith(PLATE)));
}

/** 📌️ Sets the lofts of a mass in a view: the plates of the bone go, the lofts stand where the body outline with the id of the first one stood (with or without the sheet suffix), and every other loft of the bone goes. */
function place(geometry: any, view: View, bone: string, shapes: readonly any[]): void {
  strip(geometry, view, bone);
  const list: any[] = geometry[view];
  const at = list.findIndex((shape) => shape.bone === bone && baseId(shape.id) === shapes[0].id && (shape.kind === "loft" || sheetOf(shape)));
  if (at < 0) throw new Error(`${view}: no shape "${shapes[0].id}" on bone "${bone}" to set`);
  const others = (part: any[]): any[] => part.filter((shape) => !(shape.bone === bone && shape.kind === "loft"));
  geometry[view] = [...others(list.slice(0, at)), ...shapes.map((shape) => structuredClone(shape)), ...others(list.slice(at + 1))];
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

/** 🩹️ The filling ellipsoids of a mass in one view: at every turn of the bone, the largest turn first, the silhouette of the ellipsoids and the fillings so far is united; a hole of it gets an ellipsoid about the box of the hole along its two principal directions, as deep as it is narrow, at the depth of the ellipsoids around it. A filling is refused when it would be seen beyond the silhouette of the mass at any turn (by more than `pad`), when it covers more of the holes of the bare ellipsoids at any turn than 1.3 times the hole it is made for or stands free in a hole that the ellipsoids and the fillings so far leave at any turn (a circle in an opening), or when at rest it covers a part of a hole the authored outline draws itself (`open`, relative to the bone origin): the default pose keeps its drawing. A refused filling is tried at 0.85, 0.7, 0.6 and 0.5 of its size and, for a mass with authored openings, at the other depths of the mass (the depth moves its place in the rest drawing, not in the turned one), so an authored opening closes when the wind turns the mass and no crescent with tails is left of it; an opening too large to hide a filling at rest stays open at every turn. At most `FILL_MOST` fillings and `FILL_ROUNDS` rounds. Returns the fillings and the largest number of holes left at one turn. */
function fillers(list: readonly Ellipsoid[], across: V3, up: V3, n: V3, turns: readonly number[], pad: number, open: readonly P2[][]): { fills: Ellipsoid[]; left: number } {
  const axes = (turn: number): [V3, V3, V3] => { const [c, s] = [Math.cos(turn), Math.sin(turn)]; const back = (p: V3): V3 => [c * p[0] + s * p[1], -s * p[0] + c * p[1], p[2]]; return [back(across), back(up), back(n)]; };
  const bare = turns.map((turn) => sorted(unionPolygons(list.map((e) => facets(seen(e, across, up, turn))))));
  const fills: Ellipsoid[] = [];
  let state: { count: number; holes: P2[][][] } = { count: 0, holes: bare.map((part) => part.holes) };
  const gaps = (t: number): P2[][] => {
    if (state.count !== fills.length) state = { count: fills.length, holes: turns.map((turn) => sorted(unionPolygons([...list, ...fills].map((e) => facets(seen(e, across, up, turn))))).holes) };
    return state.holes[t]!;
  };
  const order = turns.map((_, t) => t).sort((a, b) => Math.abs(turns[b]!) - Math.abs(turns[a]!));
  const shown = (e: Ellipsoid, room: number): boolean => turns.some((turn, t) => {
    const k = seen(e, across, up, turn);
    const keep = Math.max(0, 1 - (2 * pad) / Math.sqrt(Math.min(k.xx, k.yy)));
    const ring = facets(k);
    const at = (p: P2, f: number): P2 => ({ x: k.x + (p.x - k.x) * f, y: k.y + (p.y - k.y) * f });
    if ([at(ring[0]!, 0), ...ring.map((p) => at(p, keep))].some((p) => !bare[t]!.outlines.some((outline) => inside(p, outline)))) return true;
    const probes = [at(ring[0]!, 0), ...[0.33, 0.66, 0.97].flatMap((f) => ring.filter((_, i) => i % 4 === 0).map((p) => at(p, f)))];
    const covered = (probes.filter((p) => bare[t]!.holes.some((hole) => inside(p, hole))).length / probes.length) * Math.PI * Math.sqrt(k.xx * k.yy - k.xy * k.xy);
    if (covered > room) return true;
    return gaps(t).some((hole) => [at(ring[0]!, 0), ...ring].every((p) => inside(p, hole)));
  });
  const opened = (e: Ellipsoid): boolean => {
    const k = seen(e, across, up, 0);
    const ring = [{ x: k.x, y: k.y }, ...facets(k)];
    return open.some((hole) => ring.some((p) => inside(p, hole)) || hole.some((p) => inside(p, ring.slice(1))));
  };
  const given = new Set<string>();
  let left = 0;
  for (let round = 0; round < FILL_ROUNDS; round++) {
    left = 0;
    let added = 0;
    for (const t of order) {
      const turn = turns[t]!;
      if (fills.length >= FILL_MOST) break;
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
        const around = all.filter((_, i) => { const k = conics[i]!; const det = k.xx * k.yy - k.xy * k.xy; return hole.some((p) => Math.abs(Math.sqrt((k.yy * (p.x - k.x) ** 2 - 2 * k.xy * (p.x - k.x) * (p.y - k.y) + k.xx * (p.y - k.y) ** 2) / det) - 1) < 0.04); });
        const depth = (around.length > 0 ? around : all).reduce((sum, e, _, group) => sum + dot(e.c, z) / group.length, 0);
        const reach = list.flatMap((e) => { const far = Math.hypot(...e.a.map((axis) => dot(axis, z))); return [dot(e.c, z) - far, dot(e.c, z) + far]; });
        const depths = [depth, ...Array.from({ length: 13 }, (_, i) => Math.min(...reach) + ((Math.max(...reach) - Math.min(...reach)) * (i + 0.5)) / 13).sort((a, b) => Math.abs(a - depth) - Math.abs(b - depth))];
        const size = Math.abs(areaOf(hole));
        const room = 1.3 * size + 4 * pad * pad;
        const fit = (part: readonly P2[]): Ellipsoid | undefined => {
          if (part.length < 2) return undefined;
          const box = [d1, d2].map((d) => { const at = part.map((p) => p.x * d.x + p.y * d.y); return [Math.min(...at), Math.max(...at)] as const; });
          const centre = { x: ((box[0]![0] + box[0]![1]) / 2) * d1.x + ((box[1]![0] + box[1]![1]) / 2) * d2.x, y: ((box[0]![0] + box[0]![1]) / 2) * d1.y + ((box[1]![0] + box[1]![1]) / 2) * d2.y };
          const [h1, h2] = box.map(([low, high]) => ((high - low) / 2) * 1.1 * Math.SQRT2 + pad) as [number, number];
          const make = (f: number, deep: number): Ellipsoid => ({ id: `fill-${fills.length + 1}`, c: plus(plus(times(x, centre.x), y, centre.y), z, deep), a: [times(plus(times(x, d1.x), y, d1.y), h1 * f), times(plus(times(x, d2.x), y, d2.y), h2 * f), times(z, Math.min(h1, h2) * f)] });
          for (const f of [1, 0.85, 0.7, 0.6, 0.5]) for (const deep of open.length === 0 ? [depth] : depths) if (!shown(make(f, deep), room) && !opened(make(f, deep))) return make(f, deep);
          return undefined;
        };
        const whole = fills.length >= FILL_MOST ? undefined : fit(hole);
        if (whole === undefined) { given.add(key); left++; continue; }
        fills.push(whole);
        added++;
      }
    }
    if (added === 0) break;
  }
  left = Math.max(...turns.map((turn) => sorted(unionPolygons([...list, ...fills].map((e) => facets(seen(e, across, up, turn))))).holes.length));
  return { fills, left };
}

/** 🧵️ The lofts of the ellipsoids of one mass in one view, `n` the viewing direction in the frame of the bone: every ellipsoid is a bead of parallel elliptic sections across the viewing direction: the equator, sections up to the latitude its contour reaches at the largest of the `turns` of the bone (three degrees more, at least `LEAST`, a section every `STEP` degrees). The contour of an ellipsoid with the axes A seen along n is the great circle across A⁻¹n (not Aᵀn, which the first loft tool used: its eleven sections hid the error, with three sections an oblique ellipsoid was drawn up to 216 mm short, `hawthorn` 1:200 `fol-5`; the run reports such a loss as `LOST`) and a point at each pole; the beads of a mass hang on one thread of 0.6 micrometres, which the union of the engine leaves out, through a point common to every two beads that touch; a bead that touches none is reached straight through the air, where the thread has no area either, so a mass is always ONE loft. Every turn of the thread is at most a right angle between two of its points, so no section folds over the next. */
function loftsOf(ellipsoids: readonly Ellipsoid[], n: V3, turns: readonly number[]): any[][] {
  const bead = (e: Ellipsoid): any[] => {
    const m0 = e.a.map((axis) => dot(axis, n) / dot(axis, axis)) as V3;
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
    const reached = Math.max(...turns.map((turn) => { const [c, s] = [Math.cos(turn), Math.sin(turn)]; const mt = unit(e.a.map((axis) => dot(axis, [c * n[0] + s * n[1], -s * n[0] + c * n[1], n[2]]) / dot(axis, axis)) as V3); return (Math.acos(Math.max(-1, Math.min(1, dot(m, mt)))) * 180) / Math.PI; }));
    const widest = Math.min(80, Math.max(LEAST, reached + 3));
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
      if (Math.hypot(...minus(to, last())) < 60 * TURN) for (let i = 0; i < 4; i++) thread(plus(last(), heading, 20 * TURN));
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
    while (left.size > 0) visit([...left].sort((p, q) => weight(ellipsoids[q]!) - weight(ellipsoids[p]!))[0]!);
    while (sections.length > 0 && sections[sections.length - 1]!.rotation === undefined) sections.pop();
    lofts.push(sections);
  }
  return lofts;
}

/** 🔧️ The ops applied to the asset: per scale, view and bone the listed lofts are set by {@link place}. */
function applyOps(target: any, ops: any): number {
  let count = 0;
  for (const key of Object.keys(ops)) for (const view of VIEWS) {
    const wanted: any[] = ops[key]?.[view] ?? [];
    for (const bone of new Set<string>(wanted.map((shape) => shape.bone))) {
      const shapes = wanted.filter((shape) => shape.bone === bone);
      place(target.representations[key].geometry, view, bone, shapes);
      count += shapes.length;
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
    const steps = Math.max(1, Math.ceil((high - low) / 0.02));
    turns.set(bone.id, [0, ...Array.from({ length: steps + 1 }, (_, i) => low + ((high - low) * i) / steps)]);
    if (bone.id.startsWith("fol")) console.log(`turn ${bone.id}: ${((low * 180) / Math.PI).toFixed(1)} to ${((high * 180) / Math.PI).toFixed(1)} degrees over ${states.length} subjects, lateral axis off by at most ${((askew * 180) / Math.PI).toFixed(2)} degrees`);
  }
  type Item = { key: string; view: View; bone: string; common: any; sheet: P2[][]; beads: Ellipsoid[]; fills: Ellipsoid[]; n: V3; across: V3; up: V3; open: P2[][]; pad: number };
  const items: Item[] = [];
  const paperOf = (view: View): { ax: V3; ay: V3; looking: V3 } => {
    const paper = [{ x: 1, y: 0, z: 0 }, { x: 0, y: 1, z: 0 }, { x: 0, y: 0, z: 1 }].map((axis) => project(view, axis));
    const looking = lookingOf(view);
    return { ax: paper.map((p) => p.x) as V3, ay: paper.map((p) => p.y) as V3, looking: [looking.x, looking.y, looking.z] };
  };
  const build = (item: Item): number[] => {
    const lofts = loftsOf([...item.beads, ...item.fills], item.n, turns.get(item.bone)!);
    const shapes = lofts.map((sections, index) => ({ ...item.common, id: index === 0 ? item.common.id : `${item.common.id}-${index + 1}`, kind: "loft", sections }));
    for (const shape of shapes) loftShapeIssues(shape as any, `${item.key} ${item.view} ${item.bone}`, (path: string, code: string, en: string) => { const at = Number(/sections\[(\d+)\]/u.exec(path)?.[1] ?? 0); console.log(`ISSUE ${code} ${path} ${en} ${JSON.stringify(shape.sections.slice(Math.max(0, at - 3), at + 3))}`); });
    place(result.representations[item.key].geometry, item.view, item.bone, shapes);
    ((ops[item.key] ??= {})[item.view] ??= []).splice(0, Infinity, ...ops[item.key][item.view].filter((shape: any) => shape.bone !== item.bone), ...shapes);
    return lofts.map((sections) => sections.length);
  };
  for (const key of authored) {
    const geometry = result.representations[key].geometry;
    const body: any[] = asset.volumes?.[key]?.body ?? [];
    const pad = (SLACK / 1000) * Number(key);
    for (const view of VIEWS) {
      for (const bone of [...new Set<string>(geometry[view].map((shape: any) => shape.bone))]) {
        if (only !== undefined && !only.includes(bone)) continue;
        const ellipsoids = body.filter((solid) => solid.bone === bone && solid.kind === "ellipsoid" && solid.weights === undefined);
        const sheets = (geometry[view] as any[]).filter((shape) => shape.bone === bone && !String(shape.id).startsWith(PLATE) && sheetOf(shape));
        if (ellipsoids.length === 0 || body.some((solid) => solid.bone === bone && solid.kind !== "ellipsoid" && solid.role === "body") || sheets.length !== 1) continue;
        const f = rest[bone]!.frame;
        const local = (d: V3): V3 => [dot(d, f.u), dot(d, f.v), dot(d, f.w)];
        const { ax, ay, looking } = paperOf(view);
        const [n, across, up] = [looking, ax, ay].map(local) as [V3, V3, V3];
        const beads = needed(ellipsoids.map(ellipsoidOf), across, up, turns.get(bone)!, pad);
        const sheet = structuredClone(sheets[0]);
        const origin = project(view, { x: f.o[0], y: f.o[1], z: f.o[2] });
        const authoredRings: P2[][] = (flattenGeometry(sheet).rings as P2[][]).map((ring) => ring.map((p) => (view === "elevation" ? { x: p.x * dot(f.u, ax) + p.y * dot(f.v, ax), y: p.x * dot(f.u, ay) + p.y * dot(f.v, ay) } : { x: p.x - origin.x, y: p.y - origin.y })));
        const open = sorted(authoredRings).holes;
        const filled = FILL ? fillers(beads, across, up, n, turns.get(bone)!, pad, open) : { fills: [], left: 0 };
        strip(geometry, view, bone);
        const common = { ...Object.fromEntries(Object.keys(sheets[0]).filter((name) => COMMON.includes(name)).map((name) => [name, sheets[0][name]])), id: baseId(sheets[0].id) };
        const item: Item = { key, view, bone, common, sheet: authoredRings.map((ring) => ring.map((p) => ({ x: p.x + origin.x, y: p.y + origin.y }))), beads, fills: filled.fills, n, across, up, open, pad };
        items.push(item);
        const counts = build(item);
        console.log(`${key} ${view} ${bone}: ${ellipsoids.length} ellipsoids, ${beads.length} drawn + ${filled.fills.length} filling, ${open.length} authored opening${open.length === 1 ? "" : "s"}, at most ${filled.left} hole${filled.left === 1 ? "" : "s"} at one turn -> ${counts.length} loft${counts.length > 1 ? "s" : ""} "${common.id}" with ${counts.join(" + ")} sections`);
      }
    }
  }
  const speck = (SPECK / 1000);
  const drawnOf = new Map<Item, { count: number; outlines: { outlines: P2[][]; holes: P2[][] }[] }>();
  const silhouettes = (item: Item): { outlines: P2[][]; holes: P2[][] }[] => {
    const known = drawnOf.get(item);
    if (known !== undefined && known.count === item.fills.length) return known.outlines;
    const outlines = turns.get(item.bone)!.map((turn) => sorted(unionPolygons([...item.beads, ...item.fills].map((e) => facets(seen(e, item.across, item.up, turn))))));
    drawnOf.set(item, { count: item.fills.length, outlines });
    return outlines;
  };
  for (const key of authored) {
    const selector = engine.scaleSelector(Number(key));
    const mine = items.filter((item) => item.key === key);
    const queries: any[] = engine.subjects(asset, Number(key)).map((subject: any) => subject.query);
    for (let pass = 0; pass < 5; pass++) {
      const t0 = performance.now();
      const found = new Map<Item, Ellipsoid[]>();
      let holes = 0, worst = 0, blocked = 0, capped = 0;
      for (const query of queries) {
        const state = evaluate(result, query, selector);
        for (const item of mine) {
          const drawn: P2[][][] = state.shapes[item.view].filter((entry: any) => entry.shape.bone === item.bone && entry.shape.kind === "loft").map((entry: any) => entry.rings);
          const { ax, ay, looking } = paperOf(item.view);
          const frame = state.bones[item.bone].frame;
          const [o, u, v, w] = [frame.o, frame.u, frame.v, frame.w].map((p: any) => [p.x, p.y, p.z] as V3) as [V3, V3, V3, V3];
          const mean = item.beads.reduce((sum, e) => plus(sum, e.c, 1 / item.beads.length), [0, 0, 0] as V3);
          const deep = dot(plus(plus(plus(o, u, mean[0]), v, mean[1]), w, mean[2]), looking);
          const parts = drawn.map(sorted);
          const own = parts.flatMap((part) => part.holes.filter((hole) => !parts.some((other) => other !== part && other.outlines.some((outline) => inside(middle(hole), outline)))));
          const gaps = parts.length > 1 ? sorted(unionPolygons(parts.flatMap((part) => part.outlines))).holes.filter((hole) => !parts.some((part) => part.outlines.some((outline) => inside(middle(hole), outline)))) : [];
          for (const hole of [...own, ...gaps]) {
            const m = middle(hole);
            holes++;
            const extent = Math.max(Math.max(...hole.map((p) => p.x)) - Math.min(...hole.map((p) => p.x)), Math.max(...hole.map((p) => p.y)) - Math.min(...hole.map((p) => p.y)));
            if (extent > speck * Number(key)) continue;
            worst = Math.max(worst, extent);
            let radius = Math.max(extent * 0.65 + item.pad, (BALL / 1000) * Number(key));
            const ball = (shift: number): Ellipsoid => { const world = minus(plus(plus(times(ax, m.x), ay, m.y), looking, deep + shift), o); return { id: "speck", c: [dot(world, u), dot(world, v), dot(world, w)], a: [[radius, 0, 0], [0, radius, 0], [0, 0, radius]] }; };
            let edge = false;
            const sound = (e: Ellipsoid): boolean => {
              const k = seen(e, item.across, item.up, 0);
              const probes = [{ x: k.x, y: k.y }, ...facets(k)];
              if (item.open.some((ring) => (edge ? probes.every((p) => inside(p, ring)) : probes.some((p) => inside(p, ring))))) return false;
              return !turns.get(item.bone)!.some((turn, t) => {
                const round = facets(seen(e, item.across, item.up, turn));
                const { outlines, holes } = silhouettes(item)[t]!;
                return round.some((p) => !outlines.some((outline) => inside(p, outline))) || holes.some((hole) => round.every((p) => inside(p, hole)));
              });
            };
            const shifts = item.open.length === 0 ? [0] : Array.from({ length: 81 }, (_, i) => (i % 2 === 0 ? 1 : -1) * Math.ceil(i / 2) * 0.05);
            let fill = shifts.map(ball).find(sound);
            if (fill === undefined) { radius = extent * 0.5 + item.pad / 2; fill = shifts.map(ball).find(sound); }
            if (fill === undefined && extent <= (PIN / 1000) * Number(key)) { edge = true; fill = shifts.map(ball).find(sound); }
            if (fill === undefined) {
              blocked++;
              const k0 = seen(ball(0), item.across, item.up, 0);
              if (pass === 0) console.log(`no place 1:${key} ${item.view} ${item.bone} ${JSON.stringify(query)}: hole of ${mm(extent)} mm; at rest in an authored opening: ${item.open.some((ring) => inside({ x: k0.x, y: k0.y }, ring))}`);
              continue;
            }
            const known = found.get(item) ?? [];
            if (known.some((other) => Math.hypot(...minus(other.c, fill.c)) < radius)) continue;
            if (item.fills.filter((other) => other.id === "speck").length + known.length >= SPECK_MOST) { capped++; continue; }
            found.set(item, [...known, fill]);
          }
        }
      }
      console.log(`1:${key} pass ${pass + 1}: ${queries.length} subjects in ${((performance.now() - t0) / 1000).toFixed(0)} s (${((performance.now() - t0) / queries.length).toFixed(0)} ms each); ${holes} holes seen over all subjects, ${[...found.values()].flat().length} specks (largest ${mm(worst)} mm) to fill, ${blocked} without a place, ${capped} beyond the ${SPECK_MOST} of their mass`);
      if (found.size === 0) break;
      for (const [item, fills] of found) { item.fills.push(...fills); build(item); }
    }
    const state = evaluate(result, { kind: "pose", pose: asset.presentation.defaultPose }, selector);
    let far = { extent: 0, at: "" };
    for (const item of mine) {
      const rings: P2[][] = state.shapes[item.view].filter((entry: any) => entry.shape.bone === item.bone && entry.shape.kind === "loft").flatMap((entry: any) => entry.rings);
      const box = (list: P2[][]): number[] => { const p = list.flat(); return [Math.min(...p.map((q) => q.x)), Math.max(...p.map((q) => q.x)), Math.min(...p.map((q) => q.y)), Math.max(...p.map((q) => q.y))]; };
      const [a, b] = [box(rings), box(item.sheet)];
      const extent = Math.max(...a.map((value, i) => Math.abs(value - b[i]!)));
      if (extent > far.extent) far = { extent, at: `${item.view} ${item.bone}` };
const now = state.bones[item.bone].frame;
      const r = rest[item.bone]!.frame;
      const turn = Math.atan2(now.u.x * r.v[0] + now.u.y * r.v[1] + now.u.z * r.v[2], now.u.x * r.u[0] + now.u.y * r.u[1] + now.u.z * r.u[2]);
      const o = project(item.view, now.o);
      const c = box(item.beads.map((e) => facets(seen(e, item.across, item.up, turn)).map((p) => ({ x: p.x + o.x, y: p.y + o.y }))));
      const lost = Math.max(...a.map((value, i) => Math.abs(value - c[i]!)));
      if (lost > 0.01) console.log(`LOST 1:${key} ${item.view} ${item.bone}: the engine draws ${a.map((value) => value.toFixed(3)).join(" ")}, the ellipsoids reach ${c.map((value) => value.toFixed(3)).join(" ")} (${mm(lost)} mm)`);
    }
    console.log(`1:${key}: extent of the body against the authored outline in the default pose: at most ${mm(far.extent)} mm (${far.at})`);
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
