export const t = (en, de) => ({ en, de });
export const r4 = (x) => Math.round(x * 1e4) / 1e4;

export function mat(id, title, eps, alpha, rho, k, cp, source, reference) {
  return { id, title, emissivity: eps, solarAbsorptivity: alpha, densityKgM3: rho, conductivityWmK: k, specificHeatJKgK: cp, source, reference };
}

export function layer(id, title, kind, m, d, source = "authored-estimate", reference) {
  const out = { id, title, kind, material: m.id, thicknessM: d, resistanceM2KW: Number((d / m.conductivityWmK).toPrecision(5)), heatCapacityJM2K: Math.round(m.densityKgM3 * m.specificHeatJKgK * d), source };
  if (reference) out.reference = reference;
  return out;
}

export function region(o) {
  const layers = o.layers ?? [];
  const c = o.heatCapacityJM2K ?? layers.reduce((a, l) => a + l.heatCapacityJM2K, 0);
  const out = { id: o.id, title: o.title, kind: o.kind, model: o.model, material: o.material.id, areaM2: o.areaM2, characteristicLengthM: o.L, geometry: o.geometry, skyViewFactor: o.svf, layers, heatCapacityJM2K: c };
  if (o.operating) out.operating = o.operating;
  out.source = o.source ?? "authored-estimate";
  if (o.reference) out.reference = o.reference;
  return out;
}

const base = (id, bone, role, z, filled, tone, geom, o = {}) => {
  const s = { id, bone, role, z, filled, tone, ...geom };
  if (o.region) s.region = o.region;
  if (o.regions) s.regions = o.regions;
  if (o.material) s.material = o.material;
  if (o.hatch) s.hatch = o.hatch;
  if (o.penMm) s.penMm = o.penMm;
  return s;
};
export const path = (id, bone, z, tone, d, o) => base(id, bone, "body", z, true, tone, { kind: "path", d }, o);
export const rect = (id, bone, z, tone, x, y, w, h, o = {}) => base(id, bone, o.role ?? "body", z, o.filled ?? true, tone, o.rr ? { kind: "rect", x, y, w, h, r: o.rr } : { kind: "rect", x, y, w, h }, o);
export const ell = (id, bone, z, tone, cx, cy, rx, ry, o = {}) => base(id, bone, o.role ?? "body", z, o.filled ?? true, tone, { kind: "ellipse", cx, cy, rx, ry }, o);
export const cap = (id, bone, z, tone, x0, y0, x1, y1, r0, r1, o = {}) => base(id, bone, o.role ?? "body", z, o.filled ?? true, tone, { kind: "capsule", x0, y0, x1, y1, r0, r1 }, o);
export const line = (id, bone, z, pts, o = {}) => base(id, bone, o.role ?? "detail", z, false, o.tone ?? 0.3, { kind: "poly", points: pts, closed: o.closed ?? false }, o);
export const tex = (id, bone, z, pts, o = {}) => line(id, bone, z, pts, { ...o, role: "texture" });

export function rep(den, minD, maxD, pens, hatches, elevation, decisions) {
  return { status: "authored", denominator: den, validRange: { minDenominator: minD, maxDenominator: maxD }, pens, hatches, geometry: { elevation }, decisions };
}

export function unavailable(den, reason, limitingFeature, limitingSizeM, alternatives) {
  return { status: "unavailable", denominator: den, reason, limitingFeature, limitingSizeM, alternatives };
}

export function regionBindings(ids, defaults, overrides = {}) {
  const out = {};
  for (const id of ids) out[id] = { exposedFraction: 1, sunlitFraction: 0, ...defaults, ...(overrides[id] ?? {}) };
  return out;
}

export function operatingKeyDef(id, title, levels) {
  return { id, title, levels: levels.map(([v, en, de]) => ({ v, title: t(en, de) })) };
}

export function emitter(o) {
  const e = { id: o.id, kind: o.kind, title: o.title, anchor: o.anchor, directionDeg: o.dir, diameterM: o.d, operatingKey: o.key, volumeFlowM3S: o.flow, temperatureRiseK: o.rise, vapourKgPerKg: o.vapour, source: o.source ?? "authored-estimate" };
  if (o.region) e.region = o.region;
  if (o.reference) e.reference = o.reference;
  for (const k of Object.keys(e)) if (e[k] === undefined) delete e[k];
  return e;
}
