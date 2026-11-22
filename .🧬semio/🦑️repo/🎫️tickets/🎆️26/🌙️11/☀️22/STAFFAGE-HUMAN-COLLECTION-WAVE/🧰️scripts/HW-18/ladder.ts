/** 🪜️ Generator of `humans/person-carrying-ladder`: an electrician of 1.82 m (rig, body, garments and skeleton of `worker-carrying-box`, the closest existing figure: hard hat, high-visibility vest, gloves, safety shoes) walking with a 2.88 m aluminium single ladder on the right shoulder, the ladder on edge with the lower rail on the shoulder and the right hand on that rail ahead. Builds the ladder per scale (rails, D rungs, rubber feet) as body/thermal/structure solids and extruded 2D profiles, rests the lower rail on the modelled shoulder surface in every pose and clip key, solves the hands onto the rail and writes the catalog asset.
 * `bun ladder.ts` → writes `🗂️catalog/👤️humans/person-carrying-ladder/🔣️.json`.
 * @see ./lib.ts
 * @see ./ladder-texts.ts */
import { addParts, anchorWorld, binding, bodyVertices, chain, clone, extents, pruneMaterials, r4, readHuman, rigMass, setDimensions, solveArm, strip, t, writeHuman, type Json, type P3, type Part } from "./lib.ts";
import { TEXTS } from "./ladder-texts.ts";

const a = readHuman("worker-carrying-box");
const base = clone(a);
strip(a, ["box-l", "box-r"], ["box-surface"]);

const ARM_R = { arm: "upper-arm-r", forearm: "forearm-r", hand: "hand-r" };
const ARM_L = { arm: "upper-arm-l", forearm: "forearm-l", hand: "hand-l" };
const LZ = -0.17;
const HALF = 1.44;
const WIDTH = 0.4;
const RAIL = 0.025;
const LADDER_KG = 5;

a.rig.bones.push(
  { id: "ladder-reach", parent: "pelvis", title: t("Ladder position ahead of or behind the pelvis", "Lage der Leiter vor oder hinter dem Becken"), x: 0, y: 0, angleDeg: 0, lengthM: 0, joint: { type: "prismatic", axisDeg: 90, minM: -0.6, maxM: 0.6 } },
  { id: "ladder-lift", parent: "ladder-reach", title: t("Ladder height on the shoulder", "Höhe der Leiter auf der Schulter"), x: 0, y: 0, angleDeg: -90, lengthM: 0, joint: { type: "prismatic", axisDeg: 0, minM: -0.3, maxM: 1.2 } },
  { id: "ladder", parent: "ladder-lift", title: t("Aluminium single ladder on edge, lower rail on the right shoulder", "Aluminium-Anlegeleiter hochkant, unterer Holm auf der rechten Schulter"), x: 0, y: 0, z: LZ, angleDeg: 0, lengthM: HALF, joint: { type: "revolute", minDeg: -25, maxDeg: 25 }, mass: { kg: LADDER_KG, comX: 0, comY: WIDTH / 2, distribution: "point", source: "authored-estimate", reference: "en-131" } },
);
a.anchors.push(
  { id: "rail-grip", bone: "ladder", x: 0.3, y: RAIL / 2, kind: "attachment", title: t("Lower rail ahead of the shoulder, right hand", "Unterer Holm vor der Schulter, rechte Hand") },
  { id: "rail-grip-2", bone: "ladder", x: 0.52, y: RAIL / 2, kind: "attachment", title: t("Lower rail further ahead, left hand", "Unterer Holm weiter vorn, linke Hand") },
  { id: "ladder-cog", bone: "ladder", x: 0, y: WIDTH / 2, kind: "load", title: t("Centre of gravity of the ladder", "Schwerpunkt der Leiter") },
  { id: "shoulder-rest", bone: "ladder", x: 0, y: 0, kind: "attachment", title: t("Lower rail on the shoulder", "Unterer Holm auf der Schulter") },
);
a.materials.push(
  { id: "aluminium-rail", title: t("Aluminium alloy, mill finish", "Aluminiumlegierung, walzblank"), emissivity: 0.25, solarAbsorptivity: 0.4, densityKgM3: 2700, conductivityWmK: 160, specificHeatJKgK: 900, source: "literature", reference: "ashrae-2021" },
  { id: "pvc-foot", title: t("Non-slip PVC ladder foot", "Rutschfester PVC-Leiterfuß"), emissivity: 0.92, solarAbsorptivity: 0.85, densityKgM3: 1300, conductivityWmK: 0.17, specificHeatJKgK: 1000, source: "authored-estimate" },
);
a.thermal.regions.push(
  { id: "ladder", title: t("Aluminium rails and rungs", "Aluminium-Holme und -Sprossen"), kind: "surface", model: "passive", material: "aluminium-rail", areaM2: 1.08, characteristicLengthM: 0.03, geometry: "cylinder", skyViewFactor: 0.5, layers: [], heatCapacityJM2K: 4200, ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "en-131" },
  { id: "ladder-feet", title: t("PVC feet and end caps", "PVC-Füße und Endkappen"), kind: "surface", model: "passive", material: "pvc-foot", areaM2: 0.04, characteristicLengthM: 0.06, geometry: "cylinder", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 6000, source: "authored-estimate", reference: "en-131" },
);

/** 🧱️ The ladder in its own frame: origin on the underside of the lower rail at the balance point, u along the ladder (ahead), v across it towards the upper rail, w lateral. */
const LD = "ladder";
const al = { region: "ladder", material: "aluminium-rail", tone: 0.78 };
const pvc = { region: "ladder-feet", material: "pvc-foot", tone: 0.2 };
const box = (u0: number, u1: number, v0: number, v1: number): [number, number][] => [[r4(u0), r4(v0)], [r4(u1), r4(v0)], [r4(u1), r4(v1)], [r4(u0), r4(v1)]];
const RUNGS = Array.from({ length: 10 }, (_, k) => r4(-1.26 + 0.28 * k));
const rails = (scales: number[], w: [number, number], dw?: [number, number], dv?: number, front = true): Part[] => [
  { id: "rail-lower", bone: LD, ...al, scales, views: front ? ["elevation", "front"] : ["elevation"], geom: { kind: "profile", pts: box(-HALF, HALF, 0, RAIL), w }, slender: true, z2d: 7.2, ...(dv ? { draw2d: box(-HALF, HALF, 0, dv), drawW: dw } : {}) },
  { id: "rail-upper", bone: LD, ...al, scales, views: front ? ["elevation", "front", "plan"] : ["elevation", "plan"], geom: { kind: "profile", pts: box(-HALF, HALF, WIDTH - RAIL, WIDTH), w }, slender: true, z2d: 7.2, ...(dv ? { draw2d: box(-HALF, HALF, WIDTH - dv, WIDTH), drawW: dw } : {}) },
];
const RW: [number, number] = [-0.029, 0.029];
const parts: Part[] = [
  ...rails([50, 100], RW),
  ...RUNGS.map((u, k): Part => ({ id: `rung-${k}`, bone: LD, ...al, tone: 0.72, scales: [50, 100], views: ["elevation", "front"], geom: { kind: "profile", pts: box(u - 0.015, u + 0.015, RAIL, WIDTH - RAIL), w: [-0.015, 0.015] }, z2d: 7.1 })),
  { id: "foot-lower", bone: LD, ...pvc, scales: [50, 100], views: ["elevation"], geom: { kind: "profile", pts: box(-HALF - 0.012, -HALF + 0.06, -0.004, RAIL + 0.004), w: [-0.033, 0.033] }, z2d: 7.25 },
  { id: "foot-upper", bone: LD, ...pvc, scales: [50, 100], views: ["elevation", "plan"], geom: { kind: "profile", pts: box(-HALF - 0.012, -HALF + 0.06, WIDTH - RAIL - 0.004, WIDTH + 0.004), w: [-0.033, 0.033] }, draw2d: box(-HALF - 0.012, -HALF + 0.06, WIDTH - RAIL - 0.004, WIDTH + 0.004), drawW: RW, z2d: 7.25 },
  { id: "cap-lower", bone: LD, ...pvc, scales: [50], views: ["elevation", "front"], geom: { kind: "profile", pts: box(HALF - 0.03, HALF + 0.006, -0.002, RAIL + 0.002), w: [-0.031, 0.031] }, draw2d: box(HALF - 0.03, HALF + 0.006, -0.002, RAIL + 0.002), drawW: RW, z2d: 7.25 },
  { id: "cap-upper", bone: LD, ...pvc, scales: [50], geom: { kind: "profile", pts: box(HALF - 0.03, HALF + 0.006, WIDTH - RAIL - 0.002, WIDTH + 0.002), w: [-0.031, 0.031] }, draw2d: box(HALF - 0.03, HALF + 0.006, WIDTH - RAIL - 0.002, WIDTH + 0.002), drawW: RW, z2d: 7.25 },
  ...rails([200], RW, [-0.045, 0.045], 0.04),
  ...RUNGS.map((u, k): Part => ({ id: `rung-${k}`, bone: LD, ...al, tone: 0.72, scales: [200], views: ["elevation", "front"], geom: { kind: "profile", pts: box(u - 0.015, u + 0.015, RAIL, WIDTH - RAIL), w: [-0.015, 0.015] }, draw2d: box(u - 0.02, u + 0.02, 0.03, WIDTH - 0.03), drawW: [-0.025, 0.025], z2d: 7.1 })),
  { id: "feet", bone: LD, ...pvc, scales: [200], views: ["elevation", "plan"], geom: { kind: "profile", pts: box(-HALF - 0.012, -HALF + 0.06, -0.004, WIDTH + 0.004), w: [-0.033, 0.033] }, draw2d: box(-HALF - 0.012, -HALF + 0.06, -0.004, WIDTH + 0.004), drawW: [-0.045, 0.045], z2d: 7.25 },
  { id: "rail-lower", bone: LD, ...al, scales: [500], no2d: true, geom: { kind: "profile", pts: box(-HALF, HALF, 0, RAIL), w: RW }, slender: true },
  { id: "rail-upper", bone: LD, ...al, scales: [500], views: ["plan"], geom: { kind: "profile", pts: box(-HALF, HALF, WIDTH - RAIL, WIDTH), w: RW }, slender: true, draw2d: box(-HALF, HALF, WIDTH - RAIL, WIDTH), drawW: [-0.06, 0.06], z2d: 7.2 },
  { id: "ladder-frame", bone: LD, ...al, regions: ["ladder", "ladder-feet"], scales: [500], views: ["elevation"], no3d: true, filled: false, geom: { kind: "profile", pts: box(-HALF, HALF, 0.0125, WIDTH - 0.0125), w: RW }, z2d: 7.2 },
  { id: "feet", bone: LD, ...pvc, scales: [500], no2d: true, geom: { kind: "profile", pts: box(-HALF - 0.012, -HALF + 0.063, -0.004, WIDTH + 0.004), w: [-0.0375, 0.0375] } },
  { id: "ladder-end", bone: LD, ...al, scales: [500], views: ["front"], no3d: true, geom: { kind: "profile", pts: box(-HALF, HALF, 0, WIDTH), w: [-0.06, 0.06] }, z2d: 7.2 },
  ...[-1.12, -0.56, 0, 0.56, 1.12].map((u, k): Part => ({ id: `rung-${k}`, bone: LD, ...al, tone: 0.72, scales: [500], no2d: true, geom: { kind: "profile", pts: box(u - 0.0375, u + 0.0375, RAIL, WIDTH - RAIL), w: [-0.0375, 0.0375] } })),
  ...[-1.12, -0.56, 0, 0.56, 1.12].map((u, k): Part => ({ id: `rung-line-${k}`, bone: LD, ...al, tone: 0.72, scales: [500], views: ["elevation"], no3d: true, open: true, filled: false, geom: { kind: "profile", pts: [[u, 0.0125], [u, WIDTH - 0.0125]], w: [-0.015, 0.015] }, z2d: 7.1 })),
];
const marker = (id: string, c: P3, r: number): Json => ({ id, kind: "ellipsoid", bone: LD, material: "aluminium-rail", role: "detail", tone: 0.4, radii: [r, r, r], transform: { translation: c }, member: LD, part: "mass" });
for (const s of [50, 100, 200, 500]) addParts(a, s, parts, s <= 100 ? [marker("mass-ladder", [0, WIDTH / 2, 0], s === 50 ? 0.03 : 0.04)] : []);

/** 🦾️ Placement: the lower rail rests on the highest point of the modelled right shoulder (garment and deltoid solids of 1:50) under the clavicle tip; the ladder pitch is set per pose; hands are solved onto the rail. */
const legs = (o: Json): Json => Object.fromEntries(Object.entries(o).filter(([k]) => /^(thigh|shank|foot|toe)-/u.test(k)));
const FEET = ["heel-l", "contact-l", "heel-r", "contact-r"];
const SHOULDER_BONES = new Set(["torso", "clavicle-r", "upper-arm-r", "neck"]);

function shoulderTop(bones: Json): P3 {
  const fr = chain(a, bones);
  const tip = fr["clavicle-r"].frame;
  const x0 = tip.o[0] + tip.u[0] * a.rig.bones.find((b: Json) => b.id === "clavicle-r").lengthM;
  const pts = bodyVertices(a, 50, fr, (s) => SHOULDER_BONES.has(s.bone)).filter((p) => Math.abs(p[2] - LZ) < 0.03 && Math.abs(p[0] - x0) < 0.06);
  const top = pts.reduce((m, p) => (p[1] > m[1] ? p : m), [x0, -Infinity, LZ] as P3);
  return [r4(top[0]), r4(top[1] + 0.002), LZ];
}

function place(body: Json, o: { pitch: number; arms: [Json, string, Json][]; free?: Json; torso?: number; neck?: number; head?: number }): { bones: Json; residual: number } {
  const bones: Json = { ...body, torso: { angleDeg: o.torso ?? 2 }, neck: { angleDeg: o.neck ?? -2 }, head: { angleDeg: o.head ?? 0 }, "ladder-reach": { dy: 0 }, "ladder-lift": { dx: 0 }, ladder: { angleDeg: o.pitch }, ...(o.free ?? {}) };
  const s = shoulderTop(bones);
  const f0 = chain(a, bones);
  const o0 = f0.ladder.frame.o, pv = f0.pelvis.frame.v, pu = f0["ladder-reach"].frame.u;
  const [ex, ey] = [s[0] - o0[0], s[1] - o0[1]];
  const det = pv[0] * pu[1] - pv[1] * pu[0];
  bones["ladder-reach"] = { dy: r4((ex * pu[1] - ey * pu[0]) / det) };
  bones["ladder-lift"] = { dx: r4((pv[0] * ey - pv[1] * ex) / det) };
  let residual = 0;
  for (const [names, grip, opts] of o.arms) {
    const target = anchorWorld(a, chain(a, bones), grip);
    const arm = solveArm(a, bones, names, names.hand === "hand-r" ? "grip-r" : "grip-l", target, opts);
    Object.assign(bones, arm.bones);
    residual = Math.max(residual, arm.residual);
  }
  return { bones, residual };
}

const HOLD_R = { prefer: [15, 5, 0, 110, 0] };
const HOLD_L = { prefer: [30, 20, 0, 100, 0] };
const swing = (deg: number, elbow: number): Json => ({ "upper-arm-l": { angleDeg: r4(deg), abductionDeg: 3 }, "forearm-l": { angleDeg: r4(elbow) }, "hand-l": { angleDeg: -6 } });
const walkSwing = (b: Json): Json => {
  const d = b["thigh-r"].angleDeg - b["thigh-l"].angleDeg;
  return swing(0.55 * d + 4, 16 + 0.3 * Math.max(0, d));
};

const stand = base.poses.find((p: Json) => p.id === "carrying-box-low");
const walkPoses = base.poses.filter((p: Json) => p.key && p.id.startsWith("walk-"));
const PITCH = 3;
let worst = 0;
const keyPoses = walkPoses.map((p: Json) => {
  const r = place(legs(p.bones), { pitch: PITCH, arms: [[ARM_R, "rail-grip", HOLD_R]], free: walkSwing(p.bones) });
  worst = Math.max(worst, r.residual);
  return { ...p, title: TEXTS.keyTitle(p.title), description: TEXTS.keyDescription(p.description), bones: r.bones, thermal: binding(a, 3.3, "carrying-walk", TEXTS.over(), 0.5), loads: TEXTS.loads() };
});
const DEFAULT_KEY = process.env.KEY ?? "walk-midstance-l";
const walking = keyPoses.find((p: Json) => p.id === DEFAULT_KEY);
const standing = place(legs(stand.bones), { pitch: PITCH, arms: [[ARM_R, "rail-grip", HOLD_R], [ARM_L, "rail-grip-2", HOLD_L]] });
const tipping = place(legs(stand.bones), { pitch: 15, arms: [[ARM_R, "rail-grip", HOLD_R], [ARM_L, "rail-grip-2", HOLD_L]], torso: -1, neck: 4, head: 6 });
worst = Math.max(worst, standing.residual, tipping.residual);
console.log("[DEBUG] worst residual", worst.toFixed(5));

a.poses = [
  { id: "carrying", title: TEXTS.poses.carrying.title, description: TEXTS.poses.carrying.description, bones: walking.bones, ground: "auto", contacts: walking.contacts, thermal: binding(a, 3.3, "carrying-walk", TEXTS.over(), 0.5), loads: TEXTS.loads() },
  { id: "standing-shouldered", title: TEXTS.poses.standing.title, description: TEXTS.poses.standing.description, bones: standing.bones, ground: "auto", contacts: FEET, thermal: binding(a, 1.7, "rest", TEXTS.over(), 0.5), loads: TEXTS.loads() },
  { id: "tipping-up", title: TEXTS.poses.tipping.title, description: TEXTS.poses.tipping.description, bones: tipping.bones, ground: "auto", contacts: FEET, thermal: binding(a, 2.0, "steadying", TEXTS.over(), 0.5), loads: TEXTS.loads() },
  ...keyPoses,
];
const walkClip = base.clips.find((c: Json) => c.id === "carry-walk");
const tipKeys = [[0, "standing-shouldered", 0], [1.2, "tipping-up", 0], [2.2, "tipping-up", 0], [3.4, "standing-shouldered", 0]];
a.clips = [
  { ...walkClip, title: TEXTS.clips.walk.title, description: TEXTS.clips.walk.description, tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 3.3 }, { t: walkClip.durationS, v: 3.3 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }] }, { target: "emitter:plume", ease: "smooth", keys: [{ t: 0, v: 0.5 }, { t: walkClip.durationS, v: 0.5 }] }] },
  { id: "tip-and-settle", title: TEXTS.clips.tip.title, description: TEXTS.clips.tip.description, durationS: 3.4, fps: 30, loop: false, rootMotion: "none", keys: tipKeys.map(([tm, pose]) => ({ t: tm, pose, ease: "smooth" })), contacts: FEET.map((anchor) => ({ anchor, fromS: 0, toS: 3.4 })), phases: [{ id: "tip", fromS: 0, toS: 1.2, title: t("Tipping the front end up", "Vorderes Ende anheben") }, { id: "hold", fromS: 1.2, toS: 2.2, title: t("Holding it up", "Oben halten") }, { id: "settle", fromS: 2.2, toS: 3.4, title: t("Settling back level", "Zurück in die Waagerechte") }], events: [{ t: 1.2, kind: "motion", title: t("Front end 0.37 m higher", "Vorderes Ende 0,37 m höher"), target: "ladder" }], tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.7 }, { t: 1.2, v: 2.0 }, { t: 2.2, v: 2.0 }, { t: 3.4, v: 1.7 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 0.6, v: 2 }, { t: 2.8, v: 0 }] }] },
];
a.thermal.environment = TEXTS.environment;
a.thermal.baseMet = 1.6;
a.thermal.respiration.patterns = [...a.thermal.respiration.patterns.filter((p: Json) => p.id === "rest"), ...TEXTS.patterns];
a.thermal.assumptions = TEXTS.thermalAssumptions;
a.structure.assumptions = TEXTS.structureAssumptions;
a.structure.dynamics = "quasi-static";
for (const r of a.thermal.regions) if (TEXTS.regionTitles[r.id]) r.title = TEXTS.regionTitles[r.id];
Object.assign(a, { id: "person-carrying-ladder", title: TEXTS.title, summary: TEXTS.summary, tags: ["human", "adult", "man", "electrician", "trades", "ladder", "carrying", "shoulder", "walking", "hi-vis", "hard-hat"], provenance: TEXTS.provenance(a.provenance), presentation: { defaultPose: "carrying", defaultClip: "carry-walk", defaultScale: 100 } });
for (const s of ["50", "100", "200", "500"]) {
  a.representations[s].decisions = TEXTS.repDecisions[s];
  const vol = a.volumes[s];
  vol.decisions = [...vol.decisions.filter((d: Json) => !/box|carton|Karton|Kiste|hand hole|Griffloch|Grifflöcher/iu.test(d.en + d.de)), ...TEXTS.volDecisions[s]];
}
pruneMaterials(a);
setDimensions(a, rigMass(a));
console.log("[DEBUG] dimensions", JSON.stringify(a.dimensions), "extents", JSON.stringify(extents(a)));
console.log("[DEBUG] wrote", writeHuman(a));
