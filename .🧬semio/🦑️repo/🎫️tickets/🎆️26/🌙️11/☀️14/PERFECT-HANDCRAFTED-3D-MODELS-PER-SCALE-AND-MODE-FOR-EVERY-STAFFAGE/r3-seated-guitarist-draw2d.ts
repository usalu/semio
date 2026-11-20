/** ✏️ Redraws the 2D drawings of `humans/seated-guitarist` (ruling E1) from the body of every scale in the default pose. Which solids one drawn shape unites, what is left out of a view and the detail strokes are stated by hand below; the layer order of a view is the order of the parts along its viewing direction.
 *
 * How a part is drawn depends on its bone. Stool, pelvis and trunk never leave the sagittal plane: their shapes are flat outlines, the true orthographic outline solved into the frame of the bone through the product's own placement. Head, limbs, hands, feet and guitar are spread or twisted by the poses, so every shape on them is a solid of the rig that all poses project in three dimensions: a limb segment is its radius profile revolved about the bone, a part of the guitar is its outline on the top swept through its thickness (the guitar is built of prisms), and an irregular part (head, hair, hand, boot) is its true outline in the default pose laid as a thin plate through the middle of the part, in the plane of the bone that faces the view.
 *
 * Usage: `bun r3-seated-guitarist-draw2d.ts` writes `🗑️generated/r3-seated-guitarist/geometry.json` (read by `r3-seated-guitarist-build.ts`).
 * @see ./r3-seated-guitarist-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, localGeometry, projectWorld, qualityOfAsset, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { OUT, SCALES, build, candidate } from "./r3-seated-guitarist-build.ts";
import { shirtFront } from "./r3-seated-guitarist-s50.ts";

type V3 = [number, number, number];
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const FLAT_BONES = new Set(["stool", "pelvis", "torso"]);
const VIEW_AXIS: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
const PLATE = 0.0004;

/** 🧩️ The drawn shape a body solid belongs to in a view (`null`: not drawn there). */
function groupOf(id: string, view: View, scale: number): string | null {
  if (/^(shirt-(placket|button|pocket|flap)|guitar-string|boot-shaft)/.test(id)) return null;
  const hand = /^(palm|thenar|finger-\d|fingers|thumb)-([lr])$/.exec(id);
  if (hand) return `hand-${hand[2]}`;
  if (id === "nose") return "head";
  const sole = /^boot-(heel|sole)-([lr])$/.exec(id);
  if (sole) return `boot-sole-${sole[2]}`;
  if (/^(eye|brow|mouth)/.test(id) && view === "plan") return null;
  if (/^(eye|brow|ear)-l$/.test(id) && view === "elevation") return null;
  if (id === "tee-neck" && view !== "front") return null;
  if (/^guitar-(heel)$/.test(id) && view === "front") return null;
  if (scale >= 100 && id === "guitar-heel" && view !== "elevation") return null;
  return id;
}

/** 🎚️ How much nearer to the viewer than its own middle a part is laid in the layer order (metres): surface details lie on the mass that carries them. */
const LIFT: readonly (readonly [RegExp, number])[] = [[/^(eye|brow|mouth)/, 0.12], [/^(beard|hair)$/, 0.03], [/^ear-/, 0.02], [/^guitar-(soundhole|pickguard|bridge)/, 0.2], [/^guitar-top/, 0.15], [/^guitar-fretboard/, 0.05], [/^guitar-tuner/, 0.05], [/^cuff-/, 0.01], [/^shirt-collar/, 0.08], [/^tee-neck/, 0.1], [/^boot-toe-[lr]/, 0.004]];
const ROLE_DETAIL = /^(eye|brow|mouth|guitar-(soundhole|pickguard|bridge|tuner))/;

type Shape = Json;
type Part = { id: string; solids: Json[]; own: Json };

/** 🌀️ A limb segment as its radius profile revolved about the bone axis (through the middle of the stated sections). */
function revolved(part: Part): Shape | null {
  if (part.solids.length !== 1 || part.own.kind !== "loft" || !/^(sleeve|forearm|cuff|wrist|thigh|shank)-/.test(part.id)) return null;
  const sections = part.own.sections as Json[];
  const mean = (k: number): number => sections.reduce((s, c) => s + c.at[k], 0) / sections.length;
  const [cv, cw] = [mean(1), mean(2)];
  const points = [[sections[0].at[0], cv], ...sections.map((c) => [c.at[0], cv + (c.rx + c.ry) / 2]), [sections[sections.length - 1].at[0], cv]];
  return { kind: "poly", points: points.map(([x, y]) => [r4(x!), r4(y!)]), closed: true, solid: { kind: "revolved", axisBone: part.own.bone, axis: "u", axisX: r4(cv), axisZ: r4(cw), mirror: true } };
}

/** 🧇️ A part of the guitar as its outline on the plane of the top (u along the strings, w across) swept through its thickness along v; the thickness runs linearly along the bone, as the tapering sound box and the angled head do. */
function sweptAcross(part: Part, length: number, asset: Json): Shape | null {
  if (part.own.bone !== "guitar" || part.solids.length !== 1) return null;
  const s = part.own;
  const fit = (pairs: readonly (readonly [number, number])[]): [number, number] => {
    const n = pairs.length;
    const [sx, sy, sxx, sxy] = pairs.reduce(([a, b, c, d], [x, y]) => [a + x, b + y, c + x * x, d + x * y], [0, 0, 0, 0]);
    const slope = n * sxx - sx * sx < 1e-12 ? 0 : (n * sxy - sx * sy) / (n * sxx - sx * sx);
    const at0 = (sy - slope * sx) / n;
    return [at0, at0 + slope * length];
  };
  if (s.kind === "loft" && s.sections[0].rotation !== undefined && Math.abs(s.sections[0].rotation[1]) > 0.5) {
    const c = s.sections as Json[];
    const profile = [...c.map((q) => [q.at[0], q.at[2] + q.rx]), ...[...c].reverse().map((q) => [q.at[0], q.at[2] - q.rx])];
    const inside = c.filter((q) => q.at[0] >= -1e-9);
    const use = inside.length >= 2 ? inside : c;
    const lo = fit(use.map((q) => [q.at[0], q.at[1] - q.ry] as const));
    const hi = fit(use.map((q) => [q.at[0], q.at[1] + q.ry] as const));
    const taper = Math.abs(lo[1] - lo[0]) > 0.002 || Math.abs(hi[1] - hi[0]) > 0.002;
    return { kind: "poly", points: profile.map(([x, y]) => [r4(x!), r4(y!)]), closed: true, solid: { kind: "extruded", sweep: "v", z0: r4(lo[0]), z1: r4(hi[0]), ...(taper ? { zTip: [r4(lo[1]), r4(hi[1])] } : {}) } };
  }
  const g = localGeometry(s, qualityOfAsset(asset)).positions;
  const [t, radii] = [s.transform?.translation ?? [0, 0, 0], s.radii];
  let [v0, v1] = [Infinity, -Infinity];
  const box = { u0: Infinity, u1: -Infinity, w0: Infinity, w1: -Infinity };
  for (let i = 0; i < g.length; i += 3) {
    [v0, v1] = [Math.min(v0, g[i + 1]!), Math.max(v1, g[i + 1]!)];
    [box.u0, box.u1, box.w0, box.w1] = [Math.min(box.u0, g[i]!), Math.max(box.u1, g[i]!), Math.min(box.w0, g[i + 2]!), Math.max(box.w1, g[i + 2]!)];
  }
  let profile: number[][];
  if (s.kind === "ellipsoid") {
    const q = s.transform?.rotation as number[] | undefined;
    const turn = q === undefined ? 0 : 2 * Math.atan2(q[1]!, q[3]!);
    profile = Array.from({ length: 16 }, (_, i) => {
      const [x, z] = [radii[0] * Math.cos((i * Math.PI) / 8), radii[2] * Math.sin((i * Math.PI) / 8)];
      return [t[0] + x * Math.cos(turn) + z * Math.sin(turn), t[2] - x * Math.sin(turn) + z * Math.cos(turn)];
    });
  } else profile = [[box.u0, box.w0], [box.u1, box.w0], [box.u1, box.w1], [box.u0, box.w1]];
  if (v1 - v0 < 0.001) [v0, v1] = [(v0 + v1) / 2 - 0.0005, (v0 + v1) / 2 + 0.0005];
  return { kind: "poly", points: profile.map(([x, y]) => [r4(x!), r4(y!)]), closed: true, solid: { kind: "extruded", sweep: "v", z0: r4(v0), z1: r4(v1) } };
}

type Stroke = { id: string; view: View; closed: boolean; tone: number; role: string; points: V3[] };
/** 🖋️ The detail strokes on the shirt (the trunk never leaves the sagittal plane), stated in the frame of the trunk on the surface of the shirt. */
function strokes(scale: number): Stroke[] {
  if (scale > 100) return [];
  const out: Stroke[] = [];
  const stations = [0.42, 0.34, 0.26, 0.18, 0.1, 0.02, -0.065];
  if (scale === 50) for (const side of [1, -1]) out.push({ id: `shirt-placket-${side > 0 ? "l" : "r"}`, view: "front", closed: false, tone: 0.3, role: "texture", points: stations.map((s) => [s, shirtFront(s, side * 0.016) - 0.002, side * 0.016] as V3) });
  else out.push({ id: "shirt-placket", view: "front", closed: false, tone: 0.3, role: "detail", points: stations.map((s) => [s, shirtFront(s) - 0.002, 0] as V3) });
  if (scale === 50) for (const side of [1, -1]) {
    const s = side > 0 ? "l" : "r";
    const corner = (u: number, w: number): V3 => [u, shirtFront(u, side * w) - 0.002, side * w];
    const pocket = [corner(0.315, 0.048), corner(0.315, 0.094), corner(0.315, 0.14), corner(0.215, 0.14), corner(0.208, 0.094), corner(0.215, 0.048)];
    const flap = [corner(0.29, 0.048), corner(0.284, 0.094), corner(0.29, 0.14)];
    out.push({ id: `shirt-pocket-${s}`, view: "front", closed: true, tone: 0.3, role: "detail", points: pocket }, { id: `shirt-flap-${s}`, view: "front", closed: false, tone: 0.3, role: "texture", points: flap });
    if (side < 0) out.push({ id: `shirt-pocket-${s}`, view: "elevation", closed: true, tone: 0.3, role: "detail", points: pocket });
  }
  return out;
}

const geometry: Record<string, Record<string, Shape[]>> = {};
const asset = candidate([], false).asset as Json;
const REST = Object.fromEntries(Object.entries(restFrames(asset.rig) as Record<string, { frame: Frame }>).map(([id, b]) => [id, b.frame]));
const LENGTH = Object.fromEntries((asset.rig.bones as Json[]).map((b) => [b.id, b.lengthM as number]));
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
      const g = groupOf(s.id, view, scale);
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
      const meta: Json = { id, bone: own.bone, role, z: 0, filled: true, tone: own.tone, region: kit.region.get(own.id), material: own.material };
      const made = revolved(part) ?? sweptAcross(part, LENGTH[own.bone]!, asset);
      if (made !== null) {
        drawn.push({ shape: { ...meta, ...made }, near });
        continue;
      }
      const mask = projectMask(view, meshes, cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((s: number, r: Ring) => s + area(r), 0);
      if (lost > 0.03 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      const ring = simplifyRing(rings[0]!, tol);
      if (FLAT_BONES.has(own.bone)) {
        flat.push({ meta, target: ring, closed: true, near });
        continue;
      }
      const f = frames[own.bone]!;
      const facing = (a: V3): number => Math.abs(a[0] * axis[0] + a[1] * axis[1] + a[2] * axis[2]);
      const sweep = facing(f.w) >= facing(f.v) ? "w" : "v";
      const c: V3 = [middle[0] / n - f.o[0], middle[1] / n - f.o[1], middle[2] / n - f.o[2]];
      const across = sweep === "w" ? f.w : f.v;
      const c0 = c[0] * across[0] + c[1] * across[1] + c[2] * across[2];
      const base = projectWorld(view, [f.o[0] + c0 * across[0], f.o[1] + c0 * across[1], f.o[2] + c0 * across[2]]);
      const [a, b] = [projectWorld(view, f.u), projectWorld(view, sweep === "w" ? f.v : f.w)];
      const det = a.x * b.y - a.y * b.x;
      if (Math.abs(det) < 0.15) console.log(`  [DEBUG] ${scale} ${view} ${id}: plate seen at ${Math.abs(det).toFixed(2)}`);
      const points = ring.map((p) => {
        const [dx, dy] = [p.x - base.x, p.y - base.y];
        return [r4((b.y * dx - b.x * dy) / det), r4((-a.y * dx + a.x * dy) / det)];
      });
      const origin = sweep === "w" ? REST[own.bone]!.o[2] + c0 : c0;
      drawn.push({ shape: { ...meta, kind: "poly", points, closed: true, solid: { kind: "extruded", sweep, z0: r4(origin - PLATE), z1: r4(origin + PLATE) } }, near });
    }
    const torso = frames.torso!;
    for (const s of strokes(scale).filter((q) => q.view === view)) {
      const world = s.points.map((p) => [0, 1, 2].map((k) => torso.o[k]! + p[0] * torso.u[k]! + p[1] * torso.v[k]! + p[2] * torso.w[k]!) as V3);
      const near = Math.max(...world.map((p) => p[0] * axis[0] + p[1] * axis[1] + p[2] * axis[2])) + 0.1;
      flat.push({ meta: { id: s.id, bone: "torso", role: s.role, z: 0, filled: false, tone: s.tone, region: "shirt-torso", material: "flannel" }, target: world.map((p) => projectWorld(view, p)), closed: s.closed, near });
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
