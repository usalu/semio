/** 🪚️ The reproducible multiview repair of an asset of batch MF, the last step of its chain: `bun fix.ts <id> [--write] [--in <file.json>] [--out <file.json>] [--quiet] [--each]` reads `spec/<id>.json`, applies it to the catalog document (or to `--in`) in memory, runs the authors' gate of the suite on the result and prints its hard findings; `--write` writes the document back in its own indentation (the file is read right before, a second run changes nothing).
 *
 * A spec is `{ "ops"?: […], "cover"?: […], "solidify"?: […], "after"?: […], "fit"?: […] }`, applied in this order (`fit` last: the ops of `../MR/fit.ts`, run on the result until they change nothing, for a swept part whose span must be fitted to its elevation outline):
 * - `ops` and `after` are the ops of `apply.ts` (the silhouette ops `shape`, `shape-add`, `shape-set`, `shape-remove`, `set`, … plus `anchor-add`, `note`), `anchor-set` (`{ "op": "anchor-set", "id", "set": {…} }`, members of an anchor) `strips` (`{ "op": "strips", "view", "id", "except"?: [ids] }`: a front or plan sheet that stands for a curved shell is replaced, in every band that has it, by the lateral strips the side elevation builds the shell from, the elevation shapes `<id>` and `<id>-…` swept along w, so the three views are one body at every pose), `sheet-chord` (`{ "op": "sheet-chord", "view", "id", "except"?: [ids] }`: a thin sheet swept along v whose outline was drawn along the bone axis although the shell it stands for is curved in the side elevation is laid, in every band, along the chord between the two points of the elevation shapes `<id>` and `<id>-…` that the two ends of the sheet show at rest, with `zTip`; the outline at rest stays, and the sheet turns like the shell), `shape-copy` (`{ "op": "shape-copy", "from": "<scale key>", "scale", "view", "id", "as"?: "<new id>", "after"?, "set"?: {…} }`: a shape a finer band draws and the coarser one lacks, or with `as` a second copy in the same band (the rear legs behind the front legs), added once) and `contact-split` (`{ "op": "contact-split", "anchor", "parts": [{ "id", "title", "x", "y", "z", … }], "padSize"?: [a, b, c] }`, see {@link splitContact}).
 * - `cover` adds to the front or the plan a part the view hides at rest and shows when the asset is tilted (the carcass under a top, the far feet): `{ "scales"?: [keys], "view": "front" | "plan", "from": "<elevation shape id>", "id"?: "<new id, default the elevation id>", "z0", "z1", "spans"?: { "<new id>": [z0, z1] }, "z"?: paint order, "before"?: "<shape id, default first>" }` is the elevation shape swept laterally from `z0` to `z1` (asset coordinates; `spans` gives several, a left and a right leg), with the paint of the elevation shape and, unless `z` says otherwise, painted just under the shapes of its bone in that view; a shape of that id that the view already has keeps its paint and takes the swept elevation profile (the exact solid for a part whose own outline in the view is a plain rectangle); a scale whose elevation lacks the shape is passed over.
 * - `solidify` turns flat front and plan shapes of bones that a pose turns into solids. A rule is `{ "scales"?: [keys], "views"?: ["front", "plan"], "bones"?: [ids], "under"?: "<bone id: it and every bone below it>", "only"?: [shape ids], "skip"?: [shape ids], "mode"?: "auto" | "w" | "axis", "twins"?: { "<shape id>": "<elevation shape id>" }, "ranges"?: { "<shape id>": [low, high] }, "surface"?: false }`. Every closed shape of the bones becomes a solid; an open stroke stays flat. A shape needs the depth it lacks: it comes from `ranges`, else from the elevation twin (the elevation shape of the same id and bone, or the one `twins` names, or the id without what the regular expression `strip` matches: `"-[01]$"` for a left and a right copy of one elevation part; `"rename": ["-r$", "-l"]` for a right part whose elevation twin carries the name of the left one), else the shape is a skin of 2 mm on the face of the bone that the view sees (the foremost, in the plan the highest, filled elevation shape of the bone over the extent of the shape), or on the face of the elevation shape that `"on": { "<shape id>": "<elevation shape id>" }` names, unless `surface` is false. Mode `w` makes the shape the elevation profile swept laterally over the lateral range the flat shape had (elevation, front and plan agree at every pose by construction; the view then draws a rectangle at rest). Mode `axis` keeps the drawn outline: it is re-expressed in the bone frame and swept along the bone axis that points at the viewer of the view at rest (`u` or `v`, within 15 degrees: a leg or a back that rests a little inclined is swept along its own normal, and its section is the drawn outline narrowed by the lean of the sweep, so the outline at rest stays as drawn) over the range of the twin. A part that leans too far for `axis` (a splayed leg) takes `w` with the lateral span of the flat shape at the joint and, as `zTip`, at the tip of the bone. `"cuts": { "<shape id>": [u…] }` cuts such a swept rectangle along the bone into pieces `<id>`, `<id>-2`, …, the later ones 2 mm narrower on each side so that they stay behind the doors in front of them (a carcass behind two doors: the paint order of a pose follows the nearest point of each shape, so a carcass in one piece, whose foot comes forward when it is tilted back, would be painted over the door that hangs higher). `auto` takes `w` when the flat shape is an axis-parallel rectangle that the swept twin reproduces within 1 mm at rest, else `axis`, else (a bone resting inclined) `w`.
 * @see ./apply.ts — the ops
 * @see ../../../📋️multiview-brief.md — the job */
import { readFileSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { applyAll } from "./apply.ts";
import { S, W, catalogFile } from "./files.ts";

const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));
const { flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));

type V3 = readonly [number, number, number];
type P2 = [number, number];
type Frame = { o: V3; u: V3; v: V3; w: V3 };
type Rule = { scales?: string[]; views?: ("front" | "plan")[]; bones?: string[]; under?: string; only?: string[]; skip?: string[]; mode?: "auto" | "w" | "axis"; twins?: Record<string, string>; strip?: string; rename?: [string, string]; ranges?: Record<string, [number, number]>; on?: Record<string, string>; cuts?: Record<string, number[]>; surface?: boolean };
const SKIN = 0.002;
const INSET = 0.002;
const NEAR = Math.cos((15 * Math.PI) / 180);
const GEOMETRY = ["kind", "x", "y", "w", "h", "r", "points", "closed", "d", "cx", "cy", "rx", "ry", "x0", "y0", "x1", "y1", "r0", "r1", "solid", "followTip", "rotationDeg"];
const dot = (a: V3, b: V3): number => a[0] * b[0] + a[1] * b[1] + a[2] * b[2];
const r6 = (value: number): number => Math.round(value * 1e6) / 1e6 + 0;
const ringOf = (shape: any): P2[] | undefined => {
  const flat = flattenGeometry(shape);
  return flat.closed && flat.rings.length === 1 ? flat.rings[0].map((p: any) => [p.x, p.y] as P2) : undefined;
};
const allPoints = (shape: any): P2[] => flattenGeometry(shape).rings.flat().map((p: any) => [p.x, p.y] as P2);
const rangeOf = (points: readonly P2[], k: 0 | 1): [number, number] => [Math.min(...points.map((p) => p[k])), Math.max(...points.map((p) => p[k]))];
const isBox = (ring: readonly P2[]): boolean => {
  const [x, y] = [rangeOf(ring, 0), rangeOf(ring, 1)];
  return ring.every((p) => (Math.abs(p[0] - x[0]) < 1e-6 || Math.abs(p[0] - x[1]) < 1e-6) && (Math.abs(p[1] - y[0]) < 1e-6 || Math.abs(p[1] - y[1]) < 1e-6));
};

/** 🧊️ Turns the flat shapes a rule names into solids and returns one line per shape it changed or refused. */
export function solidify(asset: any, rule: Rule): string[] {
  const frames = restFrames(asset.rig) as Record<string, { frame: Frame }>;
  const log: string[] = [];
  const bones = new Set<string>(rule.bones ?? []);
  if (rule.under !== undefined) {
    bones.add(rule.under);
    for (let grew = true; grew; ) {
      grew = false;
      for (const bone of asset.rig.bones) if (!bones.has(bone.id) && bones.has(bone.parent)) grew = bones.add(bone.id).size > 0;
    }
  }
  for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
    if (representation.geometry === undefined || (rule.scales !== undefined && !rule.scales.includes(key))) continue;
    const elevation = representation.geometry.elevation as any[];
    for (const view of rule.views ?? (["front", "plan"] as const)) {
      const list = representation.geometry[view] as any[];
      const k = view === "front" ? 1 : 0;
      list.forEach((shape, index) => {
        if (!bones.has(shape.bone) || shape.solid !== undefined || shape.kind === "member") return;
        if (rule.only !== undefined && !rule.only.includes(shape.id)) return;
        if (rule.skip?.includes(shape.id)) return;
        const named = rule.only !== undefined;
        const refuse = (why: string, loud = named): void => void (loud && log.push(`1:${key} ${view} ${shape.id}: REFUSED, ${why}`));
        const frame = frames[shape.bone]!.frame;
        const twin = elevation.find((candidate) => candidate.id === (rule.twins?.[shape.id] ?? shape.id.replace(new RegExp(rule.strip ?? rule.rename?.[0] ?? "^$", "u"), rule.strip === undefined ? (rule.rename?.[1] ?? "") : "")) && candidate.bone === shape.bone);
        if (twin?.solid !== undefined && rule.ranges?.[shape.id] === undefined) return refuse("the elevation twin is a solid itself", true);
        const flat = flattenGeometry(shape);
        const rings = (flat.rings as { x: number; y: number }[][]).map((ring) => ring.map((p) => [p.x, p.y] as P2));
        if (!flat.closed || rings.length === 0) return refuse("an open stroke stays flat");
        const all = rings.flat();
        const viewer: V3 = view === "front" ? [1, 0, 0] : [0, 1, 0];
        const axis = Math.abs(dot(frame.u, viewer)) > NEAR ? "u" : Math.abs(dot(frame.v, viewer)) > NEAR ? "v" : undefined;
        const lateral = rangeOf(all, view === "front" ? 0 : 1);
        const other = rangeOf(all, view === "front" ? 1 : 0);
        const worldOf = (candidate: any, m: 0 | 1): number[] => allPoints(candidate).map(([a, b]) => frame.o[m] + a * frame.u[m] + b * frame.v[m]);
        const paint = Object.fromEntries(Object.entries(shape).filter(([member]) => !GEOMETRY.includes(member)));
        let mode = rule.mode ?? "auto";
        if (mode === "auto") {
          const world = twin === undefined ? [] : worldOf(twin, k);
          const same = twin !== undefined && rings.length === 1 && ringOf(twin) !== undefined && Math.abs(frame.w[2]) > 0.9999 && isBox(all) && Math.abs(Math.min(...world) - other[0]) < 0.001 && Math.abs(Math.max(...world) - other[1]) < 0.001;
          mode = same || (axis === undefined && twin !== undefined) ? "w" : "axis";
        }
        const lateralSweep = (): void => {
          if (twin === undefined || ringOf(twin) === undefined) return refuse("no closed elevation twin to sweep laterally", true);
          const length = asset.rig.bones.find((bone: any) => bone.id === shape.bone).lengthM as number;
          const travel = length * frame.u[k];
          const ends = Math.abs(travel) < 0.02 || isBox(all) ? [] : [0, 1].map((end) => all.filter((p) => Math.abs((p[view === "front" ? 1 : 0]! - frame.o[k]) / travel - end) < 0.25).map((p) => p[view === "front" ? 0 : 1]!));
          const spans = ends.map((values) => [Math.min(...values), Math.max(...values)]);
          const tapered = spans.length === 2 && ends.every((values) => values.length > 1) && Math.max(Math.abs(spans[0]![0]! - spans[1]![0]!), Math.abs(spans[0]![1]! - spans[1]![1]!)) > 0.001;
          const [z0, z1] = tapered ? spans[0]! : lateral;
          const profile = ringOf(twin)!;
          const cuts = rule.cuts?.[shape.id];
          if (cuts !== undefined && isBox(profile)) {
            const [along, across] = [rangeOf(profile, 0), rangeOf(profile, 1)];
            const stops = [along[0], ...cuts, along[1]];
            list.splice(index, 1, ...stops.slice(0, -1).map((from, i) => ({ ...paint, id: i === 0 ? shape.id : `${shape.id}-${i + 1}`, kind: "poly", points: [[from, across[0]], [stops[i + 1]!, across[0]], [stops[i + 1]!, across[1]], [from, across[1]]].map(([a, b]) => [r6(a!), r6(b!)]), closed: true, solid: { kind: "extruded", sweep: "w", z0: r6(z0! + (i === 0 ? 0 : INSET)), z1: r6(z1! - (i === 0 ? 0 : INSET)) } })));
            log.push(`1:${key} ${view} ${shape.id}: w ${[z0, z1].map((v) => r6(v!)).join(" … ")} in ${stops.length - 1} pieces`);
            return;
          }
          list[index] = { ...paint, kind: "poly", points: profile.map(([a, b]) => [r6(a), r6(b)]), closed: true, solid: { kind: "extruded", sweep: "w", z0: r6(z0!), z1: r6(z1!), ...(tapered ? { zTip: spans[1]!.map(r6) } : {}) } };
          log.push(`1:${key} ${view} ${shape.id}: w ${[z0, z1].map((v) => r6(v!)).join(" … ")}${tapered ? ` tip ${spans[1]!.map(r6).join(" … ")}` : ""}`);
        };
        if (mode === "w") return lateralSweep();
        if (axis === undefined) return refuse("the bone rests inclined, no axis points at the viewer", true);
        const sign = Math.sign(dot(frame[axis], viewer));
        let range = rule.ranges?.[shape.id] ?? (twin === undefined ? undefined : rangeOf(allPoints(twin), axis === "u" ? 0 : 1));
        let how = "twin";
        if (range === undefined && rule.surface !== false) {
          const bodies = elevation.filter((candidate) => candidate.bone === shape.bone && flattenGeometry(candidate).closed && (rule.on?.[shape.id] === undefined || candidate.id === rule.on[shape.id]));
          const over = bodies.filter((candidate) => { const keys = worldOf(candidate, k === 1 ? 1 : 0); return Math.max(...keys) > other[0] + 1e-6 && Math.min(...keys) < other[1] - 1e-6; });
          const faces = (over.length > 0 ? over : bodies).flatMap((candidate) => worldOf(candidate, k === 1 ? 0 : 1));
          if (faces.length === 0) return refuse("no elevation shape of the bone gives the face it lies on");
          const face = Math.max(...faces);
          const ends = [(face - SKIN - frame.o[view === "front" ? 0 : 1]) * sign, (face - frame.o[view === "front" ? 0 : 1]) * sign];
          range = [Math.min(...ends), Math.max(...ends)];
          how = "skin";
        }
        if (range === undefined) return refuse("no depth (no twin, no range)");
        const along = axis === "u" ? frame.v : frame.u;
        const middle = (range[0] + range[1]) / 2;
        const lean = (range[1] - range[0]) * Math.abs(frame[axis][k]);
        const [low, high] = other;
        if (lean > 0.8 * (high - low)) return twin !== undefined && rule.mode === undefined ? lateralSweep() : refuse(`the bone leans ${(lean * 1000).toFixed(0)} mm over the sweep and the outline is ${((high - low) * 1000).toFixed(0)} mm: no section is left`, true);
        const side = view === "front" ? 0 : 1;
        const centre = (lateral[0] + lateral[1]) / 2;
        let collapsed = false;
        const narrowed = (ring: readonly P2[]): P2[] => ring.map((p) => {
          if (lean < 1e-9) return [p[1 - side]!, p[side]!] as P2;
          if (lean < 0.25 * (high - low)) return [(low + high) / 2 + ((p[1 - side]! - (low + high) / 2) * (high - low - lean)) / (high - low), p[side]!] as P2;
          const level = p[side]! + Math.sign(centre - p[side]!) * 1e-7;
          const cuts: number[] = [];
          for (const other of rings) other.forEach((a, i) => {
            const b = other[(i + 1) % other.length]!;
            if (a[side]! > level !== b[side]! > level) cuts.push(a[1 - side]! + ((b[1 - side]! - a[1 - side]!) * (level - a[side]!)) / (b[side]! - a[side]!));
          });
          const q = p[1 - side]!;
          if (cuts.length < 2) return [q, p[side]!] as P2;
          const half = (Math.min(...cuts) + Math.max(...cuts)) / 2;
          if (Math.abs(q - Math.min(...cuts)) > 1e-5 && Math.abs(q - Math.max(...cuts)) > 1e-5) return [q, p[side]!] as P2;
          const moved = q + (Math.sign(half - q) * lean) / 2;
          if (Math.sign(half - moved) !== Math.sign(half - q)) collapsed = true;
          return [moved, p[side]!] as P2;
        });
        const local = (ring: readonly P2[]): P2[] => narrowed(ring).map(([q, across]) => [r6((q - frame.o[k] - middle * frame[axis][k]) / along[k]), r6(across - frame.o[2])]);
        const sections = rings.map(local);
        if (collapsed) return twin !== undefined && rule.mode === undefined ? lateralSweep() : refuse("the outline is thinner than the lean of the sweep somewhere: no section is left", true);
        const geometry = rings.length === 1 ? { kind: "poly", points: sections[0]!, closed: true } : { kind: "path", d: sections.map((ring) => `M ${ring.map((p) => p.join(" ")).join(" L ")} Z`).join(" ") };
        list[index] = { ...paint, ...geometry, solid: { kind: "extruded", sweep: axis, z0: r6(range[0]), z1: r6(range[1]) } };
        log.push(`1:${key} ${view} ${shape.id}: ${axis} ${how} ${range.map(r6).join(" … ")}`);
      });
    }
  }
  return log;
}

type Cover = { scales?: string[]; view: "front" | "plan"; from: string; id?: string; z0?: number; z1?: number; spans?: Record<string, [number, number]>; z?: number; before?: string };

/** 🫥️ Adds the covered parts a rule names and returns one line per shape. */
export function cover(asset: any, rule: Cover): string[] {
  const log: string[] = [];
  for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
    if (representation.geometry === undefined || (rule.scales !== undefined && !rule.scales.includes(key))) continue;
    const source = (representation.geometry.elevation as any[]).find((candidate) => candidate.id === rule.from);
    const ring = source === undefined ? undefined : ringOf(source);
    if (ring === undefined || source.solid !== undefined) continue;
    const list = representation.geometry[rule.view] as any[];
    const paint = Object.fromEntries(Object.entries(source).filter(([member]) => !GEOMETRY.includes(member)));
    for (const [id, [z0, z1]] of Object.entries(rule.spans ?? { [rule.id ?? rule.from]: [rule.z0!, rule.z1!] as [number, number] })) {
      const at = list.findIndex((candidate) => candidate.id === id);
      const under = list.filter((candidate, index) => candidate.bone === source.bone && index !== at).map((candidate) => candidate.z as number);
      const z = rule.z ?? (at >= 0 ? list[at].z : r6(Math.min(...under, source.z) - 0.05));
      const shape = { ...(at >= 0 ? Object.fromEntries(Object.entries(list[at]).filter(([member]) => !GEOMETRY.includes(member))) : paint), id, z, kind: "poly", points: ring.map(([a, b]) => [r6(a), r6(b)]), closed: true, solid: { kind: "extruded", sweep: "w", z0, z1 } };
      if (at >= 0) list[at] = shape;
      else list.splice(rule.before === undefined ? 0 : Math.max(0, list.findIndex((candidate) => candidate.id === rule.before)), 0, shape);
      log.push(`1:${key} ${rule.view} ${id}: cover ${z0} … ${z1} z ${z}`);
    }
  }
  return log;
}

type Split = { anchor: string; parts: any[]; padSize?: (number | null)[] };
const rotate = (q: readonly number[], p: readonly number[]): number[] => {
  const [x, y, z, w] = q as [number, number, number, number];
  const t = [2 * (y * p[2]! - z * p[1]!), 2 * (z * p[0]! - x * p[2]!), 2 * (x * p[1]! - y * p[0]!)];
  return [p[0]! + w * t[0]! + y * t[2]! - z * t[1]!, p[1]! + w * t[1]! + z * t[0]! - x * t[2]!, p[2]! + w * t[2]! + x * t[1]! - y * t[0]!];
};
const multiply = (a: readonly number[], b: readonly number[]): number[] => [a[3]! * b[0]! + a[0]! * b[3]! + a[1]! * b[2]! - a[2]! * b[1]!, a[3]! * b[1]! - a[0]! * b[2]! + a[1]! * b[3]! + a[2]! * b[0]!, a[3]! * b[2]! + a[0]! * b[1]! - a[1]! * b[0]! + a[2]! * b[3]!, a[3]! * b[3]! - a[0]! * b[0]! - a[1]! * b[1]! - a[2]! * b[2]!];

/** 🦶️ Replaces a lumped contact anchor by one anchor per physical contact (D45): in the anchors (a part is the lumped anchor with the members of the part laid over it), the contacts of poses and clips, the structural supports and the support pads of the volumes. A pad repeated once per contact (itself, or the body solid it refers to with `of`) becomes one pad per copy, given to the nearest part; a single pad becomes one pad per part, moved by the offset of the part from the lumped anchor, with the members of `padSize` that are not null as its size. A second run finds the lumped anchor gone and changes nothing. */
export function splitContact(asset: any, o: Split): void {
  const anchors = asset.anchors as any[];
  const at = anchors.findIndex((candidate) => candidate.id === o.anchor);
  if (at < 0) return;
  const lump = anchors[at];
  const parts = o.parts.map((part) => ({ ...lump, ...part }));
  const ids = parts.map((part) => part.id as string);
  anchors.splice(at, 1, ...parts);
  const spread = (list: string[]): string[] => list.flatMap((id) => (id === o.anchor ? ids : [id]));
  for (const pose of asset.poses) if (pose.contacts !== undefined) pose.contacts = spread(pose.contacts);
  for (const clip of asset.clips ?? []) if (clip.contacts !== undefined) clip.contacts = clip.contacts.flatMap((contact: any) => (contact.anchor === o.anchor ? ids.map((id) => ({ ...contact, anchor: id })) : [contact]));
  asset.structure.supports = spread(asset.structure.supports);
  const place = (part: any): number[] => [part.x ?? 0, part.y ?? 0, part.z ?? 0];
  for (const volume of Object.values(asset.volumes ?? {}) as any[]) {
    if (volume.structure === undefined) continue;
    volume.structure = volume.structure.flatMap((solid: any) => {
      if (solid.anchor !== o.anchor) return [solid];
      const { of, ...own } = solid;
      const source = of === undefined ? own : { ...volume.body.find((candidate: any) => candidate.id === of), ...own };
      const { repeat, ...pad } = source;
      const key = pad.origin !== undefined && pad.transform?.translation === undefined ? "origin" : "translation";
      const base: number[] = key === "origin" ? pad.origin : (pad.transform?.translation ?? [0, 0, 0]);
      const withPlace = (position: number[], extra: object = {}): any => (key === "origin" ? { ...pad, origin: position.map(r6), ...extra } : { ...pad, ...extra, transform: { ...pad.transform, ...(extra as any).transform, translation: position.map(r6) } });
      if (repeat?.count === parts.length) {
        const copies = parts.map((_, k) => {
          if (repeat.rotationRad === undefined) return { position: base.map((v, i) => v + k * (repeat.translation?.[i] ?? 0)), extra: {} };
          const half = (k * repeat.rotationRad) / 2;
          const q = [...(repeat.rotationAxis as number[]).map((v) => v * Math.sin(half)), Math.cos(half)];
          const origin: number[] = repeat.origin ?? [0, 0, 0];
          return { position: rotate(q, base.map((v, i) => v - origin[i]!)).map((v, i) => v + origin[i]!), extra: k === 0 ? {} : { transform: { rotation: multiply(q, pad.transform?.rotation ?? [0, 0, 0, 1]).map(r6) } } };
        });
        const free = new Set(parts.map((_, i) => i));
        return copies.map((copy) => {
          const nearest = [...free].sort((a, b) => Math.hypot(...place(parts[a]).map((v, i) => v - copy.position[i]!)) - Math.hypot(...place(parts[b]).map((v, i) => v - copy.position[i]!)))[0]!;
          free.delete(nearest);
          return { ...withPlace(copy.position, copy.extra), id: `support-${ids[nearest]}`, anchor: ids[nearest] };
        });
      }
      if (repeat !== undefined) throw new Error(`pad "${solid.id}" is repeated ${repeat.count} times for ${parts.length} contacts`);
      return parts.map((part) => ({ ...withPlace(base.map((v, i) => v + place(part)[i]! - place(lump)[i]!)), ...(o.padSize === undefined ? {} : { size: (pad.size as number[]).map((value, i) => o.padSize![i] ?? value) }), id: `support-${part.id}`, anchor: part.id }));
    });
  }
}

/** 🧷️ Applies a whole spec to an asset in place. */
export function applySpec(asset: any, spec: { ops?: any[]; cover?: Cover[]; solidify?: Rule[]; after?: any[] }): string[] {
  const log: string[] = [];
  const run = (ops: any[] | undefined): void => {
    for (const o of ops ?? []) {
      if (o.op === "anchor-set") {
        const anchor = asset.anchors.find((candidate: any) => candidate.id === o.id);
        if (anchor === undefined) throw new Error(`anchor "${o.id}" not found`);
        Object.assign(anchor, o.set);
      } else if (o.op === "contact-split") splitContact(asset, o);
      else if (o.op === "sheet-chord") {
        const frames = restFrames(asset.rig) as Record<string, { frame: Frame }>;
        for (const [key, representation] of Object.entries(asset.representations) as [string, any][]) {
          if (representation.geometry === undefined) continue;
          const shape = (representation.geometry[o.view] as any[]).find((candidate) => candidate.id === o.id);
          const strips = (representation.geometry.elevation as any[]).filter((candidate) => (candidate.id === o.id || candidate.id.startsWith(`${o.id}-`)) && !(o.except ?? []).includes(candidate.id) && candidate.bone === shape?.bone);
          if (shape === undefined || strips.length === 0 || shape.solid?.sweep !== "v" || shape.solid.zTip !== undefined) continue;
          const frame = frames[shape.bone]!.frame;
          const k = o.view === "plan" ? 0 : 1;
          const length = asset.rig.bones.find((bone: any) => bone.id === shape.bone).lengthM as number;
          const reach = (p: P2): number => p[0] * frame.u[k] + p[1] * frame.v[k];
          const points = strips.flatMap(allPoints);
          const sheet = (shape.solid.z0 + shape.solid.z1) / 2;
          const drawn = (shape.points as P2[]).map((p) => p[0]);
          const ends = [Math.min(...drawn), Math.max(...drawn)].map((u) => {
            const want = u * frame.u[k] + sheet * frame.v[k];
            return points.reduce((best, p) => (Math.abs(reach(p) - want) < Math.abs(reach(best) - want) ? p : best));
          });
          const [a, b] = ends as [P2, P2];
          if (Math.abs(b[0] - a[0]) < 0.01) continue;
          const slope = (b[1] - a[1]) / (b[0] - a[0]);
          const thick = shape.solid.z1 - shape.solid.z0;
          const low = Math.min(...drawn);
          const high = Math.max(...drawn);
          shape.points = (shape.points as P2[]).map(([u, w]) => [r6(a[0] + ((u - low) / (high - low)) * (b[0] - a[0])), w]);
          const z0 = a[1] - a[0] * slope;
          shape.solid = { kind: "extruded", sweep: "v", z0: r6(z0), z1: r6(z0 + thick), zTip: [r6(z0 + length * slope), r6(z0 + length * slope + thick)] };
          log.push(`1:${key} ${o.view} ${o.id}: sheet along the chord (${a.map(r6).join(", ")}) … (${b.map(r6).join(", ")})`);
        }
      } else if (o.op === "strips") {
        for (const representation of Object.values(asset.representations) as any[]) {
          if (representation.geometry === undefined) continue;
          const list = representation.geometry[o.view] as any[];
          const at = list.findIndex((candidate) => candidate.id === o.id);
          const strips = (representation.geometry.elevation as any[]).filter((candidate) => (candidate.id === o.id || candidate.id.startsWith(`${o.id}-`)) && !(o.except ?? []).includes(candidate.id) && candidate.solid?.sweep === "w");
          if (at < 0 || strips.length < 2 || list[at].solid?.sweep === "w") continue;
          const paint = Object.fromEntries(Object.entries(list[at]).filter(([member]) => !GEOMETRY.includes(member)));
          list.splice(at, 1, ...strips.map((strip) => ({ ...paint, id: strip.id, kind: strip.kind, points: structuredClone(strip.points), closed: true, solid: structuredClone(strip.solid) })));
        }
      } else if (o.op === "shape-copy") {
        const source = (asset.representations[o.from].geometry[o.view] as any[]).find((candidate) => candidate.id === o.id);
        const list = asset.representations[o.scale].geometry[o.view] as any[];
        if (source === undefined) throw new Error(`1:${o.from} ${o.view} has no shape "${o.id}"`);
        if (!list.some((candidate) => candidate.id === (o.as ?? o.id))) list.splice(o.after === undefined ? list.length : list.findIndex((candidate) => candidate.id === o.after) + 1, 0, { ...structuredClone(source), ...(o.as === undefined ? {} : { id: o.as }), ...o.set });
      }
      else applyAll(asset, [o]);
    }
  };
  run(spec.ops);
  for (const rule of spec.cover ?? []) log.push(...cover(asset, rule));
  for (const rule of spec.solidify ?? []) log.push(...solidify(asset, rule));
  run(spec.after);
  return log;
}

if (import.meta.main) {
  const args = process.argv.slice(2);
  const flag = (name: string): string | undefined => (args.includes(`--${name}`) ? args[args.indexOf(`--${name}`) + 1] : undefined);
  const slug = args[0]!;
  const file = flag("in") ?? catalogFile(slug);
  const spec = JSON.parse(readFileSync(`${W}/spec/${slug}.json`, "utf8"));
  const text = readFileSync(file, "utf8");
  const doc = JSON.parse(text);
  const log = applySpec(doc.asset, spec);
  if (spec.fit !== undefined) {
    const scratch = join(process.env.GEN ?? "/tmp", `fit-${slug}-${process.pid}`);
    writeFileSync(`${scratch}.json`, JSON.stringify(doc));
    writeFileSync(`${scratch}.ops.json`, JSON.stringify(spec.fit));
    for (let round = 0; round < 4; round++) {
      const run = Bun.spawnSync([process.execPath, join(W, "..", "MR", "fit.ts"), `${scratch}.json`, "--ops", `${scratch}.ops.json`, "--out", `${scratch}.json`]);
      const lines = run.stdout.toString().trim().split("\n");
      if (round === 0) log.push(...lines.filter((line) => !line.startsWith("written") && !line.includes(" ops, ")).map((line) => `fit ${line}`));
      if (/ 0 changed something/u.test(lines.at(-1) ?? "")) break;
    }
    doc.asset = JSON.parse(readFileSync(`${scratch}.json`, "utf8")).asset;
    rmSync(`${scratch}.json`);
    rmSync(`${scratch}.ops.json`);
  }
  if (!args.includes("--quiet")) for (const line of log) console.log(line);
  for (const line of log.filter((entry) => entry.includes("REFUSED"))) console.error(line);
  const second = text.split("\n", 2)[1] ?? "";
  const next = JSON.stringify(doc, null, second.length - second.trimStart().length) + (text.endsWith("\n") ? "\n" : "");
  if (!args.includes("--no-scan")) {
    const { scanAsset, groupsOf } = await import(join(S, "🧪️tests", "🔭️multiview-coverage", "🔎️scan.ts"));
    const row = scanAsset(doc.asset);
    for (const g of groupsOf(row.findings).filter((group: any) => group.tier === "hard")) console.log(`  ${g.code} ${g.views.join("/")} bone=${g.bone ?? ""} anchor=${g.anchor ?? ""} n=${g.count} scales=${g.scales.join(",")} poses=${g.poses.join(",")} worst ${g.worst.deltaMm ?? ""}@${g.worst.scale}/${g.worst.pose}: ${String(g.worst.message).slice(0, 170)}`);
    if (args.includes("--each")) for (const f of row.findings.filter((finding: any) => finding.tier === "hard")) console.log(`    ${f.code} 1:${f.scale} ${f.pose} ${f.bone ?? f.anchor ?? ""} ${f.deltaMm ?? ""}: ${String(f.message).slice(0, 140)}`);
    console.log(`${slug}: ${row.summary.hard} hard findings, ${row.summary.reported} reported, ${log.length} shapes solidified`);
  }
  if (flag("out") !== undefined) writeFileSync(flag("out")!, next);
  else if (args.includes("--write")) {
    if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
    if (next !== text) writeFileSync(file, next);
    console.log(next === text ? `${slug}: file unchanged` : `${slug}: written`);
  }
}
