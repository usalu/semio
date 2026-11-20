/** ✏️ Redraws the 2D drawings of `humans/sleeping-person` (ruling E1) from the body of every scale in the supine pose. Which solids one drawn shape unites, what a view leaves out and the detail roles are stated by hand below; the layer order of a view is the order of the parts along its viewing direction.
 *
 * Every bone of the woman is turned out of the sagittal plane in some pose (the legs in the supine pose, pelvis, trunk and head in the side poses), so every part on her bones is a solid of the rig: sleeves are revolved profiles and every other part is its true outline in the supine pose laid as a thin plate through the middle of the part in the bone plane that faces the view. The duvet mounds are drawn on the duvet bone in every view (in 3D they follow her trunk and legs only partly, the cloth stays on the bed); trunk, neck and head lie behind the duvet in that view and are not drawn there.
 *
 * The body under the duvet (shorts, thighs, shanks, the left arm and hand) is not drawn: the duvet covers it in every view. Upper arms, forearms and hands are spread and twisted by the poses, so a sleeve, forearm sleeve or cuff is its radius profile revolved about the bone and a hand is its true outline in the supine pose laid as a thin plate through the middle of the hand, in the plane of the bone that faces the view (the office worker's method). Every other part is a flat outline: the true orthographic outline of its solids solved into the frame of its bone through the product's own placement, exact in the default pose.
 *
 * Usage: `bun r3-sleeping-person-draw2d.ts` writes `🗑️generated/r3-sleeping-person/geometry.json` (read by `r3-sleeping-person-build.ts`).
 * @see ./r3-sleeping-person-build.ts
 * @see ./r3-seated-office-worker-draw2d.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { OUT, SCALES, build, candidate } from "./r3-sleeping-person-build.ts";

type V3 = [number, number, number];
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const SPREAD = /^(arm|forearm|hand)-[lr]$/;
const VIEW_AXIS: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
const PLATE = 0.0004;
const FIXED = /^(bed-surface|pillow|duvet|hip-lift|hip-shift)$/;
const MOUND = /^(duvet-(trunk|hips|thighs|shins|legs)|fold-(thighs|shins|legs))$/;
const FRONT_HIDDEN = /^(trunk|collar|neck|head|hair|hair-spread|nose|eyelid-[lr]|lips|ankle-[lr]|shoulders)$/;
const HIDDEN = /^(shorts|thigh-[lr]|shank-[lr]|(sleeve|foresleeve|cuff|palm|thenar|thumb|thumb-tip)-l|finger-\d(-tip)?-l)$/;

/** 🧩️ The drawn shape a body solid belongs to in a view (`null`: not drawn there). */
function groupOf(solid: Json, view: View, scale: number): string | null {
  const id = solid.id as string;
  if (HIDDEN.test(id)) return null;
  if (view === "front" && FRONT_HIDDEN.test(id)) return null;
  const hand = /^(palm|thenar|finger-\d|finger-\d-tip|thumb|thumb-tip)-([lr])$/.exec(id);
  if (hand) return `hand-${hand[2]}`;
  if (id === "thumb-r") return "hand-r";
  if (id === "nose") return view === "elevation" || scale > 50 ? "head" : "nose";
  if (/^eyelid-l$/.test(id) && view === "elevation") return null;
  const mall = /^malleolus-(in|out)-([lr])$/.exec(id);
  if (mall) return `foot-${mall[2]}`;
  const toe = /^toe-\d-([lr])$/.exec(id);
  if (toe) return `toes-${toe[1]}`;
  const side = /^(flat|edge|drape)-([lr])$/.exec(id);
  if (side) return `duvet-side-${side[2]}`;
  const folded = /^fold-(flat|edge|drape)-([lr])$/.exec(id);
  if (folded) return `duvet-fold-${folded[2]}`;
  return id;
}

/** 🎚️ How much nearer to the viewer than its own middle a part is laid in the layer order (metres): details lie on the mass that carries them. */
const LIFT: readonly (readonly [RegExp, number])[] = [[/^(eyelid|lips)/, 0.12], [/^nose$/, 0.1], [/^hair$/, 0.02], [/^cuff-/, 0.01], [/^collar$/, 0.02], [/^hand-r$/, 0.05], [/^(sleeve|foresleeve)-r$/, 0.04], [/^toes-/, 0.01]];
const ROLE_DETAIL = /^(eyelid|lips|nose|collar|cuff-)/;

type Shape = Json;
type Part = { id: string; solids: Json[]; own: Json };

/** 🌀️ A sleeve, forearm sleeve or cuff as its radius profile revolved about the bone axis. */
function revolved(part: Part): Shape | null {
  if (part.solids.length !== 1 || part.own.kind !== "loft" || !/^(sleeve|foresleeve|cuff)-/.test(part.id)) return null;
  const sections = part.own.sections as Json[];
  const mean = (k: number): number => sections.reduce((s, c) => s + c.at[k], 0) / sections.length;
  const [cv, cw] = [mean(1), mean(2)];
  const points = [[sections[0].at[0], cv], ...sections.map((c) => [c.at[0], cv + (c.rx + c.ry) / 2]), [sections[sections.length - 1].at[0], cv]];
  return { kind: "poly", points: points.map(([x, y]) => [r4(x!), r4(y!)]), closed: true, solid: { kind: "revolved", axisBone: part.own.bone, axis: "u", axisX: r4(cv), axisZ: r4(cw), mirror: true } };
}

/** 🪢️ The convex hull of projected points, counter-clockwise. */
function hull(points: readonly P2[]): P2[] {
  const p = [...points].sort((a, b) => a.x - b.x || a.y - b.y);
  const cross = (o: P2, a: P2, b: P2): number => (a.x - o.x) * (b.y - o.y) - (a.y - o.y) * (b.x - o.x);
  const lower: P2[] = [];
  for (const q of p) {
    while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop();
    lower.push(q);
  }
  const upper: P2[] = [];
  for (const q of [...p].reverse()) {
    while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop();
    upper.push(q);
  }
  return [...lower.slice(0, -1), ...upper.slice(0, -1)];
}

const geometry: Record<string, Record<string, Shape[]>> = {};
const asset = candidate([], false).asset as Json;
const REST = Object.fromEntries(Object.entries(restFrames(asset.rig) as Record<string, { frame: Frame }>).map(([id, b]) => [id, b.frame]));
for (const scale of SCALES) {
  const { volume, kit } = build(asset, scale);
  const frames = rigAt(asset, scale, { kind: "pose", pose: asset.presentation.defaultPose }).frames;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const views: Record<string, Shape[]> = {};
  const work = JSON.parse(JSON.stringify(asset)) as Json;
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of volume.body as Json[]) {
      const g = groupOf(s, view, scale);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    const drawn: { shape: Shape; near: number }[] = [];
    const flat: { meta: Json; target: P2[]; closed: boolean; near: number }[] = [];
    const axis = VIEW_AXIS[view];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids[0]!;
      const part: Part = { id, solids, own };
      const meshes = meshesIn(asset, frames, solids);
      let [sum, n] = [0, 0];
      const middle: V3 = [0, 0, 0];
      for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) {
        sum += m.world[i]! * axis[0] + m.world[i + 1]! * axis[1] + m.world[i + 2]! * axis[2];
        for (let k = 0; k < 3; k++) middle[k]! += m.world[i + k]!;
        n++;
      }
      const near = sum / n + (LIFT.find(([re]) => re.test(id))?.[1] ?? 0);
      const role = ROLE_DETAIL.test(id) ? "detail" : "body";
      const onDuvet = MOUND.test(id);
      const affected = !onDuvet && !FIXED.test(own.bone);
      const meta: Json = { id, bone: onDuvet ? "duvet" : own.bone, role, z: 0, filled: true, tone: own.tone, region: kit.region.get(own.id), material: own.material };
      const made = SPREAD.test(own.bone) ? revolved(part) : null;
      if (made !== null) {
        drawn.push({ shape: { ...meta, ...made }, near });
        continue;
      }
      if (solids.length === 1 && /^(box|ellipsoid|capsule)$/.test(own.kind) && !affected) {
        flat.push({ meta, target: simplifyRing(hull(meshes.flatMap((m) => Array.from({ length: m.world.length / 3 }, (_, i) => projectWorld(view, [m.world[i * 3]!, m.world[i * 3 + 1]!, m.world[i * 3 + 2]!])))), tol), closed: true, near });
        continue;
      }
      const mask = projectMask(view, meshes, cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((s: number, r: Ring) => s + area(r), 0);
      if (lost > 0.03 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const ring = simplifyRing(rings[0]!, tol);
      if (!affected) {
        flat.push({ meta, target: ring, closed: true, near });
        continue;
      }
      const f = frames[own.bone]!;
      const facing = (a: V3): number => Math.abs(a[0] * axis[0] + a[1] * axis[1] + a[2] * axis[2]);
      const sweep = facing(f.w as V3) >= facing(f.v as V3) ? "w" : "v";
      const c: V3 = [middle[0] / n - f.o[0], middle[1] / n - f.o[1], middle[2] / n - f.o[2]];
      const across = sweep === "w" ? f.w : f.v;
      const c0 = c[0] * across[0] + c[1] * across[1] + c[2] * across[2];
      const base = projectWorld(view, [f.o[0] + c0 * across[0], f.o[1] + c0 * across[1], f.o[2] + c0 * across[2]]);
      const [a, b] = [projectWorld(view, f.u as V3), projectWorld(view, (sweep === "w" ? f.v : f.w) as V3)];
      const det = a.x * b.y - a.y * b.x;
      if (Math.abs(det) < 0.15) console.log(`  [DEBUG] ${scale} ${view} ${id}: plate seen at ${Math.abs(det).toFixed(2)}`);
      const points = ring.map((p) => {
        const [dx, dy] = [p.x - base.x, p.y - base.y];
        return [r4((b.y * dx - b.x * dy) / det), r4((-a.y * dx + a.x * dy) / det)];
      });
      const origin = sweep === "w" ? REST[own.bone]!.o[2] + c0 : c0;
      drawn.push({ shape: { ...meta, kind: "poly", points, closed: true, solid: { kind: "extruded", sweep, z0: r4(origin - PLATE), z1: r4(origin + PLATE) } }, near });
    }
    const solved = inverse(work, scale, view, flat);
    flat.forEach((j, k) => {
      if (solved[k]!.residualM > 1e-5) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm`);
      drawn.push({ shape: { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: j.closed }, near: j.near });
    });
    drawn.sort((p, q) => p.near - q.near);
    views[view] = drawn.map(({ shape }, i) => ({ ...shape, z: r4(1 + i * 0.1) }));
    work.representations[String(scale)].geometry[view] = views[view];
    console.log(scale, view, `${views[view]!.length} shapes, ${views[view]!.reduce((n, s) => n + s.points.length, 0)} points`);
  }
  geometry[String(scale)] = views;
}
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "geometry.json"), JSON.stringify(geometry));
