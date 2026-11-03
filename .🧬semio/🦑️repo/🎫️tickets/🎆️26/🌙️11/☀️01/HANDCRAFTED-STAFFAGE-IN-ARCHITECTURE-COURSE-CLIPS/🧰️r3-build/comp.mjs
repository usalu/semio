import { ALL_EXPORTS, authored, layer, pose, ref, tx, wrap } from "../🧰️a10/lib.mjs";

export { ALL_EXPORTS, authored, ref, tx, wrap };

const r5 = (n) => Math.round(n * 1e5) / 1e5;
const SRC = "authored-estimate";
export const WINTER = { airTempC: 0, meanRadiantTempC: 0, skyTempC: -12, groundTempC: 2, relativeHumidity: 0.8, windMS: 3, windDirectionDeg: 0, solarWM2: 300, diffuseFraction: 0.25, pressurePa: 101325 };
export const REFS = {
  iso6946: ref("en-iso-6946", "EN ISO 6946:2017 Building components and building elements, thermal resistance and thermal transmittance (surface resistances R_si 0.13 and R_se 0.04 m2K/W for horizontal heat flow, R_si 0.10 for upward heat flow)."),
  iso10456: ref("en-iso-10456", "EN ISO 10456:2007 Building materials and products, hygrothermal properties (design thermal conductivity, specific heat capacity: solid brick 0.75 W/mK, lime-cement plaster 0.70 W/mK, mineral wool 0.035 W/mK)."),
  iso10077: ref("en-iso-10077-1", "EN ISO 10077-1:2017 Thermal performance of windows, doors and shutters, calculation of thermal transmittance, part 1 (U_g, U_f, U_D)."),
  din1356: ref("din-1356-1", "DIN 1356-1:1995 Building drawings, part 1: types of drawings, contents and basic principles of presentation (scales, line groups, symbols for walls, doors, windows)."),
  din128: ref("din-iso-128-50", "DIN ISO 128-50:2002 Technical drawings, general principles of presentation, part 50: basic conventions for areas on sections and cuts (hatching)."),
  gebG: ref("geg-2024", "GEG 2024 Gebaeudeenergiegesetz, Anlage 7: maximum U-values of renovated components (exterior wall 0.24, roof 0.20, windows 1.3, exterior doors 1.8 W/m2K)."),
  en12898: ref("en-12898", "EN 12898:2019 Glass in building, determination of the emissivity (uncoated float glass 0.837; hard low-e coating 0.04 on face 3)."),
  incropera: ref("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties, convection, thermal properties of building materials)."),
  en1856: ref("en-1856-1", "EN 1856-1:2009 Chimneys, requirements for metal chimneys, part 1: system chimney products (stainless liner, insulated shaft, flue gas temperature classes T200 to T600)."),
  din18160: ref("din-18160-1", "DIN 18160-1:2006 Flues and chimneys, design and construction (masonry shaft, clear flue width, insulation)."),
};

export const mat = (id, en, de, emissivity, o) => ({ id, title: tx(en, de), emissivity, source: SRC, ...o });
export const lay = (id, en, de, kind, material, d, lambda, rho, c, reference = "en-iso-10456", source = "literature") =>
  layer(id, [en, de], kind, material, d, r5(d / lambda), r5(rho * c * d), source, reference);
export const rawLay = (id, en, de, kind, material, d, resistance, capacity, reference, source = SRC) =>
  layer(id, [en, de], kind, material, d, resistance, capacity, source, reference);

export const region = (o) => ({
  id: o.id,
  title: tx(...o.title),
  kind: o.kind ?? "wall",
  model: o.model ?? "conditioned",
  material: o.material,
  areaM2: o.areaM2,
  characteristicLengthM: o.length,
  geometry: o.geometry ?? "vertical-plate",
  skyViewFactor: o.sky ?? 0.5,
  layers: o.layers,
  heatCapacityJM2K: Math.round(o.layers.reduce((s, l) => s + (l.heatCapacityJM2K ?? 0), 0)),
  ...(o.model === "passive" ? {} : { conditioned: { interiorTempC: o.interiorC ?? 20, interiorFilmM2KW: o.film ?? 0.13 } }),
  source: o.source ?? SRC,
  reference: o.reference ?? "en-iso-6946",
});

export const uValue = (layers, rsi = 0.13, rse = 0.04) => 1 / (rsi + layers.reduce((s, l) => s + l.resistanceM2KW, 0) + rse);

const shape = (kind, id, role, z, tone, o, extra) => ({ id, bone: "root", role, z, filled: o.filled ?? role === "body", tone, ...(o.region ? { region: o.region } : {}), ...(o.material ? { material: o.material } : {}), ...(o.hatch ? { hatch: o.hatch } : {}), ...(o.penMm ? { penMm: o.penMm } : {}), kind, ...extra });
export const rect = (id, role, z, tone, x, y, w, h, o = {}) => shape("rect", id, role, z, tone, o, { x: r5(x), y: r5(y), w: r5(w), h: r5(h) });
export const poly = (id, role, z, tone, points, closed, o = {}) => shape("poly", id, role, z, tone, { ...o, filled: o.filled ?? (closed && role === "body") }, { points: points.map(([x, y]) => [r5(x), r5(y)]), closed });
export const line = (id, role, z, tone, a, b, o = {}) => poly(id, role, z, tone, [a, b], false, { ...o, filled: false });

export const PENS = {
  50: { body: 0.35, detail: 0.18, texture: 0.1, glazing: null, ground: 0.5 },
  100: { body: 0.25, detail: 0.13, texture: null, glazing: null, ground: 0.35 },
  200: { body: 0.18, detail: 0.1, texture: null, glazing: null, ground: 0.25 },
  500: { body: 0.13, detail: null, texture: null, glazing: null, ground: 0.18 },
};
export const RANGES = { 50: [20, 75], 100: [75, 150], 200: [150, 350], 500: [350, 1000] };
export const hatch = (id, en, de, pattern, angleDeg, spacingMm) => ({ id, title: tx(en, de), pattern, angleDeg, spacingMm, penMm: 0.1 });

export const rep = (den, hatches, elevation, dec, pens = PENS[den]) => ({ ...authored(den, RANGES[den], pens, elevation, dec), hatches });

export const unav = (den, reason, feature, size, alternatives) => ({ status: "unavailable", denominator: den, cause: "below-printable-feature", reason: tx(...reason), limitingFeature: tx(...feature), limitingSizeM: size, alternatives });

export const mergeLayers = (layers, den, minMm = 0.25) => {
  const groups = layers.map((l) => ({ ...l, ids: [l.id], d: l.d }));
  for (let changed = true; changed; ) {
    changed = false;
    for (let i = 0; i < groups.length && groups.length > 1; i += 1) {
      if ((groups[i].d * 1000) / den >= minMm) continue;
      const into = i === 0 ? 1 : i === groups.length - 1 ? i - 1 : groups[i - 1].d >= groups[i + 1].d ? i - 1 : i + 1;
      const keep = groups[into];
      const lo = Math.min(into, i);
      const merged = { ...keep, ids: [...groups[Math.min(into, i)].ids, ...groups[Math.max(into, i)].ids], d: keep.d + groups[i].d, tone: keep.tone, hatch: keep.hatch, material: keep.material, id: keep.id };
      groups.splice(lo, 2, merged);
      changed = true;
      break;
    }
  }
  return groups;
};

export const ASSET_REFS = (...list) => list;

export const baseAsset = (c) => {
  const regions = c.regions;
  return {
    id: c.id,
    category: "components",
    title: tx(...c.title),
    summary: tx(...c.summary),
    tags: c.tags,
    provenance: { author: "Semio staffage component authors (W11 build)", method: "handcrafted", created: "2026-11-03", references: c.references, notes: tx(...c.notes) },
    dimensions: c.dimensions,
    materials: c.materials,
    rig: {
      kind: "structural",
      root: "root",
      bones: [{ id: "root", parent: null, title: tx(...c.rootTitle), x: 0, y: 0, angleDeg: 0, lengthM: c.dimensions.widthM, joint: { type: "fixed" }, mass: { kg: c.dimensions.massKg, comX: r5(c.dimensions.widthM / 2), comY: r5(c.dimensions.heightM / 2), distribution: "point", source: SRC } }],
    },
    anchors: [
      { id: "base", bone: "root", x: r5(c.dimensions.widthM / 2), y: 0, kind: "support", title: tx("Base bearing, fixed", "Fußauflager, eingespannt"), support: "fixed" },
      ...(c.anchors ?? []),
    ],
    representations: c.representations,
    symbols: [],
    poses: c.poses.map((p) =>
      pose(
        p.id,
        p.title,
        p.description,
        {},
        ["base"],
        "authored",
        { regions: Object.fromEntries(regions.map((r) => [r.id, { exposedFraction: 1, sunlitFraction: p.sunlit[r.id] ?? p.sunlit.default ?? 0 }])), emitters: p.emitters ?? {} },
      ),
    ),
    clips: c.clips,
    thermal: { environment: WINTER, passengers: [], operating: [], regions, emitters: c.emitters ?? [], assumptions: c.assumptions },
    structure: { gravityMS2: 9.81, dynamics: "quasi-static", supports: ["base"], assumptions: c.structure, clinical: false },
    presentation: { defaultPose: c.poses[0].id, defaultClip: c.clips[0].id, defaultScale: 50 },
    exports: ALL_EXPORTS,
    completion: "authored",
  };
};

export const shadeClip = (regionIds, pose) => ({
  id: "cloud-shadow",
  title: tx("Cloud shadow", "Wolkenschatten"),
  description: tx("A cloud shadow crosses the sunlit face: the surface temperature falls and recovers with the heat capacity of the outer layers.", "Ein Wolkenschatten wandert über die besonnte Fläche: die Oberflächentemperatur sinkt und erholt sich mit der Wärmekapazität der äußeren Schichten."),
  durationS: 8,
  fps: 30,
  loop: true,
  rootMotion: "none",
  keys: [{ t: 0, pose, ease: "smooth" }, { t: 8, pose, ease: "smooth" }],
  contacts: [{ anchor: "base", fromS: 0, toS: 8 }],
  phases: [{ id: "shadow", fromS: 0, toS: 8, title: tx("Cloud shadow passes", "Wolkenschatten zieht durch") }],
  events: [{ t: 4, kind: "thermal", title: tx("Deepest shade", "Tiefster Schatten") }],
  tracks: regionIds.map((id) => ({ target: `region:${id}:sunlitFraction`, ease: "smooth", keys: [{ t: 0, v: 1 }, { t: 4, v: 0.1 }, { t: 8, v: 1 }] })),
});

export const sunClip = (regionIds, firstPose, lastPose) => ({
  id: "sunrise",
  title: tx("Sunrise on the face", "Sonnenaufgang auf der Fläche"),
  description: tx("The shaded face turns into the sunlit face: the surface temperature follows the absorbed solar radiation with the heat capacity of the outer layers.", "Die beschattete Fläche wird zur besonnten: die Oberflächentemperatur folgt der absorbierten Solarstrahlung mit der Wärmekapazität der äußeren Schichten."),
  durationS: 8,
  fps: 30,
  loop: false,
  rootMotion: "none",
  keys: [{ t: 0, pose: firstPose, ease: "smooth" }, { t: 8, pose: lastPose, ease: "smooth" }],
  contacts: [{ anchor: "base", fromS: 0, toS: 8 }],
  phases: [{ id: "sun", fromS: 0, toS: 8, title: tx("Sun reaches the face", "Sonne erreicht die Fläche") }],
  events: [{ t: 4, kind: "thermal", title: tx("Face in half sun", "Fläche in halber Sonne") }],
  tracks: regionIds.map((id) => ({ target: `region:${id}:sunlitFraction`, ease: "smooth", keys: [{ t: 0, v: 0 }, { t: 8, v: 1 }] })),
});
