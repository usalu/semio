/** 🍃️ Gives the front and the plan drawing of every foliage mass rigid carriers instead of a deformed sheet with depth plates, as the last step of the chain of a plant: `bun body.ts <id|file.json> [--save ops.json | --ops ops.json] [--out file] [--write] [--keep 0.5] [--limit 0.95] [--inset 0.5] [--margin 0.08] [--scales 50,100] [--only bone,bone] [--no-verify]`.
 *
 * The depth plates `mvc-*` of `../MV-1/carry.ts` are slices of a foliage mass that show beside the drawn outline in a wind pose, and the sheets it renamed to `-mvs` are carried by the engine vertex by vertex at the depth of the middle of the elevation silhouette, which folds an outline where that depth changes fast with the height (straight edges, notches and teeth in a gale). The schema has no solid that carries a foliage mass on a bone that stands askew at rest (measured by `solids.ts`, section 2 of `../../../multiview-MV-L1.md`), so this tool draws each mass in each view with rigid carriers only, which the engine turns as real bodies:
 * - `const`: the flat sheet as authored, carried at the depth of the middle of its elevation twin: a rigid upright (front) or level (plan) sheet. It is kept untouched when it stays within `keep` × tolerance of the elevation extent over every subject of the matrix (poses, clip instants, transitions);
 * - `card`: the outline as an `extruded` solid 0.004 mm thick across one of the two bone axes in the sagittal plane (`sweep` `u` or `v`), a rigid card through the mass that stands askew in depth: at rest it is seen exactly as the authored outline, and when the wind turns the bone its outline stretches or shrinks as a whole, as the outline of a tilted body does. A card may be skinned between its bone and the parent limb (`weights` of 0.75, 0.5, 0.25 or 0 for the bone): a soft mass whose outline follows the limb more than its own turn; the rigid card is taken unless a skinned one is better by more than 0.02 tolerance per quarter. The place of the card along its sweep axis is solved so that the worst distance to the elevation extent is smallest, and the card stays inside the depth (front) or the height (plan) the elevation shapes of the bone cover, half a tolerance allowed;
 * - the carrier with the smallest worst distance is taken (the authored sheet when it is as good to 0.05 tolerance). Where it still ends more than `limit` × tolerance short of the elevation extent, the part of the mass that reaches further is drawn as a lobe `mvl-<bone>-<lo|hi><n>`: an ellipse on a card through the point of the elevation outline that sets the extent in the worst subject, its tip `inset` × tolerance inside that point, as large as its emergence in that subject (between 0.05 and 0.28 of the outline), placed where it lies inside the drawn outline at rest with `margin` × tolerance to spare and below it in the paint order. The default pose draws what it drew; in a gale the lobe shows as one round foliage lobe beyond the outline.
 * Every closed flat shape of the bone in the view takes the same carrier, so the drawing of a mass moves as one. Open strokes stay flat (a solid has no open profile). A bone whose views draw different parts in the default pose (more than half a tolerance apart) is left alone.
 * A search run (`--save`) writes the chosen shapes as absolute values to an ops file; `--ops` applies such a file: it removes every `mvc-*` plate, every `mvl-*` lobe and every suffix `-mvs`, then sets the listed shapes in the place of the shape of the same bone and id and the lobes below their bone, so the step is idempotent and runs after `../MV-1/make.sh` whatever that step wrote.
 * @see ../MV-1/carry.ts — the plates this tool replaces
 * @see ../MR/gate.ts — the measurement of the matrix
 * @see ../../🧰️silhouette/R4-P/preserve.ts — the writer that keeps every untouched byte */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, fileOf } from "../MR/lib.ts";
import { rewrite } from "../../🧰️silhouette/R4-P/preserve.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { evaluate, flattenGeometry } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

type View = "front" | "plan";
type Frame = { o: number[]; u: number[]; v: number[]; w: number[] };
type Range = [number, number];
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const file = fileOf(args[0]!);
const text = readFileSync(file, "utf8");
const previous = JSON.parse(text);
const next = structuredClone(previous);
const asset = next.asset;
const KEEP = Number(flag("keep") ?? 0.5);
const only = flag("only")?.split(",");
const scales = flag("scales")?.split(",");
const tolerance: number = engine.toleranceM(asset);
const COMPARED = ["body", "detail", "glazing"];
const PLATE = "mvc-";
const SHEET = "-mvs";
const LOBE = "mvl-";
const LIMIT = Number(flag("limit") ?? 0.95);
const INSET = Number(flag("inset") ?? 0.5);
const MARGIN = Number(flag("margin") ?? 0.08);
const THICK = 2e-6;
const COMMON = ["id", "bone", "role", "z", "filled", "tone", "region", "regions", "material", "hatch", "penMm"];
const VIEWS: readonly View[] = ["front", "plan"];
const rest = restFrames(asset.rig) as Record<string, { frame: Frame }>;
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined && (scales === undefined || scales.includes(key)));
const round = (n: number): number => Math.round(n * 1e6) / 1e6;
const mm = (m: number): string => (m * 1000).toFixed(0);
const baseId = (id: string): string => (id.endsWith(SHEET) ? id.slice(0, -SHEET.length) : id);
const isCard = (shape: any): boolean => shape.solid?.kind === "extruded" && (shape.solid.sweep === "u" || shape.solid.sweep === "v") && Math.abs(shape.solid.z1 - shape.solid.z0) < 1e-5 && !String(shape.id).startsWith(PLATE) && !String(shape.id).startsWith(LOBE);

/** 🧹️ The geometry without the depth plates and without the sheet suffix: the state the search starts from. */
function strip(geometry: any): void {
  for (const view of VIEWS) {
    geometry[view] = geometry[view].filter((shape: any) => !String(shape.id).startsWith(PLATE) && !String(shape.id).startsWith(LOBE));
    for (const shape of geometry[view]) if (String(shape.id).endsWith(SHEET)) shape.id = baseId(shape.id);
  }
}

/** 🪪️ The axis of the paper a view compares with the side elevation (height in the front, forward in the plan) as the index of the world axis, and the index of the shape coordinate that holds it. */
const axisOf = (view: View): { world: 0 | 1; own: "x" | "y"; lateral: "x" | "y" } => (view === "front" ? { world: 1, own: "y", lateral: "x" } : { world: 0, own: "x", lateral: "y" });

/** 🃏️ A flat closed shape as a card across the bone axis `sweep` at the place `c` along it, seen at rest exactly as the shape. */
function cardOf(shape: any, view: View, sweep: "u" | "v", c: number, weights?: { bone: string; weight: number }[]): any {
  const f = rest[shape.bone]!.frame;
  const [s, t] = sweep === "u" ? [f.u, f.v] : [f.v, f.u];
  const { world, own, lateral } = axisOf(view);
  const rings: { x: number; y: number }[][] = flattenGeometry(shape).rings;
  const points = (ring: { x: number; y: number }[]): number[][] => {
    const closedTwice = ring.length > 1 && ring[0]!.x === ring[ring.length - 1]!.x && ring[0]!.y === ring[ring.length - 1]!.y;
    return (closedTwice ? ring.slice(0, -1) : ring).map((p) => [round((p[own] - f.o[world]! - c * s[world]!) / t[world]!), round(p[lateral] - f.o[2]!)]);
  };
  const out: any = {};
  for (const key of Object.keys(shape)) {
    if (key === "kind") {
      if (rings.length === 1) Object.assign(out, { kind: "poly", points: points(rings[0]!), closed: true });
      else Object.assign(out, { kind: "path", d: rings.map((ring) => `M ${points(ring).map((p) => `${p[0]} ${p[1]}`).join(" L ")} Z`).join(" ") });
    } else if (COMMON.includes(key)) out[key] = shape[key];
  }
  out.id = baseId(shape.id);
  out.solid = { kind: "extruded", sweep, z0: round(c - THICK), z1: round(c + THICK), ...(weights === undefined ? {} : { weights }) };
  return out;
}

/** 📄️ A card back as the flat shape it is seen as at rest (a search run on a file this tool wrote). */
function flatOf(shape: any, view: View): any {
  const f = rest[shape.bone]!.frame;
  const sweep: "u" | "v" = shape.solid.sweep;
  const [s, t] = sweep === "u" ? [f.u, f.v] : [f.v, f.u];
  const c = (shape.solid.z0 + shape.solid.z1) / 2;
  const { world } = axisOf(view);
  const rings: { x: number; y: number }[][] = flattenGeometry({ ...shape, solid: undefined }).rings;
  const seen = (p: { x: number; y: number }): number[] => {
    const along = round(f.o[world]! + p.x * t[world]! + c * s[world]!);
    const across = round(f.o[2]! + p.y);
    return view === "front" ? [across, along] : [along, across];
  };
  const out: any = {};
  for (const key of Object.keys(shape)) {
    if (key === "kind") {
      if (rings.length === 1) Object.assign(out, { kind: "poly", points: rings[0]!.map(seen), closed: true });
      else Object.assign(out, { kind: "path", d: rings.map((ring) => `M ${ring.map((p) => seen(p).join(" ")).join(" L ")} Z`).join(" ") });
    } else if (COMMON.includes(key)) out[key] = shape[key];
  }
  return out;
}

const convertible = (shape: any): boolean => shape.solid === undefined && shape.kind !== "member" && COMPARED.includes(shape.role) && flattenGeometry(shape).closed === true && flattenGeometry(shape).rings.every((ring: unknown[]) => ring.length >= 3);

/** 📏️ Per scale, view and bone the largest distance between the extent of the bone in the view and in the side elevation over every subject, as the gate measures it. */
function drifts(candidate: any): Map<string, { m: number; subject: string }> {
  const out = new Map<string, { m: number; subject: string }>();
  for (const key of authored)
    for (const subject of engine.subjects(candidate, Number(key))) {
      const state = evaluate(candidate, subject.query, engine.scaleSelector(Number(key)));
      const range = (view: string, axis: "x" | "y"): Map<string, Range> => {
        const ranges = new Map<string, Range>();
        for (const shape of state.shapes[view]) {
          if (shape.shape.role === "ground") continue;
          const r = ranges.get(shape.shape.bone) ?? ranges.set(shape.shape.bone, [Infinity, -Infinity]).get(shape.shape.bone)!;
          for (const ring of shape.rings) for (const p of ring) { r[0] = Math.min(r[0], p[axis]); r[1] = Math.max(r[1], p[axis]); }
        }
        return ranges;
      };
      for (const view of VIEWS) {
        const { own } = axisOf(view);
        const e = range("elevation", own);
        for (const [bone, r] of range(view, own)) {
          const a = e.get(bone);
          if (a === undefined) continue;
          const m = Math.max(Math.abs(a[0] - r[0]), Math.abs(a[1] - r[1]));
          const k = `${key}|${view}|${bone}`;
          const known = out.get(k) ?? out.set(k, { m: -1, subject: subject.id }).get(k)!;
          if (m > known.m) Object.assign(known, { m, subject: subject.id });
        }
      }
    }
  return out;
}

/** 🧩️ The ops of a result: per scale and view every shape that differs from the stripped state, as absolute values. */
function opsOf(result: any, stripped: any): any {
  const ops: any = {};
  for (const key of Object.keys(result.representations)) {
    const geometry = result.representations[key].geometry;
    if (geometry === undefined) continue;
    for (const view of VIEWS) {
      const before = new Map<string, string>(stripped.representations[key].geometry[view].map((shape: any) => [`${shape.bone} ${shape.id}`, JSON.stringify(shape)]));
      const set = geometry[view].filter((shape: any) => before.get(`${shape.bone} ${baseId(shape.id)}`) !== JSON.stringify(shape));
      if (set.length > 0) ((ops[key] ??= {})[view] = set);
    }
  }
  return ops;
}

/** 🔧️ The ops applied to the asset: plates removed, suffixes removed, the listed shapes set in the place of the shape of their bone and id. */
function applyOps(target: any, ops: any): number {
  let count = 0;
  for (const key of Object.keys(target.representations)) {
    const geometry = target.representations[key].geometry;
    if (geometry === undefined) continue;
    strip(geometry);
    for (const view of VIEWS)
      for (const shape of ops[key]?.[view] ?? []) {
        const at = String(shape.id).startsWith(LOBE) ? -1 : geometry[view].findIndex((known: any) => known.bone === shape.bone && baseId(known.id) === baseId(shape.id));
        if (String(shape.id).startsWith(LOBE)) geometry[view].splice(geometry[view].findIndex((known: any) => known.bone === shape.bone && !String(known.id).startsWith(LOBE)), 0, structuredClone(shape));
        else if (at < 0) throw new Error(`${target.id} 1:${key} ${view}: no shape "${baseId(shape.id)}" on bone "${shape.bone}" to set`);
        else geometry[view][at] = structuredClone(shape);
        count++;
      }
  }
  return count;
}

const before = args.includes("--no-verify") ? undefined : drifts(structuredClone(asset));
const opsFile = flag("ops");
let result: any;
if (opsFile !== undefined) {
  result = structuredClone(asset);
  const count = applyOps(result, JSON.parse(readFileSync(opsFile, "utf8")));
  console.log(`${asset.id}: ${count} shapes set from ${opsFile}`);
} else {
  const stripped = structuredClone(asset);
  for (const key of Object.keys(stripped.representations)) {
    const geometry = stripped.representations[key].geometry;
    if (geometry === undefined) continue;
    strip(geometry);
    for (const view of VIEWS) geometry[view] = geometry[view].map((shape: any) => (isCard(shape) ? flatOf(shape, view) : shape));
  }
  const probe = structuredClone(stripped);
  for (const key of authored)
    for (const view of VIEWS) {
      const list: any[] = probe.representations[key].geometry[view];
      probe.representations[key].geometry[view] = list.flatMap((shape) => (convertible(shape) ? [shape, { ...structuredClone(shape), id: `${shape.id}${SHEET}` }] : [shape]));
    }
  type Sample = { subject: string; e: Range; other: Range; flat: Range; mid: Range; now: Frame; up?: Frame };
  const parents = new Map<string, string | null>(asset.rig.bones.map((bone: any) => [bone.id, bone.parent]));
  type Score = { over: number; lo: number; hi: number; worst: number };
  const samples = new Map<string, Sample[]>();
  const vec = (p: { x: number; y: number; z: number }): number[] => [p.x, p.y, p.z];
  for (const key of authored)
    for (const subject of engine.subjects(probe, Number(key))) {
      const state = evaluate(probe, subject.query, engine.scaleSelector(Number(key)));
      for (const view of VIEWS) {
        const { own } = axisOf(view);
        const group = new Map<string, { e: Range; other: Range; flat: Range; mid: Range }>();
        const entry = (bone: string) => group.get(bone) ?? group.set(bone, { e: [Infinity, -Infinity], other: [Infinity, -Infinity], flat: [Infinity, -Infinity], mid: [Infinity, -Infinity] }).get(bone)!;
        const widen = (r: Range, rings: any[]): void => { for (const ring of rings) for (const p of ring) { r[0] = Math.min(r[0], p[own]); r[1] = Math.max(r[1], p[own]); } };
        for (const shape of state.shapes.elevation) if (shape.shape.role !== "ground") widen(entry(shape.shape.bone).e, shape.rings);
        for (const shape of state.shapes[view]) {
          if (shape.shape.role === "ground") continue;
          const g = entry(shape.shape.bone);
          const twin = String(shape.shape.id).endsWith(SHEET);
          const sheet = convertible(twin ? { ...shape.shape, id: baseId(shape.shape.id) } : shape.shape);
          widen(!sheet ? g.other : twin ? g.mid : g.flat, shape.rings);
        }
        for (const [bone, g] of group) {
          if (!Number.isFinite(g.flat[0]) || !Number.isFinite(g.e[0])) continue;
          const f = state.bones[bone].frame;
          const k = `${key}|${view}|${bone}`;
          const p = parents.get(bone) == null ? undefined : state.bones[parents.get(bone)!]?.frame;
          (samples.get(k) ?? samples.set(k, []).get(k)!).push({ subject: subject.id, ...g, now: { o: vec(f.o), u: vec(f.u), v: vec(f.v), w: vec(f.w) }, ...(p === undefined ? {} : { up: { o: vec(p.o), u: vec(p.u), v: vec(p.v), w: vec(p.w) } }) });
        }
      }
    }
  result = structuredClone(stripped);
  const lines: string[] = [];
  const inRing = (x: number, y: number, rings: { x: number; y: number }[][]): boolean => {
    let inside = false;
    for (const ring of rings)
      for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
        const [p, q] = [ring[i]!, ring[j]!];
        if (p.y > y !== q.y > y && x < ((q.x - p.x) * (y - p.y)) / (q.y - p.y) + p.x) inside = !inside;
      }
    return inside;
  };
  const edgeDistance = (x: number, y: number, rings: { x: number; y: number }[][]): number => {
    let best = Infinity;
    for (const ring of rings)
      for (let i = 0, j = ring.length - 1; i < ring.length; j = i++) {
        const [p, q] = [ring[i]!, ring[j]!];
        const [dx, dy] = [q.x - p.x, q.y - p.y];
        const t = dx * dx + dy * dy === 0 ? 0 : Math.max(0, Math.min(1, ((x - p.x) * dx + (y - p.y) * dy) / (dx * dx + dy * dy)));
        best = Math.min(best, Math.hypot(x - p.x - t * dx, y - p.y - t * dy));
      }
    return best;
  };
  for (const k of [...samples.keys()].sort()) {
    const [key, view, bone] = k.split("|") as [string, View, string];
    if (only !== undefined && !only.includes(bone)) continue;
    const list = samples.get(k)!;
    const { world, own, lateral } = axisOf(view);
    const f = rest[bone]!.frame;
    const atRest = list.find((sample) => sample.subject === `pose ${asset.presentation.defaultPose}`) ?? list[0]!;
    if (Math.max(Math.abs(atRest.flat[0] - atRest.e[0]), Math.abs(atRest.flat[1] - atRest.e[1])) > 0.5 * tolerance) continue;
    const scoreOf = (rangeOf: (sample: Sample, index: number) => Range): Score => {
      const score = { over: -Infinity, lo: -Infinity, hi: -Infinity, worst: 0 };
      list.forEach((sample, index) => {
        const r = rangeOf(sample, index);
        const lo = Math.min(r[0], sample.other[0]) - sample.e[0];
        const hi = sample.e[1] - Math.max(r[1], sample.other[1]);
        score.over = Math.max(score.over, -lo, -hi);
        score.lo = Math.max(score.lo, lo);
        score.hi = Math.max(score.hi, hi);
      });
      score.worst = Math.max(score.over, score.lo, score.hi);
      return score;
    };
    type Candidate = { kind: "const" | "mid" | "card"; sweep?: "u" | "v"; c?: number; w?: number; score: Score; range: (sample: Sample, index: number) => Range };
    const flat: Candidate = { kind: "const", score: scoreOf((sample) => sample.flat), range: (sample) => sample.flat };
    const mid: Candidate = { kind: "mid", score: scoreOf((sample) => sample.mid), range: (sample) => sample.mid };
    const sheets: any[] = stripped.representations[key].geometry[view].filter((shape: any) => shape.bone === bone && convertible(shape));
    const sheetRings: { x: number; y: number }[][] = sheets.flatMap((shape) => flattenGeometry(shape).rings);
    const pointsOf = sheetRings.flat();
    const local: { x: number; y: number }[] = stripped.representations[key].geometry.elevation.filter((shape: any) => shape.bone === bone && shape.role !== "ground" && shape.solid === undefined && shape.kind !== "member").flatMap((shape: any) => flattenGeometry(shape).rings.flat());
    const hidden = world === 1 ? 0 : 1;
    const hiddenAt = local.map((p) => f.o[hidden]! + p.x * f.u[hidden]! + p.y * f.v[hidden]!);
    const room: Range = [Math.min(...hiddenAt) - tolerance / 2, Math.max(...hiddenAt) + tolerance / 2];
    const cards: Candidate[] = [];
    if (Math.abs(f.w[2]!) > 0.999 && local.length > 0)
      for (const sweep of ["u", "v"] as const) {
        const [s, t] = sweep === "u" ? [f.u, f.v] : [f.v, f.u];
        if (Math.abs(t[world]!) < 0.2) continue;
        const a0 = pointsOf.map((p) => (p[own] - f.o[world]!) / t[world]!);
        const b = pointsOf.map((p) => p[lateral] - f.o[2]!);
        const slope = -s[world]! / t[world]!;
        const depthAt = (a: number, c: number): number => f.o[hidden]! + (a + c * slope) * t[hidden]! + c * s[hidden]!;
        const [aLo, aHi] = [Math.min(...a0), Math.max(...a0)];
        const inside = (c: number): boolean => [aLo, aHi].every((a) => depthAt(a, c) >= room[0] && depthAt(a, c) <= room[1]);
        const pr = parents.get(bone) == null ? undefined : rest[parents.get(bone)!]?.frame;
        const dot3 = (p: number[], q: number[]): number => p[0]! * q[0]! + p[1]! * q[1]! + p[2]! * q[2]!;
        const span = 2 * (room[1] - room[0]) + 4;
        for (const w of pr === undefined || list.some((sample) => sample.up === undefined) ? [1] : [1, 0.75, 0.5, 0.25, 0]) {
          const pre = list.map((sample) => {
            const [sn, tn] = sweep === "u" ? [sample.now.u, sample.now.v] : [sample.now.v, sample.now.u];
            const up = sample.up;
            const carried = (x: number[]): number => dot3(x, pr!.u) * up!.u[world]! + dot3(x, pr!.v) * up!.v[world]! + dot3(x, pr!.w) * up!.w[world]!;
            let lo = Infinity, hi = -Infinity;
            for (let i = 0; i < a0.length; i++) {
              const own = sample.now.o[world]! + a0[i]! * tn[world]! + b[i]! * sample.now.w[world]!;
              const value = w === 1 ? own : w * own + (1 - w) * (up!.o[world]! + carried([0, 1, 2].map((n) => f.o[n]! + a0[i]! * t[n]! + b[i]! * f.w[n]! - pr!.o[n]!)));
              lo = Math.min(lo, value); hi = Math.max(hi, value);
            }
            return { lo, hi, k: w * (sn[world]! + slope * tn[world]!) + (w === 1 ? 0 : (1 - w) * carried([0, 1, 2].map((n) => s[n]! + slope * t[n]!))) };
          });
          let best: Candidate | undefined;
          for (let i = 0; i <= 2000; i++) {
            const c = round(-span + (2 * span * i) / 2000);
            if (!inside(c)) continue;
            const range = (_: Sample, index: number): Range => [pre[index]!.lo + pre[index]!.k * c, pre[index]!.hi + pre[index]!.k * c];
            const score = scoreOf(range);
            if (best === undefined || score.worst < best.score.worst) best = { kind: "card", sweep, c, w, score, range };
          }
          if (best !== undefined) cards.push(best);
        }
      }
    const all = [flat, ...cards];
    const least = all.slice().sort((p, q) => p.score.worst + 0.02 * tolerance * (1 - (p.w ?? 1)) - q.score.worst - 0.02 * tolerance * (1 - (q.w ?? 1)))[0]!;
    const best = flat.score.worst <= least.score.worst + 0.05 * tolerance ? flat : least;
    const fair = all.filter((candidate) => candidate.score.over <= LIMIT * tolerance).sort((p, q) => Math.max(p.score.lo, p.score.hi) - Math.max(q.score.lo, q.score.hi));
    const base = flat.score.worst <= KEEP * tolerance ? flat : best.score.worst <= LIMIT * tolerance ? best : fair.length === 0 ? all.slice().sort((p, q) => p.score.over - q.score.over)[0]! : fair.includes(flat) && Math.max(flat.score.lo, flat.score.hi) <= Math.max(fair[0]!.score.lo, fair[0]!.score.hi) + 0.05 * tolerance ? flat : fair[0]!;
    const listed: any[] = result.representations[key].geometry[view];
    for (let i = 0; i < listed.length; i++) {
      const shape = listed[i];
      if (shape.bone !== bone || shape.solid !== undefined) continue;
      if (base.kind === "card" && convertible(shape)) listed[i] = cardOf(shape, view, base.sweep!, base.c!, base.w === 1 ? undefined : base.w === 0 ? [{ bone: parents.get(bone)!, weight: 1 }] : [{ bone, weight: base.w! }, { bone: parents.get(bone)!, weight: round(1 - base.w!) }]);
    }
    const lobes: any[] = [];
    const notes: string[] = [];
    const extremes: { lo: number; hi: number }[] = list.map(() => ({ lo: Infinity, hi: -Infinity }));
    const union = (sample: Sample, index: number): Range => { const r = base.range(sample, index); return [Math.min(r[0], extremes[index]!.lo), Math.max(r[1], extremes[index]!.hi)]; };
    if (base.score.worst > LIMIT * tolerance && Math.abs(f.w[2]!) > 0.999 && sheets.length > 0) {
      const sweep: "u" | "v" = Math.abs(f.u[world]!) >= Math.abs(f.v[world]!) ? "v" : "u";
      const [s, t] = sweep === "u" ? [f.u, f.v] : [f.v, f.u];
      const g = Math.sign(t[world]!);
      const along = pointsOf.map((p) => p[own]);
      const across = pointsOf.map((p) => p[lateral]);
      const [height, width, middle] = [Math.max(...along) - Math.min(...along), Math.max(...across) - Math.min(...across), (Math.max(...across) + Math.min(...across)) / 2];
      const paper = sheetRings.map((ring) => ring.map((p) => ({ x: p[lateral], y: p[own] })));
      const largest = sheets[0]!;
      const zLow = Math.min(...listed.filter((shape) => shape.bone === bone).map((shape) => shape.z));
      const look = Object.fromEntries(["role", "tone", "region", "regions", "material"].filter((name) => largest[name] !== undefined).map((name) => [name, largest[name]]));
      for (const end of ["lo", "hi"] as const)
        for (let n = 1; n <= 3; n++) {
          let worst = { m: -Infinity, index: -1 };
          list.forEach((sample, index) => {
            const r = union(sample, index);
            const m = end === "lo" ? Math.min(r[0], sample.other[0]) - sample.e[0] : sample.e[1] - Math.max(r[1], sample.other[1]);
            if (m > worst.m) worst = { m, index };
          });
          if (worst.m <= LIMIT * tolerance) break;
          const now = list[worst.index]!.now;
          const value = (p: { x: number; y: number }): number => now.o[world]! + p.x * now.u[world]! + p.y * now.v[world]!;
          const support = local.reduce((best, p) => ((end === "hi" ? value(p) > value(best) : value(p) < value(best)) ? p : best), local[0]!);
          const [aStar, cStar] = sweep === "u" ? [support.y, support.x] : [support.x, support.y];
          const outward = end === "hi" ? g : -g;
          let placed: any;
          for (const inset of [INSET, INSET + 0.1, INSET + 0.2, INSET + 0.3]) {
            if (inset > LIMIT - 0.05) break;
            const tip = aStar - (outward * inset * tolerance) / Math.abs(t[world]!);
            const tipAt = f.o[world]! + tip * t[world]! + cStar * s[world]!;
            for (let rv = Math.min(Math.max(worst.m - inset * tolerance, 0.08 * height), 0.28 * height); rv >= 0.05 * height && placed === undefined; rv *= 0.88) {
              const rl = Math.min(1.3 * rv, 0.45 * width);
              const centreAt = end === "hi" ? tipAt - rv : tipAt + rv;
              const fits = (z: number): boolean => Array.from({ length: 40 }, (_, i) => (2 * Math.PI * i) / 40).every((angle) => { const [x, y] = [z + rl * Math.cos(angle), centreAt + rv * Math.sin(angle)]; return inRing(x, y, paper) && edgeDistance(x, y, paper) >= MARGIN * tolerance; });
              const places = Array.from({ length: 81 }, (_, i) => middle + ((i - 40) / 40) * (width / 2)).filter(fits).sort((p, q) => Math.abs(p - middle) - Math.abs(q - middle));
              if (places.length === 0) continue;
              const ra = rv / Math.abs(t[world]!);
              placed = { id: `${LOBE}${bone}-${end}${n}`, bone, ...look, z: round(zLow - 1e-4 * (lobes.length + 1)), filled: true, kind: "ellipse", cx: round(tip - outward * ra), cy: round(places[0]! - f.o[2]!), rx: round(ra), ry: round(rl), solid: { kind: "extruded", sweep, z0: round(cStar - THICK), z1: round(cStar + THICK) } };
              list.forEach((sample, index) => {
                const [sn, tn] = sweep === "u" ? [sample.now.u, sample.now.v] : [sample.now.v, sample.now.u];
                const at = (a: number): number => sample.now.o[world]! + a * tn[world]! + cStar * sn[world]!;
                const ends = [at(placed.cx - placed.rx), at(placed.cx + placed.rx)];
                extremes[index]!.lo = Math.min(extremes[index]!.lo, ...ends);
                extremes[index]!.hi = Math.max(extremes[index]!.hi, ...ends);
              });
            }
            if (placed !== undefined) break;
          }
          if (placed === undefined) { notes.push(`NO ROOM for a lobe at the ${end} end (${mm(worst.m)} mm short @ ${list[worst.index]!.subject})`); break; }
          lobes.push(placed);
        }
      if (lobes.length > 0) listed.splice(listed.findIndex((shape) => shape.bone === bone), 0, ...lobes);
    }
    const final = scoreOf(union);
    const name = (candidate: Candidate): string => `${candidate.kind}${candidate.kind === "card" ? ` ${candidate.sweep} at ${candidate.c!.toFixed(3)}${candidate.w === 1 ? "" : ` w ${candidate.w}`}` : ""} ${mm(candidate.score.worst)} (over ${mm(candidate.score.over)}, short ${mm(candidate.score.lo)}/${mm(candidate.score.hi)})`;
    lines.push(`${key} ${view} ${bone} (rest u ${((Math.atan2(f.u[1]!, f.u[0]!) * 180) / Math.PI).toFixed(1)} deg): ${[flat, ...[...cards].sort((p, q) => p.score.worst - q.score.worst).slice(0, 2), mid].map(name).join("; ")} -> ${base === flat ? "" : `${name(base)} `} ${base.kind}${lobes.length > 0 ? ` + ${lobes.length} lobes` : ""}: ${mm(final.worst)} mm (over ${mm(final.over)}, short ${mm(final.lo)}/${mm(final.hi)})${notes.map((note) => ` ${note}`).join("")}${final.worst > tolerance ? "  BEYOND" : final.worst > LIMIT * tolerance ? "  near" : ""}`);
  }
  for (const line of lines) console.log(line);
  const save = flag("save");
  if (save !== undefined) {
    const ops = opsOf(result, stripped);
    writeFileSync(save, `${JSON.stringify(ops)}\n`);
    console.log(`${asset.id}: ops saved to ${save} (${Object.values(ops).flatMap((views: any) => Object.values(views).flat()).length} shapes)`);
  }
}
if (before !== undefined) {
  const after = drifts(structuredClone(result));
  let worst = { k: "", m: -1, subject: "" };
  for (const [k, { m, subject }] of after) {
    const was = before.get(k)?.m ?? 0;
    if (m > worst.m) worst = { k, m, subject };
    if (m > tolerance || Math.abs(m - was) > 0.05 * tolerance) console.log(`verified ${k.replaceAll("|", " ")}: ${mm(was)} -> ${mm(m)} mm @ ${subject}${m > tolerance ? "  BEYOND" : ""}`);
  }
  console.log(`${asset.id}: worst bone drift after ${mm(worst.m)} mm (${worst.k.replaceAll("|", " ")} @ ${worst.subject})`);
}
let plates = 0, sheets = 0, cards = 0, lobed = 0;
for (const key of Object.keys(result.representations)) for (const view of VIEWS) for (const shape of result.representations[key].geometry?.[view] ?? []) { if (String(shape.id).startsWith(PLATE)) plates++; else if (String(shape.id).startsWith(LOBE)) lobed++; else if (String(shape.id).endsWith(SHEET)) sheets++; else if (isCard(shape)) cards++; }
console.log(`${asset.id}: tolerance ${mm(tolerance)} mm; ${plates} depth plates, ${sheets} sheets carried by the silhouette, ${cards} cards, ${lobed} lobes`);
next.asset = result;
const out = flag("out");
if (out !== undefined || args.includes("--write")) {
  if (readFileSync(file, "utf8") !== text) throw new Error(`${file} changed on disk since it was read: nothing written, run again`);
  const written = rewrite(text, previous, next);
  if (written === text && out === undefined) console.log(`${asset.id}: unchanged`);
  else { writeFileSync(out ?? file, written); console.log(`${asset.id}: written to ${out ?? file} (${text.length} -> ${written.length} characters)`); }
}
