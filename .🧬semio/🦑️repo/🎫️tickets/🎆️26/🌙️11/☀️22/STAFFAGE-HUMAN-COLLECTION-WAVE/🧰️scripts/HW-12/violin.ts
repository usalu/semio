/** 🎻️ The handcrafted violin (4/4, body 356 mm, overall 593 mm) and bow (750 mm) of the standing violinist, per scale as prisms in the bone-local frames:
 * violin `u` from the end button to the scroll, `v` the normal of the top plate, `w` towards the bass (G) side; bow `u` from the button to the tip, `v` towards the hair, `w` across.
 * Dimensions after the usual 4/4 pattern (body length 356, upper bout 168, C-bout 112, lower bout 208, rib height 31 mm; bow 750 mm, 60 g).
 * @see https://en.wikipedia.org/wiki/Violin — overall dimensions of a full-size violin
 * @see https://en.wikipedia.org/wiki/Bow_(music) — bow length and mass */
import { mirrored, rect, type P2, type Prism, type Scale } from "./lib.ts";

/** 🎻️ Half widths of the body outline along the axis, with the notches of the four corners. */
export const BODY: readonly P2[] = [
  [0.002, 0.0], [0.004, 0.03], [0.009, 0.055], [0.02, 0.078], [0.038, 0.095], [0.06, 0.103], [0.085, 0.104], [0.11, 0.099], [0.13, 0.09], [0.142, 0.086],
  [0.148, 0.075], [0.16, 0.063], [0.18, 0.056], [0.2, 0.057], [0.218, 0.064], [0.228, 0.074], [0.234, 0.08], [0.25, 0.082], [0.27, 0.084], [0.292, 0.082],
  [0.312, 0.075], [0.33, 0.062], [0.344, 0.045], [0.352, 0.025], [0.356, 0.0],
];
const inset = (k: number, u0: number, u1: number): P2[] => mirrored(BODY.map(([u, h]) => [u0 + ((u - 0.002) / 0.354) * (u1 - u0), h * k] as P2));
const scroll = (u0: number, v0: number): P2[] => {
  const ring: P2[] = [];
  for (let i = 0; i < 18; i++) { const t = (Math.PI * 2 * i) / 18; ring.push([u0 + 0.025 + 0.023 * Math.cos(t), v0 + 0.02 + 0.021 * Math.sin(t)]); }
  return ring;
};

const V = { bone: "violin", region: "violin", material: "violin-wood" } as const;
const E = { bone: "violin", region: "violin", material: "ebony" } as const;
const B = { bone: "bow", region: "bow", material: "pernambuco-horsehair" } as const;

/** 🎻️ The violin prisms of one scale. */
export function violinPrisms(scale: Scale): Prism[] {
  if (scale === "50") return [
    { ...V, id: "violin-ribs", sweep: "v", profile: mirrored(BODY), from: -0.016, to: 0.016, tone: 0.34, z: 3.2 },
    { ...V, id: "violin-top", sweep: "v", profile: inset(0.86, 0.012, 0.346), from: 0.015, to: 0.027, tone: 0.42, z: 3.3 },
    { ...V, id: "violin-back", sweep: "v", profile: inset(0.86, 0.012, 0.346), from: -0.027, to: -0.015, tone: 0.3, z: 3.1 },
    { ...V, id: "violin-neck", sweep: "v", profile: rect(0.35, -0.0115, 0.492, 0.0115), from: 0.002, to: 0.028, tone: 0.34, z: 3.25 },
    { ...E, id: "violin-fingerboard", sweep: "v", profile: [[0.232, -0.021], [0.498, -0.0125], [0.498, 0.0125], [0.232, 0.021]], from: 0.026, to: 0.037, tone: 0.1, z: 3.4 },
    { ...V, id: "violin-pegbox", sweep: "v", profile: rect(0.49, -0.0125, 0.548, 0.0125), from: -0.002, to: 0.038, tone: 0.34, z: 3.35 },
    { ...E, id: "violin-pegs", sweep: "v", profile: [...rect(0.502, -0.034, 0.51, 0.034)], from: 0.024, to: 0.034, tone: 0.1, z: 3.45 },
    { ...E, id: "violin-pegs-2", sweep: "v", profile: [...rect(0.525, -0.034, 0.533, 0.034)], from: 0.024, to: 0.034, tone: 0.1, z: 3.45 },
    { ...V, id: "violin-scroll", sweep: "w", profile: scroll(0.544, -0.004), from: -0.019, to: 0.019, tone: 0.36, z: 3.5 },
    { ...E, id: "violin-tailpiece", sweep: "v", profile: [[0.008, -0.012], [0.112, -0.02], [0.118, 0], [0.112, 0.02], [0.008, 0.012]], from: 0.025, to: 0.034, tone: 0.1, z: 3.4 },
    { ...E, id: "violin-chinrest", sweep: "v", profile: [[0.004, 0.008], [0.03, 0.006], [0.062, 0.018], [0.07, 0.045], [0.058, 0.074], [0.03, 0.082], [0.008, 0.07], [0.002, 0.04]], from: 0.022, to: 0.042, tone: 0.12, z: 3.45 },
    { ...V, id: "violin-bridge", sweep: "w", profile: rect(0.192, 0.026, 0.2, 0.049), from: -0.02, to: 0.02, tone: 0.6, z: 3.5 },
  ];
  if (scale === "100") return [
    { ...V, id: "violin-body", sweep: "v", profile: mirrored(BODY), from: -0.022, to: 0.024, tone: 0.36, z: 3.2 },
    { ...E, id: "violin-neck", sweep: "v", profile: [[0.24, -0.021], [0.548, -0.013], [0.548, 0.013], [0.24, 0.021]], from: 0.002, to: 0.038, tone: 0.12, z: 3.3 },
    { ...V, id: "violin-scroll", sweep: "w", profile: [[0.5, -0.004], [0.565, -0.006], [0.59, 0.006], [0.592, 0.028], [0.575, 0.042], [0.5, 0.04]], from: -0.016, to: 0.016, tone: 0.36, z: 3.4 },
    { ...E, id: "violin-chinrest", sweep: "v", profile: [[0.004, 0.006], [0.07, 0.02], [0.07, 0.07], [0.006, 0.075]], from: 0.022, to: 0.04, tone: 0.12, z: 3.45 },
  ];
  if (scale === "200") return [
    { ...V, id: "violin-body", sweep: "v", profile: mirrored([[0.002, 0.0], [0.012, 0.07], [0.06, 0.104], [0.13, 0.092], [0.185, 0.058], [0.24, 0.08], [0.29, 0.084], [0.34, 0.055], [0.356, 0.0]]), from: -0.022, to: 0.024, tone: 0.38, z: 3.2 },
    { ...V, id: "violin-neck", sweep: "v", profile: [[0.3, -0.017], [0.593, -0.016], [0.593, 0.016], [0.3, 0.017]], from: 0.0, to: 0.04, tone: 0.2, z: 3.3 },
  ];
  return [
    { ...V, id: "violin-body", sweep: "v", profile: [[0.0, -0.075], [0.06, -0.104], [0.3, -0.084], [0.356, -0.04], [0.593, -0.014], [0.593, 0.014], [0.356, 0.04], [0.3, 0.084], [0.06, 0.104], [0.0, 0.075]], from: -0.037, to: 0.038, tone: 0.38, z: 3.2 },
  ];
}

/** 🏹️ The bow prisms of one scale. */
export function bowPrisms(scale: Scale): Prism[] {
  const stick: Prism = { ...B, id: "bow-stick", sweep: "w", profile: [[0, -0.0043], [0.745, -0.0027], [0.745, 0.0027], [0, 0.0043]], from: -0.004, to: 0.004, tone: 0.3, z: 7.1, slender: true };
  if (scale === "50" || scale === "100") return [
    stick,
    { ...B, id: "bow-hair", sweep: "w", profile: [[0.046, 0.0175], [0.728, 0.0085], [0.728, 0.0115], [0.046, 0.0205]], from: -0.0055, to: 0.0055, tone: 0.85, z: 7.05, slender: true },
    { ...B, id: "bow-frog", sweep: "w", profile: [[-0.002, 0.003], [0.05, 0.003], [0.05, 0.022], [0.036, 0.026], [0.004, 0.026], [-0.002, 0.018]], from: -0.008, to: 0.008, tone: 0.12, z: 7.15 },
    ...(scale === "50" ? [{ ...B, id: "bow-head", sweep: "w", profile: [[0.718, -0.003], [0.75, -0.003], [0.75, 0.002], [0.735, 0.013], [0.718, 0.012]], from: -0.0042, to: 0.0042, tone: 0.3, z: 7.15 } as Prism] : []),
  ];
  return [{ ...B, id: "bow", sweep: "w", profile: [[0, -0.0043], [0.05, -0.0043], [0.75, -0.003], [0.75, 0.011], [0.05, 0.02], [0, 0.02]], from: -0.005, to: 0.005, tone: 0.3, z: 7.1, slender: true }];
}
