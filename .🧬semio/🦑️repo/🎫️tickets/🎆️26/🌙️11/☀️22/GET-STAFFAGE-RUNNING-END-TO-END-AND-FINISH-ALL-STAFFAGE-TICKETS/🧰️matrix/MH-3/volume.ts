/** 🧊️ (MH-3 copy of ../MH-1/fit.ts of 2026-11-25 04:42, unchanged below this sentence; used for its op `volume` as the step after `fit.ts` in the chain of an asset that has `ops2/<id>.json`: `bun volume.ts <id|file.json> --ops ops2/<id>.json [--write] [--out <file>]`.) The multiview repair of one asset of batch MH-1 (the tool of `../MR/fit.ts`, copied and extended: `extrude` takes `twins` and `span` and reads the lateral span of a flat front outline directly from it when the bone leans more than 45 degrees towards the viewer, `copy` takes `fromScale`, `anchor-split` splits a lumped contact), the last step of its chain: `bun fit.ts <id|file.json> --ops <ops.json> [--write] [--out <file>]` applies the ops of the file in order and prints what each did (an op that cannot be applied is refused by name, the others still run, exit code 1); without `--write` nothing is written. Every op takes the ELEVATION twin (the shape of the same `id` and bone in `geometry.elevation`, or `twin`) as the profile of the part and rebuilds or refits the front or plan shape from it, so a second run changes nothing.
 *
 * Ops (`scale` is the representation key, `id` the shape id in the target view, `bone` only when the id is not unique):
 * - `{ "op": "front-sweep", scale, id }`: the front shape becomes (or stays) an `extruded` solid with `sweep: "v"`; its outline is kept (a flat front outline is first re-expressed in the bone-local (u, w) plane), cut along the bone to the u-range of the elevation twin (stretched to it where it falls short; `"range": "stretch"` only stretches, `"keep"` leaves it; `"lateral": "plan"` cuts it the same way across the bone to the lateral range of the plan twin), and `z0`, `z1`, `zTip` are fitted so that the (u, v) footprint of the prism has the support function of the elevation outline in every direction (so the heights agree at every sagittal angle).
 * - `{ "op": "extrude", scale, id, "view": "front" | "plan" }`: the exact alternative for a part whose front outline is a plain taper (and for the plan of a part that is a prism): the shape takes the elevation outline as its profile and becomes an `extruded` solid with `sweep: "w"` whose lateral span `z0`, `z1`, `zTip` is fitted to the (u, w) footprint the shape had; elevation and this view then agree at every pose by construction, and the view draws the hull of the swept outline instead of its own outline. It is also the op for a bone that points at the viewer of the front view (a foot, a toe): there no front outline can be swept along v and `front-sweep` refuses.
 * - `{ "op": "plan-revolve", scale, id }`: the plan shape becomes (or stays) a `revolved` solid about the bone axis. An elevation twin that is itself revolved is copied (one body in three views). Else an existing revolved profile is kept in shape, cut or stretched to the u-range of the elevation twin, and its axis offset and radius scale are fitted to the (u, v) footprint of the elevation twin and the (u, w) footprint of the front twin; a flat plan shape gets a new profile of the mean half thickness of the two twins at nine stations (`"stations": n`).
 * - `{ "op": "copy", scale, id, "from": view, "to": [view…] }`: the geometry and the solid of the shape in `from` replace those of its twins in `to` (the same revolved or extruded body in several views); `"create": true` adds a missing twin with the paint fields of the source.
 * - `{ "op": "set", scale, view, id, "set": {…} }` and `{ "op": "remove", scale, view, id }`: a hand-made correction of single fields, or the removal of a shape.
 * - `{ "op": "anchor", id, "set": {…} }`: binds an anchor (`bone`, `x`, `y`, `z`).
 * - `extrude` also takes `"twins": [id…]` (the profile is the convex hull of several elevation shapes of the bone: a boot and its sole drawn as two shapes are one prism) and `"span": [z0, z1]` (the lateral span in asset coordinates, where the shape it replaces gives none).
 * - `copy` also takes `"fromScale"`: the source shape is read from another representation (a band that already draws the part as the right solid).
 * - `{ "op": "plan-along", scale, id }`: a plan shape that is a thin `extruded` slab with `sweep: "u"` (a transverse (v, w) outline pinned at one station of the bone, which does not lengthen when the bone leans), or a flat plan outline of a bone that stands upright in the sagittal plane (re-expressed as that transverse outline), becomes the prism of that outline over the length of the part: `z0`, `z1` are fitted so that the (u, v) footprint of the prism has the support function of the elevation twin (`twin`, or the hull of `twins`); the outline is kept (`"clip": true` cuts it to the thickness of the elevation twin).
 * - `{ "op": "anchor-split", id, "into": [{ id, z, title, set? }…] }`: one lumped contact becomes one anchor per physical contact at its lateral place (D45); every pose contact list, clip contact and load that names the old id names the new ones.
 * - `{ "op": "volume", scale, id, "as": "revolved" | "prism-w" | "prism-v" | "prism-u", "views"?, "solids"?, "axis"?, "profile"?, "ids"? }`: the part is redrawn in every view that draws it (or in `views`) as ONE solid read from the body volume of the scale (all body solids of the bone when the shape is the only one the bone draws, with those of the bones below it that draw no shape of their own in one of the views (the fingers of a hand; a bone that draws in every view keeps its own subtree), unless `"undrawn": false`; else the body solid of the same id; or the solids named in `solids`): a body of revolution about the bone axis through the middle of the volume (`axis`: [v, w] states it) with the mean half thickness of the volume along the bone, or the prism of the side outline of the volume over its lateral span (`prism-w`; `"profile": "elevation"` keeps the drawn elevation outline as the profile), or the prism of its top outline over its thickness (`prism-v`), or the prism of its transverse outline over its length (`prism-u`); `"ids": { view: id }` names the shape of a view whose id differs (an elevation `shoe-r` whose front and plan twin is `foot-r`). The three views are then projections of one body at every pose, twisted rest placements included, and that body is the one the silhouette guard compares the drawing with.
 * @see ../MR/gate.ts — the measurement that says which bones need an op */
import { join } from "node:path";
import { S, dot, readDoc, round, sub, support, writeDoc, type V3 } from "../MR/lib.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { worldSolids } = await import(join(S, "🔨️modules", "🗂️catalog", "🧊️volumes", "🌓️silhouette", "🟦️.ts"));

type P2 = readonly [number, number];
type Frame = { o: V3; u: V3; v: V3; w: V3 };
const ANGLES = Array.from({ length: 72 }, (_, i) => (2 * Math.PI * i) / 72);

/** 🧵️ The repair of one asset document in memory: returns the function that applies one op to `asset` and says what it did (it throws when the op cannot be applied). */
export function fitter(asset: any): (op: any) => string {
  const frames = restFrames(asset.rig) as Record<string, { frame: Frame }>;
  const lengthOf = (bone: string): number => asset.rig.bones.find((candidate: any) => candidate.id === bone).lengthM;
  const shapesOf = (scale: string, view: string): any[] => asset.representations[scale].geometry[view];
  const find = (scale: string, view: string, id: string, bone?: string): any => shapesOf(scale, view).find((shape) => shape.id === id && (bone === undefined || shape.bone === bone));
  const need = (scale: string, view: string, id: string, bone?: string): any => {
    const shape = find(scale, view, id, bone);
    if (shape === undefined) throw new Error(`1:${scale} ${view} has no shape "${id}"${bone ? ` on ${bone}` : ""}`);
    return shape;
  };
  const pointsOf = (shape: any): P2[] => (shape.kind === "poly" ? (shape.points as P2[]) : flattenGeometry(shape).rings.flat().map((p: any) => [p.x, p.y] as const));
  const rangeOf = (points: readonly P2[], k: 0 | 1): [number, number] => [Math.min(...points.map((p) => p[k])), Math.max(...points.map((p) => p[k]))];
  const GEOMETRY_KEYS = ["points", "closed", "x", "y", "w", "h", "x0", "y0", "x1", "y1", "r0", "r1", "r", "cx", "cy", "rx", "ry", "d", "followTip", "solid"];

  /** 🧭️ The local offset (v, w) of the axis of a revolved solid of a bone, and the profile points as (u, radius). */
  function revolvedOf(shape: any, frame: Frame): { av: number; aw: number; radial: P2[] } {
    const offset: V3 = [shape.solid.axisX, 0, shape.solid.axisZ ?? 0];
    const [av, aw] = [dot(offset, frame.v), dot(offset, frame.w)];
    return { av, aw, radial: pointsOf(shape).map(([u, y]) => [u, Math.hypot(y - av, (shape.solid.axisZ ?? 0) - aw)] as const) };
  }

  /** 🦶️ The footprint of a shape's implied solid in the bone-local (u, v) plane (`plane` 1) or (u, w) plane (`plane` 2), as points whose hull it is; undefined where the shape gives none. */
  function footprint(shape: any, view: string, frame: Frame, length: number, plane: 1 | 2): P2[] | undefined {
    const points = pointsOf(shape);
    const solid = shape.solid;
    if (solid?.kind === "revolved") {
      const { av, aw, radial } = revolvedOf(shape, frame);
      const centre = plane === 1 ? av : aw;
      return radial.flatMap(([u, r]) => [[u, centre - r] as const, [u, centre + r] as const]);
    }
    if (solid?.kind === "extruded" && solid.sweep === "v") {
      if (plane === 2) return points;
      return points.flatMap(([u]) => {
        const t = solid.zTip === undefined || length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
        return [[u, solid.z0 + ((solid.zTip?.[0] ?? solid.z0) - solid.z0) * t] as const, [u, solid.z1 + ((solid.zTip?.[1] ?? solid.z1) - solid.z1) * t] as const];
      });
    }
    if (solid?.kind === "extruded" && solid.sweep === "w") return plane === 1 ? points : points.flatMap(([u]) => [[u, solid.z0 - frame.o[2]] as const, [u, solid.z1 - frame.o[2]] as const]);
    if (solid !== undefined) return undefined;
    if (view === "elevation") return plane === 1 ? points : undefined;
    if (view === "front" && plane === 2) return frontLocal(points, frame);
    return undefined;
  }

  /** 🔁️ A flat front outline (lateral, height) in asset metres at rest re-expressed in the bone-local (u, w) plane through the bone origin, seen along the forward axis. */
  function frontLocal(points: readonly P2[], frame: Frame): P2[] {
    const det = frame.u[1] * frame.w[2] - frame.w[1] * frame.u[2];
    if (Math.abs(det) < 0.2) throw new Error("the (u, w) plane of the bone is nearly edge-on in the front view: no front outline can be swept along v");
    return points.map(([z, y]) => {
      const [dy, dz] = [y - frame.o[1], z - frame.o[2]];
      return [(dy * frame.w[2] - dz * frame.w[1]) / det, (frame.u[1] * dz - frame.u[2] * dy) / det] as const;
    });
  }

  /** 🪨️ Keeps the geometry a shape had when the refit differs from it by less than 0.3 mm in every number (the rounding of a fit of a fit), so a second run changes nothing; says whether the shape changed. */
  function settle(shape: any, before: { kind: unknown; points: unknown; solid: unknown }): boolean {
    const close = (a: unknown, b: unknown): boolean => {
      if (typeof a === "number" && typeof b === "number") return Math.abs(a - b) <= 0.0003;
      if (Array.isArray(a) && Array.isArray(b)) return a.length === b.length && a.every((value, i) => close(value, b[i]));
      if (a !== null && b !== null && typeof a === "object" && typeof b === "object") return Object.keys(a).length === Object.keys(b).length && Object.entries(a).every(([key, value]) => close(value, (b as Record<string, unknown>)[key]));
      return a === b;
    };
    if (!close(before, { kind: shape.kind, points: shape.points, solid: shape.solid })) return true;
    Object.assign(shape, structuredClone(before));
    return false;
  }
  const snapshot = (shape: any): { kind: unknown; points: unknown; solid: unknown } => structuredClone({ kind: shape.kind, points: shape.points, solid: shape.solid });

  /** 🎯️ Minimises a function of a few parameters by a pattern search with halving steps. */
  function search(start: readonly number[], cost: (p: readonly number[]) => number, step = 0.02, floor = 0.0001): number[] {
    const p = [...start];
    let best = cost(p);
    for (let s = step; s >= floor; s /= 2) {
      for (let moved = true, guard = 0; moved && guard < 200; guard++) {
        moved = false;
        for (let i = 0; i < p.length; i++)
          for (const d of [s, -s]) {
            p[i]! += d;
            const c = cost(p);
            if (c < best - 1e-9) (best = c), (moved = true);
            else p[i]! -= d;
          }
      }
    }
    return p;
  }

  const mismatch = (a: readonly P2[], b: readonly P2[]): number => {
    let worst = 0;
    let sum = 0;
    for (const angle of ANGLES) {
      const d = Math.abs(support(a, angle) - support(b, angle));
      worst = Math.max(worst, d);
      sum += d;
    }
    return worst + (0.05 * sum) / ANGLES.length;
  };

  const stretch = (points: readonly P2[], to: readonly [number, number]): P2[] => {
    const from = rangeOf(points, 0);
    const k = from[1] === from[0] ? 1 : (to[1] - to[0]) / (from[1] - from[0]);
    return points.map(([u, y]) => [to[0] + (u - from[0]) * k, y] as const);
  };

  /** ✂️ A closed outline cut to the u-interval (what lies beyond the ends of the elevation twin is hidden length inside the neighbouring part), then stretched to it where it falls short. */
  const clipStretch = (points: readonly P2[], to: readonly [number, number]): P2[] => {
    let ring = [...points];
    for (const [limit, keep] of [[to[0], (u: number) => u >= to[0] - 0.00015], [to[1], (u: number) => u <= to[1] + 0.00015]] as const) {
      const next: P2[] = [];
      for (let i = 0; i < ring.length; i++) {
        const [p, q] = [ring[i]!, ring[(i + 1) % ring.length]!];
        if (keep(p[0])) next.push(p);
        if (keep(p[0]) !== keep(q[0])) next.push([limit, p[1] + ((q[1] - p[1]) * (limit - p[0])) / (q[0] - p[0])] as const);
      }
      ring = next;
    }
    const clean = ring.filter((p, i) => Math.hypot(p[0] - ring[(i + 1) % ring.length]![0], p[1] - ring[(i + 1) % ring.length]![1]) > 1e-6);
    return clean.length < 3 ? stretch(points, to) : stretch(clean, to);
  };

  function frontSweep(op: any): string {
    const shape = need(op.scale, "front", op.id, op.bone);
    const frame = frames[shape.bone]!.frame;
    const length = lengthOf(shape.bone);
    const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
    if (twin === undefined) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
    const target = footprint(twin, "elevation", frame, length, 1);
    if (target === undefined || twin.solid?.kind === "revolved") throw new Error(`1:${op.scale} "${op.id}": the elevation twin is a revolved solid, use the op "copy"`);
    if (shape.solid !== undefined && !(shape.solid.kind === "extruded" && shape.solid.sweep === "v")) throw new Error(`1:${op.scale} front "${op.id}" is a ${shape.solid.kind} solid, not a sweep along v`);
    const before = snapshot(shape);
    let profile = shape.solid === undefined ? frontLocal(pointsOf(shape), frame) : pointsOf(shape);
    const range = rangeOf(target, 0);
    if (op.range !== "keep") profile = op.range === "stretch" ? stretch(profile, range) : clipStretch(profile, range);
    if (op.lateral === "plan") {
      const planTwin = find(op.scale, "plan", op.planTwin ?? op.id, shape.bone);
      const across = planTwin === undefined ? undefined : footprint(planTwin, "plan", frame, length, 2);
      if (across === undefined) throw new Error(`1:${op.scale} "${op.id}": "lateral": "plan" needs a revolved or laterally extruded plan twin`);
      const flip = (points: readonly P2[]): P2[] => points.map(([x, y]) => [y, x] as const);
      profile = flip(clipStretch(flip(profile), rangeOf(across, 1)));
    }
    const [a, b] = rangeOf(profile, 0);
    const prism = (p: readonly number[]): P2[] =>
      [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b].flatMap((u) => {
        const t = length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
        return [[u, p[0]! + (p[2]! - p[0]!) * t] as const, [u, p[1]! + (p[3]! - p[1]!) * t] as const];
      });
    const [v0, v1] = rangeOf(target, 1);
    const fitted = search([v0, v1, v0, v1], (p) => mismatch(prism(p), target) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
    for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
    Object.assign(shape, { kind: "poly", points: profile.map(([u, w]) => [round(u, 4), round(w, 4)]), closed: true, solid: { kind: "extruded", sweep: "v", z0: round(fitted[0]!), z1: round(fitted[1]!), zTip: [round(fitted[2]!), round(fitted[3]!)] } });
    const left = mismatch(prism(fitted), target);
    return `${settle(shape, before) ? "fitted" : "unchanged"}: u [${range.map((v) => v.toFixed(3))}], span ${JSON.stringify(shape.solid)} footprint mismatch ${(left * 1000).toFixed(1)} mm`;
  }

  function frontExtrude(op: any): string {
    const shape = need(op.scale, op.view ?? "front", op.id, op.bone);
    const frame = frames[shape.bone]!.frame;
    const length = lengthOf(shape.bone);
    const twin = find(op.scale, "elevation", op.twins?.[0] ?? op.twin ?? op.id, shape.bone);
    if (twin === undefined || twin.solid?.kind === "revolved") throw new Error(`1:${op.scale} "${op.id}" needs a flat or extruded elevation twin on ${shape.bone}`);
    const before = snapshot(shape);
    const profile = op.twins === undefined ? pointsOf(twin) : hull((op.twins as string[]).flatMap((id) => pointsOf(need(op.scale, "elevation", id, shape.bone))));
    const [a, b] = rangeOf(profile, 0);
    const edgeOn = Math.abs(frame.u[1] * frame.w[2] - frame.w[1] * frame.u[2]) < 0.7;
    const lateral = shape.solid !== undefined ? undefined : op.view === "plan" ? rangeOf(pointsOf(shape), 1).map((z) => z - frame.o[2]) : edgeOn ? rangeOf(pointsOf(shape), 0).map((z) => z - frame.o[2]) : undefined;
    const across = op.span !== undefined ? [a, b].flatMap((u) => (op.span as number[]).map((z) => [u, z - frame.o[2]] as const)) : shape.solid?.kind === "extruded" && shape.solid.sweep === "w" ? undefined : lateral !== undefined ? [a, b].flatMap((u) => lateral.map((w) => [u, w] as const)) : footprint(shape, op.view ?? "front", frame, length, 2);
    if (across !== undefined) {
      const prism = (p: readonly number[]): P2[] =>
        [a, Math.min(Math.max(0, a), b), Math.max(Math.min(length, b), a), b].flatMap((u) => {
          const t = length <= 0 ? 0 : Math.min(Math.max(u / length, 0), 1);
          return [[u, p[0]! + (p[2]! - p[0]!) * t] as const, [u, p[1]! + (p[3]! - p[1]!) * t] as const];
        });
      const [w0, w1] = rangeOf(across, 1);
      const fitted = search([w0, w1, w0, w1], (p) => mismatch(prism(p), across) + (p[0]! >= p[1]! - 0.002 || p[2]! >= p[3]! - 0.002 ? 1 : 0));
      const z = frame.o[2];
      for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
      Object.assign(shape, { kind: "poly", points: structuredClone(profile), closed: true, solid: { kind: "extruded", sweep: "w", z0: round(fitted[0]! + z), z1: round(fitted[1]! + z), zTip: [round(fitted[2]! + z), round(fitted[3]! + z)] } });
    } else if (shape.solid?.kind === "extruded" && shape.solid.sweep === "w") {
      const kept = structuredClone(shape.solid);
      for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
      Object.assign(shape, { kind: "poly", points: structuredClone(profile), closed: true, solid: kept });
    } else throw new Error(`1:${op.scale} ${op.view ?? "front"} "${op.id}" is a ${shape.solid?.kind} solid that states no lateral span: give "span": [z0, z1]`);
    return `${settle(shape, before) ? "extruded" : "unchanged"}: the elevation outline swept laterally, span ${JSON.stringify(shape.solid)}`;
  }

  function copyGeometry(source: any, target: any): void {
    for (const key of GEOMETRY_KEYS) delete target[key];
    target.kind = source.kind;
    for (const key of GEOMETRY_KEYS) if (source[key] !== undefined) target[key] = structuredClone(source[key]);
  }

  function planRevolve(op: any): string {
    const shape = need(op.scale, "plan", op.id, op.bone);
    const frame = frames[shape.bone]!.frame;
    const length = lengthOf(shape.bone);
    const twin = find(op.scale, "elevation", op.twin ?? op.id, shape.bone);
    if (twin === undefined) throw new Error(`1:${op.scale} "${op.id}" has no elevation twin on ${shape.bone}`);
    const before = snapshot(shape);
    if (twin.solid?.kind === "revolved") {
      copyGeometry(twin, shape);
      return `${settle(shape, before) ? "copied" : "unchanged"}: the revolved elevation twin`;
    }
    const side = footprint(twin, "elevation", frame, length, 1)!;
    const frontTwin = find(op.scale, "front", op.frontTwin ?? op.id, shape.bone);
    const across = frontTwin === undefined ? undefined : footprint(frontTwin, "front", frame, length, 2);
    const range = rangeOf(side, 0);
    let radial: P2[];
    if (shape.solid?.kind === "revolved" && op.rebuild !== true) radial = op.range === "stretch" ? stretch(revolvedOf(shape, frame).radial, range) : clipStretch(revolvedOf(shape, frame).radial, range);
    else {
      const n = op.stations ?? 9;
      const half = (points: readonly P2[], u: number): number | undefined => {
        const hits: number[] = [];
        for (let i = 0; i < points.length; i++) {
          const [p, q] = [points[i]!, points[(i + 1) % points.length]!];
          if (Math.min(p[0], q[0]) > u || Math.max(p[0], q[0]) < u) continue;
          if (q[0] === p[0]) hits.push(p[1], q[1]);
          else hits.push(p[1] + ((q[1] - p[1]) * (u - p[0])) / (q[0] - p[0]));
        }
        return hits.length < 2 ? undefined : (Math.max(...hits) - Math.min(...hits)) / 2;
      };
      radial = Array.from({ length: n }, (_, i) => {
        const u = range[0] + ((range[1] - range[0]) * (i === 0 ? 0.02 : i === n - 1 ? n - 1.02 : i)) / (n - 1);
        const values = [half(pointsOf(twin), u), across === undefined ? undefined : half(across, u)].filter((value): value is number => value !== undefined);
        return [i === 0 ? range[0] : i === n - 1 ? range[1] : u, values.length === 0 ? 0 : values.reduce((x, y) => x + y, 0) / values.length] as const;
      });
    }
    const band = (centre: number, scale: number): P2[] => radial.flatMap(([u, r]) => [[u, centre - r * scale] as const, [u, centre + r * scale] as const]);
    const mid = (points: readonly P2[]): number => (rangeOf(points, 1)[0] + rangeOf(points, 1)[1]) / 2;
    const cost = (p: readonly number[]): number => Math.max(mismatch(band(p[0]!, p[2]!), side), across === undefined ? 0 : mismatch(band(p[1]!, p[2]!), across)) + (p[2]! < 0.2 ? 1 : 0);
    const held = shape.solid?.kind === "revolved" && op.rebuild !== true ? [revolvedOf(shape, frame).av, revolvedOf(shape, frame).aw, 1] : undefined;
    const found = search(held ?? [mid(side), across === undefined ? 0 : mid(across), 1], cost, 0.02, 0.00005);
    const [av, aw, scale] = (held !== undefined && cost(held) - cost(found) < 0.0003 ? held : found) as [number, number, number];
    revolvedGeometry(shape, frame, av, aw, radial.map(([u, r]) => [u, r * scale] as const), `1:${op.scale} "${op.id}"`);
    const left = [mismatch(band(av, scale), side), across === undefined ? 0 : mismatch(band(aw, scale), across)];
    return `${settle(shape, before) ? "fitted" : "unchanged"}: u [${range.map((v) => v.toFixed(3))}], axis v ${av.toFixed(4)} w ${aw.toFixed(4)}, radius × ${scale.toFixed(3)}, footprint mismatch side ${(left[0]! * 1000).toFixed(1)} mm, front ${(left[1]! * 1000).toFixed(1)} mm`;
  }

  /** 🥚️ The convex hull of points (monotone chain), counter-clockwise. */
  function hull(points: readonly P2[]): P2[] {
    const sorted = [...points].sort((p, q) => p[0] - q[0] || p[1] - q[1]);
    const turn = (o: P2, p: P2, q: P2): number => (p[0] - o[0]) * (q[1] - o[1]) - (p[1] - o[1]) * (q[0] - o[0]);
    const half = (list: readonly P2[]): P2[] => {
      const out: P2[] = [];
      for (const p of list) {
        while (out.length >= 2 && turn(out[out.length - 2]!, out[out.length - 1]!, p) <= 1e-12) out.pop();
        out.push(p);
      }
      return out.slice(0, -1);
    };
    return [...half(sorted), ...half(sorted.reverse())].map(([x, y]) => [round(x, 4), round(y, 4)] as const);
  }

  function anchorSplit(op: any): string {
    const index = asset.anchors.findIndex((candidate: any) => candidate.id === op.id);
    const ids = (op.into as any[]).map((part) => part.id as string);
    if (index < 0) return ids.every((id) => asset.anchors.some((candidate: any) => candidate.id === id)) ? "unchanged" : (() => { throw new Error(`no anchor "${op.id}"`); })();
    const old = asset.anchors[index];
    asset.anchors.splice(index, 1, ...(op.into as any[]).map((part) => ({ ...structuredClone(old), id: part.id, title: part.title, ...(part.set ?? {}), z: part.z })));
    let renamed = 0;
    const visit = (node: any): void => {
      if (Array.isArray(node)) {
        for (let i = node.length - 1; i >= 0; i--) {
          if (node[i] === op.id) (node.splice(i, 1, ...ids), renamed++);
          else if (node[i] !== null && typeof node[i] === "object" && node[i].anchor === op.id) (node.splice(i, 1, ...ids.map((id) => ({ ...structuredClone(node[i]), anchor: id }))), renamed++);
          else visit(node[i]);
        }
      } else if (node !== null && typeof node === "object") for (const value of Object.values(node)) visit(value);
    };
    for (const key of ["poses", "clips", "structure", "thermal", "presentation"]) visit(asset[key]);
    return `split into ${ids.join(", ")}, ${renamed} references renamed`;
  }

  function planAlong(op: any): string {
    const shape = need(op.scale, "plan", op.id, op.bone);
    const frame = frames[shape.bone]!.frame;
    if (shape.solid !== undefined && !(shape.solid.kind === "extruded" && shape.solid.sweep === "u")) throw new Error(`1:${op.scale} plan "${op.id}" is a solid that is not swept along the bone`);
    const first = snapshot(shape);
    if (shape.solid === undefined) {
      if (Math.abs(frame.v[0]) < 0.5 || Math.abs(frame.w[2]) < 0.9) throw new Error(`1:${op.scale} plan "${op.id}": the bone does not stand upright in the sagittal plane, its flat plan outline is no transverse section`);
      const flat = pointsOf(shape).map(([x, z]) => [round((x - frame.o[0]) / frame.v[0], 4), round((z - frame.o[2]) / frame.w[2], 4)] as const);
      for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
      Object.assign(shape, { kind: "poly", points: flat, closed: true, solid: { kind: "extruded", sweep: "u", z0: 0, z1: 0.01 } });
    }
    const length = lengthOf(shape.bone);
    const target = op.twins !== undefined ? hull((op.twins as string[]).flatMap((id) => pointsOf(need(op.scale, "elevation", id, shape.bone)))) : footprint(need(op.scale, "elevation", op.twin ?? op.id, shape.bone), "elevation", frame, length, 1);
    if (target === undefined) throw new Error(`1:${op.scale} "${op.id}": the elevation twin gives no (u, v) footprint`);
    const before = first;
    if (op.clip === true) {
      shape.points = clipStretch(pointsOf(shape), rangeOf(target, 1)).map(([v, w]) => [round(v, 4), round(w, 4)]);
      shape.kind = "poly";
      shape.closed = true;
    }
    const [v0, v1] = rangeOf(pointsOf(shape), 0);
    const [a, b] = rangeOf(target, 0);
    const box = (p: readonly number[]): P2[] => [[p[0]!, v0], [p[0]!, v1], [p[1]!, v0], [p[1]!, v1]];
    const fitted = search([a, b], (p) => mismatch(box(p), target) + (p[0]! >= p[1]! - 0.002 ? 1 : 0));
    const { zTip: _tip, ...kept } = shape.solid;
    shape.solid = { ...kept, z0: round(fitted[0]!), z1: round(fitted[1]!) };
    return `${settle(shape, before) ? "fitted" : "unchanged"}: u [${fitted.map((v) => v.toFixed(3))}] of elevation [${a.toFixed(3)},${b.toFixed(3)}], v [${v0.toFixed(3)},${v1.toFixed(3)}] of elevation [${rangeOf(target, 1).map((v) => v.toFixed(3))}], footprint mismatch ${(mismatch(box(fitted), target) * 1000).toFixed(1)} mm`;
  }

  function copy(op: any): string {
    const source = need(op.fromScale ?? op.scale, op.from, op.id, op.bone);
    const done: string[] = [];
    for (const view of op.to as string[]) {
      let target = find(op.scale, view, op.as ?? op.id, source.bone);
      if (target === undefined) {
        if (op.create !== true) throw new Error(`1:${op.scale} ${view} has no shape "${op.as ?? op.id}" on ${source.bone} (state "create": true to add it)`);
        target = { ...structuredClone(source), id: op.as ?? op.id, ...(op.paint ?? {}) };
        shapesOf(op.scale, view).push(target);
        done.push(`${view} created`);
        continue;
      }
      const before = JSON.stringify(target);
      copyGeometry(source, target);
      done.push(`${view} ${JSON.stringify(target) === before ? "unchanged" : "copied"}`);
    }
    return done.join(", ");
  }

  /** 🌀️ Writes a body of revolution about the bone axis into a shape: the axis offset (av, aw) from the bone origin in the bone-local (v, w) plane and the radius profile (u, r). */
  function revolvedGeometry(shape: any, frame: Frame, av: number, aw: number, radial: readonly P2[], label: string): void {
    const t = Math.abs(frame.u[1]) < 0.05 ? 0 : -(av * frame.v[1] + aw * frame.w[1]) / frame.u[1];
    const offset: V3 = [av * frame.v[0] + aw * frame.w[0] + t * frame.u[0], 0, av * frame.v[2] + aw * frame.w[2] + t * frame.u[2]];
    if (Math.abs(dot(offset, frame.v) - av) > 0.0005) throw new Error(`${label}: the bone lies level, so the axis of a revolved solid cannot be offset along its normal by ${(av * 1000).toFixed(0)} mm (the offset of an axis is horizontal): use the op "extrude" with "view": "plan"`);
    for (const key of GEOMETRY_KEYS) if (key !== "closed") delete shape[key];
    const skew = round(offset[2]) - aw;
    const range = [radial[0]![0], radial[radial.length - 1]![0]];
    const outline = [...radial.map(([u, r]) => [round(u, 4), round(av + Math.sqrt(Math.max(0, r ** 2 - skew ** 2)), 4)]), ...(radial.some(([, r]) => r < Math.abs(skew) + 0.0005) ? [] : [[round(range[1]!, 4), round(av, 4)], [round(range[0]!, 4), round(av, 4)]])];
    Object.assign(shape, { kind: "poly", points: outline, closed: true, solid: { kind: "revolved", axisBone: shape.bone, axis: "u", axisX: round(offset[0]), axisZ: round(offset[2]), mirror: true } });
  }

  /** 🧊️ The body of a part as the volume of the scale models it: the vertices of the body solids of the bone (or of the solids named) in the bone-local (u, v, w) frame of the rest pose, as stations along the bone with the extent across it. */
  function volumeLocal(scale: string, bone: string, ids: readonly string[] | undefined, bones: readonly string[] = [bone]): V3[] {
    const vec = (a: V3) => ({ x: a[0], y: a[1], z: a[2] });
    const posed = Object.fromEntries(Object.entries(frames).map(([id, { frame }]) => [id, { frame: { o: vec(frame.o), u: vec(frame.u), v: vec(frame.v), w: vec(frame.w) } }]));
    const worlds = (worldSolids(asset, posed, Number(scale)) as any[]).filter((world) => world.texture !== true && (ids === undefined ? bones.includes(world.bone) : ids.includes(world.id)));
    if (worlds.length === 0) throw new Error(`1:${scale}: the volume has no body solid ${ids === undefined ? `on ${bone}` : ids.join(", ")}`);
    const f = frames[bone]!.frame;
    const local: V3[] = [];
    for (const world of worlds)
      for (let i = 0; i < world.positions.length; i += 3) {
        const d: V3 = [world.positions[i] - f.o[0], world.positions[i + 1] - f.o[1], world.positions[i + 2] - f.o[2]];
        local.push([dot(d, f.u), dot(d, f.v), dot(d, f.w)]);
      }
    return local;
  }

  /** 🪜️ The vertices of a part as stations along the bone with the extent across it. */
  function volumeStations(local: readonly V3[], count: number): { u: number; v: [number, number]; w: [number, number] }[] {
    const extent = (points: readonly V3[], k: 0 | 1 | 2): [number, number] => points.reduce<[number, number]>(([lo, hi], p) => [Math.min(lo, p[k]), Math.max(hi, p[k])], [Infinity, -Infinity]);
    const [a, b] = extent(local, 0);
    const station = (points: readonly V3[], u: number) => ({ u, v: extent(points, 1), w: extent(points, 2) });
    const out = [station(local.filter((p) => p[0] <= a + 0.0005), a)];
    for (let i = 0; i < count; i++) {
      const [lo, hi] = [a + ((b - a) * i) / count, a + ((b - a) * (i + 1)) / count];
      const inside = local.filter((p) => p[0] > Math.max(lo, a + 0.0005) && p[0] <= hi && p[0] < b - 0.0005);
      if (inside.length > 0) out.push(station(inside, inside.reduce((sum, p) => sum + p[0], 0) / inside.length));
    }
    out.push(station(local.filter((p) => p[0] >= b - 0.0005), b));
    return out;
  }

  /** 📈️ The straight line through values over u by least squares with weights, read at two places. */
  const lineAt = (samples: readonly (readonly [number, number, number])[], at: readonly number[]): number[] => {
    const sw = samples.reduce((sum, [, , weight]) => sum + weight, 0);
    const mu = samples.reduce((sum, [u, , weight]) => sum + weight * u, 0) / sw;
    const my = samples.reduce((sum, [, y, weight]) => sum + weight * y, 0) / sw;
    const suu = samples.reduce((sum, [u, , weight]) => sum + weight * (u - mu) ** 2, 0);
    const k = suu < 1e-12 ? 0 : samples.reduce((sum, [u, y, weight]) => sum + weight * (u - mu) * (y - my), 0) / suu;
    return at.map((u) => my + k * (u - mu));
  };

  function volumeSolid(op: any): string {
    const idIn = (view: string): string => op.ids?.[view] ?? op.id;
    const views = ((op.views as string[] | undefined) ?? ["elevation", "front", "plan"]).filter((view) => find(op.scale, view, idIn(view), op.bone) !== undefined);
    if (views.length === 0) throw new Error(`1:${op.scale} has no shape "${op.id}"`);
    const first = need(op.scale, views[0]!, idIn(views[0]!), op.bone);
    const frame = frames[first.bone]!.frame;
    const length = lengthOf(first.bone);
    const named = (asset.volumes[op.scale]?.body ?? []).filter((solid: any) => solid.bone === first.bone && views.some((view) => solid.id === idIn(view))).map((solid: any) => solid.id as string);
    const alone = views.every((view) => shapesOf(op.scale, view).filter((shape) => shape.bone === first.bone && shape.role !== "ground").length === 1);
    const drawsIn = (bone: string, view: string): boolean => shapesOf(op.scale, view).some((shape) => shape.bone === bone && shape.role !== "ground");
    const below = (bone: string): string[] => asset.rig.bones.filter((candidate: any) => candidate.parent === bone && !views.every((view) => drawsIn(candidate.id, view))).flatMap((candidate: any) => [candidate.id as string, ...below(candidate.id)]);
    const carried = op.undrawn === false ? [] : below(first.bone);
    const local = volumeLocal(op.scale, first.bone, op.solids ?? (named.length > 0 && !alone ? named : undefined), [first.bone, ...carried]);
    const all = volumeStations(local, op.stations ?? 24);
    const half = (s: (typeof all)[number]): number => (s.v[1] - s.v[0] + s.w[1] - s.w[0]) / 4;
    const body = all.filter((s) => half(s) > 0.002);
    const built: any = { bone: first.bone };
    const ring = (upper: readonly P2[], lower: readonly P2[]): number[][] => [...upper, ...[...lower].reverse()].map(([u, y]) => [round(u, 4), round(y, 4)]).filter((p, i, list) => Math.hypot(p[0]! - list[(i + 1) % list.length]![0]!, p[1]! - list[(i + 1) % list.length]![1]!) > 0.0002);
    if (op.as === "revolved") {
      const weight = body.reduce((sum, s) => sum + half(s), 0);
      const av = op.axis?.[0] ?? body.reduce((sum, s) => sum + (half(s) * (s.v[0] + s.v[1])) / 2, 0) / weight;
      const aw = op.axis?.[1] ?? body.reduce((sum, s) => sum + (half(s) * (s.w[0] + s.w[1])) / 2, 0) / weight;
      revolvedGeometry(built, frame, av, aw, all.map((s) => [s.u, Math.max(half(s), 0.0005)] as const), `1:${op.scale} "${op.id}"`);
    } else if (op.as === "prism-w" || op.as === "prism-v") {
      const lateral = op.as === "prism-w";
      const across = (s: (typeof all)[number]): [number, number] => (lateral ? s.w : s.v);
      const span = [0, 1].map((k) => lineAt(body.map((s) => [s.u, across(s)[k]!, half(s)] as const), [0, length]));
      if (span[0]![0]! >= span[1]![0]! - 0.002 || span[0]![1]! >= span[1]![1]! - 0.002) throw new Error(`1:${op.scale} "${op.id}": the volume gives no span across the profile`);
      const base = lateral ? frame.o[2] : 0;
      const drawn = op.profile === "elevation" && lateral ? need(op.scale, "elevation", idIn("elevation"), first.bone) : undefined;
      if (drawn !== undefined && drawn.solid !== undefined && !(drawn.solid.kind === "extruded" && drawn.solid.sweep === "w")) throw new Error(`1:${op.scale} elevation "${drawn.id}" is a solid whose points are no side outline`);
      const outline = drawn !== undefined ? structuredClone(pointsOf(drawn)) : ring(all.map((s) => [s.u, (lateral ? s.v : s.w)[1]] as const), all.map((s) => [s.u, (lateral ? s.v : s.w)[0]] as const));
      Object.assign(built, { kind: "poly", points: outline, closed: true, solid: { kind: "extruded", sweep: lateral ? "w" : "v", z0: round(span[0]![0]! + base), z1: round(span[1]![0]! + base), zTip: [round(span[0]![1]! + base), round(span[1]![1]! + base)] } });
    } else if (op.as === "prism-u") {
      Object.assign(built, { kind: "poly", points: hull(local.map((p) => [p[1], p[2]] as const)), closed: true, solid: { kind: "extruded", sweep: "u", z0: round(all[0]!.u), z1: round(all[all.length - 1]!.u) } });
    } else throw new Error(`"as" must be "revolved", "prism-w", "prism-v" or "prism-u"`);
    const done: string[] = [];
    for (const view of views) {
      const shape = need(op.scale, view, idIn(view), first.bone);
      const before = snapshot(shape);
      copyGeometry(built, shape);
      done.push(`${view} ${settle(shape, before) ? "built" : "unchanged"}`);
    }
    return `${done.some((line) => line.endsWith("built")) ? "" : "unchanged: "}${done.join(", ")}: ${op.as} of the volume, ${all.length} stations, u [${all[0]!.u.toFixed(3)},${all[all.length - 1]!.u.toFixed(3)}], solid ${JSON.stringify(built.solid)}`;
  }

  function apply(op: any): string {
    switch (op.op) {
      case "front-sweep": return frontSweep(op);
      case "plan-revolve": return planRevolve(op);
      case "extrude": return frontExtrude(op);
      case "copy": return copy(op);
      case "anchor-split": return anchorSplit(op);
      case "plan-along": return planAlong(op);
      case "volume": return volumeSolid(op);
      case "set": {
        const shape = need(op.scale, op.view, op.id, op.bone);
        const before = JSON.stringify(shape);
        for (const [key, value] of Object.entries(op.set)) value === null ? delete shape[key] : (shape[key] = structuredClone(value));
        return JSON.stringify(shape) === before ? "unchanged" : "set";
      }
      case "remove": {
        const list = shapesOf(op.scale, op.view);
        const index = list.findIndex((shape) => shape.id === op.id && (op.bone === undefined || shape.bone === op.bone));
        if (index < 0) return "unchanged";
        list.splice(index, 1);
        return "removed";
      }
      case "anchor": {
        const anchor = asset.anchors.find((candidate: any) => candidate.id === op.id);
        if (anchor === undefined) throw new Error(`no anchor "${op.id}"`);
        const before = JSON.stringify(anchor);
        for (const [key, value] of Object.entries(op.set)) value === null ? delete anchor[key] : (anchor[key] = value);
        return JSON.stringify(anchor) === before ? "unchanged" : "set";
      }
      default: throw new Error(`unknown op "${op.op}"`);
    }
  }
  return apply;
}

/** 🧾️ Applies a list of ops in order; an op that cannot be applied is refused by name and the others still run. */
export function applyOps(asset: any, ops: readonly any[]): { lines: string[]; changed: number; refused: number } {
  const apply = fitter(asset);
  const lines: string[] = [];
  let changed = 0;
  let refused = 0;
  for (const op of ops) {
    let result: string;
    try {
      result = apply(op);
    } catch (error) {
      refused++;
      result = `unchanged, REFUSED: ${(error as Error).message}`;
    }
    if (!result.startsWith("unchanged") && !/^(\w+ unchanged(, )?)+$/.test(result)) changed++;
    lines.push(`${asset.id} ${op.op} ${op.scale ? `1:${op.scale} ` : ""}${op.view ?? ""}${op.view ? " " : ""}${op.id}: ${result}`);
  }
  return { lines, changed, refused };
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const flag = (name: string): string | undefined => {
    const i = args.indexOf(`--${name}`);
    return i < 0 ? undefined : args[i + 1];
  };
  const read = readDoc(args[0]!);
  const ops = JSON.parse(await Bun.file(flag("ops")!).text()) as any[];
  const { lines, changed, refused } = applyOps(read.doc.asset, ops);
  for (const line of lines) console.log(line);
  if (args.includes("--write") || flag("out") !== undefined) console.log(writeDoc(read, flag("out")) ? `written: ${flag("out") ?? read.file}` : "file unchanged");
  console.log(`${read.doc.asset.id}: ${ops.length} ops, ${changed} changed something, ${refused} refused`);
  process.exit(refused === 0 ? 0 : 1);
}
