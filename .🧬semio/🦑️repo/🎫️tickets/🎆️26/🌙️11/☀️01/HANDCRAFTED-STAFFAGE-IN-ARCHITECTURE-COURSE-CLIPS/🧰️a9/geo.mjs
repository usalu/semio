import { layer, tx } from "./lib.mjs";

export const SRC = "authored-estimate";
const r5 = (n) => Math.round(n * 1e5) / 1e5;

export const circlePts = (cx, cy, r, n = 40, a0 = 0) => Array.from({ length: n }, (_, i) => [r5(cx + r * Math.cos(a0 + (2 * Math.PI * i) / n)), r5(cy + r * Math.sin(a0 + (2 * Math.PI * i) / n))]);

export const arcPts = (cx, cy, rx, ry, a0, a1, n = 16) => Array.from({ length: n + 1 }, (_, i) => {
  const a = a0 + ((a1 - a0) * i) / n;
  return [r5(cx + rx * Math.cos(a)), r5(cy + ry * Math.sin(a))];
});

export const rrectPts = (x0, y0, x1, y1, r, n = 6) => [
  ...arcPts(x1 - r, y1 - r, r, r, 0, Math.PI / 2, n),
  ...arcPts(x0 + r, y1 - r, r, r, Math.PI / 2, Math.PI, n),
  ...arcPts(x0 + r, y0 + r, r, r, Math.PI, (3 * Math.PI) / 2, n),
  ...arcPts(x1 - r, y0 + r, r, r, (3 * Math.PI) / 2, 2 * Math.PI, n),
];

export const ringPath = (...rings) => rings.map((pts) => `M ${pts.map(([x, y]) => `${r5(x)} ${r5(y)}`).join(" L ")} Z`).join(" ");

export const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "iso-10456-2007") => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, reference);

export const regionOf = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K, extra = {}, reference = "iso-10456-2007", skyViewFactor = 0.4) => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor,
  layers,
  heatCapacityJM2K,
  ...extra,
  source: SRC,
  reference,
});

export const exp = (exposedFraction, contact) => ({ exposedFraction, sunlitFraction: 0, ...(contact ? { contact } : {}) });
