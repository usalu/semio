/** 📏️ Ticket tool of work package H2: measures species documents where the eye cannot — how far every clip reaches around the feet (stroke included, the idle loop running underneath as on the stage), the `reach` of the encounter clips (greet, cuddle, squabble: how far they lean out beyond half the width on the side the pet faces), and the contrast of the body colour of every state (tint or palette) against both page colours.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/h2_measure.ts" [--clips a,b] <species.json>...
 *
 * @see https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio — the contrast ratio
 * @see ./render_species_preview.mjs — what the eye judges
 */
import { readFileSync } from "node:fs";
import type { Clip, Shape, Species } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { clipTicks, sampleClip } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🟦️.ts";
import { restPose, solveRig, type Pose } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🟦️.ts";

const PAGES = { light: "#f7f3e3", dark: "#001117" } as const;
const ENCOUNTERS = ["greet", "cuddle", "squabble"] as const;

const args = process.argv.slice(2);
const at = args.indexOf("--clips");
const only = at < 0 ? null : args.splice(at, 2)[1]!.split(",");

/** 🌗️ The relative luminance of an sRGB colour. */
const luminance = (hex: string): number => {
  const channel = (offset: number): number => {
    const value = parseInt(hex.slice(offset, offset + 2), 16) / 255;
    return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5);
};

/** ⚖️ The contrast ratio of two colours. */
const contrast = (one: string, other: string): number => {
  const [high, low] = [luminance(one), luminance(other)].sort((a, b) => b - a) as [number, number];
  return (high + 0.05) / (low + 0.05);
};

/** 📍️ The points that bound a shape (control points of curves included, so the box is never too small). */
function outline(shape: Shape): [number, number][] {
  if (shape.kind === "line") return [[shape.x1, shape.y1], [shape.x2, shape.y2]];
  if (shape.kind === "rect") return [[shape.x, shape.y], [shape.x + shape.width, shape.y], [shape.x, shape.y + shape.height], [shape.x + shape.width, shape.y + shape.height]];
  if (shape.kind === "ellipse") return Array.from({ length: 24 }, (_, index) => [shape.cx + shape.rx * Math.cos((index * Math.PI) / 12), shape.cy + shape.ry * Math.sin((index * Math.PI) / 12)] as [number, number]);
  const points: [number, number][] = [];
  const tokens = shape.d.match(/[a-zA-Z]|-?\d*\.?\d+(?:e-?\d+)?/g) ?? [];
  let index = 0;
  let command = "M";
  let x = 0;
  let y = 0;
  let startX = 0;
  let startY = 0;
  const number = (): number => Number(tokens[index++]);
  while (index < tokens.length) {
    if (/[a-zA-Z]/.test(tokens[index]!)) command = tokens[index++]!;
    const relative = command === command.toLowerCase();
    const upper = command.toUpperCase();
    if (upper === "Z") {
      x = startX;
      y = startY;
      continue;
    }
    const counts: Record<string, number> = { M: 2, L: 2, H: 1, V: 1, Q: 4, T: 2, C: 6, S: 4, A: 7 };
    const count = counts[upper] ?? 2;
    const values = Array.from({ length: count }, number);
    if (upper === "H") x = relative ? x + values[0]! : values[0]!;
    else if (upper === "V") y = relative ? y + values[0]! : values[0]!;
    else if (upper === "A") {
      x = relative ? x + values[5]! : values[5]!;
      y = relative ? y + values[6]! : values[6]!;
    } else {
      const absolute: [number, number][] = [];
      for (let pair = 0; pair < count; pair += 2) absolute.push([relative ? x + values[pair]! : values[pair]!, relative ? y + values[pair + 1]! : values[pair + 1]!]);
      const curve: [number, number][] = [[x, y], ...absolute];
      if (upper === "Q" || upper === "C") {
        for (let step = 1; step <= 16; step++) {
          const t = step / 16;
          const level = curve.map((point) => [...point] as [number, number]);
          for (let order = level.length - 1; order > 0; order--) for (let k = 0; k < order; k++) level[k] = [level[k]![0] + (level[k + 1]![0] - level[k]![0]) * t, level[k]![1] + (level[k + 1]![1] - level[k]![1]) * t];
          points.push(level[0]!);
        }
      } else points.push(...absolute);
      x = absolute[absolute.length - 1]![0];
      y = absolute[absolute.length - 1]![1];
    }
    points.push([x, y]);
    if (upper === "M") {
      startX = x;
      startY = y;
      command = relative ? "l" : "L";
    }
  }
  return points;
}

/** 🧩️ One pose on top of another, as the stage layers a clip over the idle loop. */
const layer = (under: Pose, over: Pose): Pose => under.map((bone, index) => ({ x: bone.x + over[index]!.x, y: bone.y + over[index]!.y, rotation: bone.rotation + over[index]!.rotation, scaleX: bone.scaleX * over[index]!.scaleX, scaleY: bone.scaleY * over[index]!.scaleY }));

/** 📦️ How far the drawing of a pose reaches: left, right, top, bottom around the feet, strokes included. */
function extent(species: Species, pose: Pose): [number, number, number, number] {
  const world = solveRig(species, pose);
  let [left, right, top, bottom] = [Infinity, -Infinity, Infinity, -Infinity];
  const add = (bone: string, points: [number, number][], pad: number): void => {
    const index = species.bones.findIndex((candidate) => candidate.id === bone);
    const [a, b, c, d, e, f] = world.slice(index * 6, index * 6 + 6) as [number, number, number, number, number, number];
    const size = Math.sqrt(Math.abs(a * d - b * c));
    if (size < 0.15) return;
    for (const [px, py] of points) {
      const wx = a * px + c * py + e;
      const wy = b * px + d * py + f;
      left = Math.min(left, wx - pad);
      right = Math.max(right, wx + pad);
      top = Math.min(top, wy - pad);
      bottom = Math.max(bottom, wy + pad);
    }
  };
  for (const part of species.parts) add(part.bone, outline(part.shape), part.stroke === "none" ? 0 : (part.strokeWidth ?? 2) / 2);
  for (const eye of species.face.eyes) add(eye.bone, outline({ kind: "ellipse", cx: eye.x, cy: eye.y, rx: eye.radius, ry: eye.radius }), 0.75);
  return [left, right, top, bottom];
}

const round = (value: number): string => value.toFixed(1);

for (const file of args) {
  const species = JSON.parse(readFileSync(file, "utf8")) as Species;
  const idle = species.clips.find((clip) => clip.id === species.repertoire.idle?.[0]);
  const half = species.size.width / 2;
  console.log(`\n== ${species.id} ${species.size.width}x${species.size.height} (half ${half}) grip ${species.grip} reach ${species.reach}`);
  const [l0, r0, t0, b0] = extent(species, restPose(species));
  console.log(`rest: x ${round(l0)} .. ${round(r0)}  y ${round(t0)} .. ${round(b0)}`);
  const sweep = (clip: Clip, layered: boolean): [number, number, number, number] => {
    let box: [number, number, number, number] = [Infinity, -Infinity, Infinity, -Infinity];
    const ticks = clipTicks(clip);
    for (let tick = 0; tick <= ticks; tick++) {
      const over = sampleClip(species, clip, tick);
      const pose = layered && idle !== undefined && idle.id !== clip.id ? layer(sampleClip(species, idle, tick), over) : over;
      const [l, r, t, b] = extent(species, pose);
      box = [Math.min(box[0], l), Math.max(box[1], r), Math.min(box[2], t), Math.max(box[3], b)];
    }
    return box;
  };
  let reach = 0;
  for (const activity of ENCOUNTERS) {
    for (const id of species.repertoire[activity] ?? []) {
      const clip = species.clips.find((candidate) => candidate.id === id)!;
      const [, right] = sweep(clip, true);
      reach = Math.max(reach, right - half);
      console.log(`encounter ${activity} ${id}: front ${round(right)} → beyond half ${round(right - half)}`);
    }
  }
  console.log(`reach (measured, rounded up to 0.5): ${Math.ceil(Math.max(0, reach) * 2) / 2}`);
  for (const clip of species.clips) {
    if (only !== null && !only.includes(clip.id)) continue;
    const [l, r, t, b] = sweep(clip, true);
    const flags = [l < -half - 0.5 ? `left +${round(-half - l)}` : "", r > half + 0.5 ? `right +${round(r - half)}` : "", t < -species.size.height - 0.5 ? `top +${round(-species.size.height - t)}` : "", b > 0.5 ? `below +${round(b)}` : ""].filter(Boolean).join(", ");
    console.log(`clip ${clip.id.padEnd(18)} x ${round(l).padStart(6)} .. ${round(r).padStart(5)}  y ${round(t).padStart(6)} .. ${round(b).padStart(5)}  ${flags}`);
  }
  for (const state of species.states) {
    const body = state.tint?.body ?? species.palette.body;
    console.log(`state ${state.id.padEnd(10)} body ${body}  vs light ${contrast(body, PAGES.light).toFixed(2)}  vs dark ${contrast(body, PAGES.dark).toFixed(2)}${state.tint === undefined ? " (palette)" : ""}`);
  }
}
