/** 📏️ Ticket tool of work package H3: measures species documents — the contrast of every palette and state tint against both page colours, how far every clip reaches beyond the size box (and the `reach` the encounter clips need), one-shot clips that do not start or end at rest, and props hidden at rest.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/h3_measure.ts" [--clips] <species.json>...
 *
 * @see https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio — the contrast formula
 */
import { readFileSync } from "node:fs";
import type { Clip, Shape, Species } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { clipTicks, sampleClip } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🟦️.ts";
import { restPose, solveRig, type Pose } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🟦️.ts";

const PAGES = { light: "#f7f3e3", dark: "#001117" } as const;
const ENCOUNTERS = ["greet", "cuddle", "squabble"] as const;

const channel = (value: number): number => (value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4);
const luminance = (hex: string): number => {
  const [r, g, b] = [1, 3, 5].map((at) => channel(Number.parseInt(hex.slice(at, at + 2), 16) / 255)) as [number, number, number];
  return 0.2126 * r + 0.7152 * g + 0.0722 * b;
};
const contrast = (one: string, other: string): number => {
  const [high, low] = [luminance(one), luminance(other)].sort((a, b) => b - a) as [number, number];
  return (high + 0.05) / (low + 0.05);
};

/** ✏️ Points that bound a shape (control points included, so a curve lies inside their box). */
function shapePoints(shape: Shape): [number, number][] {
  if (shape.kind === "rect") return [[shape.x, shape.y], [shape.x + shape.width, shape.y + shape.height], [shape.x, shape.y + shape.height], [shape.x + shape.width, shape.y]];
  if (shape.kind === "line") return [[shape.x1, shape.y1], [shape.x2, shape.y2]];
  if (shape.kind === "ellipse") return Array.from({ length: 24 }, (_, index) => [shape.cx + shape.rx * Math.cos((index * Math.PI) / 12), shape.cy + shape.ry * Math.sin((index * Math.PI) / 12)] as [number, number]);
  const tokens = shape.d.match(/[a-zA-Z]|-?(?:\d+\.?\d*|\.\d+)(?:e-?\d+)?/g) ?? [];
  const arity: Record<string, number> = { M: 2, L: 2, H: 1, V: 1, C: 6, S: 4, Q: 4, T: 2, A: 7, Z: 0 };
  const points: [number, number][] = [];
  let command = "M";
  let x = 0;
  let y = 0;
  let sx = 0;
  let sy = 0;
  let index = 0;
  while (index < tokens.length) {
    const token = tokens[index]!;
    if (/^[a-zA-Z]$/.test(token)) {
      command = token;
      index++;
      if (command.toUpperCase() === "Z") {
        x = sx;
        y = sy;
      }
      continue;
    }
    const kind = command.toUpperCase();
    const count = arity[kind]!;
    const values = tokens.slice(index, index + count).map(Number);
    index += count;
    const relative = command !== kind;
    const ox = relative ? x : 0;
    const oy = relative ? y : 0;
    if (kind === "H") x = ox + values[0]!;
    else if (kind === "V") y = oy + values[0]!;
    else if (kind === "A") {
      const nx = ox + values[5]!;
      const ny = oy + values[6]!;
      points.push([x - values[0]!, y - values[1]!], [x + values[0]!, y + values[1]!], [nx - values[0]!, ny - values[1]!], [nx + values[0]!, ny + values[1]!]);
      x = nx;
      y = ny;
    } else {
      for (let pair = 0; pair + 1 < count; pair += 2) points.push([ox + values[pair]!, oy + values[pair + 1]!]);
      x = ox + values[count - 2]!;
      y = oy + values[count - 1]!;
    }
    if (kind === "M") {
      sx = x;
      sy = y;
      command = relative ? "l" : "L";
    }
    points.push([x, y]);
  }
  return points;
}

type Box = { left: number; right: number; top: number; bottom: number };

/** 📦️ The box of a posed species, strokes included. */
function boxOf(species: Species, pose: Pose): Box {
  const world = solveRig(species, pose);
  const box: Box = { left: Infinity, right: -Infinity, top: Infinity, bottom: -Infinity };
  for (const part of species.parts) {
    const bone = species.bones.findIndex((candidate) => candidate.id === part.bone);
    const [a, b, c, d, e, f] = world.slice(bone * 6, bone * 6 + 6) as [number, number, number, number, number, number];
    const half = part.stroke === "none" ? 0 : ((part.strokeWidth ?? 2) / 2) * Math.sqrt(Math.abs(a * d - b * c));
    const placed = shapePoints(part.shape).map(([px, py]) => [a * px + c * py + e, b * px + d * py + f] as const);
    const xs = placed.map(([x]) => x);
    const ys = placed.map(([, y]) => y);
    if (Math.max(...xs) - Math.min(...xs) < 0.2 && Math.max(...ys) - Math.min(...ys) < 0.2) continue;
    for (const [x, y] of placed) {
      box.left = Math.min(box.left, x - half);
      box.right = Math.max(box.right, x + half);
      box.top = Math.min(box.top, y - half);
      box.bottom = Math.max(box.bottom, y + half);
    }
  }
  return box;
}

/** 🧩️ Offsets and rotations add, scales multiply — the stage's layering of an activity clip over the idle loop. */
const layered = (under: Pose, over: Pose): Pose => under.map((bone, index) => ({ x: bone.x + over[index]!.x, y: bone.y + over[index]!.y, rotation: bone.rotation + over[index]!.rotation, scaleX: bone.scaleX * over[index]!.scaleX, scaleY: bone.scaleY * over[index]!.scaleY }));

/** 📐️ How far a clip reaches beyond the box at worst (over every tick, laid over the idle loop at four offsets). */
function reachOf(species: Species, clip: Clip, idle: Clip | undefined): Box {
  const worst: Box = { left: -Infinity, right: -Infinity, top: -Infinity, bottom: -Infinity };
  const half = species.size.width / 2;
  const ticks = clipTicks(clip);
  for (let tick = 0; tick <= ticks; tick++) {
    const over = sampleClip(species, clip, tick);
    for (const offset of idle === undefined || idle.id === clip.id ? [0] : [0, 0.25, 0.5, 0.75]) {
      const pose = idle === undefined || idle.id === clip.id ? over : layered(sampleClip(species, idle, Math.round(offset * clipTicks(idle)) + tick), over);
      const box = boxOf(species, pose);
      worst.left = Math.max(worst.left, -half - box.left);
      worst.right = Math.max(worst.right, box.right - half);
      worst.top = Math.max(worst.top, -species.size.height - box.top);
      worst.bottom = Math.max(worst.bottom, box.bottom);
    }
  }
  return worst;
}

const fixed = (value: number): string => (Math.round(value * 100) / 100).toFixed(2).padStart(6);
const showClips = process.argv.includes("--clips");
for (const file of process.argv.slice(2).filter((argument) => !argument.startsWith("--"))) {
  const species = JSON.parse(readFileSync(file, "utf8")) as Species;
  console.log(`\n=== ${species.id} ${species.size.width}x${species.size.height} grip ${species.grip} reach ${species.reach}`);
  for (const state of species.states) {
    const palette = { ...species.palette, ...state.tint };
    const tones = (["body", "accent", "detail"] as const).map((tone) => `${tone} ${palette[tone]} L ${fixed(contrast(palette[tone], PAGES.light))} D ${fixed(contrast(palette[tone], PAGES.dark))}`);
    console.log(`contrast ${state.id.padEnd(12)} ${tones.join(" | ")}`);
  }
  const rest = boxOf(species, restPose(species));
  console.log(`rest box: left ${fixed(-species.size.width / 2 - rest.left)} right ${fixed(rest.right - species.size.width / 2)} top ${fixed(-species.size.height - rest.top)} bottom ${fixed(rest.bottom)} (beyond the size box)`);
  const world = solveRig(species, restPose(species));
  const hidden = species.parts.filter((part) => {
    const bone = species.bones.findIndex((candidate) => candidate.id === part.bone);
    const [a, b, c, d] = world.slice(bone * 6, bone * 6 + 4) as [number, number, number, number];
    const points = shapePoints(part.shape).map(([px, py]) => [a * px + c * py, b * px + d * py]);
    const xs = points.map((point) => point[0]!);
    const ys = points.map((point) => point[1]!);
    return Math.max(...xs) - Math.min(...xs) < 0.2 && Math.max(...ys) - Math.min(...ys) < 0.2;
  });
  console.log(`hidden at rest: ${hidden.map((part) => part.id).join(", ") || "none"}`);
  const idle = species.clips.find((clip) => clip.id === species.repertoire.idle?.[0]);
  let reach = 0;
  for (const activity of ENCOUNTERS) {
    for (const id of species.repertoire[activity] ?? []) {
      const clip = species.clips.find((candidate) => candidate.id === id)!;
      const box = reachOf(species, clip, idle);
      reach = Math.max(reach, box.right);
      console.log(`encounter ${activity.padEnd(9)} ${id.padEnd(16)} right ${fixed(box.right)}`);
    }
  }
  console.log(`=> reach needed ${fixed(reach)} (document ${species.reach})`);
  for (const clip of species.clips) {
    if (!clip.loop) {
      const offRest = clip.tracks.filter((track) => {
        const rest = track.channel.startsWith("scale") ? 1 : 0;
        return Math.abs(track.keys[0]!.value - rest) > 1e-9 || Math.abs(track.keys[track.keys.length - 1]!.value - rest) > 1e-9;
      });
      if (offRest.length > 0) console.log(`once clip ${clip.id} starts or ends off rest: ${offRest.map((track) => `${track.bone}.${track.channel}`).join(", ")}`);
    }
    if (showClips) {
      const box = reachOf(species, clip, idle);
      console.log(`clip ${clip.id.padEnd(18)} ${clip.loop ? "loop" : "once"} ${String(clip.seconds).padStart(4)}s  left ${fixed(box.left)} right ${fixed(box.right)} top ${fixed(box.top)} bottom ${fixed(box.bottom)}`);
    }
  }
}
