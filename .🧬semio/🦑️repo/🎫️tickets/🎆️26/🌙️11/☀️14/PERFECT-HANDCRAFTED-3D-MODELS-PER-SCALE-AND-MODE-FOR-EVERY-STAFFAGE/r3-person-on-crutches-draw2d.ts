/** ✏️ Redraws the 2D drawings of `humans/person-on-crutches` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. Which solids one drawn shape unites, the layer order of every view, the cut of the flexed shorts leg at the sweatshirt hem and the strokes (slender crutch rods, drawstrings, mouth) are stated by hand below.
 *
 * Usage: `bun r3-person-on-crutches-draw2d.ts` writes `🗑️generated/r3-person-on-crutches/geometry.json` (geometry of every scale and view, and the extents of the default pose).
 * @see ./r3-person-on-crutches-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { ASSET } from "./r3-person-on-crutches-kit.ts";
import { OUT, volumes } from "./r3-person-on-crutches-build.ts";
import { HOODIE, SHAFT, surfaceX } from "./r3-person-on-crutches-parts.ts";

type V3 = [number, number, number];
const VOLUMES = volumes();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const side = (id: string): string => (/-l(\d|$|-)/.test(id) || /-l$/.test(id) ? "l" : "r");

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not drawn as a filled shape). */
function groupOf(s: Json, view: View): string | null {
  const id: string = s.id;
  if (s.slender === true || s.texture === true) return null;
  if (s.bone.startsWith("hand-")) return s.bone;
  if (/^(nose|nostrils|brow|lips|chin|cheek-[lr])$/.test(id)) return "head";
  if (/^eyebrow-/.test(id)) return "hair";
  if (id === "larynx") return "neck";
  if (/^knee-/.test(id)) return `thigh-g-${side(id)}`;
  if (/^crutch-(lower|clip|joint)-/.test(id)) return `crutch-tube-${side(id)}`;
  if (/^crutch-cuff-mount-/.test(id)) return `crutch-cuff-${side(id)}`;
  if (/^boot-strap-[34]$/.test(id) && view === "plan") return "shoe-rear-l";
  if (/^boot-strap-[012]$/.test(id) && view === "plan") return "boot-shaft-l";
  if (/^boot-strut-/.test(id)) return view === "elevation" && id === "boot-strut-i-l" ? "boot-strut" : view === "elevation" ? "boot-shaft-l" : "boot-shaft-l";
  if (id === "shoe-tongue-r") return "shoe-rear-r";
  if (view === "elevation") {
    if (id === "ear-l" || id === "eye-l") return "head";
    if (id === "pocket") return "torso";
  }
  if (view === "plan") {
    if (/^eye-/.test(id) || id === "pocket") return id === "pocket" ? "torso" : "head";
    const sole = /^sole-(front-)?([lr])$/.exec(id);
    if (sole) return `shoe-${sole[1] ? "front" : "rear"}-${sole[2]}`;
  }
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ARM = (n: string): RegExp[] => [new RegExp(`^crutch-tube-${n}`), new RegExp(`^crutch-tip-${n}`), new RegExp(`^crutch-bracket-${n}`), new RegExp(`^sleeve-up-${n}`), new RegExp(`^sleeve-fore-${n}`), new RegExp(`^crutch-cuff-${n}`), new RegExp(`^cuff-s-${n}`), new RegExp(`^wrist-${n}`), new RegExp(`^crutch-grip-${n}`), new RegExp(`^thumb-${n}`), new RegExp(`^hand-${n}`)];
const LEG = (n: string): RegExp[] => [new RegExp(`^thigh-g-${n}`), new RegExp(`^shank-g-${n}`), new RegExp(`^sock-${n}`), new RegExp(`^sole-${n}`), new RegExp(`^shoe-rear-${n}`), new RegExp(`^sole-front-${n}`), new RegExp(`^shoe-front-${n}`), new RegExp(`^shoe-${n}`), new RegExp(`^boot-shaft-${n}`), /^boot-strut/, /^boot-strap/, new RegExp(`^skirt-${n}`)];
const ORDER: Record<View, RegExp[]> = {
  elevation: [...ARM("l"), ...LEG("l"), ...LEG("r"), /^hip/, /^neck/, /^hood/, /^torso/, /^collar/, /^head$/, /^eye-/, /^mouth/, /^hair/, /^ear/, ...ARM("r")],
  front: [/^hood/, /^neck/, /^hip/, ...LEG("r"), /^crutch-tube-/, /^crutch-tip-/, /^crutch-bracket-/, /^sleeve-up-/, /^torso/, /^pocket/, /^drawstring/, /^collar/, ...LEG("l"), /^sleeve-fore-/, /^crutch-cuff-/, /^cuff-s-/, /^wrist-/, /^crutch-grip-/, /^thumb-/, /^hand-/, /^hair$/, /^ear/, /^head$/, /^eye-/, /^mouth/, /^hair-top/],
  plan: [/^sole-/, /^shoe-rear-/, /^shoe-front-/, /^shoe-[lr]/, /^sock-/, /^shank-g-/, /^boot-shaft-/, /^boot-strap/, /^thigh-g-/, /^skirt-/, /^crutch-tip-/, /^crutch-tube-/, /^hip/, /^crutch-grip-/, /^thumb-/, /^hand-/, /^wrist-/, /^cuff-s-/, /^crutch-bracket-/, /^sleeve-fore-/, /^crutch-cuff-/, /^torso/, /^sleeve-up-/, /^hood/, /^collar/, /^neck/, /^head$/, /^ear/, /^hair/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
function regionOf(s: Json): string {
  if (s.material === "fleece-hoodie") return "hoodie";
  if (s.material === "polyester-short") return "shorts";
  if (s.material === "boot-shell") return "boot";
  if (s.material === "sneaker-mesh" || s.material === "cotton-sock") return "sneaker";
  if (s.material === "hair-dark") return "hair";
  if (s.material === "alu-anodised") return "crutch-frame";
  if (s.material === "cuff-polymer") return "crutch-cuffs";
  if (s.material === "rubber-black") return /grip/.test(s.id) ? "crutch-grips" : "crutch-tips";
  if (s.material === "skin-light") return /^(head|neck)$/.test(s.bone) ? "face-skin" : /^(hand|forearm)/.test(s.bone) ? "hands" : "legs-skin";
  throw new Error(`no region for ${s.id}`);
}

const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
/** 🖊️ The pen of the slender crutch rod where its true 22 mm falls below the printable line: millimetres on paper. */
const ROD_PEN: Record<number, number> = { 200: 0.15, 500: 0.13 };
/** 🛞 The base radius of the rubber ferrule: at 1:500 the ferrule is no shape of its own, the rod ends in a stroke across its 41 mm base on the ground. */
const FERRULE_R = 0.0205;
/** ✂️ The front view shows the flexed left shorts leg only below the hem of the sweatshirt it comes out of. */
const HEM_Y = 0.93;
/** 💇 The front view shows the hair above the forehead hairline over the face, the rest of it behind the head. */
const HAIRLINE_Y = 1.762;

function below(ring: readonly P2[], y: number): P2[] {
  const out: P2[] = [];
  for (let i = 0; i < ring.length; i++) {
    const [a, b] = [ring[i]!, ring[(i + 1) % ring.length]!];
    if (a.y <= y) out.push(a);
    if (a.y <= y !== b.y <= y) { const t = (y - a.y) / (b.y - a.y); out.push({ x: a.x + (b.x - a.x) * t, y }); }
  }
  return out;
}

type Stroke = { id: string; bone: string; role: string; after: string; tone: number; region: string; closed: boolean; points: V3[]; penMm?: number; regions?: string[] };
/** 🖋️ The strokes of a scale and view, stated as points of the standing figure. */
function strokes(scale: number, view: View, frames: Record<string, Frame>, body: readonly Json[]): Stroke[] {
  const at = (bone: string, p: V3): V3 => { const f = frames[bone]!; return [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3; };
  const out: Stroke[] = [];
  for (const s of body) {
    if (s.slender !== true || !/^crutch-tube-/.test(s.id)) continue;
    const path = (s.sections as Json[]).map((q) => at(s.bone, q.at as V3));
    const end = s.sections[s.sections.length - 1].at as V3;
    const across: V3 = view === "elevation" ? [0, FERRULE_R, 0] : [-FERRULE_R * SHAFT[2], 0, FERRULE_R * SHAFT[0]];
    const base: V3[] = scale === 500 ? [at(s.bone, [end[0] + across[0], end[1] + across[1], end[2] + across[2]]), at(s.bone, [end[0] - across[0], end[1] - across[1], end[2] - across[2]])] : [];
    const ends: V3[] = [path[0]!, path[2]!, path[path.length - 1]!, ...base];
    out.push({ id: s.id, bone: s.bone, role: "body", after: s.id, tone: s.tone, region: "crutch-frame", closed: false, points: ends, penMm: ROD_PEN[scale] ?? 0.22, ...(scale === 500 ? { regions: ["crutch-frame", "crutch-grips", "crutch-tips"] } : {}) });
  }
  if (scale === 50) {
    if (view === "front") {
      for (const z of [0.026, -0.026]) out.push({ id: `drawstring-${z > 0 ? "l" : "r"}`, bone: "torso", role: "detail", after: "drawstring", tone: 0.85, region: "hoodie", closed: false, points: [1.545, 1.45, 1.35].map((y, i) => [surfaceX(HOODIE, y, z, 1) + 0.004, y, z * (1 + 0.2 * i)] as V3) });
      out.push({ id: "mouth", bone: "head", role: "detail", after: "mouth", tone: 0.4, region: "face-skin", closed: false, points: [[-0.035, -0.097, -0.022], [-0.036, -0.098, 0], [-0.035, -0.097, 0.022]].map((p) => at("head", p as V3)) });
    }
    if (view === "elevation") out.push({ id: "mouth", bone: "head", role: "detail", after: "mouth", tone: 0.4, region: "face-skin", closed: false, points: [[-0.035, -0.098, -0.01], [-0.034, -0.078, -0.03]].map((p) => at("head", p as V3)) });
  }
  return out;
}

const geometry: Record<string, Record<string, Json[]>> = {};
const dimensions: Json = {};
const work = JSON.parse(JSON.stringify(ASSET)) as Json;
work.volumes = VOLUMES;
for (const scale of [50, 100, 200, 500]) {
  const frames = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const body: Json[] = VOLUMES[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const views: Record<string, Json[]> = {};
  if (scale === 50) {
    const lo = [Infinity, Infinity, Infinity];
    const hi = [-Infinity, -Infinity, -Infinity];
    for (const m of meshesIn(work, frames, body)) for (let i = 0; i < m.world.length; i += 3) for (let k = 0; k < 3; k++) { lo[k] = Math.min(lo[k]!, m.world[i + k]!); hi[k] = Math.max(hi[k]!, m.world[i + k]!); }
    Object.assign(dimensions, { heightM: Math.round(hi[1]! * 100) / 100, widthM: Math.round((hi[0]! - lo[0]!) * 100) / 100, depthM: Math.round((hi[2]! - lo[2]!) * 100) / 100 });
    console.log("extent of the default pose", lo.map((x) => x.toFixed(4)).join(" "), hi.map((x) => x.toFixed(4)).join(" "));
  }
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const g = groupOf(s, view);
      if (g !== null) (groups.get(g) ?? groups.set(g, []).get(g)!).push(s);
    }
    const jobs: { meta: Json; target: P2[]; closed: boolean }[] = [];
    for (const [id, solids] of groups) {
      const own = solids.find((s) => s.id === id) ?? solids[0]!;
      const mask = projectMask(view, meshesIn(work, frames, solids), cell, 4 + Math.ceil(0.03 / cell));
      if (mask === null) continue;
      const rings = contours(mask.cells, mask.width, mask.height, mask.box, mask.cell).filter((r: Ring) => area(r) > 0).sort((a: Ring, b: Ring) => area(b) - area(a));
      const lost = rings.slice(1).reduce((n: number, r: Ring) => n + area(r), 0);
      if (lost > 0.02 * area(rings[0]!)) console.log(`  [DEBUG] ${scale} ${view} ${id}: ${rings.length - 1} smaller ring(s) left out, ${(lost * 1e4).toFixed(1)} cm2`);
      let ring: P2[] = simplifyRing(rings[0]!, tol);
      if (view === "front" && id === "skirt-l") ring = below(ring, HEM_Y);
      const meta: Json = { id, bone: own.bone, role: own.role === "detail" ? "detail" : "body", z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own) };
      if (scale === 50 && view === "elevation" && /^(torso|sleeve-(up|fore)-r)$/.test(id)) meta.hatch = "knit";
      if (scale === 50 && view === "elevation" && /^sleeve-(up|fore)-l$/.test(id)) meta.hatch = "far-side";
      jobs.push({ meta, target: ring, closed: true });
      if (view === "front" && id === "hair") jobs.push({ meta: { ...meta, id: "hair-top", z: layer("hair-top", view) }, target: below(ring.map((p) => ({ x: p.x, y: -p.y })), -HAIRLINE_Y).map((p) => ({ x: p.x, y: -p.y })), closed: true });
    }
    for (const s of strokes(scale, view, frames, body)) jobs.push({ meta: { id: s.id, bone: s.bone, role: s.role, z: r4(layer(s.after, view) + 0.1), filled: false, tone: s.tone, ...(s.regions ? { regions: s.regions } : { region: s.region }), ...(s.penMm ? { penMm: s.penMm } : {}) }, target: s.points.map((p) => projectWorld(view, p)), closed: s.closed });
    const solved = inverse(work, scale, view, jobs);
    const shapes = jobs.map((j, k) => {
      if (solved[k]!.residualM > 1e-5 || solved[k]!.edgeOn) console.log(`  [DEBUG] ${scale} ${view} ${j.meta.id}: residual ${(solved[k]!.residualM * 1000).toFixed(3)} mm edgeOn ${solved[k]!.edgeOn}`);
      return { ...j.meta, kind: "poly", points: solved[k]!.points.map((p) => [r4(p.x), r4(p.y)]), closed: j.closed };
    });
    shapes.sort((a, b) => a.z - b.z);
    views[view] = shapes;
    work.representations[String(scale)].geometry[view] = shapes;
    console.log(scale, view, `${shapes.length} shapes, ${shapes.reduce((n, s) => n + s.points.length, 0)} points`);
  }
  geometry[String(scale)] = views;
}
mkdirSync(OUT, { recursive: true });
writeFileSync(join(OUT, "geometry.json"), JSON.stringify({ geometry, dimensions }));
