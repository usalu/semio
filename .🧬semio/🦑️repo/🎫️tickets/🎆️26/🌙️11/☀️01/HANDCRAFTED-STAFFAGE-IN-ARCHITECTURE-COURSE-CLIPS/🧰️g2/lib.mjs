export const tx = (en, de) => ({ en, de });
const r5 = (n) => Math.round(n * 1e5) / 1e5;
const num = (v) => (typeof v === "number" ? r5(v) : v);

const base = (id, bone, role, z, filled, tone, extra) => ({ id, bone, role, z, filled, tone, ...extra });
const tail = (o) => {
  const out = {};
  if (o.region) out.region = o.region;
  if (o.regions) out.regions = o.regions;
  if (o.material) out.material = o.material;
  if (o.penMm) out.penMm = o.penMm;
  return out;
};
export const rect = (id, bone, role, z, filled, tone, x, y, w, h, o = {}) => base(id, bone, role, z, filled, tone, { kind: "rect", x: num(x), y: num(y), w: num(w), h: num(h), ...(o.r ? { r: o.r } : {}), ...tail(o) });
export const path = (id, bone, role, z, filled, tone, d, o = {}) => base(id, bone, role, z, filled, tone, { kind: "path", d, ...tail(o) });
export const ellipse = (id, bone, role, z, filled, tone, cx, cy, rx, ry, o = {}) => base(id, bone, role, z, filled, tone, { kind: "ellipse", cx: num(cx), cy: num(cy), rx: num(rx), ry: num(ry), ...tail(o) });
export const poly = (id, bone, role, z, filled, tone, points, closed, o = {}) => base(id, bone, role, z, filled, tone, { kind: "poly", points: points.map(([x, y]) => [num(x), num(y)]), closed, ...tail(o) });

export const decisions = (d) => Object.fromEntries(Object.entries(d).map(([k, [en, de]]) => [k, tx(en, de)]));

export const unavailable = (denominator, reason, limitingFeature, limitingSizeM, alternatives) => ({
  status: "unavailable",
  denominator,
  reason: tx(...reason),
  limitingFeature: tx(...limitingFeature),
  limitingSizeM,
  alternatives,
});

export const authored = (denominator, validRange, pens, elevation, dec) => ({
  status: "authored",
  denominator,
  validRange: { minDenominator: validRange[0], maxDenominator: validRange[1] },
  pens,
  hatches: [],
  geometry: { elevation },
  decisions: decisions(dec),
});

export const bone = (id, parent, title, x, y, lengthM, joint, mass) => ({
  id,
  parent,
  title: tx(...title),
  x,
  y,
  angleDeg: 0,
  lengthM,
  joint,
  mass: { ...mass, distribution: "point" },
});

export const layer = (id, title, kind, material, thicknessM, resistanceM2KW, heatCapacityJM2K, source, reference) => ({
  id,
  title: tx(...title),
  kind,
  material,
  thicknessM,
  resistanceM2KW,
  heatCapacityJM2K,
  source,
  ...(reference ? { reference } : {}),
});

export const region = (r) => r;

export const pose = (id, title, description, bones, contacts, ground, thermal) => ({
  id,
  title: tx(...title),
  description: tx(...description),
  bones,
  ground,
  contacts,
  thermal,
  loads: [],
});

export const exposure = (map) => Object.fromEntries(Object.entries(map).map(([k, v]) => [k, v]));

export const clip = (id, title, description, durationS, fps, keys, contacts, phases, events, tracks) => ({
  id,
  title: tx(...title),
  description: tx(...description),
  durationS,
  fps,
  loop: false,
  rootMotion: "none",
  keys,
  contacts,
  phases: phases.map(([pid, fromS, toS, t]) => ({ id: pid, fromS, toS, title: tx(...t) })),
  events: events.map(([t, kind, title, target]) => ({ t, kind, title: tx(...title), ...(target ? { target } : {}) })),
  tracks: tracks.map(([target, ease, keysT]) => ({ target, ease, keys: keysT.map(([t, v]) => ({ t, v })) })),
});

export const level = (v, en, de) => ({ v, title: tx(en, de) });

export const ALL_EXPORTS = ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component"];

export const wrap = (asset) => ({ $schema: "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", version: "semio.staffage/v2", asset });

export const ref = (id, citation) => ({ id, citation });
export const INCROPERA = ref("incropera-2018", "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2018). Fundamentals of Heat and Mass Transfer, 8th ed., Wiley (radiative properties of surfaces, convection, plumes, thermal properties of metals, glass and polymers).");
export const ISO10456 = ref("iso-10456-2007", "ISO 10456:2007 Building materials and products - Hygrothermal properties - Tabulated design values and procedures for determining declared and design thermal values.");
export const WILKINS_HOSNI = ref("wilkins-hosni-2011", "Wilkins, C. K., Hosni, M. H. (2011). Plug load design factors. ASHRAE Journal 53(5), 30-34 (measured heat gains of laptops, displays and other office equipment in active, idle and sleep states).");
export const HOSNI_1999 = ref("hosni-1999", "Hosni, M. H., Jones, B. W., Xu, H. (1999). Experimental results for heat gain and radiant/convective split from equipment in buildings. ASHRAE Transactions 105(2), 527-539.");
export const VDI_2078 = ref("vdi-2078-2015", "VDI 2078:2015 Calculation of thermal loads and room temperatures (internal heat sources of office equipment and lighting).");
