/** 🦵️ Redraws the moving parts of a person as `loft` shapes of the body solids of the band (D54): `bun limbs.ts <id|file.json> --plan <lofts.json> [--write] [--out <file>]`. Every entry `{ scale, bone, id, "views": [view…], "solids": [solid id…], "stations"?, "exponent"? }` replaces the shape `id` on `bone` of every named view in place (its place in the list, its paint fields role, z, filled, tone, region, regions, material, hatch, penMm stay). One loft solid of that bone without weights gives its own sections (what `../MX-2/loft.ts` does); anything else, several solids and solids of other bones (the twelve finger capsules of a hand no pose moves, the waist and skirt of a coat drawn rigid to the trunk), is fitted: the triangles of all of them are carried in the rest pose into the frame of `bone`, cut at `stations` equal steps along u, and every cut becomes the superellipse of the box of its hull with the exponent of its area (or the given `exponent`), stated with the limb rotation. A second run changes nothing. `oriented` lets the boxes of a fit share one turn about u, the one with the least area (a palm set askew, a thumb beside it); `path` draws a chain of capsules, in the order given, as one loft along their joints (a finger), `inset` takes metres off both half extents and `halfWidth` caps the lateral one (the core of a coat beneath its drawn plan outline); `skip` leaves out joints where the path would fold; `as` adds the loft as a further shape after `id` with its paint instead of replacing it, or before it and one thousandth of z lower with `beneath`. `--write` writes the catalog document only when it did not change on disk since it was read; `--out` writes a trial copy.
 * @see ../MX-2/loft.ts — the general tool, whose section fit this one shares
 * @see ../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/README.md — "Loft shapes" */
import { join } from "node:path";
import { S, readDoc, writeDoc } from "../MR/lib.ts";

const { copyMesh } = await import(join(S, "🔨️modules/🗂️catalog/🧊️volumes/🌓️silhouette/🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

type V3 = [number, number, number];
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };
type Section = { at: V3; rx: number; ry: number; exponent?: number; rotation?: number[] };
type Entry = { scale: number | string; bone: string; id: string; views: string[]; solids: string[]; stations?: number; exponent?: number; oriented?: boolean; as?: string; path?: boolean; skip?: number[]; inset?: number; halfWidth?: number; beneath?: boolean };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => {
  const i = args.indexOf(`--${name}`);
  return i < 0 ? undefined : args[i + 1];
};
const read = readDoc(args[0]!);
const asset = read.doc.asset;
const entries = JSON.parse(await Bun.file(flag("plan")!).text()) as Entry[];
const rest = restFrames(asset.rig) as Record<string, { frame: Frame }>;
const QUALITY = { chordM: 0.0005, minSegments: 16, maxSegments: 64 };
const PAINT = ["role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const LIMB = [0.5, 0.5, 0.5, 0.5];
const round = (value: number): number => Math.round(value * 1e5) / 1e5;
const dot = (a: number[], b: number[]): number => a[0]! * b[0]! + a[1]! * b[1]! + a[2]! * b[2]!;

/** 🚚️ A point of one bone frame in the frame of another bone, both at rest. */
function carried(p: V3, from: Frame, to: Frame): V3 {
  const d = [0, 1, 2].map((k) => from.o[k]! + p[0] * from.u[k]! + p[1] * from.v[k]! + p[2] * from.w[k]! - to.o[k]!);
  return [dot(d, to.u), dot(d, to.v), dot(d, to.w)];
}

const gamma = (x: number): number => {
  const g = [676.5203681218851, -1259.1392167224028, 771.32342877765313, -176.61502916214059, 12.507343278686905, -0.13857109526572012, 9.9843695780195716e-6, 1.5056327351493116e-7];
  if (x < 0.5) return Math.PI / (Math.sin(Math.PI * x) * gamma(1 - x));
  let a = 0.99999999999980993;
  for (let i = 0; i < g.length; i++) a += g[i]! / (x + i);
  const t = x + 6.5;
  return Math.sqrt(2 * Math.PI) * t ** (x - 0.5) * Math.exp(-t) * a;
};
const fullness = (n: number): number => gamma(1 + 1 / n) ** 2 / gamma(1 + 2 / n);


/** 🖐️ The sections of a chain of capsules (a finger) as one loft: a circle at every joint, perpendicular to the path, and a rounded tip. */
function chained(solids: any[], bone: string): Section[] {
  const to = rest[bone]!.frame;
  const ends = solids.map((solid) => [solid.p0, solid.p1].map((p: V3) => carried(p, rest[solid.bone]!.frame, to)));
  const joints = [{ p: ends[0]![0]!, r: solids[0].r0 }, ...solids.slice(1).map((solid, i) => ({ p: ends[i + 1]![0]!, r: (solids[i].r1 + solid.r0) / 2 })), { p: ends.at(-1)![1]!, r: solids.at(-1).r1 }];
  const [a, b] = ends.at(-1)!;
  const length = Math.hypot(b![0] - a![0], b![1] - a![1], b![2] - a![2]);
  const tip = joints.at(-1)!;
  const caps = [45, 75].map((deg) => (deg * Math.PI) / 180).map((phi) => ({ p: [0, 1, 2].map((k) => b![k]! + ((b![k]! - a![k]!) / length) * tip.r * Math.sin(phi)) as V3, r: tip.r * Math.cos(phi) }));
  return [...joints, ...caps].map(({ p, r }) => ({ at: p.map(round) as V3, rx: round(r), ry: round(r) }));
}

/** ✂️ The sections of one loft through several solids, in the frame of `bone`. */
function fitted(solids: any[], bone: string, stations: number, exponent: number | undefined, oriented: boolean): Section[] {
  const to = rest[bone]!.frame;
  const triangles: V3[][] = solids.flatMap((solid) => {
    const { positions, indices } = copyMesh(solid, QUALITY) as { positions: Float64Array; indices: Uint32Array };
    const from = rest[solid.bone]!.frame;
    const points = Array.from({ length: positions.length / 3 }, (_, i) => carried([positions[3 * i]!, positions[3 * i + 1]!, positions[3 * i + 2]!], from, to));
    return Array.from({ length: indices.length / 3 }, (_, i) => [0, 1, 2].map((k) => points[indices[3 * i + k]!]!));
  });
  const us = triangles.flat().map((p) => p[0]);
  const [low, high] = [us.reduce((a, b) => Math.min(a, b)), us.reduce((a, b) => Math.max(a, b))];
  const inset = (high - low) / (8 * stations);
  const cuts: { u: number; hull: [number, number][] }[] = [];
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
    cuts.push({ u, hull });
  }
  const turned = (hull: [number, number][], angle: number): [number, number][] => hull.map(([v, w]) => [v * Math.cos(angle) + w * Math.sin(angle), w * Math.cos(angle) - v * Math.sin(angle)]);
  const boxOf = (hull: [number, number][]): [number, number, number, number] => [Math.min(...hull.map((p) => p[0])), Math.max(...hull.map((p) => p[0])), Math.min(...hull.map((p) => p[1])), Math.max(...hull.map((p) => p[1]))];
  const waste = (angle: number): number => cuts.reduce((sum, { hull }) => { const [v0, v1, w0, w1] = boxOf(turned(hull, angle)); return sum + (v1 - v0) * (w1 - w0); }, 0);
  const angles = oriented ? Array.from({ length: 45 }, (_, i) => ((i * 2 - 44) * Math.PI) / 180) : [0];
  const angle = angles.reduce((best, next) => (waste(next) < waste(best) - 1e-12 ? next : best), 0);
  const [sh, ch] = [Math.sin(angle / 2), Math.cos(angle / 2)];
  const rotation = angle === 0 ? LIMB : [0.5 * ch + 0.5 * sh, 0.5 * ch - 0.5 * sh, 0.5 * ch + 0.5 * sh, 0.5 * ch - 0.5 * sh].map(round);
  return cuts.flatMap(({ u, hull }) => {
    const ring = turned(hull, angle);
    const area = Math.abs(ring.reduce((sum, p, i) => sum + p[0] * ring[(i + 1) % ring.length]![1] - ring[(i + 1) % ring.length]![0] * p[1], 0)) / 2;
    const [v0, v1, w0, w1] = boxOf(ring);
    const [rx, ry] = [(v1 - v0) / 2, (w1 - w0) / 2];
    if (!(rx > 1e-5 && ry > 1e-5)) return [];
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
    const [a, b] = [(v0 + v1) / 2, (w0 + w1) / 2];
    return [{ at: [round(u), round(a * Math.cos(angle) - b * Math.sin(angle)), round(a * Math.sin(angle) + b * Math.cos(angle))] as V3, rx: round(rx), ry: round(ry), ...(n === 2 ? {} : { exponent: n }), rotation }];
  });
}

let changed = 0;
let refused = 0;
for (const entry of entries) {
  const scale = String(entry.scale);
  const body: any[] = asset.volumes?.[scale]?.body ?? [];
  const solids = entry.solids.map((id) => body.find((solid) => solid.id === id));
  const refuse = (reason: string): void => {
    console.log(`${asset.id} 1:${scale} ${entry.bone} ${entry.id}: REFUSED, ${reason}`);
    refused++;
  };
  if (solids.some((solid) => solid === undefined)) {
    refuse(`no body solid ${entry.solids.filter((_, i) => solids[i] === undefined).join(", ")}`);
    continue;
  }
  const own = solids.length === 1 && solids[0].kind === "loft" && solids[0].bone === entry.bone && !solids[0].weights?.length && solids[0].transform === undefined && solids[0].repeat === undefined;
  const chain = entry.path === true && solids.every((solid) => solid.kind === "capsule" && solid.transform === undefined && solid.repeat === undefined);
  const sections: Section[] = own ? [...solids[0].sections] : chain ? chained(solids, entry.bone).filter((_, i) => !entry.skip?.includes(i)) : fitted(solids, entry.bone, entry.stations ?? 10, entry.exponent, entry.oriented === true);
  if (entry.inset !== undefined || entry.halfWidth !== undefined)
    sections.splice(0, sections.length, ...sections.map((section) => ({ ...section, rx: round(section.rx - (entry.inset ?? 0)), ry: round(Math.min(section.ry - (entry.inset ?? 0), entry.halfWidth ?? Infinity)) })));
  if (sections.length < 2) {
    refuse("fewer than two sections");
    continue;
  }
  for (const view of entry.views) {
    const list: any[] = asset.representations[scale].geometry[view];
    const at = list.findIndex((shape) => shape.id === entry.id && shape.bone === entry.bone);
    if (at < 0) {
      refuse(`no such shape in ${view}`);
      continue;
    }
    const before = list[at];
    const next = { id: entry.as ?? entry.id, kind: "loft", bone: entry.bone, ...Object.fromEntries(PAINT.filter((key) => before[key] !== undefined).map((key) => [key, before[key]])), ...(entry.beneath ? { z: Math.round((before.z - 0.001) * 1000) / 1000 } : {}), sections };
    const place = entry.as === undefined ? at : list.findIndex((shape) => shape.id === entry.as && shape.bone === entry.bone);
    if (place < 0) list.splice(entry.beneath ? at : at + 1, 0, next);
    else if (JSON.stringify(next) === JSON.stringify(list[place])) continue;
    else list[place] = next;
    changed++;
  }
  console.log(`${asset.id} 1:${scale} ${entry.bone} ${entry.id}: ${own ? "own" : chain ? "chained" : "fitted"} ${sections.length} sections in ${entry.views.join(", ")}`);
}
console.log(`${asset.id}: ${entries.length} lofts, ${changed} shapes changed, ${refused} refused`);
if (flag("out") !== undefined) writeDoc(read, flag("out"));
else if (args.includes("--write")) console.log(writeDoc(read) ? "written" : "file unchanged");
