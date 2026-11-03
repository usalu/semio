import { ellipse, path, poly, rect } from "./lib.mjs";

const R = 0.37;
const AXLE_F = 1.5;
const AXLE_R = -1.5;

export const capsule = (id, bone, role, z, filled, tone, x0, y0, x1, y1, r0, r1, o = {}) => ({
  kind: "capsule",
  x0,
  y0,
  x1,
  y1,
  r0,
  r1,
  id,
  bone,
  role,
  z,
  filled,
  tone,
  ...(o.region ? { region: o.region } : {}),
  ...(o.regions ? { regions: o.regions } : {}),
  ...(o.material ? { material: o.material } : {}),
  ...(o.penMm ? { penMm: o.penMm } : {}),
});

const r4 = (n) => Math.round(n * 1e4) / 1e4;
const polar = (r, deg) => [r4(r * Math.cos((deg * Math.PI) / 180)), r4(r * Math.sin((deg * Math.PI) / 180))];

const wheel = (sc, k) => {
  const bone = `wheel-${k}`;
  const out = [];
  const tyre = (o = {}) => ellipse(`tyre-${k}`, bone, "body", 4, true, 0.08, 0, 0, R, R, { region: "tyres", material: "rubber", ...o });
  if (sc === 500) {
    out.push(tyre({ regions: ["tyres", "brakes"], region: undefined }));
    out.push(capsule(`bar-${k}`, bone, "body", 5, true, 0.7, -0.2, 0, 0.2, 0, 0.09, 0.09, { material: "steel-rim" }));
    return out;
  }
  if (sc === 200) {
    out.push(tyre());
    out.push(ellipse(`rim-${k}`, bone, "detail", 4.2, true, 0.55, 0, 0, 0.25, 0.25, { regions: ["brakes"], material: "steel-rim" }));
    for (const a of [0, 90]) {
      const [x, y] = polar(0.2, a);
      out.push(capsule(`spoke-${k}-${a}`, bone, "detail", 5, true, 0.2, -x, -y, x, y, 0.03, 0.03, { material: "steel-rim" }));
    }
    out.push(ellipse(`hub-${k}`, bone, "detail", 5.2, true, 0.8, 0, 0, 0.07, 0.07, { material: "nickel-plate" }));
    return out;
  }
  const n = sc === 100 ? 8 : 12;
  const spokeR = sc === 100 ? 0.016 : 0.011;
  out.push(tyre());
  if (sc === 50) out.push(ellipse(`bead-${k}`, bone, "detail", 4.1, false, 0.3, 0, 0, 0.3, 0.3, { material: "rubber" }));
  out.push(ellipse(`rim-${k}`, bone, "detail", 4.2, true, 0.55, 0, 0, 0.25, 0.25, { material: "steel-rim" }));
  out.push(ellipse(`drum-${k}`, bone, "detail", 4.4, true, 0.3, 0, 0, 0.17, 0.17, { region: "brakes", material: "cast-iron" }));
  for (let i = 0; i < n; i++) {
    const a = 90 + (360 / n) * i;
    const [x0, y0] = polar(0.075, a);
    const [x1, y1] = polar(0.245, a);
    out.push(capsule(`spoke-${k}-${i + 1}`, bone, "detail", 5, true, 0.7, x0, y0, x1, y1, spokeR, spokeR, { material: "steel-rim" }));
  }
  out.push(ellipse(`hub-${k}`, bone, "detail", 5.2, true, 0.8, 0, 0, 0.065, 0.065, { material: "nickel-plate" }));
  const [vx0, vy0] = polar(0.236, 22);
  const [vx1, vy1] = polar(0.3, 22);
  out.push(capsule(`valve-${k}`, bone, "detail", 5.3, true, 0.9, vx0, vy0, vx1, vy1, 0.009, 0.009, { material: "steel-rim" }));
  return out;
};

const FRONT_FENDER = "M 0.96,0.47 C 1.00,0.68 1.20,0.85 1.50,0.85 C 1.84,0.85 2.04,0.72 2.14,0.52 L 2.11,0.47 C 2.02,0.64 1.80,0.80 1.50,0.80 C 1.22,0.80 1.05,0.64 1.01,0.45 Z";
const REAR_FENDER = "M -0.98,0.47 C -0.96,0.70 -1.15,0.85 -1.50,0.85 C -1.80,0.85 -1.98,0.72 -2.03,0.52 L -2.00,0.47 C -1.96,0.62 -1.78,0.80 -1.50,0.80 C -1.22,0.80 -1.03,0.64 -1.01,0.45 Z";
const TUB = "M -1.90,0.52 L 0.60,0.52 L 0.60,1.14 L -1.86,1.14 C -1.95,1.14 -1.99,1.06 -1.99,0.96 L -1.99,0.62 C -1.99,0.56 -1.96,0.52 -1.90,0.52 Z";
const CABIN = "M 0.60,1.14 L 0.50,1.62 C 0.49,1.68 0.52,1.70 0.58,1.70 L -1.50,1.70 C -1.60,1.70 -1.66,1.66 -1.70,1.58 L -1.84,1.14 Z";
const HOOD = "M 0.62,0.58 L 0.62,1.17 C 0.95,1.215 1.45,1.215 1.74,1.18 C 1.77,1.175 1.78,1.16 1.78,1.14 L 1.78,0.58 Z";

export function elevation(sc) {
  const c = "chassis";
  const s = [];
  if (sc === 500) {
    s.push(path("underbody", c, "body", 1, true, 0.25, "M -2.145,0.62 L -2.145,0.4 L 2.245,0.4 L 2.245,0.62 Z", { regions: ["exhaust-system"], material: "exhaust-steel" }));
    s.push(path("silhouette", c, "body", 2, true, 0.55, "M -1.99,0.5 L -1.99,1.1 L -1.82,1.14 L -1.7,1.58 C -1.6,1.68 -1.5,1.7 -1.4,1.7 L 0.58,1.7 L 0.5,1.58 L 0.62,1.18 C 1.0,1.22 1.5,1.22 1.88,1.2 L 1.9,0.5 L 2.15,0.5 L 2.15,0.62 C 2.0,0.88 1.8,0.88 1.5,0.88 C 1.2,0.88 1.05,0.85 0.96,0.5 Z", { regions: ["body-panels", "hood", "fenders", "radiator", "engine-bay", "roof"], material: "paint" }));
    s.push(path("windows", c, "glazing", 6, true, 0.85, "M -1.62,1.22 L -1.62,1.55 L 0.42,1.55 L 0.5,1.22 Z", { region: "glazing", material: "glass" }));
    s.push(...wheel(500, "f"), ...wheel(500, "r"));
    return s;
  }
  const hi = sc <= 100;
  const fine = sc === 50;
  s.push(capsule("frame", c, "body", 1, true, 0.18, -1.95, 0.38, 2.05, 0.38, 0.03, 0.03, { material: "exhaust-steel" }));
  s.push(rect("running-board", c, "body", 1.5, true, 0.42, -1, 0.4, 2, 0.06, { region: "fenders", material: "paint" }));
  s.push(path("tub", c, "body", 2, true, 0.6, TUB, { region: "body-panels", material: "paint" }));
  s.push(path("cabin", c, "body", 2.1, true, 0.58, CABIN, { region: "body-panels", material: "paint" }));
  s.push(path("roof-slab", c, "detail", 2.4, true, 0.78, "M 0.62,1.63 L 0.62,1.7 L -1.5,1.7 C -1.6,1.7 -1.66,1.66 -1.7,1.58 L -1.52,1.63 Z", { region: "roof", material: "fabric-roof" }));
  s.push(path("hood", c, "body", 2.3, true, 0.62, HOOD, { region: "hood", material: "paint" }));
  s.push(path("fender-front", c, "body", 3, true, 0.5, FRONT_FENDER, { region: "fenders", material: "paint" }));
  s.push(path("fender-rear", c, "body", 3, true, 0.5, REAR_FENDER, { region: "fenders", material: "paint" }));
  s.push(rect("radiator-shell", c, "body", 2.6, true, 0.78, 1.78, 0.56, 0.1, 0.7, { r: 0.03, region: "radiator", material: "nickel-plate" }));
  s.push(capsule("radiator-neck", c, "detail", 2.7, true, 0.78, 1.83, 1.26, 1.83, 1.3, 0.018, 0.018, { material: "nickel-plate" }));
  s.push(ellipse("radiator-cap", c, "detail", 2.8, true, 0.78, 1.83, 1.335, 0.05, 0.032, { material: "nickel-plate" }));
  const louvres = hi ? (fine ? 10 : 6) : 0;
  for (let i = 0; i < louvres; i++) {
    const x = fine ? 0.92 + 0.07 * i : 0.95 + 0.12 * i;
    s.push(capsule(`louvre-${i + 1}`, c, "detail", 2.5, true, 0.12, x, 0.97, x, 1.12, fine ? 0.012 : 0.02, fine ? 0.012 : 0.02, { region: "engine-bay", material: "engine-metal" }));
  }
  if (!hi) s.push(rect("louvre-band", c, "detail", 2.5, true, 0.12, 0.95, 0.97, 0.7, 0.15, { region: "engine-bay", material: "engine-metal" }));
  s.push(ellipse("headlamp-bowl", c, "body", 3.2, true, 0.74, 1.78, 0.99, 0.105, 0.11, { material: "nickel-plate" }));
  s.push(ellipse("headlamp-lens", c, "glazing", 3.3, true, 0.88, 1.866, 0.99, 0.018, 0.085, { material: "glass" }));
  if (hi) s.push(capsule("lamp-stalk", c, "detail", 3.1, true, 0.5, 1.76, 0.8, 1.76, 0.9, 0.015, 0.015, { material: "nickel-plate" }));
  s.push(path("window-front", c, "glazing", 6, true, 0.85, "M 0.51,1.21 L 0.44,1.58 L -0.27,1.58 L -0.27,1.21 Z", { region: "glazing", material: "glass" }));
  s.push(path("window-rear", c, "glazing", 6, true, 0.85, "M -0.41,1.21 L -0.41,1.58 L -1.07,1.58 L -1.07,1.21 Z", { region: "glazing", material: "glass" }));
  s.push(path("window-quarter", c, "glazing", 6, true, 0.85, "M -1.17,1.21 L -1.17,1.58 L -1.62,1.58 L -1.74,1.21 Z", { region: "glazing", material: "glass" }));
  s.push(path("windscreen", c, "glazing", 6.1, false, 0.85, "M 0.575,1.17 L 0.47,1.6", { region: "glazing", material: "glass", penMm: sc === 200 ? 0.25 : 0.3 }));
  if (hi) {
    s.push(path("door-hinge-gap", c, "detail", 7, false, 0.5, "M 0.57,1.14 L 0.57,0.56"));
    s.push(path("door-gap-b", c, "detail", 7, false, 0.5, "M -0.34,1.14 L -0.34,0.56"));
    s.push(path("door-gap-rear", c, "detail", 7, false, 0.5, "M -1.12,1.14 L -1.12,0.56"));
    s.push(path("belt-line", c, "detail", 7, false, 0.5, "M -1.86,1.14 L 0.6,1.14"));
  }
  if (fine) {
    s.push(capsule("handle-front", c, "detail", 7.2, true, 0.8, -0.2, 1.04, -0.1, 1.04, 0.012, 0.012, { material: "nickel-plate" }));
    s.push(capsule("handle-rear", c, "detail", 7.2, true, 0.8, -0.5, 1.04, -0.4, 1.04, 0.012, 0.012, { material: "nickel-plate" }));
    s.push(path("seat-front", c, "detail", 6.5, true, 0.3, "M -0.02,1.21 L 0.0,1.44 L 0.04,1.5 L 0.12,1.5 L 0.16,1.44 L 0.18,1.21 Z"));
    s.push(path("seat-rear", c, "detail", 6.5, true, 0.3, "M -0.97,1.21 L -0.95,1.44 L -0.91,1.5 L -0.83,1.5 L -0.79,1.44 L -0.77,1.21 Z"));
    s.push(capsule("steering-column", c, "detail", 6.5, true, 0.3, 0.42, 1.24, 0.33, 1.36, 0.01, 0.01, { material: "exhaust-steel" }));
    s.push(ellipse("steering-wheel", c, "detail", 6.6, false, 0.3, 0.31, 1.39, 0.016, 0.1, { material: "exhaust-steel", rotationDeg: 0 }));
    s.push(rect("number-plate", c, "detail", 7, true, 0.9, -2.015, 0.58, 0.03, 0.12, { material: "paint" }));
    s.push(capsule("fender-lamp", c, "detail", 7, true, 0.85, 2.0, 0.66, 2.1, 0.58, 0.02, 0.02, { material: "nickel-plate" }));
  }
  s.push(rect("tail-lamp", c, "detail", 7, true, 0.3, -2.03, 0.9, 0.05, 0.1, { material: "paint" }));
  s.push(capsule("bumper-front", c, "body", 3.4, true, 0.8, 2.22, 0.66, 2.22, 0.4, 0.022, 0.022, { material: "nickel-plate" }));
  s.push(capsule("bumper-rear", c, "body", 3.4, true, 0.8, -2.12, 0.66, -2.12, 0.4, 0.022, 0.022, { material: "nickel-plate" }));
  if (hi) {
    s.push(capsule("bracket-front", c, "detail", 3.3, true, 0.6, 2.2, 0.5, 2.05, 0.43, 0.012, 0.012, { material: "exhaust-steel" }));
    s.push(capsule("bracket-rear", c, "detail", 3.3, true, 0.6, -2.1, 0.5, -1.95, 0.43, 0.012, 0.012, { material: "exhaust-steel" }));
  }
  s.push(capsule("exhaust-pipe", c, "detail", 0.8, true, 0.35, 0.9, 0.27, -1.05, 0.27, 0.03, 0.03, { region: "exhaust-system", material: "exhaust-steel" }));
  s.push(capsule("silencer", c, "detail", 0.9, true, 0.4, -1.1, 0.27, -1.75, 0.27, 0.075, 0.075, { region: "exhaust-system", material: "exhaust-steel" }));
  s.push(capsule("tail-pipe", c, "detail", 0.9, true, 0.45, -1.75, 0.27, -2.08, 0.25, 0.03, 0.025, { region: "exhaust-system", material: "exhaust-steel" }));
  s.push(ellipse("spare-tyre", c, "body", 3.6, true, 0.08, 0.86, 0.78, 0.31, 0.31, { region: "tyres", material: "rubber" }));
  s.push(ellipse("spare-cover", c, "detail", 3.7, true, 0.62, 0.86, 0.78, 0.23, 0.23, { material: "paint" }));
  if (hi) s.push(ellipse("spare-ring", c, "detail", 3.65, false, 0.3, 0.86, 0.78, 0.27, 0.27, { material: "rubber" }));
  s.push(ellipse("spare-boss", c, "detail", 3.8, true, 0.8, 0.86, 0.78, 0.04, 0.04, { material: "nickel-plate" }));
  s.push(...wheel(sc, "f"), ...wheel(sc, "r"));
  return s;
}

const FP = "M 0.96,0.52 L 0.96,0.80 C 1.05,0.85 1.30,0.86 1.55,0.86 C 1.85,0.86 2.05,0.78 2.12,0.68 C 2.15,0.62 2.15,0.55 2.13,0.52 Z";
const RP = "M -0.98,0.52 L -0.98,0.78 C -1.05,0.84 -1.25,0.85 -1.50,0.85 C -1.80,0.85 -1.96,0.80 -2.02,0.70 L -2.03,0.52 Z";

const mirrorPath = (d) => d.replace(/(-?\d+\.?\d*),(-?\d+\.?\d*)/g, (_, x, y) => `${x},${r4(-parseFloat(y))}`);

export function plan(sc) {
  const c = "chassis";
  const s = [];
  if (sc === 500) {
    s.push(path("plan-silhouette", c, "body", 1, true, 0.55, "M -2.1,-0.74 L -2.1,0.74 L -1.99,0.85 L 0.9,0.85 L 1.5,0.86 L 2.12,0.7 L 2.25,0.74 L 2.25,-0.74 L 2.12,-0.7 L 1.5,-0.86 L 0.9,-0.85 L -1.99,-0.85 Z", { regions: ["body-panels", "hood", "fenders", "radiator"], material: "paint" }));
    s.push(path("plan-roof", c, "body", 2, true, 0.75, "M 0.5,-0.55 L -1.7,-0.55 L -1.7,0.55 L 0.5,0.55 Z", { region: "roof", material: "fabric-roof" }));
    return s;
  }
  const hi = sc <= 100;
  const fine = sc === 50;
  s.push(path("plan-tub", c, "body", 1, true, 0.6, "M -1.90,-0.74 L 0.60,-0.74 L 0.60,0.74 L -1.90,0.74 C -1.96,0.74 -1.99,0.70 -1.99,0.64 L -1.99,-0.64 C -1.99,-0.70 -1.96,-0.74 -1.90,-0.74 Z", { region: "body-panels", material: "paint" }));
  s.push(path("plan-roof", c, "body", 2, true, 0.78, "M 0.50,-0.62 L -1.58,-0.62 C -1.66,-0.62 -1.70,-0.56 -1.70,-0.48 L -1.70,0.48 C -1.70,0.56 -1.66,0.62 -1.58,0.62 L 0.50,0.62 Z", { region: "roof", material: "fabric-roof" }));
  s.push(rect("plan-windscreen", c, "glazing", 2.2, true, 0.88, 0.5, -0.58, 0.1, 1.16, { region: "glazing", material: "glass" }));
  s.push(rect("plan-rear-glass", c, "glazing", 2.2, true, 0.88, -1.82, -0.46, 0.12, 0.92, { region: "glazing", material: "glass" }));
  s.push(path("plan-hood", c, "body", 2, true, 0.62, "M 0.64,-0.44 C 1.0,-0.43 1.5,-0.38 1.77,-0.3 L 1.77,0.3 C 1.5,0.38 1.0,0.43 0.64,0.44 Z", { region: "hood", material: "paint" }));
  if (hi) s.push(path("plan-hood-ridge", c, "detail", 2.1, false, 0.5, "M 0.64,0 L 1.77,0"));
  for (const sign of [1, -1]) {
    const n = sign > 0 ? "l" : "r";
    const m = (d) => (sign > 0 ? d : mirrorPath(d));
    s.push(rect(`plan-board-${n}`, c, "body", 2.5, true, 0.42, -0.98, sign > 0 ? 0.74 : -0.85, 1.94, 0.11, { region: "fenders", material: "paint" }));
    s.push(path(`plan-fender-front-${n}`, c, "body", 3, true, 0.5, m(FP), { region: "fenders", material: "paint" }));
    s.push(path(`plan-fender-rear-${n}`, c, "body", 3, true, 0.5, m(RP), { region: "fenders", material: "paint" }));
    s.push(rect(`plan-spare-${n}`, c, "body", 3.6, true, 0.1, 0.55, sign > 0 ? 0.68 : -0.82, 0.62, 0.14, { r: 0.06, region: "tyres", material: "rubber" }));
    s.push(ellipse(`plan-lamp-${n}`, c, "body", 3.2, true, 0.74, 1.78, sign * 0.55, 0.105, 0.105, { material: "nickel-plate" }));
    if (hi) s.push(capsule(`plan-lamp-bar-${n}`, c, "detail", 3.1, true, 0.5, 1.78, sign * 0.3, 1.78, sign * 0.46, 0.014, 0.014, { material: "nickel-plate" }));
  }
  s.push(rect("plan-radiator", c, "body", 2.6, true, 0.78, 1.77, -0.27, 0.11, 0.54, { r: 0.03, region: "radiator", material: "nickel-plate" }));
  if (hi) s.push(ellipse("plan-radiator-cap", c, "detail", 2.8, true, 0.78, 1.825, 0, 0.04, 0.04, { material: "nickel-plate" }));
  s.push(rect("plan-bumper-front", c, "body", 3.4, true, 0.8, 2.2, -0.74, 0.045, 1.48, { material: "nickel-plate" }));
  s.push(rect("plan-bumper-rear", c, "body", 3.4, true, 0.8, -2.145, -0.74, 0.045, 1.48, { material: "nickel-plate" }));
  if (fine) {
    s.push(path("plan-door-gap-b", c, "detail", 2.3, false, 0.5, "M -0.34,-0.74 L -0.34,-0.62 M -0.34,0.62 L -0.34,0.74"));
    s.push(path("plan-roof-seam", c, "detail", 2.3, false, 0.5, "M 0.5,0 L -1.7,0"));
  }
  return s;
}
