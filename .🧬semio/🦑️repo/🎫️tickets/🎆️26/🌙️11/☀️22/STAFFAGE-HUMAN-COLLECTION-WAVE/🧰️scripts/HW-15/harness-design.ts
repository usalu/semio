/** 🦺️ Design inputs of the guide-dog harness of `person-with-guide-dog`: a 50 mm leather girth round the chest behind the elbows and a rigid handle bar with a padded grip from the top of the girth up to the handler's left hand; the dog placement beside the handler and the grip of the hand.
 * Guide-dog harness after common Y-front harnesses with a rigid handle (handle 0.4 to 0.5 m, held at hip height, dog on the handler's left); qualitative, dimensions authored.
 * @see ./gen-guide-dog.ts
 */
import { join } from "node:path";
import { S, SCALES, restFrames, toWorld, toLocal, v3, clone, type V3 } from "./lib.ts";
import { P } from "./dog-merge.ts";

const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));

/** 📐️ Placement and harness sizes (metres). */
export const HARNESS = {
  dogX: 0, dogZ: 0.34, dogShift: 0.02,
  girth: { u0: 0.06, u1: 0.11, clearance: 0.006 },
  handleStart: [0.085, 0, 0] as V3,
  gripWorld: [0.18, 0.88, 0.235] as V3, gripFromPelvis: [0.18, -0.031, 0.235] as V3,
  handleR: 0.01, gripR: 0.017, gripLength: 0.11, handleLength: 0,
  fingers: [0.85, -0.3, -0.05] as V3, palm: [0, -1, 0.15] as V3, gripAlongHand: 0.07, gripBelowPalm: 0.015,
  massGirthKg: 0.45, massHandleKg: 0.3,
};

const round = (x: number) => Math.round(x * 1e5) / 1e5;

/** 📏️ The chest loft section (centre v, half width rx along w, half height ry along v) of the dog at a position u of its thorax. */
function chestAt(lab: any, scale: string, u: number): { v: number; rx: number; ry: number } {
  const loft = lab.volumes[scale].body.find((s: any) => s.bone === "thorax" && s.kind === "loft");
  const sec = loft.sections;
  for (let i = 0; i + 1 < sec.length; i++) {
    const [a, b] = [sec[i], sec[i + 1]];
    if (u >= a.at[0] && u <= b.at[0]) {
      const f = (u - a.at[0]) / (b.at[0] - a.at[0] || 1);
      return { v: a.at[1] + f * (b.at[1] - a.at[1]), rx: a.rx + f * (b.rx - a.rx), ry: a.ry + f * (b.ry - a.ry) };
    }
  }
  throw new Error("[DEBUG] chest section");
}

/** 🔺️ Convex hull of 2D points (monotone chain). */
function hull(points: number[][]): number[][] {
  const p = [...points].sort((a, b) => a[0]! - b[0]! || a[1]! - b[1]!);
  const cross = (o: number[], a: number[], b: number[]) => (a[0]! - o[0]!) * (b[1]! - o[1]!) - (a[1]! - o[1]!) * (b[0]! - o[0]!);
  const lower: number[][] = [], upper: number[][] = [];
  for (const q of p) { while (lower.length >= 2 && cross(lower[lower.length - 2]!, lower[lower.length - 1]!, q) <= 0) lower.pop(); lower.push(q); }
  for (const q of p.reverse()) { while (upper.length >= 2 && cross(upper[upper.length - 2]!, upper[upper.length - 1]!, q) <= 0) upper.pop(); upper.push(q); }
  return lower.slice(0, -1).concat(upper.slice(0, -1)).map(([x, y]) => [round(x!), round(y!)]);
}

/** 💊️ A closed side profile symmetric about the bone axis from u0 to u1 with radius r0 to r1 (square ends). */
const rod = (u0: number, u1: number, r0: number, r1: number) => [[u0, -r0], [u1, -r1], [u1, r1], [u0, r0]].map(([u, v]) => [round(u!), round(v!)]);

/** 🦺️ Adds the handle bone, the girth and handle shapes of every view and scale, their solids, the harness region, material and masses. */
export function harness(a: any, lab: any, rest: { x: number; y: number; z: number }): void {
  const st = evaluate(lab, { kind: "pose", pose: "standing-alert" });
  const th = clone(st.bones["thorax"].frame);
  const shift: V3 = [rest.x + HARNESS.dogShift, 0, rest.z];
  for (const k of ["o"]) th[k] = { x: v3(th[k])[0] + shift[0], y: v3(th[k])[1] + shift[1], z: v3(th[k])[2] + shift[2] };
  const c50 = chestAt(lab, "50", HARNESS.handleStart[0]);
  const start: V3 = [HARNESS.handleStart[0], round(c50.v + c50.ry + HARNESS.girth.clearance), 0];
  const local = toLocal(th, HARNESS.gripWorld);
  const d: V3 = [local[0] - start[0], local[1] - start[1], local[2] - start[2]];
  const L = Math.hypot(...d);
  HARNESS.handleLength = round(L);
  const angleDeg = Math.atan2(d[1], d[0]) * 180 / Math.PI, abductionDeg = Math.asin(d[2] / L) * 180 / Math.PI;
  a.rig.bones.push({ id: P("harness-handle"), parent: P("thorax"), title: { en: "Guide dog harness handle", de: "Führgeschirrbügel" }, x: start[0], y: start[1], z: start[2], angleDeg: round(angleDeg), abductionDeg: round(abductionDeg), lengthM: round(L), joint: { type: "revolute", minDeg: -45, maxDeg: 80 }, mass: { kg: HARNESS.massHandleKg, comX: round(L * 0.55), comY: 0, distribution: "uniform", source: "authored-estimate", reference: "guide-dog-harness" } });
  const thorax = a.rig.bones.find((b: any) => b.id === P("thorax"));
  thorax.mass = { ...thorax.mass, kg: round(thorax.mass.kg + HARNESS.massGirthKg) };
  a.anchors.push({ id: P("handle-grip"), bone: P("harness-handle"), x: round(L), y: 0, kind: "attachment", title: { en: "Handler's left hand on the harness handle", de: "Linke Hand der führenden Person am Geschirrbügel" } });
  a.materials.push({ id: "harness-leather", title: { en: "Black leather and webbing of the guide-dog harness", de: "Schwarzes Leder und Gurtband des Führgeschirrs" }, emissivity: 0.9, solarAbsorptivity: 0.85, source: "authored-estimate", reference: "ashrae-2021" });

  const restF = restFrames(a.rig);
  const thR = restF[P("thorax")].frame;
  const handleProfile = (scale: string) => scale === "500" ? rod(0, L, 0.009, 0.009) : scale === "200" ? rod(0, L, HARNESS.handleR * 1.6, HARNESS.handleR * 1.6) : rod(0, L - HARNESS.gripLength, HARNESS.handleR, HARNESS.handleR);
  let area = 0;
  for (const scale of SCALES) {
    const { u0, u1, clearance } = HARNESS.girth;
    const ring = (u: number, n = 32): V3[] => { const c = chestAt(lab, scale, u); return Array.from({ length: n }, (_, i) => { const t = 2 * Math.PI * i / n; return [u, c.v + (c.ry + clearance) * Math.cos(t), (c.rx + clearance) * Math.sin(t)] as V3; }); };
    const pts = [...ring(u0), ...ring(u1)];
    const world = pts.map((p) => toWorld(thR, p));
    const base = { role: "body", filled: true, tone: 0.12, region: "harness", material: "harness-leather", kind: "poly", closed: true, penMm: lab.representations[scale].pens.body };
    const g = a.representations[scale].geometry;
    g.elevation.push({ id: P("harness-girth"), bone: P("thorax"), ...base, z: -16.7, points: hull(pts.map((p) => [p[0], p[1]])) });
    g.front.push({ id: P("harness-girth"), bone: P("thorax"), ...base, filled: false, z: 23.3, points: hull(world.map((p) => [p[2], p[1]])) });
    g.plan.push({ id: P("harness-girth"), bone: P("thorax"), ...base, z: -16.5, points: hull(world.map((p) => [p[0], p[2]])) });
    const solidShape = (id: string, points: number[][], z: number) => ({ id, bone: P("harness-handle"), ...base, z, solid: { kind: "revolved", axisBone: P("harness-handle"), axis: "u", axisX: 0, axisZ: 0, mirror: true }, points });
    const zh = (view: string) => (view === "front" ? 23.8 : -10);
    const handle = [solidShape(P("harness-handle"), handleProfile(scale), 0)];
    if (scale === "50" || scale === "100") handle.push(solidShape(P("harness-grip"), rod(L - HARNESS.gripLength, L + 0.012, HARNESS.gripR, HARNESS.gripR), 0.01));
    for (const view of ["elevation", "front", "plan"]) g[view].push(...handle.map((s) => ({ ...clone(s), z: zh(view) + s.z })));
    const ca = chestAt(lab, scale, u0), cb = chestAt(lab, scale, u1);
    const sec = (u: number, c: any) => ({ at: [u, round(c.v), 0], rx: round(c.ry + clearance), ry: round(c.rx + clearance), exponent: 2, rotation: [0.5, 0.5, 0.5, 0.5] });
    const vol = a.volumes[scale];
    const revolve = (id: string, prof: number[][], slender?: boolean) => ({ id, kind: "revolve", bone: P("harness-handle"), material: "harness-leather", role: "body", tone: 0.12, profile: [[prof[0]![0], 0], [prof[0]![0], Math.abs(prof[0]![1]!)], [prof[1]![0], Math.abs(prof[1]![1]!)], [prof[1]![0], 0]], axis: [1, 0, 0], origin: [0, 0, 0], ...(slender ? { slender: true } : {}) });
    const solids: any[] = [
      { id: P("harness-girth"), kind: "loft", bone: P("thorax"), material: "harness-leather", role: "body", tone: 0.12, sections: [sec(u0, ca), sec(u1, cb)], capStart: true, capEnd: true },
      revolve(P("harness-handle"), handleProfile(scale), scale === "500"),
    ];
    if (scale === "50" || scale === "100") solids.push(revolve(P("harness-grip"), rod(L - HARNESS.gripLength, L + 0.012, HARNESS.gripR, HARNESS.gripR)));
    vol.body.push(...solids);
    vol.structure.push({ id: P("harness-handle-member"), kind: "capsule", bone: P("harness-handle"), material: "harness-leather", role: "body", tone: 0.12, p0: [0, 0, 0], p1: [round(L), 0, 0], r0: 0.009, r1: 0.009, slender: true, member: P("harness-handle"), part: "member" });
    vol.thermal.push(...solids.map((s) => ({ id: `th-${s.id}`, of: s.id, region: "harness" })));
    if (scale === "50") {
      const perim = (c: any) => Math.PI * (3 * (c.rx + c.ry) - Math.sqrt((3 * c.rx + c.ry) * (c.rx + 3 * c.ry)));
      area = (perim({ rx: ca.rx + clearance, ry: ca.ry + clearance }) + perim({ rx: cb.rx + clearance, ry: cb.ry + clearance })) / 2 * (u1 - u0) + 2 * Math.PI * HARNESS.handleR * (L - HARNESS.gripLength) + 2 * Math.PI * HARNESS.gripR * HARNESS.gripLength;
    }
  }
  a.thermal.regions.push({ id: "harness", title: { en: "Guide-dog harness: girth, handle and grip", de: "Führgeschirr: Brustgurt, Bügel und Griff" }, kind: "surface", model: "passive", material: "harness-leather", areaM2: round(area), characteristicLengthM: 0.05, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 3000, source: "authored-estimate", reference: "guide-dog-harness" });
  for (const p of a.poses) p.thermal.regions["harness"] = { exposedFraction: 0.9, sunlitFraction: 0.5 };
}
