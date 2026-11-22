/** 🖼️ The handcrafted beech A-frame (lyre) easel with a 50 × 60 cm stretched canvas, and the brush and palette of the painter: every bar is a bone of its own (fixed joint) along its axis, so its prism draws and models true in every view.
 * Easel frame: `x` ahead (away from the painter), `y` up, `z` left, origin on the ground between the front feet. Typical studio A-frame easel: front legs 1.85 m, 35 × 22 mm beech, mast 45 × 25 mm, tray at 0.88 m, back leg hinged at the top.
 * @see https://en.wikipedia.org/wiki/Easel — easel types
 * @see https://en.wikipedia.org/wiki/Palette_(painting) — palette */
import { add, cross, mirrored, mul, rect, sub, unit, type P2, type Prism, type Scale, type V3 } from "./lib.ts";

/** 🪵️ One straight member of the easel: a bone from `a` to `b` (easel frame) with the bar section `depth` (along the bone normal) × `width` (across). */
export type Bar = { id: string; a: V3; b: V3; depth: number; width: number; up?: V3 };

const LEAN = 0.3;
const TOP = 1.82;
export const TRAY_Y = 0.88;
const at = (y: number): number => (LEAN * y) / TOP;

/** 🦵️ The bars: front legs splayed in an A, the mast between them, the back leg, the tray and the lower brace. */
export const BARS: readonly Bar[] = [
  { id: "easel-leg-l", a: [0, 0, 0.33], b: [LEAN, TOP, 0.035], depth: 0.022, width: 0.035 },
  { id: "easel-leg-r", a: [0, 0, -0.33], b: [LEAN, TOP, -0.035], depth: 0.022, width: 0.035 },
  { id: "easel-mast", a: [at(0.42), 0.42, 0], b: [LEAN + 0.004, TOP + 0.04, 0], depth: 0.025, width: 0.045 },
  { id: "easel-leg-back", a: [LEAN + 0.62, 0, 0], b: [LEAN + 0.03, TOP - 0.06, 0], depth: 0.022, width: 0.035 },
  { id: "easel-brace", a: [at(0.42) + 0.012, 0.42, -0.26], b: [at(0.42) + 0.012, 0.42, 0.26], depth: 0.03, width: 0.022, up: [0, 1, 0] },
  { id: "easel-tray", a: [at(TRAY_Y) - 0.045, TRAY_Y, -0.29], b: [at(TRAY_Y) - 0.045, TRAY_Y, 0.29], depth: 0.03, width: 0.07, up: [0, 1, 0] },
];

/** 🧭️ The rest frame of a bar in the easel frame: u along the bar, v the normal (towards the painter for an upright bar, `up` for a horizontal one), w = u × v. */
export function barFrame(bar: Bar): { o: V3; u: V3; v: V3 } {
  const u = unit(sub(bar.b, bar.a));
  const hint: V3 = bar.up ?? [-1, 0, 0];
  const w = unit(cross(u, hint));
  const v = unit(cross(w, u));
  return { o: bar.a, u, v };
}
export const lengthOf = (bar: Bar): number => Math.hypot(bar.b[0] - bar.a[0], bar.b[1] - bar.a[1], bar.b[2] - bar.a[2]);

/** 🖼️ Canvas frame in the easel frame: u up the canvas, v its normal towards the painter, w to the left; bottom edge on the tray. */
export const CANVAS = { width: 0.5, height: 0.6, depth: 0.022 };
export function canvasFrame(): { o: V3; u: V3; v: V3 } {
  const u = unit([LEAN, TOP, 0]);
  const v = unit(cross([0, 0, 1], u));
  const o = add([at(TRAY_Y) - 0.0352, TRAY_Y + 0.015, 0], mul(v, 0.0));
  return { o, u, v: mul(v, 1) };
}

const W = { region: "easel", material: "oiled-beech" } as const;

/** 🪵️ The prism of one bar at one scale, and whether it is drawn there. */
export function barPrism(bar: Bar, scale: Scale): Prism | undefined {
  const L = lengthOf(bar);
  if (scale === "500" && (bar.id === "easel-tray" || bar.id === "easel-brace")) return undefined;
  const f = (0.15 * Number(scale)) / 1000;
  let [d, w] = scale === "500" ? [0.03, 0.035] : [bar.depth, bar.width];
  const slender = L / Math.max(d, w) >= 20;
  if (!slender) [d, w] = [Math.max(d, f + 0.001), Math.max(w, f + 0.001)];
  return { ...W, id: bar.id, bone: bar.id, sweep: "w", profile: rect(0, -d / 2, L, d / 2), from: -w / 2, to: w / 2, tone: 0.55, z: 1, ...(slender ? { slender: true } : {}) };
}

/** 🖼️ The canvas prisms of one scale: the stretched canvas and, at 1:50 and 1:100, the painted field on its face. */
export function canvasPrisms(scale: Scale): Prism[] {
  const { width, height, depth } = CANVAS;
  const C = { bone: "canvas", region: "canvas", material: "primed-canvas" } as const;
  const panel: Prism = { ...C, id: "canvas", sweep: "v", profile: rect(0, -width / 2, height, width / 2), from: -depth, to: 0, tone: 0.9, z: 1.5, sheet: scale === "200" || scale === "500" ? true : undefined };
  if (scale === "200" || scale === "500") return [{ ...panel, sheet: true }];
  return [
    { ...panel, sheet: undefined },
    { ...C, id: "canvas-painting", role: "detail", sweep: "v", profile: [[0.06, -0.2], [0.28, -0.21], [0.42, -0.12], [0.55, 0.02], [0.52, 0.2], [0.3, 0.21], [0.12, 0.16]], from: 0, to: 0.0015, tone: 0.45, z: 1.6, sheet: true },
  ];
}

/** 🖌️ The brush (300 mm, flat hog bristle no. 10) along its bone from the end of the handle to the bristle tip. */
export function brushPrisms(scale: Scale): Prism[] {
  const B = { bone: "brush", region: "brush", material: "lacquered-brush" } as const;
  if (scale === "50") return [
    { ...B, id: "brush-handle", sweep: "w", profile: [[0, -0.0028], [0.12, -0.0042], [0.25, -0.0036], [0.25, 0.0036], [0.12, 0.0042], [0, 0.0028]], from: -0.004, to: 0.004, tone: 0.25, z: 7.2, slender: true },
    { ...B, id: "brush-ferrule", sweep: "w", profile: rect(0.25, -0.0045, 0.278, 0.0045), from: -0.005, to: 0.005, tone: 0.75, z: 7.25 },
    { ...B, id: "brush-bristles", sweep: "w", profile: [[0.278, -0.004], [0.3, -0.0038], [0.302, 0], [0.3, 0.0038], [0.278, 0.004]], from: -0.008, to: 0.008, tone: 0.85, z: 7.3 },
  ];
  return [{ ...B, id: "brush", sweep: "w", profile: [[0, -0.0035], [0.25, -0.0045], [0.302, -0.004], [0.302, 0.004], [0.25, 0.0045], [0, 0.0035]], from: -0.0045, to: 0.0045, tone: 0.3, z: 7.2, slender: true }];
}

/** 🎨️ The palette: a 400 × 290 mm kidney-shaped plywood board 6 mm thick in its bone frame (u away from the thumb hole, w across, v its normal), with paint dabs at 1:50. */
export const PALETTE: readonly P2[] = [[0.0, 0.0], [0.02, 0.085], [0.07, 0.125], [0.16, 0.14], [0.27, 0.125], [0.36, 0.08], [0.4, 0.0], [0.38, -0.085], [0.3, -0.135], [0.18, -0.145], [0.08, -0.12], [0.03, -0.07], [0.055, -0.03], [0.05, 0.02]];
export function palettePrisms(scale: Scale): Prism[] {
  const P = { bone: "palette", region: "palette", material: "palette-plywood" } as const;
  const board: Prism = { ...P, id: "palette", sweep: "v", profile: PALETTE.map(([a, b]) => [a - 0.04, b] as P2), from: -0.003, to: 0.003, tone: 0.65, z: 5.5, sheet: true };
  if (scale === "500") return [{ ...board, profile: [[-0.04, -0.06], [0.02, -0.12], [0.3, -0.13], [0.36, -0.06], [0.36, 0.06], [0.3, 0.12], [0.02, 0.12], [-0.04, 0.06]], sheet: true }];
  if (scale !== "50") return [board];
  const dab = (id: string, u: number, w: number, tone: number): Prism => ({ ...P, id, sweep: "v", profile: mirrored([[u - 0.016, 0.004], [u - 0.012, 0.012], [u, 0.016], [u + 0.012, 0.012], [u + 0.016, 0.004]]).map(([a, b]) => [a, b + w] as P2), from: 0.002, to: 0.011, tone, z: 5.6 });
  return [board, dab("paint-white", 0.09, 0.09, 0.95), dab("paint-red", 0.22, 0.11, 0.35), dab("paint-blue", 0.32, 0.0, 0.2)];
}
