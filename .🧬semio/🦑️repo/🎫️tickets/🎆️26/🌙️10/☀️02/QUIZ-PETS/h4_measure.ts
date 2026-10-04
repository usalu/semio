/** 📏️ Ticket tool of work package H4: measures species documents — the contrast of every state's colours against both page colours, how far the encounter clips reach beyond the size box towards a partner (`reach`), and the drawn extent of any clip over its whole length (front, back, top, bottom, relative to the size box and the ground).
 *
 * Usage (from the repository root):
 *   bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/h4_measure.ts" [--clips a,b|all] <species.json>...
 *
 * The geometry is sampled from the product's own rig and clip sampling (`sampleClip`, `solveRig`); outlines are
 * sampled along their curves and widened by half their transformed stroke.
 *
 * @see https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio — the contrast ratio
 * @see ../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🟦️.ts — `solveRig`, `restPose`
 */
import { readFileSync } from "node:fs";
import type { Clip, Shape, Species } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { clipTicks, sampleClip } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🟦️.ts";
import { restPose, solveRig } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🟦️.ts";

const PAGES = { light: "#f7f3e3", dark: "#001117" } as const;

/** 💡️ The relative luminance of an sRGB colour. */
const luminance = (hex: string): number => {
  const channel = (offset: number): number => {
    const value = Number.parseInt(hex.slice(offset, offset + 2), 16) / 255;
    return value <= 0.04045 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(1) + 0.7152 * channel(3) + 0.0722 * channel(5);
};

/** ⚖️ The contrast ratio of two colours. */
const contrast = (one: string, two: string): number => {
  const [high, low] = [luminance(one), luminance(two)].sort((left, right) => right - left) as [number, number];
  return (high + 0.05) / (low + 0.05);
};

/** 🧭️ Points along a path's outline (absolute and relative M, L, H, V, Q, C, Z; arcs by their end points). */
const pathPoints = (d: string): [number, number][] => {
  const tokens = d.match(/[a-zA-Z]|-?\d*\.?\d+(?:e-?\d+)?/g) ?? [];
  const points: [number, number][] = [];
  let index = 0;
  let command = "M";
  let x = 0;
  let y = 0;
  let startX = 0;
  let startY = 0;
  const next = (): number => Number(tokens[index++]);
  while (index < tokens.length) {
    if (/[a-zA-Z]/.test(tokens[index]!)) command = tokens[index++]!;
    const relative = command === command.toLowerCase();
    const upper = command.toUpperCase();
    const base = (value: number, origin: number): number => (relative ? origin + value : value);
    if (upper === "Z") {
      x = startX;
      y = startY;
      continue;
    }
    if (upper === "M" || upper === "L") {
      x = base(next(), x);
      y = base(next(), y);
      if (upper === "M") [startX, startY] = [x, y];
      points.push([x, y]);
      if (upper === "M") command = relative ? "l" : "L";
    } else if (upper === "H") {
      x = base(next(), x);
      points.push([x, y]);
    } else if (upper === "V") {
      y = base(next(), y);
      points.push([x, y]);
    } else if (upper === "Q") {
      const cx = base(next(), x);
      const cy = base(next(), y);
      const ex = base(next(), x);
      const ey = base(next(), y);
      for (let step = 1; step <= 12; step++) {
        const t = step / 12;
        points.push([(1 - t) ** 2 * x + 2 * (1 - t) * t * cx + t * t * ex, (1 - t) ** 2 * y + 2 * (1 - t) * t * cy + t * t * ey]);
      }
      [x, y] = [ex, ey];
    } else if (upper === "C") {
      const c1x = base(next(), x);
      const c1y = base(next(), y);
      const c2x = base(next(), x);
      const c2y = base(next(), y);
      const ex = base(next(), x);
      const ey = base(next(), y);
      for (let step = 1; step <= 16; step++) {
        const t = step / 16;
        const u = 1 - t;
        points.push([u ** 3 * x + 3 * u * u * t * c1x + 3 * u * t * t * c2x + t ** 3 * ex, u ** 3 * y + 3 * u * u * t * c1y + 3 * u * t * t * c2y + t ** 3 * ey]);
      }
      [x, y] = [ex, ey];
    } else if (upper === "A") {
      for (let skip = 0; skip < 5; skip++) next();
      x = base(next(), x);
      y = base(next(), y);
      points.push([x, y]);
    } else index++;
  }
  return points;
};

/** 🔷️ Points along the outline of a shape in its bone's coordinates. */
const outline = (shape: Shape): [number, number][] => {
  if (shape.kind === "line") return [[shape.x1, shape.y1], [shape.x2, shape.y2]];
  if (shape.kind === "ellipse") return Array.from({ length: 32 }, (_, step) => [shape.cx + shape.rx * Math.cos((step / 32) * 2 * Math.PI), shape.cy + shape.ry * Math.sin((step / 32) * 2 * Math.PI)] as [number, number]);
  if (shape.kind === "rect") {
    const radius = Math.min(shape.radius ?? 0, shape.width / 2, shape.height / 2);
    const corners: [number, number, number][] = [[shape.x + shape.width - radius, shape.y + radius, -0.25], [shape.x + shape.width - radius, shape.y + shape.height - radius, 0], [shape.x + radius, shape.y + shape.height - radius, 0.25], [shape.x + radius, shape.y + radius, 0.5]];
    return corners.flatMap(([cx, cy, start]) => Array.from({ length: 7 }, (_, step) => [cx + radius * Math.cos((start + step / 24) * 2 * Math.PI), cy + radius * Math.sin((start + step / 24) * 2 * Math.PI)] as [number, number]));
  }
  return pathPoints(shape.d);
};

type Extent = { front: number; back: number; top: number; bottom: number };

/** 📦️ The drawn extent of a species in a pose given as solved bone matrices: x to the front and back, y up and down, strokes included. */
const extentOf = (species: Species, bones: readonly number[]): Extent => {
  const extent: Extent = { front: -Infinity, back: Infinity, top: Infinity, bottom: -Infinity };
  const index = new Map(species.bones.map((bone, position) => [bone.id, position]));
  const drawn = [...species.parts.map((part) => ({ bone: part.bone, shape: part.shape, stroke: part.stroke === "none" ? 0 : (part.strokeWidth ?? 2) })), ...species.face.eyes.map((eye) => ({ bone: eye.bone, shape: { kind: "ellipse", cx: eye.x, cy: eye.y, rx: eye.radius, ry: eye.radius } as Shape, stroke: 1.5 }))];
  for (const item of drawn) {
    const at = index.get(item.bone)! * 6;
    const [a, b, c, d, e, f] = bones.slice(at, at + 6) as [number, number, number, number, number, number];
    const half = (item.stroke / 2) * Math.sqrt(Math.abs(a * d - b * c));
    for (const [px, py] of outline(item.shape)) {
      const x = a * px + c * py + e;
      const y = b * px + d * py + f;
      extent.front = Math.max(extent.front, x + half);
      extent.back = Math.min(extent.back, x - half);
      extent.top = Math.min(extent.top, y - half);
      extent.bottom = Math.max(extent.bottom, y + half);
    }
  }
  return extent;
};

/** 👀️ A pose leant towards what the eyes look at, as the stage leans it (`leant` of `👀️attention`: the bone of the first eye 1.5 px and 5° per unit of gaze across); `across` 1 looks straight ahead at a partner. */
const leaning = (species: Species, pose: ReturnType<typeof sampleClip>, across: number): ReturnType<typeof sampleClip> => {
  const eye = species.face.eyes[0];
  return eye === undefined ? pose : pose.map((bone, index) => (species.bones[index]!.id === eye.bone ? { ...bone, x: bone.x + 1.5 * across, rotation: bone.rotation + 5 * across } : bone));
};

/** 🎞️ The extent of a clip over its whole length, sampled at every tick, leant by `across` units of gaze. */
const clipExtent = (species: Species, clip: Clip, across = 0): Extent => {
  const total: Extent = { front: -Infinity, back: Infinity, top: Infinity, bottom: -Infinity };
  const ticks = clipTicks(clip);
  for (let tick = 0; tick <= ticks; tick++) {
    const extent = extentOf(species, solveRig(species, leaning(species, sampleClip(species, clip, tick), across)));
    total.front = Math.max(total.front, extent.front);
    total.back = Math.min(total.back, extent.back);
    total.top = Math.min(total.top, extent.top);
    total.bottom = Math.max(total.bottom, extent.bottom);
  }
  return total;
};

const fixed = (value: number): string => value.toFixed(2).padStart(7);
const args = process.argv.slice(2);
const clipsAt = args.indexOf("--clips");
const wanted = clipsAt >= 0 ? args[clipsAt + 1]!.split(",") : [];
const files = args.filter((_, position) => clipsAt < 0 || (position !== clipsAt && position !== clipsAt + 1));

for (const file of files) {
  const species = JSON.parse(readFileSync(file, "utf8")) as Species;
  const half = species.size.width / 2;
  const height = species.size.height;
  console.log(`\n## ${species.id} ${species.size.width}×${height}, grip ${species.grip}, reach ${species.reach}`);
  console.log("contrast (state: paint colour → light page / dark page)");
  for (const state of species.states) {
    const colours = { ...species.palette, ...state.tint };
    const row = (["body", "accent", "detail"] as const).map((paint) => `${paint} ${colours[paint]} ${contrast(colours[paint], PAGES.light).toFixed(2)}/${contrast(colours[paint], PAGES.dark).toFixed(2)}`).join("  ");
    console.log(`  ${state.id.padEnd(12)} ${row}`);
  }
  const rest = extentOf(species, solveRig(species, restPose(species)));
  console.log(`rest extent: front ${fixed(rest.front - half)} back ${fixed(-half - rest.back)} above box ${fixed(-height - rest.top)} below feet ${fixed(rest.bottom)}`);
  let reach = 0;
  let leant = 0;
  for (const activity of ["greet", "cuddle", "squabble"] as const) {
    for (const id of species.repertoire[activity] ?? []) {
      const clip = species.clips.find((candidate) => candidate.id === id)!;
      const extent = clipExtent(species, clip);
      const towards = clipExtent(species, clip, 1);
      reach = Math.max(reach, extent.front - half);
      leant = Math.max(leant, towards.front - half);
      console.log(`encounter ${activity.padEnd(8)} ${id.padEnd(16)} front ${fixed(extent.front - half)} beyond the box, ${fixed(towards.front - half)} leant towards the partner`);
    }
  }
  console.log(`reach measured: ${reach.toFixed(2)} as authored, ${leant.toFixed(2)} leant (authored ${species.reach})`);
  for (const clip of species.clips) {
    if (clip.loop) continue;
    const off = clip.tracks.filter((track) => {
      const rest = track.channel === "scaleX" || track.channel === "scaleY" ? 1 : 0;
      const ends = [track.keys[0]!.value, track.keys[track.keys.length - 1]!.value];
      return ends.some((value) => (track.channel === "rotation" ? Math.abs(value % 360) > 1e-9 : Math.abs(value - rest) > 1e-9));
    });
    if (off.length > 0) console.log(`one-shot ${clip.id} does not start or end at rest: ${off.map((track) => `${track.bone}.${track.channel}`).join(", ")}`);
  }
  const shown = wanted.includes("all") ? species.clips.map((clip) => clip.id) : wanted;
  for (const id of shown) {
    const clip = species.clips.find((candidate) => candidate.id === id);
    if (clip === undefined) continue;
    const extent = clipExtent(species, clip);
    console.log(`clip ${id.padEnd(18)} front ${fixed(extent.front - half)} back ${fixed(-half - extent.back)} above box ${fixed(-height - extent.top)} below feet ${fixed(extent.bottom)}  (front x ${fixed(extent.front)})`);
  }
}
