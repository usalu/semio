/** 🧪️ Ticket tool (work package A9), browser side: mounts the product's own gear, tilt, tint and particle painters on a page so `wp_a9_gear_sheet.mjs` can photograph them in all their parameter ranges. Bundled by that script; never part of the product. */
import { restPose, solveRig, type ActorFrame, type Species } from "@semio-tech/pets";
import { depict, paint, tiltedPlacement, tintPalette } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🖌️depiction/🟦️.ts";
import { equip, paintLadders, paintTools, rackLadders, type GearLadder, type GearTool } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/🧰️gear/🟦️.ts";
import { paintEffects, stageEffects, tintEffects, type EffectParticle } from "../../../../../../../🧰️framework/🛍️products/🐾️pets/🎯️targets/⚛️react/🔨️modules/✨️effects/🟦️.ts";

type Cell = {
  readonly label: string;
  readonly species: number;
  readonly facing?: 1 | -1;
  readonly tilt?: number;
  readonly pivot?: "grip" | "feet";
  readonly tools?: readonly GearTool[];
  readonly ladders?: readonly GearLadder[];
  readonly particles?: readonly Omit<EffectParticle, "species">[];
  readonly tint?: { body?: string; accent?: string; detail?: string };
  readonly lift?: number;
  readonly wide?: number;
  readonly tall?: number;
  readonly marks?: readonly (readonly [number, number])[];
};
type Row = { readonly title: string; readonly cells: readonly Cell[] };

declare const SHEET: { readonly species: readonly Species[]; readonly zoom: number; readonly themes: readonly string[] };

const EMITTERS = [
  { id: "spark", bone: "root", x: 0, y: 0, shape: { kind: "path", d: "M 0 -4 L 1 -1 L 4 0 L 1 1 L 0 4 L -1 1 L -4 0 L -1 -1 Z" }, fill: "accent", stroke: "ink", strokeWidth: 0.75, motion: "burst", count: 8, life: 0.9, speed: 40, spread: 1 },
  { id: "drop", bone: "root", x: 0, y: 0, shape: { kind: "path", d: "M 0 -3 Q 2 0 0 1.6 Q -2 0 0 -3 Z" }, fill: "detail", stroke: "none", motion: "fall", count: 6, life: 0.8, speed: 60, spread: 0.1 },
  { id: "ring", bone: "root", x: 0, y: 0, shape: { kind: "ellipse", cx: 0, cy: 0, rx: 4, ry: 4 }, fill: "none", stroke: "body", strokeWidth: 1.5, motion: "rise", count: 4, life: 1.2, speed: 20, spread: 0.2 },
] as const;

const UP_RIGHT = { x: 46, y: -70 };

const rows = (): readonly Row[] => [
  {
    title: "parachute: open 0.15 … 1.25 (overshoot), then sway and tilt about the grip, mirrored, a canopy of its own",
    cells: [
      ...[0.15, 0.4, 0.7, 1, 1.25].map((open): Cell => ({ label: `open ${open}`, species: 1, tools: [{ kind: "chute", open, sway: 0 }], lift: 30, tall: 190 })),
      { label: "sway +0.04, tilt −15°", species: 1, tilt: -15 / 360, tools: [{ kind: "chute", open: 1, sway: 0.04 }], lift: 30, tall: 190 },
      { label: "sway −0.04, tilt +15°, facing left", species: 1, facing: -1, tilt: 15 / 360, tools: [{ kind: "chute", open: 1, sway: -0.04 }], lift: 30, tall: 190 },
      { label: "small pet, open 1", species: 0, tools: [{ kind: "chute", open: 1, sway: 0 }], lift: 30, tall: 190 },
      { label: "own canopy (species 2)", species: 2, tools: [{ kind: "chute", open: 1, sway: 0.02 }], lift: 30, tall: 190 },
    ],
  },
  {
    title: "rope and hook: taut, slack 12, slack −8; from the gun; hanging on it (tilt ±20° about the grip); mirrored",
    cells: [
      { label: "taut, from the pivot (grip)", species: 1, tools: [{ kind: "rope", ...UP_RIGHT, slack: 0 }, { kind: "hook", ...UP_RIGHT }], marks: [[UP_RIGHT.x, UP_RIGHT.y]] },
      { label: "slack 12", species: 1, tools: [{ kind: "rope", ...UP_RIGHT, slack: 12 }, { kind: "hook", ...UP_RIGHT }] },
      { label: "slack −8", species: 1, tools: [{ kind: "rope", ...UP_RIGHT, slack: -8 }, { kind: "hook", ...UP_RIGHT }] },
      { label: "gun aims, rope from the muzzle", species: 1, pivot: "feet", tools: [{ kind: "gun", aim: -0.16 }, { kind: "rope", ...UP_RIGHT, slack: 0 }, { kind: "hook", ...UP_RIGHT }] },
      { label: "facing left, gun, slack 6", species: 1, facing: -1, pivot: "feet", tools: [{ kind: "gun", aim: 0.5 + 0.16 }, { kind: "rope", x: -46, y: -70, slack: 6 }, { kind: "hook", x: -46, y: -70 }] },
      { label: "reel: tilt +20° on the rope", species: 1, tilt: 20 / 360, lift: 24, tools: [{ kind: "rope", x: 26, y: -96, slack: 0 }, { kind: "hook", x: 26, y: -96 }] },
      { label: "reel: tilt −20°, facing left", species: 1, facing: -1, tilt: -20 / 360, lift: 24, tools: [{ kind: "rope", x: -26, y: -96, slack: 0 }, { kind: "hook", x: -26, y: -96 }] },
      { label: "hook alone (points up)", species: 0, tools: [{ kind: "hook", x: 30, y: -50 }] },
    ],
  },
  {
    title: "grappling gun by aim: right, up-right, up, up-left (facing left), left, down-right; tilted pet",
    cells: [
      { label: "aim 0", species: 1, pivot: "feet", tools: [{ kind: "gun", aim: 0 }] },
      { label: "aim −0.125", species: 1, pivot: "feet", tools: [{ kind: "gun", aim: -0.125 }] },
      { label: "aim −0.25 (up)", species: 1, pivot: "feet", tools: [{ kind: "gun", aim: -0.25 }] },
      { label: "aim 0.625, facing left", species: 1, facing: -1, pivot: "feet", tools: [{ kind: "gun", aim: 0.625 }] },
      { label: "aim 0.5, facing left", species: 1, facing: -1, pivot: "feet", tools: [{ kind: "gun", aim: 0.5 }] },
      { label: "aim 0.08 (down-right)", species: 1, pivot: "feet", tools: [{ kind: "gun", aim: 0.08 }] },
      { label: "small pet, aim −0.1", species: 0, pivot: "feet", tools: [{ kind: "gun", aim: -0.1 }] },
      { label: "tilt −20° about the grip", species: 1, tilt: -20 / 360, lift: 20, tools: [{ kind: "gun", aim: -0.125 }] },
    ],
  },
  {
    title: "carried ladder: lying (lean 0.25), tipped (0.21), raised (0.04); lengths 60 and 110; mirrored; small pet",
    cells: [
      { label: "lean 0.25, length 60", species: 1, pivot: "feet", tools: [{ kind: "ladder", lean: 0.25, length: 60 }], wide: 150 },
      { label: "lean 0.21, length 110", species: 1, pivot: "feet", tools: [{ kind: "ladder", lean: 0.21, length: 110 }], wide: 190 },
      { label: "lean −0.21, facing left", species: 1, facing: -1, pivot: "feet", tools: [{ kind: "ladder", lean: -0.21, length: 110 }], wide: 190 },
      { label: "lean 0.04, length 110", species: 1, pivot: "feet", tools: [{ kind: "ladder", lean: 0.04, length: 110 }], tall: 190 },
      { label: "small pet, lean 0.23, length 80", species: 0, pivot: "feet", tools: [{ kind: "ladder", lean: 0.23, length: 80 }], wide: 160 },
    ],
  },
  {
    title: "standing ladders (Frame.ladders): lean ratio 0.25 with 6 and 10 rungs, half faded, raising (60°), a pet beside each",
    cells: [
      { label: "rise 70, 6 rungs", species: 1, ladders: [{ x0: 40, y0: 0, x1: 57.5, y1: -70, rungs: 6, opacity: 1 }], wide: 150 },
      { label: "rise 120, 11 rungs", species: 1, ladders: [{ x0: 40, y0: 0, x1: 70, y1: -120, rungs: 11, opacity: 1 }], wide: 150, tall: 190 },
      { label: "opacity 0.5", species: 1, ladders: [{ x0: 40, y0: 0, x1: 57.5, y1: -70, rungs: 6, opacity: 0.5 }], wide: 150 },
      { label: "raising: 60° from upright", species: 1, ladders: [{ x0: 34, y0: 0, x1: 34 + 72 * Math.sin(Math.PI / 3), y1: -72 * Math.cos(Math.PI / 3), rungs: 6, opacity: 1 }], wide: 170 },
      { label: "to the left, 2 ladders", species: 0, ladders: [{ x0: -34, y0: 0, x1: -52, y1: -72, rungs: 6, opacity: 1 }, { x0: 34, y0: 0, x1: 44, y1: -40, rungs: 3, opacity: 1 }], wide: 170 },
    ],
  },
  {
    title: "tilt of the whole drawing about its pivot (red dot): hang ±20° about the grip, both facings; tumble about the middle",
    cells: [
      { label: "tilt 0", species: 1, lift: 16 },
      { label: "tilt +20° (feet swing left)", species: 1, tilt: 20 / 360, lift: 16 },
      { label: "tilt −20°", species: 1, tilt: -20 / 360, lift: 16 },
      { label: "tilt +20°, facing left", species: 1, facing: -1, tilt: 20 / 360, lift: 16 },
      { label: "tilt −20°, facing left", species: 1, facing: -1, tilt: -20 / 360, lift: 16 },
      { label: "tilt 0.3 (tumble)", species: 0, tilt: 0.3, lift: 24 },
      { label: "tilt 0.3, facing left", species: 0, facing: -1, tilt: 0.3, lift: 24 },
    ],
  },
  {
    title: "state tints over the palette and particles of three emitters (burst, fall, rise) in the species' paints",
    cells: [
      { label: "palette", species: 1 },
      { label: "tint body", species: 1, tint: { body: "#ffe45c" } },
      { label: "tint body + accent + detail", species: 1, tint: { body: "#46606a", accent: "#2fa89a", detail: "#8ea9a8" } },
      {
        label: "burst of sparks",
        species: 1,
        particles: Array.from({ length: 8 }, (_, index) => ({ emitter: "spark", x: Math.cos((index / 8) * 2 * Math.PI) * (22 + (index % 3) * 6), y: -30 + Math.sin((index / 8) * 2 * Math.PI) * (22 + (index % 3) * 6), scale: 1 - (index % 3) * 0.2, rotation: index / 16, opacity: 1 - (index % 3) * 0.25 })),
      },
      { label: "falling drops, tinted", species: 1, tint: { detail: "#34d1bf" }, particles: Array.from({ length: 6 }, (_, index) => ({ emitter: "drop", x: -18 + index * 7, y: -8 + ((index * 5) % 9) * 2, scale: 1, rotation: 0, opacity: 0.6 + (index % 2) * 0.4 })) },
      { label: "rising rings", species: 0, particles: Array.from({ length: 4 }, (_, index) => ({ emitter: "ring", x: 6 * Math.sin(index), y: -34 - index * 9, scale: 0.5 + index * 0.25, rotation: 0, opacity: 1 - index * 0.22 })) },
    ],
  },
];

const frameOf = (species: Species, x: number, y: number, facing: 1 | -1): ActorFrame => ({ species: species.id, x, y, facing, activity: "idle", opacity: 1, bones: solveRig(species, restPose(species)), eyes: species.face.eyes.map(() => ({ x: 0.5, y: 0, lid: 0 })), mood: 0.4 }) as ActorFrame;

const mount = (cell: Cell, zoom: number): HTMLElement => {
  const species = { ...SHEET.species[cell.species]!, emitters: EMITTERS } as unknown as Species;
  const figure = document.createElement("figure");
  const wide = cell.wide ?? 124;
  const tall = cell.tall ?? 150;
  figure.className = "cell";
  figure.style.width = `${wide * zoom}px`;
  figure.style.height = `${(tall + 14) * zoom}px`;
  const feet = { x: wide / 2 - (cell.ladders ? 26 : 0), y: tall - 8 - (cell.lift ?? 0) };
  const ground = document.createElement("div");
  ground.className = "ground";
  ground.style.top = `${(tall - 8) * zoom}px`;
  const depiction = depict(species, document);
  const facing = cell.facing ?? 1;
  const pivot = { x: 0, y: cell.pivot === "feet" ? 0 : -species.grip };
  const tilt = cell.tilt ?? 0;
  paint(depiction, frameOf(species, feet.x * zoom, feet.y * zoom, facing), zoom);
  depiction.element.style.transform = tiltedPlacement(feet.x * zoom, feet.y * zoom, facing * zoom, zoom, tilt, pivot);
  tintPalette(depiction.element, species.palette, cell.tint);
  const equipment = equip(species, document);
  depiction.element.prepend(equipment.back);
  depiction.element.append(equipment.front);
  const offset = (tool: GearTool): GearTool => (tool.kind === "rope" || tool.kind === "hook" ? { ...tool, x: tool.x + feet.x, y: tool.y + feet.y } : tool);
  paintTools(equipment, { x: feet.x, y: feet.y, facing, tilt, pivot, tools: (cell.tools ?? []).map(offset) });
  const rack = rackLadders(document);
  paintLadders(rack, (cell.ladders ?? []).map((ladder) => ({ ...ladder, x0: ladder.x0 + feet.x, x1: ladder.x1 + feet.x, y0: ladder.y0 + feet.y, y1: ladder.y1 + feet.y })), zoom);
  const effects = stageEffects(document);
  const kinds = new Map([[species.id, species]]);
  tintEffects(effects, species, cell.tint);
  paintEffects(effects, kinds, (cell.particles ?? []).map((particle) => ({ ...particle, species: species.id, x: particle.x + feet.x, y: particle.y + feet.y })), zoom);
  const caption = document.createElement("figcaption");
  caption.textContent = cell.label;
  figure.append(ground, rack.element, depiction.element, effects.element, caption);
  const dots: (readonly [number, number, string])[] = [[facing * pivot.x, pivot.y, "pivot"], ...(cell.marks ?? []).map(([x, y]) => [x, y, "mark"] as const)];
  if (cell.tilt !== undefined || cell.marks !== undefined) {
    for (const [x, y, kind] of dots) {
      const dot = document.createElement("i");
      dot.className = kind;
      dot.style.left = `${(feet.x + x) * zoom}px`;
      dot.style.top = `${(feet.y + y) * zoom}px`;
      figure.append(dot);
    }
  }
  return figure;
};

for (const theme of SHEET.themes) {
  const section = document.createElement("section");
  section.className = `sheet ${theme}`;
  for (const row of rows()) {
    const heading = document.createElement("h2");
    heading.textContent = `${row.title} — ${theme}, ×${SHEET.zoom}`;
    const strip = document.createElement("div");
    strip.className = "row";
    for (const cell of row.cells) strip.append(mount(cell, SHEET.zoom));
    const band = document.createElement("div");
    band.className = "band";
    band.append(heading, strip);
    section.append(band);
  }
  document.body.append(section);
}
document.body.dataset.ready = "true";
