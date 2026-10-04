/** 📏️ Ticket tool (work package H1): numbers the eye cannot judge on a sheet — per species the contrast of every state tint against both page colours, how far each clip's drawing reaches around the feet when it plays on top of the idle loop (stroke included), the `reach` its encounter clips need, the grip point, and the particles alive per emitter.
 *
 * Usage (from the repository root): bun ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/h1_measure.ts" <species.json>...
 *
 * @see ./render_species_preview.mjs — the sheets these numbers go with
 * @see https://www.w3.org/TR/WCAG21/#dfn-contrast-ratio — the contrast ratio
 * @see https://www.w3.org/TR/SVG11/implnote.html#ArcConversionEndpointToCenter — the arc conversion used to flatten paths
 */
import { readFileSync } from "node:fs";
import type { Clip, Pose, Shape, Species } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🧬️schema/🟦️.ts";
import { blendPose, clipTicks, sampleClip } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🎞️animation/🟦️.ts";
import { restPose, solveRig } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/🦴️rig/🟦️.ts";
import { lifeTicks, particlesOf, swarmOf } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🔨️modules/✨️effects/🟦️.ts";

type Point = { x: number; y: number };

/** 🌗️ The relative luminance of a `#rrggbb` colour. */
const luminance = (hex: string): number => {
  const channel = (index: number): number => {
    const value = parseInt(hex.slice(1 + index * 2, 3 + index * 2), 16) / 255;
    return value <= 0.03928 ? value / 12.92 : ((value + 0.055) / 1.055) ** 2.4;
  };
  return 0.2126 * channel(0) + 0.7152 * channel(1) + 0.0722 * channel(2);
};

/** ⚖️ The WCAG contrast ratio of two colours. */
const contrast = (first: string, second: string): number => {
  const [a, b] = [luminance(first), luminance(second)].sort((left, right) => right - left) as [number, number];
  return (a + 0.05) / (b + 0.05);
};

/** ✏️ Points along an SVG path, curves and arcs sampled. */
function pathPoints(d: string): Point[] {
  const tokens = d.match(/[a-zA-Z]|-?\d*\.?\d+(?:e-?\d+)?/g) ?? [];
  const points: Point[] = [];
  let index = 0;
  let command = "";
  let current = { x: 0, y: 0 };
  let start = { x: 0, y: 0 };
  let control = { x: 0, y: 0 };
  const number = (): number => Number(tokens[index++]);
  const push = (point: Point): void => void points.push(point);
  while (index < tokens.length) {
    if (/[a-zA-Z]/.test(tokens[index]!)) command = tokens[index++]!;
    const relative = command === command.toLowerCase();
    const base = relative ? current : { x: 0, y: 0 };
    switch (command.toUpperCase()) {
      case "M": {
        current = { x: base.x + number(), y: base.y + number() };
        start = current;
        push(current);
        command = relative ? "l" : "L";
        break;
      }
      case "L": {
        current = { x: base.x + number(), y: base.y + number() };
        push(current);
        break;
      }
      case "H": {
        current = { x: (relative ? current.x : 0) + number(), y: current.y };
        push(current);
        break;
      }
      case "V": {
        current = { x: current.x, y: (relative ? current.y : 0) + number() };
        push(current);
        break;
      }
      case "Q":
      case "T": {
        const q = command.toUpperCase() === "Q" ? { x: base.x + number(), y: base.y + number() } : { x: 2 * current.x - control.x, y: 2 * current.y - control.y };
        const end = { x: base.x + number(), y: base.y + number() };
        for (let step = 1; step <= 16; step++) {
          const t = step / 16;
          push({ x: (1 - t) ** 2 * current.x + 2 * (1 - t) * t * q.x + t * t * end.x, y: (1 - t) ** 2 * current.y + 2 * (1 - t) * t * q.y + t * t * end.y });
        }
        control = q;
        current = end;
        break;
      }
      case "C":
      case "S": {
        const first = command.toUpperCase() === "C" ? { x: base.x + number(), y: base.y + number() } : { x: 2 * current.x - control.x, y: 2 * current.y - control.y };
        const second = { x: base.x + number(), y: base.y + number() };
        const end = { x: base.x + number(), y: base.y + number() };
        for (let step = 1; step <= 16; step++) {
          const t = step / 16;
          const u = 1 - t;
          push({ x: u ** 3 * current.x + 3 * u * u * t * first.x + 3 * u * t * t * second.x + t ** 3 * end.x, y: u ** 3 * current.y + 3 * u * u * t * first.y + 3 * u * t * t * second.y + t ** 3 * end.y });
        }
        control = second;
        current = end;
        break;
      }
      case "A": {
        let rx = Math.abs(number());
        let ry = Math.abs(number());
        const phi = (number() * Math.PI) / 180;
        const large = number() !== 0;
        const sweep = number() !== 0;
        const end = { x: base.x + number(), y: base.y + number() };
        const cos = Math.cos(phi);
        const sin = Math.sin(phi);
        const dx = (current.x - end.x) / 2;
        const dy = (current.y - end.y) / 2;
        const x1 = cos * dx + sin * dy;
        const y1 = -sin * dx + cos * dy;
        const scale = (x1 * x1) / (rx * rx) + (y1 * y1) / (ry * ry);
        if (scale > 1) {
          rx *= Math.sqrt(scale);
          ry *= Math.sqrt(scale);
        }
        const sign = large === sweep ? -1 : 1;
        const factor = sign * Math.sqrt(Math.max(0, (rx * rx * ry * ry - rx * rx * y1 * y1 - ry * ry * x1 * x1) / (rx * rx * y1 * y1 + ry * ry * x1 * x1)));
        const cx1 = (factor * rx * y1) / ry;
        const cy1 = (-factor * ry * x1) / rx;
        const cx = cos * cx1 - sin * cy1 + (current.x + end.x) / 2;
        const cy = sin * cx1 + cos * cy1 + (current.y + end.y) / 2;
        const angle = (ux: number, uy: number, vx: number, vy: number): number => Math.atan2(ux * vy - uy * vx, ux * vx + uy * vy);
        const theta = angle(1, 0, (x1 - cx1) / rx, (y1 - cy1) / ry);
        let delta = angle((x1 - cx1) / rx, (y1 - cy1) / ry, (-x1 - cx1) / rx, (-y1 - cy1) / ry);
        if (!sweep && delta > 0) delta -= 2 * Math.PI;
        if (sweep && delta < 0) delta += 2 * Math.PI;
        for (let step = 1; step <= 24; step++) {
          const a = theta + (delta * step) / 24;
          push({ x: cx + rx * Math.cos(a) * cos - ry * Math.sin(a) * sin, y: cy + rx * Math.cos(a) * sin + ry * Math.sin(a) * cos });
        }
        current = end;
        break;
      }
      case "Z": {
        current = start;
        break;
      }
      default:
        index++;
    }
    if (command.toUpperCase() !== "Q" && command.toUpperCase() !== "T" && command.toUpperCase() !== "C" && command.toUpperCase() !== "S") control = current;
  }
  return points;
}

/** 🔷️ Points around a shape in its own coordinates. */
function shapePoints(shape: Shape): Point[] {
  if (shape.kind === "path") return pathPoints(shape.d);
  if (shape.kind === "line") return [{ x: shape.x1, y: shape.y1 }, { x: shape.x2, y: shape.y2 }];
  if (shape.kind === "rect") return [{ x: shape.x, y: shape.y }, { x: shape.x + shape.width, y: shape.y }, { x: shape.x, y: shape.y + shape.height }, { x: shape.x + shape.width, y: shape.y + shape.height }];
  const points: Point[] = [];
  for (let step = 0; step < 48; step++) points.push({ x: shape.cx + shape.rx * Math.cos((step / 48) * 2 * Math.PI), y: shape.cy + shape.ry * Math.sin((step / 48) * 2 * Math.PI) });
  return points;
}

type Box = { left: number; right: number; top: number; bottom: number };

/** 📦️ The box the drawing of a pose covers, strokes included (feet at the origin, facing right). */
function extent(species: Species, pose: Pose): Box {
  const bones = solveRig(species, pose);
  const box: Box = { left: Infinity, right: -Infinity, top: Infinity, bottom: -Infinity };
  for (const part of species.parts) {
    const at = species.bones.findIndex((bone) => bone.id === part.bone);
    const [a, b, c, d, e, f] = bones.slice(at * 6, at * 6 + 6) as [number, number, number, number, number, number];
    const half = part.stroke === "none" ? 0 : ((part.strokeWidth ?? 2) / 2) * Math.sqrt(Math.abs(a * d - b * c));
    for (const point of shapePoints(part.shape)) {
      const x = a * point.x + c * point.y + e;
      const y = b * point.x + d * point.y + f;
      box.left = Math.min(box.left, x - half);
      box.right = Math.max(box.right, x + half);
      box.top = Math.min(box.top, y - half);
      box.bottom = Math.max(box.bottom, y + half);
    }
  }
  return box;
}

/** 🧩️ One pose on top of another, as the projection layers them. */
const layer = (under: Pose, over: Pose): Pose => under.map((bone, index) => ({ x: bone.x + over[index]!.x, y: bone.y + over[index]!.y, rotation: bone.rotation + over[index]!.rotation, scaleX: bone.scaleX * over[index]!.scaleX, scaleY: bone.scaleY * over[index]!.scaleY }));

const REPLACING = new Set(["walk", "hop", "fall", "land", "sleep"]);

/** 🎞️ The box a clip covers over its whole length, played on top of the idle loop (or in its place) at eight idle phases. */
function clipExtent(species: Species, clip: Clip, replaces: boolean, overlay?: Clip): Box {
  const idle = species.clips.find((candidate) => candidate.id === species.repertoire.idle?.[0]);
  const rest = restPose(species);
  const box: Box = { left: Infinity, right: -Infinity, top: Infinity, bottom: -Infinity };
  const ticks = clipTicks(clip);
  for (let step = 0; step <= 32; step++) {
    const tick = Math.round((step / 32) * ticks);
    for (let shift = 0; shift < 8; shift++) {
      const under = idle === undefined || replaces ? rest : sampleClip(species, idle, tick + Math.round((shift / 8) * clipTicks(idle)));
      const base = overlay === undefined ? under : layer(under, sampleClip(species, overlay, tick + shift * 13));
      const pose = clip.id === idle?.id ? base : layer(base, blendPose(rest, sampleClip(species, clip, tick), 1));
      const seen = extent(species, pose);
      box.left = Math.min(box.left, seen.left);
      box.right = Math.max(box.right, seen.right);
      box.top = Math.min(box.top, seen.top);
      box.bottom = Math.max(box.bottom, seen.bottom);
    }
  }
  return box;
}

const fixed = (value: number): string => value.toFixed(1).padStart(6);

for (const file of process.argv.slice(2)) {
  const species = JSON.parse(readFileSync(file, "utf8")) as Species;
  const half = species.size.width / 2;
  const hover = species.locomotion.hover ?? 0;
  console.log(`\n== ${species.id} ${species.size.width}×${species.size.height} (half ${half}) hover ${hover} grip ${species.grip} reach ${species.reach} mood ${species.mood} gear [${species.gear.join(", ")}]`);
  console.log("-- tints: colour · against #f7f3e3 · against #001117 (3:1 wanted on both, or the ink outline carries it)");
  const rows = [{ id: "palette", tint: species.palette as Record<string, string> }, ...species.states.map((state) => ({ id: state.id, tint: (state.tint ?? {}) as Record<string, string> }))];
  for (const row of rows) {
    const cells = (["body", "accent", "detail"] as const).filter((tone) => row.tint[tone] !== undefined).map((tone) => `${tone} ${row.tint[tone]} ${contrast(row.tint[tone]!, "#f7f3e3").toFixed(2)}/${contrast(row.tint[tone]!, "#001117").toFixed(2)}`);
    console.log(`   ${row.id.padEnd(10)} ${cells.join(" · ") || "(palette)"}`);
  }
  const rest = extent(species, restPose(species));
  console.log(`-- rest box: x ${fixed(rest.left)} … ${fixed(rest.right)}  y ${fixed(rest.top)} … ${fixed(rest.bottom)}`);
  const activities = new Map<string, string[]>();
  for (const [activity, ids] of Object.entries(species.repertoire)) for (const id of ids ?? []) activities.set(id, [...(activities.get(id) ?? []), activity]);
  for (const state of species.states) if (state.clip !== undefined) activities.set(state.clip, [...(activities.get(state.clip) ?? []), `state ${state.id}`]);
  for (const trick of species.tricks) activities.set(trick.clip, [...(activities.get(trick.clip) ?? []), `trick ${trick.id}`]);
  console.log("-- clips on the idle loop: x left … right (beyond the half width) · y top … bottom (0 = feet line)");
  let reach = 0;
  for (const clip of species.clips) {
    const uses = activities.get(clip.id) ?? [];
    const replaces = uses.length > 0 && uses.every((use) => REPLACING.has(use));
    const box = clipExtent(species, clip, replaces);
    const out = Math.max(0, box.right - half);
    if (uses.some((use) => use === "greet" || use === "cuddle" || use === "squabble")) reach = Math.max(reach, out);
    console.log(`   ${clip.id.padEnd(16)} ${String(clip.seconds).padStart(5)}s ${clip.loop ? "loop" : "once"} x ${fixed(box.left)} … ${fixed(box.right)} (${(box.right - half).toFixed(1)} beyond) y ${fixed(box.top)} … ${fixed(box.bottom)}  [${uses.join(", ")}]`);
  }
  console.log(`-- reach wanted by the encounter clips (right edge beyond the half width): ${reach.toFixed(1)} (document says ${species.reach})`);
  console.log("-- emitters: most particles alive over 4 s of running");
  for (const emitter of species.emitters) {
    let most = 0;
    for (let tick = 0; tick < 256; tick += 2) most = Math.max(most, particlesOf(emitter, { x: 0, y: 0 }, 1, 0, null, tick, 7).length);
    console.log(`   ${emitter.id.padEnd(14)} ${emitter.motion.padEnd(6)} count ${emitter.count} (swarm ${swarmOf(emitter)}) life ${emitter.life}s (${lifeTicks(emitter)} ticks) speed ${emitter.speed} spread ${emitter.spread} → at most ${most} alive`);
  }
}
