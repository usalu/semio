import { layer, tx } from "../🧰️a10/lib.mjs";

export const SRC = "authored-estimate";
export const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
export const fixed = { type: "fixed" };

export const single = (id, en, de, kind, material, thicknessM, res, cap, reference = "incropera-2018") => layer(id, [en, de], kind, material, thicknessM, res, cap, SRC, reference);

export const region = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, layers, heatCapacityJM2K, extra = {}, reference = "incropera-2018") => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor: 0.4,
  layers,
  heatCapacityJM2K,
  ...extra,
  source: SRC,
  reference,
});

export const wallContacts = (wall, d) => wall.map((anchor) => ({ anchor, fromS: 0, toS: d }));

export const supportAnchors = (bone, top, bottom, titles, rollerNormalDeg = 0) => [
  { id: top.id, bone, x: top.x, y: top.y, kind: "support", title: tx(...titles[0]), support: "pin", normalDeg: 0, footprintLengthM: 0.016, footprintWidthM: 0.016 },
  { id: bottom.id, bone, x: bottom.x, y: bottom.y, kind: "support", title: tx(...titles[1]), support: "roller", normalDeg: rollerNormalDeg, footprintLengthM: 0.016, footprintWidthM: 0.016, frictionCoefficient: 0.4 },
];
