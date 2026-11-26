/** 🧵️ Draws parts of an asset as `loft` shapes (D54), the same body in the side elevation, the front elevation and the plan: `bun loft.ts <id|file.json> --ops <lofts.json> [--write] [--out <file>]`. Every entry `{ scale, bone, paint, "views"?: [view…], "solids"?: [id…], "replace"?: [shape id…], "fit"?: [{ id, "solids"?: [id…], stations, "exponent"? }], "swell"?: { <solid id>: metres } }` takes the body solids of `volumes.<scale>` on `bone` (all, or those named) and writes one loft shape per solid into every named view (default all three), with the id of the solid:
 * - a `loft` solid gives its sections as they are (a `transform` of translation and uniform scale is applied to them, any other is refused),
 * - an `ellipsoid` gives nine elliptic sections along its longest axis (from 10 to 170 degrees of its meridian), turned and moved by its `transform`,
 * - a `capsule` gives its two end circles and three sections of each cap (`lateralScale` on the half extent the kernel stretches),
 * - a solid of another kind is not a loft and is reported (draw it as a prism with `../MH-6/bodies.ts`, or name it under `fit`).
 * `fit` draws several solids as ONE loft where the scale cannot print them apart (the fingers of a hand): the solids named (default all of the entry) are cut at `stations` equal steps along the bone and every cut becomes the superellipse of its box with the exponent of its area (or the given `exponent`), stated with the limb rotation; the solids of a fit are not drawn singly.
 * `swell` adds metres to the half extents of every section of a loft: a part that stands a fraction of a printed gap (0.15 mm on paper) beside its neighbour is drawn touching it.
 * The shapes of the bone named in `replace`, and those that carry the id of a new loft, leave the view; the new shapes stand where the first of them stood. Paint (role, z, filled, tone, region, regions, material, hatch, penMm) comes from the shape of the same id in that view when there is one, else from the shape `paint` of that view, else from `paint` in the side elevation; `"filled": true` in the entry sets the loft filled whatever the paint says (a stroke that becomes a body). A second run changes nothing. Solids with skin weights or a `repeat` are refused. `--write` writes the catalog document only when it did not change on disk since it was read; `--out` writes a trial copy.
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Loft shapes"
 * @see ../MH-6/bodies.ts — prisms and bodies of revolution from the volume solids */
import { join } from "node:path";
import { S, readDoc, writeDoc } from "../MR/lib.ts";

const { copyMesh } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));

type V3 = [number, number, number];
type Q = [number, number, number, number];
type Section = { at: V3; rx: number; ry: number; exponent?: number; rotation?: Q };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const read = readDoc(args[0]!);
const asset = read.doc.asset;
const entries = JSON.parse(await Bun.file(flag("ops")!).text()) as any[];
const QUALITY = { chordM: 0.0005, minSegments: 16, maxSegments: 64 };
const PAINT = ["role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const LIMB: Q = [0.5, 0.5, 0.5, 0.5];
const ALONG: Record<number, Q> = { 0: LIMB, 1: [-0.5, -0.5, -0.5, 0.5], 2: [0, 0, 0, 1] };
const round = (value: number): number => Math.round(value * 1e5) / 1e5;
const multiply = (a: Q, b: Q): Q => [a[3] * b[0] + a[0] * b[3] + a[1] * b[2] - a[2] * b[1], a[3] * b[1] - a[0] * b[2] + a[1] * b[3] + a[2] * b[0], a[3] * b[2] + a[0] * b[1] - a[1] * b[0] + a[2] * b[3], a[3] * b[3] - a[0] * b[0] - a[1] * b[1] - a[2] * b[2]];
const turn = (q: Q, v: V3): V3 => {
  const [x, y, z, w] = q;
  const [tx, ty, tz] = [2 * (y * v[2] - z * v[1]), 2 * (z * v[0] - x * v[2]), 2 * (x * v[1] - y * v[0])];
  return [v[0] + w * tx + y * tz - z * ty, v[1] + w * ty + z * tx - x * tz, v[2] + w * tz + x * ty - y * tx];
};

/** 🧭️ The transform of a solid as a uniform scale, a rotation and a translation, or the reason it cannot be applied to sections. */
function placement(solid: any): { scale: number; rotation: Q; translation: V3 } | string {
  const t = solid.transform ?? {};
  const scale: V3 = t.scale ?? [1, 1, 1];
  if (Math.abs(scale[0] - scale[1]) > 1e-9 || Math.abs(scale[0] - scale[2]) > 1e-9) return "its transform scales the axes differently";
  return { scale: scale[0], rotation: (t.rotation ?? [0, 0, 0, 1]) as Q, translation: (t.translation ?? [0, 0, 0]) as V3 };
}

/** 🥚️ The sections of one body solid, or the reason it is not a loft. */
function sectionsOf(solid: any): Section[] | string {
  const place = placement(solid);
  if (typeof place === "string") return place;
  const move = (p: V3): V3 => {
    const q = turn(place.rotation, [p[0] * place.scale, p[1] * place.scale, p[2] * place.scale]);
    return [round(q[0] + place.translation[0]), round(q[1] + place.translation[1]), round(q[2] + place.translation[2])];
  };
  const turned = solid.transform?.rotation !== undefined;
  if (solid.kind === "loft") {
    if (!turned && place.scale === 1 && place.translation.every((value) => value === 0)) return solid.sections;
    if (turned && solid.sections.some((section: Section) => section.rotation === undefined)) return "its transform turns sections that state no rotation";
    return solid.sections.map((section: Section) => ({ ...section, at: move(section.at), rx: round(section.rx * place.scale), ry: round(section.ry * place.scale), ...(turned ? { rotation: multiply(place.rotation, section.rotation!).map(round) as Q } : {}) }));
  }
  if (solid.kind === "ellipsoid") {
    const radii = solid.radii as V3;
    const axis = radii.indexOf(Math.max(...radii));
    const [j, k] = [(axis + 1) % 3, (axis + 2) % 3];
    const rotation = (turned ? multiply(place.rotation, ALONG[axis]!) : ALONG[axis]!).map(round) as Q;
    return Array.from({ length: 9 }, (_, i) => {
      const phi = ((10 + 20 * i) * Math.PI) / 180;
      const at: V3 = [0, 0, 0];
      at[axis] = -radii[axis]! * Math.cos(phi);
      return { at: move(at), rx: round(radii[j]! * Math.sin(phi) * place.scale), ry: round(radii[k]! * Math.sin(phi) * place.scale), rotation };
    });
  }
  if (solid.kind === "capsule") {
    if (turned) return "its transform turns a capsule";
    const [p0, p1] = [solid.p0 as V3, solid.p1 as V3];
    const length = Math.hypot(p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]);
    const d: V3 = [(p1[0] - p0[0]) / length, (p1[1] - p0[1]) / length, (p1[2] - p0[2]) / length];
    const wide = solid.lateralScale ?? 1;
    const ring = (from: V3, back: number, radius: number): Section => ({ at: move([from[0] + d[0] * back, from[1] + d[1] * back, from[2] + d[2] * back]), rx: round(radius * wide * place.scale), ry: round(radius * place.scale) });
    const caps = [15, 45, 75].map((deg) => (deg * Math.PI) / 180);
    return [...(solid.r0 > 0 ? caps.map((phi) => ring(p0, -solid.r0 * Math.cos(phi), solid.r0 * Math.sin(phi))) : []), ...(solid.r0 > 0 ? [ring(p0, 0, solid.r0)] : []), ...(solid.r1 > 0 ? [ring(p1, 0, solid.r1)] : []), ...(solid.r1 > 0 ? [...caps].reverse().map((phi) => ring(p1, solid.r1 * Math.cos(phi), solid.r1 * Math.sin(phi))) : [])];
  }
  return `a ${solid.kind} is not a loft`;
}

/** 🎈️ The sections with `metres` added to every half extent. */
const swollen = (sections: Section[], metres: number): Section[] => (metres === 0 ? sections : sections.map((section) => ({ ...section, rx: round(section.rx + metres), ry: round(section.ry + metres) })));

const gamma = (x: number): number => {
  const g = [676.5203681218851, -1259.1392167224028, 771.32342877765313, -176.61502916214059, 12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7];
  if (x < 0.5) return Math.PI / (Math.sin(Math.PI * x) * gamma(1 - x));
  let a = 0.99999999999980993;
  for (let i = 0; i < g.length; i++) a += g[i]! / (x + i);
  const t = x + 6.5;
  return Math.sqrt(2 * Math.PI) * t ** (x - 0.5) * Math.exp(-t) * a;
};
const fullness = (n: number): number => gamma(1 + 1 / n) ** 2 / gamma(1 + 2 / n);

/** 🧤️ One loft through several solids: their triangles cut at equal steps along the bone, each cut the superellipse of its box with the exponent of the area of its hull. */
function fitted(solids: any[], stations: number, exponent: number | undefined): Section[] {
  const triangles: V3[][] = solids.flatMap((solid) => {
    const { positions, indices } = copyMesh(solid, QUALITY) as { positions: Float64Array; indices: Uint32Array };
    return Array.from({ length: indices.length / 3 }, (_, i) => [0, 1, 2].map((k): V3 => [positions[3 * indices[3 * i + k]!]!, positions[3 * indices[3 * i + k]! + 1]!, positions[3 * indices[3 * i + k]! + 2]!]));
  });
  const us = triangles.flat().map((p) => p[0]);
  const [low, high] = [us.reduce((a, b) => Math.min(a, b)), us.reduce((a, b) => Math.max(a, b))];
  const inset = (high - low) / (8 * stations);
  const out: Section[] = [];
  for (let s = 0; s <= stations; s++) {
    const u = s === 0 ? low + inset : s === stations ? high - inset : low + ((high - low) * s) / stations;
    const cut: [number, number][] = [];
    for (const [a, b, c] of triangles)
      for (const [p, q] of [[a, b], [b, c], [c, a]] as const) {
        if ((p![0] - u) * (q![0] - u) > 0 || p![0] === q![0]) continue;
        const t = (u - p![0]) / (q![0] - p![0]);
        cut.push([p![1] + t * (q![1] - p![1]), p![2] + t * (q![2] - p![2])]);
      }
    if (cut.length < 3) continue;
    const sorted = [...cut].sort((a, b) => a[0] - b[0] || a[1] - b[1]);
    const cross = (o: [number, number], a: [number, number], b: [number, number]): number => (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0]);
    const half = (list: [number, number][]): [number, number][] => {
      const ring: [number, number][] = [];
      for (const p of list) {
        while (ring.length >= 2 && cross(ring[ring.length - 2]!, ring[ring.length - 1]!, p) <= 0) ring.pop();
        ring.push(p);
      }
      ring.pop();
      return ring;
    };
    const hull = [...half(sorted), ...half([...sorted].reverse())];
    const area = Math.abs(hull.reduce((sum, p, i) => sum + p[0] * hull[(i + 1) % hull.length]![1] - hull[(i + 1) % hull.length]![0] * p[1], 0)) / 2;
    const [v0, v1, w0, w1] = [Math.min(...hull.map((p) => p[0])), Math.max(...hull.map((p) => p[0])), Math.min(...hull.map((p) => p[1])), Math.max(...hull.map((p) => p[1]))];
    const [rx, ry] = [(v1 - v0) / 2, (w1 - w0) / 2];
    if (!(rx > 1e-5 && ry > 1e-5)) continue;
    let n = exponent ?? 2;
    if (exponent === undefined) {
      const target = Math.min(0.999, Math.max(0.5, area / (4 * rx * ry)));
      let [lo, hi] = [1, 40];
      for (let i = 0; i < 40; i++) {
        const mid = (lo + hi) / 2;
        if (fullness(mid) < target) lo = mid;
        else hi = mid;
      }
      n = Math.round(((lo + hi) / 2) * 10) / 10;
    }
    out.push({ at: [round(u), round((v0 + v1) / 2), round((w0 + w1) / 2)], rx: round(rx), ry: round(ry), ...(n === 2 ? {} : { exponent: n }), rotation: LIMB });
  }
  return out;
}

let changed = 0;
let refused = 0;
for (const entry of entries) {
  const scale = String(entry.scale);
  const volume = asset.volumes?.[scale];
  const geometry = asset.representations?.[scale]?.geometry;
  const views: string[] = entry.views ?? ["elevation", "front", "plan"];
  const solids: any[] = (volume?.body ?? []).filter((solid: any) => solid.bone === entry.bone && (entry.solids === undefined || entry.solids.includes(solid.id)));
  const refuse = (reason: string): void => {
    console.log(`${asset.id} 1:${scale} ${entry.bone}: REFUSED, ${reason}`);
    refused++;
  };
  if (geometry === undefined || solids.length === 0) {
    refuse("the band has no drawing or no body solid on the bone");
    continue;
  }
  const bound = solids.find((solid) => solid.weights?.length || solid.repeat !== undefined);
  if (bound !== undefined) {
    refuse(`solid "${bound.id}" has skin weights or a repeat`);
    continue;
  }
  const fits: { id: string; solids?: string[]; stations: number; exponent?: number }[] = entry.fit ?? [];
  const merged = new Set(fits.flatMap((fit) => fit.solids ?? solids.map((solid) => solid.id)));
  const lofts: { id: string; sections: Section[] }[] = [];
  let failed = false;
  for (const solid of solids) {
    if (merged.has(solid.id)) continue;
    const sections = sectionsOf(solid);
    if (typeof sections === "string") {
      refuse(`solid "${solid.id}": ${sections}`);
      failed = true;
    } else lofts.push({ id: solid.id, sections: swollen(sections, entry.swell?.[solid.id] ?? 0) });
  }
  for (const fit of fits) lofts.push({ id: fit.id, sections: fitted(solids.filter((solid) => fit.solids === undefined || fit.solids.includes(solid.id)), fit.stations, fit.exponent) });
  const short = lofts.find((loft) => loft.sections.length < 2);
  if (short !== undefined) {
    refuse(`loft "${short.id}" has fewer than two sections`);
    failed = true;
  }
  if (failed) continue;
  const leaving = new Set<string>([...(entry.replace ?? []), ...lofts.map((loft) => loft.id)]);
  for (const view of views) {
    const list: any[] = geometry[view];
    const paintOf = (id: string): any => {
      const source = list.find((shape) => shape.id === id) ?? list.find((shape) => shape.id === entry.paint) ?? geometry.elevation.find((shape: any) => shape.id === entry.paint);
      if (source === undefined) throw new Error(`${asset.id} 1:${scale} ${view}: no shape "${id}" or "${entry.paint}" to take the paint from`);
      return Object.fromEntries(PAINT.filter((key) => source[key] !== undefined).map((key) => [key, source[key]]));
    };
    const made = lofts.map((loft) => {
      const paint = paintOf(loft.id);
      return { id: loft.id, kind: "loft", bone: entry.bone, ...paint, ...(entry.filled === undefined ? {} : { filled: entry.filled }), sections: loft.sections };
    });
    const first = list.findIndex((shape) => shape.bone === entry.bone && leaving.has(shape.id));
    const kept = list.filter((shape) => !(shape.bone === entry.bone && leaving.has(shape.id)));
    const at = first < 0 ? kept.length : list.slice(0, first).filter((shape) => !(shape.bone === entry.bone && leaving.has(shape.id))).length;
    const next = [...kept.slice(0, at), ...made, ...kept.slice(at)];
    if (JSON.stringify(next) !== JSON.stringify(list)) {
      geometry[view] = next;
      changed++;
    }
    console.log(`${asset.id} 1:${scale} ${view} ${entry.bone}: ${made.length} lofts (${made.map((shape) => `${shape.id} ${shape.sections.length}`).join(", ")}), ${list.length - kept.length} shapes left the view`);
  }
}
console.log(`${asset.id}: ${changed} views changed, ${refused} refused`);
if (flag("out") !== undefined) writeDoc(read, flag("out"));
else if (args.includes("--write")) console.log(writeDoc(read) ? "written" : "file unchanged");
