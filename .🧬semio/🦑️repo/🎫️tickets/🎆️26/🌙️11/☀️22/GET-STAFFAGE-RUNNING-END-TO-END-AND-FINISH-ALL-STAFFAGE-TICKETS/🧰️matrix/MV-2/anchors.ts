/** ⚓️ Finds, for every landmark anchor of a plant that leaves the drawn shapes of its bone, the bone that draws its place: `bun anchors.ts <id|file.json> [--ops ops.json] [--margin 0.6] [--reach 4] [--near 8] [--keep 10] [--only a,b] [--bones regex] [--stay]`.
 *
 * Pass 1 runs the landmark rule of the matrix (`anchorLandmarks`) over every subject (poses, clip instants, transitions) of every authored scale and lists the anchors with a finding. For each of them the candidates are its place at rest (`o + x·u + y·v + z·w` of the rest frame of its bone) expressed in the rest frame of every bone that draws a shape, and, when none of those holds, places on a grid of half a tolerance (then of a whole one) up to `reach` steps around it on the `near` bones that draw nearest to it; an emitter with a thermal region is only bound to a bone that draws that region. A candidate is measured against the OWN shapes of its bone (stricter than the rule, which also accepts descendant bones: a trunk would else hold every anchor of its crown), in every view that draws the bone, first in the default pose (the `keep` best go on), then over every subject. Of the candidates whose worst distance stays within `margin` × tolerance the choice is: the smallest move, then the bone drawn in most scales and views, then the smallest worst distance. `--stay` keeps the bone and only moves. The result is written as `anchor` ops (`bone`, `x`, `y`, `z`) into the ops file, which `apply.ts` applies; ops of other anchors and other kinds in the file stay.
 * @see ./apply.ts
 * @see ../MR/gate.ts */
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { S, readDoc } from "../MR/lib.ts";

const T = join(S, "🧪️tests", "🔭️multiview-coverage");
const engine = await import(join(T, "🧰️.ts"));
const { anchorLandmarks, distanceToShapes } = await import(join(T, "⚓️landmarks.ts"));
const { evaluate } = await import(join(S, "🔨️modules", "🧮️evaluation", "🟦️.ts"));
const { restFrames } = await import(join(S, "🔨️modules", "🦴️rig", "🟦️.ts"));

type V = readonly number[];
type Frame = { o: V; u: V; v: V; w: V };
type Candidate = { anchor: string; bone: string; x: number; y: number; z: number; move: number; region: boolean; coverage: number; worst: number; where: string; rest: number };
const args = process.argv.slice(2);
const flag = (name: string): string | undefined => (args.indexOf(`--${name}`) < 0 ? undefined : args[args.indexOf(`--${name}`) + 1]);
const asset = readDoc(args[0]!).doc.asset;
const MARGIN = Number(flag("margin") ?? 0.6);
const REACH = Number(flag("reach") ?? 4);
const NEAR = Number(flag("near") ?? 8);
const KEEP = Number(flag("keep") ?? 10);
const only = flag("only")?.split(",");
const bonesLike = flag("bones") === undefined ? undefined : new RegExp(flag("bones")!);
const stay = args.includes("--stay");
const tolerance: number = engine.toleranceM(asset);
const authored = Object.keys(asset.representations).filter((key) => engine.authoredOf(asset, key) !== undefined);
const rest = restFrames(asset.rig) as Record<string, { frame: Frame }>;
const VIEWS = ["elevation", "front", "plan"] as const;
const round = (n: number): number => Math.round(n * 1e4) / 1e4;
const at = (f: Frame, x: number, y: number, z: number): number[] => [0, 1, 2].map((i) => f.o[i]! + x * f.u[i]! + y * f.v[i]! + z * f.w[i]!);
const into = (f: Frame, p: V): [number, number, number] => {
  const d = [0, 1, 2].map((i) => p[i]! - f.o[i]!);
  const dot = (a: V): number => d[0]! * a[0]! + d[1]! * a[1]! + d[2]! * a[2]!;
  return [dot(f.u), dot(f.v), dot(f.w)];
};
const seen = (p: V, view: (typeof VIEWS)[number]): { x: number; y: number } => (view === "elevation" ? { x: p[0]!, y: p[1]! } : view === "front" ? { x: p[2]!, y: p[1]! } : { x: p[0]!, y: p[2]! });
const byBone = (shapes: any[]): Map<string, any[]> => {
  const out = new Map<string, any[]>();
  for (const shape of shapes) if (shape.shape.role !== "ground") (out.get(shape.shape.bone) ?? out.set(shape.shape.bone, []).get(shape.shape.bone)!).push(shape);
  return out;
};
const regionOf = new Map<string, string>();
for (const emitter of asset.thermal?.emitters ?? []) for (const id of [emitter.anchor, ...(emitter.anchors ?? [])]) if (emitter.region !== undefined) regionOf.set(id, emitter.region);
const drawn = new Map<string, { coverage: number; regions: Set<string> }>();
for (const key of authored)
  for (const view of VIEWS) {
    const bones = new Set<string>();
    for (const shape of asset.representations[key].geometry[view]) {
      if (shape.role === "ground") continue;
      const entry = drawn.get(shape.bone) ?? drawn.set(shape.bone, { coverage: 0, regions: new Set() }).get(shape.bone)!;
      if (!bones.has(shape.bone)) entry.coverage++;
      bones.add(shape.bone);
      for (const region of [shape.region, ...(shape.regions ?? [])]) if (region !== undefined) entry.regions.add(region);
    }
  }

/** 🔁️ Every subject of every authored scale, evaluated once per call. */
function* states(posesOnly = false): Generator<{ key: string; subject: any; state: any }> {
  for (const key of authored)
    for (const subject of engine.subjects(asset, Number(key))) {
      if (posesOnly && !subject.isDefault) continue;
      yield { key, subject, state: evaluate(asset, subject.query, engine.scaleSelector(Number(key))) };
    }
}

/** 📏️ The worst distance of each candidate to the own shapes of its bone over the states. */
function measure(candidates: Candidate[], posesOnly: boolean): void {
  for (const candidate of candidates) Object.assign(candidate, { worst: 0, where: "" });
  for (const { key, subject, state } of states(posesOnly)) {
    const views = { elevation: byBone(state.shapes.elevation), front: byBone(state.shapes.front), plan: byBone(state.shapes.plan) };
    for (const candidate of candidates) {
      if (candidate.worst > MARGIN * tolerance && !posesOnly) continue;
      const now = state.bones[candidate.bone].frame;
      const p = at({ o: [now.o.x, now.o.y, now.o.z], u: [now.u.x, now.u.y, now.u.z], v: [now.v.x, now.v.y, now.v.z], w: [now.w.x, now.w.y, now.w.z] }, candidate.x, candidate.y, candidate.z);
      if (!p.every(Number.isFinite)) throw new Error(`no place for ${candidate.anchor} on ${candidate.bone}`);
      for (const view of VIEWS) {
        const shapes = views[view].get(candidate.bone);
        if (shapes === undefined) continue;
        const d = distanceToShapes(seen(p, view), shapes);
        if (Number.isFinite(d) && d > candidate.worst) Object.assign(candidate, { worst: d, where: `1:${key} ${view} ${subject.id}` });
      }
    }
  }
}

const failing = new Map<string, { n: number; mm: number; where: string }>();
for (const { key, subject, state } of states())
  for (const finding of anchorLandmarks(state, tolerance)) {
    const entry = failing.get(finding.anchor) ?? failing.set(finding.anchor, { n: 0, mm: 0, where: "" }).get(finding.anchor)!;
    entry.n++;
    if (finding.deltaMm > entry.mm) Object.assign(entry, { mm: finding.deltaMm, where: `1:${key} ${finding.view} ${subject.id}` });
  }
console.log(`${asset.id}: tolerance ${(tolerance * 1000).toFixed(0)} mm, ${failing.size} anchors with a finding`);
const ops: any[] = [];
for (const [id, found] of failing) {
  if (only !== undefined && !only.includes(id)) continue;
  const anchor = asset.anchors.find((candidate: any) => candidate.id === id);
  const place = at(rest[anchor.bone]!.frame, anchor.x, anchor.y, anchor.z ?? 0);
  const region = regionOf.get(id);
  const regional = region !== undefined && [...drawn.values()].some((entry) => entry.regions.has(region));
  const bones = [...drawn.keys()].filter((bone) => (stay ? bone === anchor.bone : (bonesLike === undefined || bonesLike.test(bone) || bone === anchor.bone) && (!regional || drawn.get(bone)!.regions.has(region!))));
  const make = (bone: string, p: V): Candidate => {
    const [x, y, z] = into(rest[bone]!.frame, p);
    return { anchor: id, bone, x: round(x), y: round(y), z: round(z), move: Math.hypot(p[0]! - place[0]!, p[1]! - place[1]!, p[2]! - place[2]!), region: region !== undefined && drawn.get(bone)!.regions.has(region), coverage: drawn.get(bone)!.coverage, worst: 0, where: "", rest: 0 };
  };
  const first = bones.map((bone) => make(bone, place));
  measure(first, true);
  for (const candidate of first) candidate.rest = candidate.worst;
  let pool = first.filter((candidate) => candidate.worst <= MARGIN * tolerance);
  const order = (a: Candidate, b: Candidate): number => Number(b.region) - Number(a.region) || a.move - b.move || b.coverage - a.coverage || a.worst - b.worst;
  let chosen: Candidate | undefined;
  for (const round2 of [0, 1, 2]) {
    if (round2 > 0) {
      const near = first.slice().sort((a, b) => Number(b.region) - Number(a.region) || a.rest - b.rest).slice(0, NEAR).map((candidate) => candidate.bone);
      const step = (tolerance / 2) * round2;
      const grid: Candidate[] = [];
      for (const bone of near)
        for (let i = -REACH; i <= REACH; i++) for (let j = -REACH; j <= REACH; j++) for (let k = -REACH; k <= REACH; k++) if (i !== 0 || j !== 0 || k !== 0) grid.push(make(bone, [place[0]! + i * step, place[1]! + j * step, place[2]! + k * step]));
      measure(grid, true);
      pool = grid.filter((candidate) => candidate.worst <= 0.25 * MARGIN * tolerance);
    }
    const shortlist = pool.sort(order).slice(0, round2 === 0 ? KEEP : 4 * KEEP);
    measure(shortlist, false);
    chosen = shortlist.filter((candidate) => candidate.worst <= MARGIN * tolerance).sort(order)[0];
    if (chosen !== undefined) break;
  }
  if (chosen === undefined) {
    console.log(`  ${id} on ${anchor.bone}: ${found.n} findings, worst ${found.mm.toFixed(0)} mm @ ${found.where}: NO candidate holds`);
    continue;
  }
  console.log(`  ${id} on ${anchor.bone}: ${found.n} findings, worst ${found.mm.toFixed(0)} mm @ ${found.where} -> ${chosen.bone} (${chosen.x}, ${chosen.y}, ${chosen.z}) moved ${(chosen.move * 1000).toFixed(0)} mm, region ${chosen.region ? "yes" : "no"}, drawn in ${chosen.coverage} of ${authored.length * 3} views, worst ${(chosen.worst * 1000).toFixed(0)} mm @ ${chosen.where || "-"}`);
  ops.push({ op: "anchor", id, set: { bone: chosen.bone, x: chosen.x, y: chosen.y, z: chosen.z } });
}
const file = flag("ops");
if (file !== undefined) {
  const known: any[] = existsSync(file) ? JSON.parse(readFileSync(file, "utf8")) : [];
  const next = [...known.filter((op) => op.op !== "anchor" || !ops.some((mine) => mine.id === op.id)), ...ops];
  writeFileSync(file, `[\n${next.map((op) => `  ${JSON.stringify(op)}`).join(",\n")}\n]\n`);
  console.log(`${asset.id}: ${ops.length} anchor ops written, ${next.length} ops in ${file}`);
}
