/** 🏃️ Re-solves the stance legs of the jogger's `run-cycle` so that the centre of mass follows a physical run: constant forward speed, a vertical ground force `W·(P·sin(πt/Ts) + Q·sin(2πt/Ts))` in stance (the impulse of one step carries the weight, `Q` lifts the toe-off above the landing), a ballistic flight between the authored toe-off and landing heights and a heel impact of {@link IMPACT_MS} at landing.
 *
 * The landing pose is kept. Solved per side (the right poses mirror the left ones): thigh, shank and foot of the stance leg in loading, midstance, propulsion and toe-off, thigh and shank of both legs in the flight pose (they set the velocities at toe-off and landing through the spline tangents), and the pose roots the flight is splined through. The sole stays flat while the heel span lasts and keeps its authored heel lift afterwards.
 *
 * `bun <this file> <asset json> [--write]`: prints the solved values and the remaining errors; with `--write` the numbers are replaced in the file as text (angles four decimals, roots six, `advanceM` eight, heel spans end at the loading key) and the result is compared with the intended values before it is written.
 *
 * NOT APPLIED to the catalog: on a scratch copy (2026-11-23) the solution left the centre of mass up to 8.8 mm off the vertical target, drove `foot-l` of midstance to 38.04 degrees (limit 38) and still showed ground forces between -4 and +5 body weights, because twelve keys 60 ms apart cannot hold the landing and toe-off tangents. It is the starting point for a re-authoring with keys every 1/30 s.
 *
 * @see ../fix-originals.md — why the clip fails and what was measured */
import { readFileSync, writeFileSync } from "node:fs";
import { sampleOf } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { apply, solveChain } from "../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const G = 9.80665;
const STANCE_S = 0.24;
const FLIGHT_S = 0.12;
const STEP_S = STANCE_S + FLIGHT_S;
const IMPACT_MS = 0.15;
const H = 1 / 240;
const CLIP = "run-cycle";

type Pose = { id: string; bones: Record<string, { angleDeg?: number }>; root?: { y: number } };
type Asset = { rig: { bones: { id: string; mass?: { kg: number; comX: number; comY: number } }[] }; anchors: { id: string; bone: string; x: number; y: number }[]; poses: Pose[]; clips: { id: string; advanceM?: number; contacts: { anchor: string; fromS: number; toS: number }[] }[] };
type Parameter = { readonly pose: string; readonly bone?: string };

const file = process.argv[2]!;
const text = readFileSync(file, "utf8");
const source = JSON.parse(text).asset as Asset;
const other = (name: string): string => (name.endsWith("-l") ? `${name.slice(0, -2)}-r` : `${name.slice(0, -2)}-l`);
const parameters: Parameter[] = [
  ...["loading-l", "midstance-l", "propulsion-l", "toe-off-l"].flatMap((pose) => ["thigh-l", "shank-l", "foot-l"].map((bone) => ({ pose, bone }))),
  ...["thigh-l", "shank-l", "thigh-r", "shank-r"].map((bone) => ({ pose: "flight-l", bone })),
  ...["loading-l", "propulsion-l", "flight-l"].map((pose) => ({ pose })),
];
const HEEL_END: Record<string, number> = { "heel-l": 0.06, "heel-r": STEP_S + 0.06 };
const poseOf = (asset: Asset, id: string): Pose => asset.poses.find((pose) => pose.id === id)!;
const read = (asset: Asset, parameter: Parameter): number => (parameter.bone === undefined ? poseOf(asset, parameter.pose).root!.y : poseOf(asset, parameter.pose).bones[parameter.bone]!.angleDeg!);
const start = parameters.map((parameter) => read(source, parameter));

function withValues(values: readonly number[], roots: Readonly<Record<string, number>> = {}): Asset {
  const poses = structuredClone(source.poses);
  const set = (pose: string, bone: string | undefined, value: number): void => {
    const target = poses.find((candidate) => candidate.id === pose)!;
    if (bone === undefined) target.root = { ...target.root, y: value };
    else target.bones[bone] = { ...target.bones[bone], angleDeg: value };
  };
  parameters.forEach((parameter, index) => {
    set(parameter.pose, parameter.bone, values[index]!);
    set(other(parameter.pose), parameter.bone === undefined ? undefined : other(parameter.bone), values[index]!);
  });
  for (const [pose, y] of Object.entries(roots)) {
    set(pose, undefined, y);
    set(other(pose), undefined, y);
  }
  return { ...source, poses, clips: source.clips.map((clip) => (clip.id === CLIP ? { ...clip, contacts: clip.contacts.map((span) => (HEEL_END[span.anchor] === undefined ? span : { ...span, toS: HEEL_END[span.anchor]! })) } : clip)) };
}

function framesAt(asset: Asset, query: { kind: "clip"; clip: string; timeS: number } | { kind: "pose"; pose: string }) {
  const sample = sampleOf(asset as never, query as never);
  return { sample, frames: solveChain(asset.rig as never, sample.offsets, sample.root) };
}

function centre(asset: Asset, query: Parameters<typeof framesAt>[1]): { x: number; y: number } {
  const { frames } = framesAt(asset, query);
  let [m, x, y] = [0, 0, 0];
  for (const bone of asset.rig.bones) {
    if (bone.mass === undefined || !(bone.mass.kg > 0)) continue;
    const f = frames[bone.id]!.frame;
    m += bone.mass.kg;
    x += bone.mass.kg * (f.o[0] + bone.mass.comX * f.u[0] + bone.mass.comY * f.v[0]);
    y += bone.mass.kg * (f.o[1] + bone.mass.comX * f.u[1] + bone.mass.comY * f.v[1]);
  }
  return { x: x / m, y: y / m };
}

function heelLift(asset: Asset, pose: string): number {
  const { frames } = framesAt(asset, { kind: "pose", pose });
  const height = (id: string): number => {
    const anchor = asset.anchors.find((candidate) => candidate.id === id)!;
    return apply(frames[anchor.bone]!.affine, anchor.x, anchor.y).y;
  };
  return height("heel-l") - height("contact-l");
}

function build(values: readonly number[]): Asset {
  const draft = withValues(values);
  const roots: Record<string, number> = {};
  for (const pose of ["initial-contact-l", "toe-off-l"]) roots[pose] = sampleOf(draft as never, { kind: "pose", pose } as never).root.y;
  return withValues(values, roots);
}

const authored = build(start);
const takeoffY = centre(authored, { kind: "pose", pose: "toe-off-l" }).y;
const lifts: Record<string, number> = { "loading-l": 0, "midstance-l": 0, "propulsion-l": heelLift(authored, "propulsion-l"), "toe-off-l": heelLift(authored, "toe-off-l") };
const speed = source.clips.find((clip) => clip.id === CLIP)!.advanceM! / 2 / STEP_S;
const omega = Math.PI / STANCE_S;

function profile(landingY: number) {
  const takeoffV = (landingY - takeoffY) / FLIGHT_S + (G * FLIGHT_S) / 2;
  const stanceV = takeoffV - G * FLIGHT_S + IMPACT_MS;
  const p = ((takeoffV - stanceV + G * STANCE_S) * omega) / (2 * G);
  const q = (((takeoffY - landingY) / STANCE_S - stanceV + (G * STANCE_S) / 2 - (G * p) / omega) * 2 * omega) / G;
  const stance = (t: number): number => landingY + stanceV * t - (G * t * t) / 2 + ((G * p) / omega) * (t - Math.sin(omega * t) / omega) + ((G * q) / (2 * omega)) * (t - Math.sin(2 * omega * t) / (2 * omega));
  const flight = (t: number): number => takeoffY + takeoffV * (t - STANCE_S) - (G * (t - STANCE_S) ** 2) / 2;
  const force = (t: number): number => p * Math.sin(omega * t) + q * Math.sin(2 * omega * t);
  return { p, q, takeoffV, stanceV, y: (t: number): number => (t < STANCE_S ? stance(t) : flight(t)), force };
}

function residuals(values: readonly number[]): number[] {
  const asset = build(values);
  const at = (timeS: number) => centre(asset, { kind: "clip", clip: CLIP, timeS });
  const landing = at(0);
  const target = profile(landing.y);
  const out: number[] = [];
  for (let i = 0; i < Math.round(STEP_S * 120); i++) {
    const t = i / 120;
    const c = at(t);
    out.push((t < STANCE_S ? 1 : 0.5) * (c.x - landing.x - speed * t), c.y - target.y(t));
  }
  out.push(at(STEP_S).x - landing.x - speed * STEP_S);
  const second = (t: number) => {
    const [a, b, c] = [at(t - H), at(t), at(t + H)];
    return { x: (c.x - 2 * b.x + a.x) / H, y: (c.y - 2 * b.y + a.y) / H };
  };
  const kink = second(STEP_S);
  out.push(0.2 * kink.x, 0.2 * (kink.y - IMPACT_MS + G * H));
  for (const t of [0.09, 0.18]) {
    const a = second(t);
    out.push((0.12 * a.x) / G, (0.12 * (a.y / H - G * (target.force(t) - 1))) / G / 240);
  }
  for (const [pose, lift] of Object.entries(lifts)) out.push(10 * (heelLift(asset, pose) - lift));
  parameters.forEach((parameter, index) => {
    if (parameter.bone !== undefined) out.push(1e-5 * (values[index]! - start[index]!));
  });
  return out;
}

function solve(a: number[][], b: number[]): number[] {
  const n = b.length;
  const m = a.map((row, i) => [...row, b[i]!]);
  for (let col = 0; col < n; col++) {
    let pivot = col;
    for (let row = col + 1; row < n; row++) if (Math.abs(m[row]![col]!) > Math.abs(m[pivot]![col]!)) pivot = row;
    [m[col], m[pivot]] = [m[pivot]!, m[col]!];
    for (let row = 0; row < n; row++) {
      if (row === col) continue;
      const factor = m[row]![col]! / m[col]![col]!;
      for (let k = col; k <= n; k++) m[row]![k]! -= factor * m[col]![k]!;
    }
  }
  return m.map((row, i) => row[n]! / row[i]!);
}

const cost = (r: readonly number[]): number => r.reduce((sum, value) => sum + value * value, 0);
let values = [...start];
let current = residuals(values);
let damping = 1e-6;
console.log("[DEBUG] start cost", cost(current).toExponential(4));
for (let iteration = 0; iteration < 40; iteration++) {
  const jacobian = parameters.map((parameter, index) => {
    const step = parameter.bone === undefined ? 1e-5 : 1e-3;
    const moved = residuals(values.map((value, k) => (k === index ? value + step : value)));
    return moved.map((value, k) => (value - current[k]!) / step);
  });
  let improved = false;
  for (let attempt = 0; attempt < 8; attempt++) {
    const normal = parameters.map((_, i) => parameters.map((__, j) => jacobian[i]!.reduce((sum, value, k) => sum + value * jacobian[j]![k]!, 0) * (i === j ? 1 + damping : 1)));
    const gradient = parameters.map((_, i) => -jacobian[i]!.reduce((sum, value, k) => sum + value * current[k]!, 0));
    const delta = solve(normal, gradient);
    const candidate = values.map((value, k) => value + delta[k]!);
    const next = residuals(candidate);
    if (cost(next) < cost(current)) {
      improved = cost(current) - cost(next) > 1e-14;
      values = candidate;
      current = next;
      damping = Math.max(damping / 4, 1e-9);
      break;
    }
    damping *= 10;
  }
  console.log("[DEBUG] iteration", iteration, "cost", cost(current).toExponential(4));
  if (!improved) break;
}

const rounded = values.map((value, index) => (parameters[index]!.bone === undefined ? Math.round(value * 1e6) / 1e6 : Math.round(value * 1e4) / 1e4));
const draft = withValues(rounded);
const roots: Record<string, number> = {};
for (const pose of ["initial-contact-l", "toe-off-l"]) roots[pose] = Math.round(sampleOf(draft as never, { kind: "pose", pose } as never).root.y * 1e6) / 1e6;
const solved = withValues(rounded, roots);
const clip = solved.clips.find((candidate) => candidate.id === CLIP)!;
const rootX = (timeS: number): number => sampleOf(solved as never, { kind: "clip", clip: CLIP, timeS } as never).root.x;
clip.advanceM = Math.round((rootX(2 * STEP_S - 1e-9) - rootX(0)) * 1e8) / 1e8;
const final = profile(centre(solved, { kind: "clip", clip: CLIP, timeS: 0 }).y);
console.log("force model P", final.p.toFixed(4), "Q", final.q.toFixed(4), "take-off", final.takeoffV.toFixed(4), "m/s, stance start", final.stanceV.toFixed(4), "m/s, speed", speed.toFixed(6), "m/s, advanceM", clip.advanceM);
parameters.forEach((parameter, index) => console.log(`${parameter.pose} ${parameter.bone ?? "root.y"}: ${start[index]} -> ${rounded[index]}`));
for (const [pose, y] of Object.entries(roots)) console.log(`${pose} root.y: ${poseOf(source, pose).root!.y} -> ${y}`);
let [worstX, worstY] = [0, 0];
const zero = centre(solved, { kind: "clip", clip: CLIP, timeS: 0 });
for (let i = 0; i <= Math.round(STEP_S * 240); i++) {
  const t = i / 240;
  const c = centre(solved, { kind: "clip", clip: CLIP, timeS: t });
  worstX = Math.max(worstX, Math.abs(c.x - zero.x - speed * t));
  if (t < STEP_S) worstY = Math.max(worstY, Math.abs(c.y - final.y(t)));
}
console.log("largest deviation of the centre of mass from the target: forward", (worstX * 1000).toFixed(3), "mm, vertical", (worstY * 1000).toFixed(3), "mm");
for (const pose of Object.keys(lifts)) console.log(`${pose} heel lift ${(heelLift(solved, pose) * 1000).toFixed(4)} mm (target ${(lifts[pose]! * 1000).toFixed(4)} mm)`);

if (process.argv.includes("--write")) {
  const lines = text.split("\n");
  const block = (id: string): [number, number] => {
    const from = lines.findIndex((line) => line === `        "id": "${id}",`);
    if (from < 0) throw new Error(`no block "${id}"`);
    const next = lines.findIndex((line, index) => index > from && /^ {8}"id": "/.test(line));
    return [from, next < 0 ? lines.length : next];
  };
  const put = (index: number, value: number): void => {
    lines[index] = lines[index]!.replace(/: -?[\d.e+-]+(,?)$/, `: ${String(value)}$1`);
  };
  const replace = (id: string, head: RegExp, field: string, value: number): void => {
    const [from, to] = block(id);
    const at = lines.findIndex((line, index) => index >= from && index < to && head.test(line) && lines[index + 1]!.trimStart().startsWith(`"${field}": `));
    if (at < 0) throw new Error(`no ${field} under ${head} in "${id}"`);
    put(at + 1, value);
  };
  for (const pose of solved.poses) {
    const before = poseOf(source, pose.id);
    for (const [bone, offset] of Object.entries(pose.bones)) if (offset.angleDeg !== before.bones[bone]?.angleDeg) replace(pose.id, new RegExp(`^ {10}"${bone}": \\{$`), "angleDeg", offset.angleDeg!);
    if (pose.root !== undefined && pose.root.y !== before.root?.y) replace(pose.id, /^ {8}"root": \{$/, "y", pose.root.y);
  }
  const [from, to] = block(CLIP);
  put(lines.findIndex((line, index) => index >= from && index < to && line.trimStart().startsWith('"advanceM": ')), clip.advanceM!);
  for (const [anchor, end] of Object.entries(HEEL_END)) {
    const at = lines.findIndex((line, index) => index >= from && index < to && line.trim() === `"anchor": "${anchor}",` && lines[index + 2]!.trimStart().startsWith('"toS": '));
    if (at < 0) throw new Error(`no span of ${anchor}`);
    put(at + 2, end);
  }
  const next = lines.join("\n");
  const written = JSON.parse(next).asset as Asset;
  if (JSON.stringify(written.poses) !== JSON.stringify(solved.poses) || JSON.stringify(written.clips) !== JSON.stringify(solved.clips)) throw new Error("the text edit does not equal the solved values; nothing written");
  writeFileSync(file, next);
  console.log("written", file);
}
