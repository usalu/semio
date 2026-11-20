/** ✏️ Redraws the 2D drawings of `humans/parent-carrying-toddler` (ruling E1): every filled shape of every scale and view is the orthographic outline, in the default pose, of the body solids it stands for, solved into the frame of its bone through the product's own placement and written as an economical polygon. Which solids one drawn shape unites, the layer order of every view and the detail strokes are stated by hand below.
 *
 * Usage: `bun r3-parent-carrying-toddler-draw2d.ts` writes `🗑️generated/r3-parent-carrying-toddler/shapes.json` (spliced by `r3-parent-carrying-toddler-roots.py`).
 * @see ./r3-parent-carrying-toddler-build.ts */
import { mkdirSync, writeFileSync } from "node:fs";
import { contours, inverse, projectMask } from "./r3-project2d.ts";
import { meshesIn, rigAt, type Frame } from "./r3-lib.ts";
import { area, projectWorld, simplifyRing, type Json, type P2, type Ring, type View } from "./volumes-lib.ts";
import { volumes } from "./r3-parent-carrying-toddler-build.ts";
import { OUT, candidate } from "./r3-parent-carrying-toddler-io.ts";
import { regionOf } from "./r3-parent-carrying-toddler-lists50.ts";
import { coatX, spine } from "./r3-parent-carrying-toddler-s50.ts";

type V3 = [number, number, number];
const asset = candidate(false);
const VOLUMES = await volumes();
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** 🧩 The drawn shape a body solid belongs to in a view (`null`: not drawn as a filled shape). */
function groupOf(id: string, view: View, scale: number): string | null {
  if (id === "toddler-zip") return null;
  const glove = /^glove-([lr])/.exec(id);
  if (glove) return `glove-${glove[1]}`;
  const mitten = /^toddler-mitten(?:-thumb)?-([lr])$/.exec(id);
  if (mitten) return `toddler-mitten-${mitten[1]}`;
  const sole = /^boot-(heel|sole)-([lr])$/.exec(id);
  if (sole) return view === "plan" ? `boot-${sole[2]}` : `boot-sole-${sole[2]}`;
  const toeSole = /^boot-toe-sole-([lr])$/.exec(id);
  if (toeSole) return view === "plan" ? `boot-toe-${toeSole[1]}` : `boot-toe-sole-${toeSole[1]}`;
  const tsole = /^toddler-boot-sole-([lr])$/.exec(id);
  if (tsole) return `toddler-boot-${tsole[1]}`;
  if (/^head-(nose|brow|lips)$/.test(id)) return "head";
  if (/^toddler-(nose|lips)$/.test(id)) return "toddler-head";
  if (id === "toddler-pompom") return "toddler-hat";
  if (/^coat-button-/.test(id)) return view === "front" ? id : null;
  if (/^scarf-end/.test(id)) return view === "front" ? "scarf-end" : null;
  if (/^coat-pocket-([lr])$/.test(id)) return view === "front" ? id : view === "elevation" && id.endsWith("-r") ? id : `coat-skirt-${id.slice(-1)}`;
  if (id === "hair") return view === "elevation" ? "hair" : null;
  if (id === "beanie-cuff") return view === "plan" ? "beanie" : id;
  if (id === "toddler-hat-cuff") return view === "plan" ? "toddler-hat" : id;
  if (/^head-eye-l$|^toddler-eye-l$/.test(id)) return view === "plan" ? null : id;
  if (/^head-eye-r$|^toddler-eye-r$/.test(id)) return view === "plan" ? null : id;
  if (/^head-ear-l$/.test(id)) return view === "plan" ? "head" : id;
  if (/^toddler-ear-l$/.test(id)) return view === "plan" ? "toddler-head" : id;
  if (/^(head-ear-r|toddler-ear-r)$/.test(id)) return view === "plan" ? (id.startsWith("toddler") ? "toddler-head" : "head") : id;
  if (id === "toddler-collar" && view === "plan") return "toddler-suit";
  
  return id;
}

/** 🥞 The layer order of a view, bottom first: the part farthest from the viewer comes first. */
const ORDER: Record<View, RegExp[]> = {
  elevation: [/^boot-sole-l/, /^boot-toe-sole-l/, /^boot-l/, /^boot-toe-l/, /^shank-l/, /^thigh-l/, /^coat-skirt-l/, /^glove-l/, /^glove-cuff-l/, /^forearm-l/, /^sleeve-l/, /^jeans-seat/, /^boot-sole-r/, /^boot-toe-sole-r/, /^boot-r/, /^boot-toe-r/, /^shank-r/, /^thigh-r/, /^coat-skirt-r/, /^coat-pocket-r/, /^coat$/, /^collar/, /^neck/, /^scarf$/, /^head-ear-l/, /^head-eye-l/, /^hair$/, /^head$/, /^head-ear-r/, /^head-eye-r/, /^beanie$/, /^beanie-cuff/, /^sleeve-r/, /^forearm-r/, /^glove-cuff-r/, /^glove-r/, /^toddler-boot-l/, /^toddler-boot-shaft-l/, /^toddler-shank-l/, /^toddler-thigh-l/, /^toddler-forearm-l/, /^toddler-mitten-l/, /^toddler-sleeve-l/, /^toddler-suit/, /^toddler-collar/, /^toddler-neck/, /^toddler-ear-l/, /^toddler-eye-l/, /^toddler-head$/, /^toddler-ear-r/, /^toddler-eye-r/, /^toddler-hat$/, /^toddler-hat-cuff/, /^toddler-thigh-r/, /^toddler-shank-r/, /^toddler-boot-shaft-r/, /^toddler-boot-r/, /^toddler-sleeve-r/, /^toddler-forearm-r/, /^toddler-mitten-r/],
  front: [/^jeans-seat/, /^thigh-/, /^shank-/, /^boot-sole-/, /^boot-toe-sole-/, /^boot-[lr]/, /^boot-toe-[lr]/, /^coat-skirt-/, /^coat$/, /^coat-pocket/, /^coat-button/, /^collar/, /^scarf$/, /^scarf-end/, /^sleeve-l/, /^forearm-l/, /^glove-cuff-l/, /^glove-l/, /^neck/, /^head-ear/, /^head$/, /^head-eye/, /^beanie$/, /^beanie-cuff/, /^sleeve-r/, /^forearm-r/, /^glove-cuff-r/, /^glove-r/, /^toddler-suit/, /^toddler-collar/, /^toddler-neck/, /^toddler-ear/, /^toddler-head$/, /^toddler-eye/, /^toddler-hat$/, /^toddler-hat-cuff/, /^toddler-sleeve-/, /^toddler-forearm-/, /^toddler-thigh-/, /^toddler-mitten-/, /^toddler-shank-/, /^toddler-boot-shaft-/, /^toddler-boot-[lr]/],
  plan: [/^boot-[lr]/, /^boot-toe-[lr]/, /^shank-/, /^thigh-/, /^jeans-seat/, /^coat-skirt-/, /^glove-l/, /^glove-cuff-l/, /^forearm-l/, /^glove-r/, /^glove-cuff-r/, /^forearm-r/, /^toddler-boot-[lr]/, /^toddler-boot-shaft-/, /^toddler-shank-/, /^toddler-thigh-/, /^toddler-forearm-/, /^toddler-mitten-/, /^coat$/, /^sleeve-/, /^collar/, /^toddler-sleeve-/, /^toddler-suit/, /^toddler-neck/, /^scarf$/, /^neck/, /^head$/, /^beanie$/, /^toddler-head$/, /^toddler-hat$/],
};
function layer(id: string, view: View): number {
  const i = ORDER[view].findIndex((re) => re.test(id));
  if (i < 0) throw new Error(`no layer for ${id} in ${view}`);
  return r4(1 + i * 0.2);
}
const DETAIL = /^(head-eye|toddler-eye|coat-button|coat-pocket|scarf-end|beanie-cuff|toddler-hat-cuff|head-ear|toddler-ear|collar|toddler-collar|glove-cuff)/;

const TOL: Record<number, number> = { 50: 0.0009, 100: 0.0018, 200: 0.0035, 500: 0.007 };
type Stroke = { id: string; bone: string; after: string; tone: number; region: string; material: string; points: V3[] };
/** 🖋️ The detail strokes of a scale and view, stated as points of the standing figure (x ahead, y up, z left). */
function strokes(scale: number, view: View): Stroke[] {
  const out: Stroke[] = [];
  const tz = -0.17;
  if (scale <= 100 && view === "front") {
    out.push({ id: "coat-front-edge", bone: "torso", after: "coat-button", tone: 0.12, region: "coat-torso", material: "wool", points: [1.44, 1.3, 1.15, 1.0].map((y) => [coatX(y, 0.0, 1) + 0.002, y, 0.0] as V3) });
  }
  if (scale === 50 && view === "front") {
    out.push({ id: "mouth-line", bone: "head", after: "head-eye-l", tone: 0.25, region: "face", material: "skin", points: [[0.064, 1.552, 0.018], [0.067, 1.551, 0], [0.064, 1.552, -0.018]] });
    out.push({ id: "toddler-mouth-line", bone: "toddler-head", after: "toddler-eye-l", tone: 0.3, region: "toddler-face", material: "skin", points: [[0.25, 1.537, tz + 0.01], [0.252, 1.536, tz], [0.25, 1.537, tz - 0.01]] });
  }
  if (scale === 50 && view === "elevation") out.push({ id: "mouth-line", bone: "head", after: "head-eye-r", tone: 0.25, region: "face", material: "skin", points: [[0.066, 1.552, -0.012], [0.052, 1.551, -0.028]] });
  return out;
}

const geometry: Record<string, Record<string, Json[]>> = {};
for (const scale of [50, 100, 200, 500]) {
  const work = JSON.parse(JSON.stringify(asset)) as Json;
  const frames: Record<string, Frame> = rigAt(work, scale, { kind: "pose", pose: work.presentation.defaultPose }).frames;
  const body: Json[] = VOLUMES[String(scale)].body;
  const tol = TOL[scale]!;
  const cell = tol / 3;
  const views: Record<string, Json[]> = {};
  for (const view of ["elevation", "front", "plan"] as View[]) {
    const groups = new Map<string, Json[]>();
    for (const s of body) {
      const g = groupOf(s.id, view, scale);
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
      const ring = simplifyRing(rings[0]!, tol);
      const meta: Json = { id, bone: own.bone, role: DETAIL.test(id) && scale <= 100 ? "detail" : "body", z: layer(id, view), filled: true, tone: own.tone, region: regionOf(own), material: own.material };
      jobs.push({ meta, target: ring, closed: true });
    }
    for (const s of strokes(scale, view)) jobs.push({ meta: { id: s.id, bone: s.bone, role: "detail", z: r4(layer(s.after, view) + 0.1), filled: false, tone: s.tone, region: s.region, material: s.material }, target: s.points.map((p) => projectWorld(view, p)), closed: false });
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
writeFileSync(`${OUT}/shapes.json`, JSON.stringify(geometry));
