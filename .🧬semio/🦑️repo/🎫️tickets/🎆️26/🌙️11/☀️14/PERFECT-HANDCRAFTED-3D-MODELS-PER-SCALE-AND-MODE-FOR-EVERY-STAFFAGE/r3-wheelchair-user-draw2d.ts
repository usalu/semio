/** ✏️ Redraws the 2D drawings of `humans/wheelchair-user` (ruling E1) from the body of every scale in the default pose. Which solids one drawn shape unites, what a view leaves out and the strokes are stated by hand below; the layer order of a view is the order of the parts along its viewing direction.
 *
 * How a part is drawn depends on its bone. The frame, pelvis, trunk, neck, head and legs never leave the sagittal plane in any pose (their offsets are pitch only): their shapes are flat outlines, the true orthographic outline solved into the frame of the bone through the product's own placement. The rear wheels and castors turn about their axles in every clip, so each of their parts is its side profile (a disc, or an annulus drawn as two half rings) with an `extruded` solid over its real lateral span: the product draws front and plan from that one definition and they stay right however far the wheel has turned. Upper arms, forearms and hands are spread and twisted by the poses, so every shape on them is a solid of the rig that all poses project in three dimensions: a sleeve or forearm segment is its radius profile revolved about the bone, a hand is its true outline in the default pose laid as a thin plate through the middle of the hand. Slender parts (spokes, drawstrings, the 1:500 tubes) are strokes along their centre lines; the kangaroo pocket is an outline on the front.
 *
 * Usage: `bun r3-wheelchair-user-draw2d.ts` writes `🗑️generated/r3-wheelchair-user/geometry.json` (read by `r3-wheelchair-user-build.ts`).
 * @see ./r3-wheelchair-user-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";

import { restFrames } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { OUT, SCALES, build, candidate } from "./r3-wheelchair-user-build.ts";

type V3 = [number, number, number];
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const SPREAD = /^(arm|forearm|hand)-[lr]$/;
const WHEEL = /^(rear-wheel|castor)$/;
const VIEW_AXIS: Record<View, V3> = { elevation: [0, 0, -1], front: [1, 0, 0], plan: [0, 1, 0] };
const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
const PLATE = 0.0004;
const ARC: Record<number, number> = { 50: 48, 100: 36, 200: 28, 500: 20 };

/** 🧩️ The drawn shape a body solid belongs to in a view (`null`: not drawn there). */
function groupOf(solid: Json, view: View, scale: number): string | null {
  const id = solid.id as string;
  if (view !== "elevation" && /^axle-plate-[lr]$/.test(id)) return `side-tube-${id.slice(-1)}`;
  if (view !== "elevation" && scale === 100 && /^rear-hub-[lr]$/.test(id)) return null;
  if (/^hand-rim-tabs-/.test(id)) return null;
  if (/^spokes-/.test(id)) return view === "elevation" ? id : null;
  if (solid.texture === true) return view === "front" ? id : null;
  if (solid.slender === true && /^drawstring-/.test(id)) return view === "front" ? id : null;
  const glove = /^(glove-thenar|glove-finger-\d|glove-thumb|thumb)-([lr])$/.exec(id);
  if (glove && (solid.material === "leather-glove")) return `glove-${glove[2]}`;
  if (/^(nose|chin-pad|earlobe-[lr])$/.test(id)) return "head";
  if (/^(eye|brow|mouth)/.test(id) && view === "plan") return null;
  if (/^(eye|brow)-l$/.test(id) && view === "elevation") return null;
  if (/^lock-(clamp|shoe)-/.test(id)) return id.replace(/clamp|shoe/, "lever");
  if (/^lock-knob-/.test(id)) return id.replace("knob", "lever");
  if (/^(fork-crown|fork-stem|fork-leg-[ab]|castor-axle)-/.test(id)) return `castor-fork-${id.slice(-1)}`;
  if (/^(axle-receiver|axle-strut)-/.test(id)) return view === "elevation" ? `axle-plate-${id.slice(-1)}` : `side-tube-${id.slice(-1)}`;
  if (/^back-cross-arm-/.test(id)) return "back-cross";
  if (/^seat-sling$/.test(id)) return "seat-cushion";
  if (/^hoodie-hem$/.test(id) || /^hood-collar$/.test(id)) return id;
  return id;
}

/** 🎚️ How much nearer to the viewer than its own middle a part is laid in the layer order (metres): details lie on the mass that carries them. */
const LIFT: readonly (readonly [RegExp, number])[] = [[/^(eye|brow|mouth)/, 0.12], [/^beanie-cuff$/, 0.02], [/^hood-collar$/, 0.04], [/^hoodie-hem$/, 0.01], [/^hoodie-pocket$/, 0.2], [/^drawstring-/, 0.25], [/^sleeve-cuff-|^glove-cuff-/, 0.01], [/^lock-lever-/, 0.02], [/^hand-rim-/, 0.01]];
const ROLE_DETAIL = /^(eye|brow|mouth|beanie-cuff|hood-collar|hoodie-hem|sleeve-cuff|glove-cuff|lock-lever|hoodie-pocket|drawstring|rear-hub|rear-rim|hand-rim|castor-fork|axle-plate|push-grip)/;

type Shape = Json;
type Part = { id: string; solids: Json[]; own: Json };

/** 🌀️ A sleeve or forearm segment as its radius profile revolved about the bone axis (through the middle of the stated sections). */
function revolved(part: Part): Shape | null {
  if (part.solids.length !== 1 || part.own.kind !== "loft" || !/^(sleeve|forearm|sleeve-cuff|glove-cuff)-/.test(part.id)) return null;
  const sections = part.own.sections as Json[];
  const mean = (k: number): number => sections.reduce((s, c) => s + c.at[k], 0) / sections.length;
  const [cv, cw] = [mean(1), mean(2)];
  const points = [[sections[0].at[0], cv], ...sections.map((c) => [c.at[0], cv + (c.rx + c.ry) / 2]), [sections[sections.length - 1].at[0], cv]];
  return { kind: "poly", points: points.map(([x, y]) => [r4(x!), r4(y!)]), closed: true, solid: { kind: "revolved", axisBone: part.own.bone, axis: "u", axisX: r4(cv), axisZ: r4(cw), mirror: true } };
}

/** 🛞️ The side profile of a revolved wheel part about its lateral axle: a disc, or an annulus as an upper and a lower half ring; each with its lateral span as an `extruded` solid. */
function wheelProfile(solid: Json, n: number): { suffix: string; points: number[][]; z0: number; z1: number }[] {
  const prof = solid.profile as number[][];
  const ax = prof.map((p) => p[0]!);
  const rad = prof.map((p) => p[1]!);
  const [z0, z1] = [Math.min(...ax), Math.max(...ax)];
  const [r0, r1] = [Math.min(...rad), Math.max(...rad)];
  const arc = (r: number, a0: number, a1: number): number[][] => Array.from({ length: n / 2 + 1 }, (_, i) => {
    const a = a0 + ((a1 - a0) * i) / (n / 2);
    return [r4(r * Math.cos(a)), r4(r * Math.sin(a))];
  });
  if (r0 < 0.002) return [{ suffix: "", points: [...arc(r1, 0, Math.PI), ...arc(r1, Math.PI, 2 * Math.PI).slice(1, -1)], z0: r4(z0), z1: r4(z1) }];
  return [0, 1].map((h) => ({ suffix: `-${h + 1}`, points: [...arc(r1, h * Math.PI, (h + 1) * Math.PI), ...arc(r0, (h + 1) * Math.PI, h * Math.PI)], z0: r4(z0), z1: r4(z1) }));
}

/** 🪢️ The centre line of a slender solid in its bone frame (capsule ends, or the section centres of a bar), as world points; a repeated capsule gives every copy. */
function centreLines(solid: Json, f: Frame): V3[][] {
  const toWorld = (p: number[]): V3 => [0, 1, 2].map((k) => f.o[k]! + p[0]! * f.u[k]! + p[1]! * f.v[k]! + p[2]! * f.w[k]!) as V3;
  const base: number[][] = solid.kind === "capsule" ? [solid.p0, solid.p1] : (solid.sections as Json[]).map((s) => s.at);
  const count = solid.repeat?.count ?? 1;
  const step = solid.repeat?.rotationRad ?? 0;
  return Array.from({ length: count }, (_, i) => base.map((p) => {
    const a = step * i;
    return toWorld([p[0]! * Math.cos(a) - p[1]! * Math.sin(a), p[0]! * Math.sin(a) + p[1]! * Math.cos(a), p[2]!]);
  }));
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
    const axis = VIEW_AXIS[view];
    const depth = (p: readonly number[]): number => p[0]! * axis[0] + p[1]! * axis[1] + p[2]! * axis[2];
    const groups = new Map<string, Json[]>();
    for (const s of volume.body as Json[]) {
      const g = groupOf(s, view, scale);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    const drawn: { shape: Shape; near: number }[] = [];
    const flat: { meta: Json; target: P2[]; closed: boolean; near: number }[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids[0]!;
      const role = ROLE_DETAIL.test(id) && scale < 500 ? "detail" : "body";
      const meta: Json = { id, bone: own.bone, role, z: 0, filled: true, tone: own.tone, ...(scale === 500 && /^forearm-/.test(id) ? { regions: ["hoodie", "gloves", "hand-skin"] } : { region: kit.region.get(own.id) }), material: own.material };
      if (own.slender === true) {
        const f = frames[own.bone]!;
        for (const [i, line] of centreLines(own, f).entries()) {
          if (WHEEL.test(own.bone)) {
            if (view !== "elevation") continue;
            const local = line.map((p) => [p[0] - f.o[0], p[1] - f.o[1]]);
            const [a, b] = [local[0]!, local[local.length - 1]!];
            const n = [-(b[1]! - a[1]!), b[0]! - a[0]!];
            const l = Math.hypot(n[0]!, n[1]!);
            const h = 0.0006;
            const q = (p: number[], s: number): number[] => [r4(p[0]! + (s * h * n[0]!) / l), r4(p[1]! + (s * h * n[1]!) / l)];
            drawn.push({ shape: { ...meta, id: `${id}-${i + 1}`, role: "texture", filled: false, kind: "poly", points: [q(a, -1), q(a, 1), q(b, 1), q(b, -1)], closed: true, penMm: 0.1, solid: { kind: "extruded", sweep: "w", z0: r4(own.p1?.[2] ?? 0) - 0.001, z1: r4(own.p1?.[2] ?? 0) + 0.001 } }, near: Math.max(...line.map(depth)) });
          } else flat.push({ meta: { ...meta, id: centreLines(own, f).length > 1 ? `${id}-${i + 1}` : id, filled: false, role: scale === 500 ? "body" : "detail" }, target: line.map((p) => projectWorld(view, p)), closed: false, near: Math.max(...line.map(depth)) + (LIFT.find(([re]) => re.test(id))?.[1] ?? 0) });
        }
        continue;
      }
      const meshes = meshesIn(asset, frames, solids);
      let [sum, n] = [0, 0];
      const middle: V3 = [0, 0, 0];
      for (const m of meshes) for (let i = 0; i < m.world.length; i += 3) {
        sum += depth([m.world[i]!, m.world[i + 1]!, m.world[i + 2]!]);
        for (let k = 0; k < 3; k++) middle[k]! += m.world[i + k]!;
        n++;
      }
      const near = sum / n + (LIFT.find(([re]) => re.test(id))?.[1] ?? 0);
      if (WHEEL.test(own.bone) && own.kind === "revolve") {
        for (const p of wheelProfile(own, ARC[scale]!)) drawn.push({ shape: { ...meta, id: `${id}${p.suffix}`, kind: "poly", points: p.points, closed: true, solid: { kind: "extruded", sweep: "w", z0: p.z0, z1: p.z1 } }, near });
        continue;
      }
      const made = SPREAD.test(own.bone) ? revolved({ id, solids, own }) : null;
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
      const texture = own.texture === true;
      if (!SPREAD.test(own.bone)) {
        flat.push({ meta: texture ? { ...meta, filled: false, role: "detail" } : meta, target: ring, closed: true, near });
        continue;
      }
      const f = frames[own.bone]!;
      const facing = (a: readonly number[]): number => Math.abs(depth(a));
      const sweep = facing(f.w) >= facing(f.v) ? "w" : "v";
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
  const ground = (asset.representations[String(scale)].geometry.elevation as Json[]).find((s) => s.role === "ground");
  if (ground !== undefined) views.elevation!.unshift(ground);
  geometry[String(scale)] = views;
}
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "geometry.json"), JSON.stringify(geometry));
