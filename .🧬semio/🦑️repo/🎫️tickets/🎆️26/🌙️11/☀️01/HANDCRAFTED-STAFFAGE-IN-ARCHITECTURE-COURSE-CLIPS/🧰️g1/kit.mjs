import { layer, tx } from "./lib.mjs";

export const SRC = "authored-estimate";
export const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
export const fixed = { type: "fixed" };

export const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "incropera-2018") => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, reference);

export const region = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K, extra = {}, reference = "incropera-2018", skyViewFactor = 0.4) => ({
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

export const supportAnchor = (id, bone, x, y, title, support, extra = {}) => ({
  id,
  bone,
  x,
  y,
  kind: "support",
  title: tx(...title),
  support,
  normalDeg: 0,
  footprintLengthM: 0.016,
  footprintWidthM: 0.016,
  ...extra,
});

export const environment = (airTempC = 21, meanRadiantTempC = 21) => ({ airTempC, meanRadiantTempC, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 });
