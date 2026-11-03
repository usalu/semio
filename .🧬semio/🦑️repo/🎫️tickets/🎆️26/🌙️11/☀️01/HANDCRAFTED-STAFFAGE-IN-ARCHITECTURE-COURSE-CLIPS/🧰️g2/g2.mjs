import { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap } from "./lib.mjs";
import { materialOf, space, sum } from "./space.mjs";

export { ALL_EXPORTS, INCROPERA, authored, clip, layer, level, pose, ref, tx, unavailable, wrap, materialOf, space, sum };

export const SRC = "authored-estimate";
export const m = (kg, comX, comY) => ({ kg, comX, comY, distribution: "point", source: SRC });
export const fixed = { type: "fixed" };

export const space2 = (defs) => {
  const sp = space(defs);
  const r5 = (n) => Math.round(n * 1e5) / 1e5;
  const bones = sp.bones.map((b) => {
    if (!b.mass) return b;
    const [ax, ay] = sp.abs[b.id];
    return { ...b, mass: { ...b.mass, comX: r5(b.mass.comX - ax), comY: r5(b.mass.comY - ay) } };
  });
  return { ...sp, bones, anchor: (id, bone, x, y, kind, title, extra = {}) => ({ id, bone, x: sp.local(bone, x, y)[0], y: sp.local(bone, x, y)[1], kind, title: tx(...title), ...extra }) };
};
export const ASHRAE = ref("ashrae-2021", "ASHRAE (2021). Handbook - Fundamentals, SI edition, Atlanta (heat transfer, psychrometrics, refrigeration cycles and air-conditioning components).");
export const EN_14511 = ref("en-14511-2022", "DIN EN 14511:2022 Air conditioners, liquid chilling packages and heat pumps for space heating and cooling - Terms, test conditions and performance (rating conditions A7/W35 and A35/W7).");
export const EN_308 = ref("en-308-1997", "DIN EN 308:1997 Heat exchangers - Test procedures for establishing performance of air to air and flue gases heat recovery devices.");
export const VDI_3803 = ref("vdi-3803-2010", "VDI 3803 Part 1:2010 Air-conditioning - Central air-handling units - Structural and technical principles.");

export const reg = (id, title, kind, model, material, areaM2, characteristicLengthM, geometry, thicknessM, resistance, cap, extra = {}, o = {}) => ({
  id,
  title: tx(...title),
  kind,
  model,
  material,
  areaM2,
  characteristicLengthM,
  geometry,
  skyViewFactor: o.sky ?? 0.5,
  layers: [layer(`${id}-skin`, o.layerTitle ?? title, o.layerKind ?? "metal", material, thicknessM, resistance, cap, o.source ?? SRC, o.reference ?? "incropera-2018")],
  heatCapacityJM2K: cap,
  ...extra,
  source: o.source ?? SRC,
  reference: o.reference ?? "incropera-2018",
});

export const bindOf = (regionDefaults) => (operating, emitters = {}, over = {}) => ({
  regions: Object.fromEntries(
    Object.entries(regionDefaults).map(([id, d]) => [id, { sunlitFraction: 0, ...d, ...(over[id] ?? {}) }]),
  ),
  emitters,
  operating,
});

export const dec = (silhouette, retained, merged, omitted, thinElements, overlays, pens, hatching) => ({
  silhouette,
  retained,
  merged,
  omitted,
  thinElements,
  hatching: hatching ?? ["None.", "Keine."],
  styles: [
    `Darker tones carry hot or heavy parts, lighter tones thin parts; body pen ${pens.body} mm, detail ${pens.detail ?? "none"} mm, ground ${pens.ground} mm.`,
    `Dunklere Töne tragen heiße oder schwere Teile, hellere Töne dünne Teile; Körperstift ${pens.body} mm, Detail ${pens.detail ?? "keiner"} mm, Boden ${pens.ground} mm.`,
  ],
  overlays,
});

export const u500 = (featureEn, featureDe, sizeM, widthMm, alt = [200, 100]) =>
  unavailable(
    500,
    [`At 1:500 the asset is only ${widthMm} mm wide on paper and its smallest part, ${featureEn}, is far below every print profile.`, `Bei 1:500 ist das Gerät nur ${widthMm} mm breit auf Papier und sein kleinstes Teil, ${featureDe}, liegt weit unter jedem Druckprofil.`],
    [featureEn, featureDe],
    sizeM,
    alt.map((denominator) => ({ kind: "scale", denominator })),
  );

export const rep = (denominator, validRange, pens, shapes, d) => authored(denominator, validRange, pens, shapes, d);

export const mkAsset = (a) => {
  const massKg = Math.round(sum(a.bones.filter((b) => b.mass).map((b) => b.mass.kg)) * 1e6) / 1e6;
  const supports = a.supports ?? ["base-contact"];
  return wrap({
    id: a.id,
    category: a.category,
    title: tx(...a.title),
    summary: tx(...a.summary),
    tags: a.tags,
    provenance: {
      author: "Semio staffage content (G2)",
      method: "handcrafted",
      created: "2026-11-01",
      references: [...a.references, INCROPERA],
      notes: tx(...a.notes),
    },
    dimensions: { heightM: a.dims[0], widthM: a.dims[1], depthM: a.dims[2], massKg },
    materials: a.materials,
    rig: { kind: "mechanical", root: a.root, bones: a.bones.map((b) => ({ ...b, ...(b.mass ? { mass: { ...b.mass } } : {}) })) },
    anchors: a.anchors,
    representations: { 500: a.reps[500], 200: a.reps[200], 100: a.reps[100], 50: a.reps[50] },
    symbols: [],
    poses: a.poses,
    clips: a.clips,
    thermal: {
      environment: { airTempC: 21, meanRadiantTempC: 21, relativeHumidity: 0.5, windMS: 0.1, windDirectionDeg: 0, solarWM2: 0, diffuseFraction: 0, pressurePa: 101325 },
      passengers: [],
      operating: a.operating,
      regions: a.regions,
      emitters: a.emitters,
      assumptions: a.assumptions.map((t) => tx(...t)),
    },
    structure: { gravityMS2: 9.80665, dynamics: "quasi-static", supports, assumptions: a.structure.map((t) => tx(...t)), clinical: false },
    presentation: { defaultPose: a.defaultPose, defaultClip: a.defaultClip, defaultScale: a.defaultScale ?? 50, viewpoints: ["elevation"] },
    exports: ALL_EXPORTS,
    completion: "authored",
  });
};

export const baseContact = (bone, x, y, lenM, widM) => ({ id: "base-contact", bone, x, y, kind: "contact", title: tx("Base on the surface", "Fuß auf der Unterlage"), support: "contact", footprintLengthM: lenM, footprintWidthM: widM, frictionCoefficient: 0.4 });
export const contacts = (supports, d) => supports.map((anchor) => ({ anchor, fromS: 0, toS: d }));

export const un = (denominator, featureEn, featureDe, sizeM, widthMm, alt = [100, 50]) =>
  unavailable(
    denominator,
    [`At 1:${denominator} the asset is only ${widthMm} mm wide on paper and its smallest part, ${featureEn}, is far below every print profile.`, `Bei 1:${denominator} ist das Gerät nur ${widthMm} mm breit auf Papier und sein kleinstes Teil, ${featureDe}, liegt weit unter jedem Druckprofil.`],
    [featureEn, featureDe],
    sizeM,
    alt.map((d) => ({ kind: "scale", denominator: d })),
  );
