/** 🛻️ Generator of `humans/worker-with-pallet-jack`: a warehouse operative of 1.82 m (rig, body, garments and skeleton of `courier-with-hand-truck`, the closest existing figure) pulling a manual hand pallet truck with a stretch-wrapped Euro pallet behind him with the right hand on the tiller. Builds the truck per scale (chassis with steering wheels, pump and tiller, lifting fork frame, EPAL pallet, carton load) as body/thermal/structure solids and extruded 2D profiles, solves the right arm onto the tiller grip in every pose and clip key, and writes the catalog asset.
 * `bun pallet-jack.ts` → writes `🗂️catalog/👤️humans/worker-with-pallet-jack/🔣️.json`.
 * @see ./lib.ts
 * @see ./pallet-jack-texts.ts */
import { addParts, anchorWorld, binding, chain, clone, lowest, pruneMaterials, r4, readHuman, rigMass, setDimensions, solveArm, strip, t, writeHuman, extents, type Json, type P3, type Part } from "./lib.ts";
import { TEXTS } from "./pallet-jack-texts.ts";

/** 🚚️ The truck and its load stand behind the operator who pulls them: seen from ahead they are painted under his legs and trunk (their authored front order is kept among themselves). */
const TRUCK_BONES = new Set(["pallet-jack", "forks", "tiller", "load"]);
const SCALES_FRONT = ["50", "100", "200", "500"];

const a = readHuman("courier-with-hand-truck");
const base = clone(a);
strip(a, ["truck-reach"], ["hand-truck", "tyres", "parcels"]);

const ARM_R = { arm: "arm-r", forearm: "forearm-r", hand: "hand-r" };
const ARM_L = { arm: "arm-l", forearm: "forearm-l", hand: "hand-l" };
const PIVOT_V = 0.22;
const TILLER = 0.88;
const GRIP_W = -0.1;
const LOAD_KG = 324;
const PALLET_KG = 25;

/** 🦴️ Rig of the truck: horizontal reach and vertical float under the pelvis (as the hand truck it replaces), the steering unit, the tiller on its pivot, the fork frame on its lift and the pallet with its load. */
a.rig.bones.push(
  { id: "jack-yaw", parent: "pelvis", title: t("Heading of the pallet truck relative to the operative", "Richtung des Hubwagens gegenüber der Person"), x: 0, y: 0, angleDeg: 0, lengthM: 0, joint: { type: "revolute", minDeg: -1, maxDeg: 1, twistMinDeg: -180, twistMaxDeg: 180 } },
  { id: "jack-reach", parent: "jack-yaw", title: t("Pallet truck distance from the operative", "Abstand des Hubwagens von der Person"), x: -0.85745, y: 1.3, angleDeg: 0, lengthM: 0, joint: { type: "prismatic", axisDeg: 90, minM: -0.9, maxM: 0.6 } },
  { id: "jack-lift", parent: "jack-reach", title: t("Pallet truck vertical float on the floor", "Senkrechte Führung des Hubwagens am Boden"), x: 0, y: 0, angleDeg: -90, lengthM: 0, joint: { type: "prismatic", axisDeg: 0, minM: -0.15, maxM: 0.15 } },
  { id: "pallet-jack", parent: "jack-lift", title: t("Steering unit: steering wheels, yoke and hydraulic pump", "Lenkeinheit: Lenkräder, Gabelkopf und Hydraulikpumpe"), x: 0, y: 0, angleDeg: 0, lengthM: 0.3, joint: { type: "revolute", minDeg: -3, maxDeg: 3 }, mass: { kg: 30, comX: -0.02, comY: 0.12, distribution: "point", source: "authored-estimate", reference: "pallet-truck-2500" } },
  { id: "tiller", parent: "pallet-jack", title: t("Tiller with loop handle", "Deichsel mit Bügelgriff"), x: 0, y: PIVOT_V, angleDeg: 90, lengthM: TILLER, joint: { type: "revolute", minDeg: -75, maxDeg: 10 }, mass: { kg: 6, comX: 0.48, comY: 0, distribution: "point", source: "authored-estimate", reference: "pallet-truck-2500" } },
  { id: "forks", parent: "pallet-jack", title: t("Fork frame, raised by the pump", "Gabelrahmen, von der Pumpe gehoben"), x: -0.17, y: -0.1, angleDeg: 0, lengthM: 1.15, joint: { type: "prismatic", axisDeg: 90, minM: 0, maxM: 0.1 }, mass: { kg: 34, comX: -0.55, comY: 0.07, distribution: "point", source: "authored-estimate", reference: "pallet-truck-2500" } },
  { id: "load", parent: "forks", title: t("Euro pallet with stretch-wrapped cartons", "Europalette mit gewickelten Kartons"), x: 0, y: 0, angleDeg: 0, lengthM: 1.2, joint: { type: "fixed" }, mass: { kg: LOAD_KG + PALLET_KG, comX: -0.605, comY: 0.49, distribution: "point", source: "authored-estimate", reference: "epal-1990" } },
);

a.anchors.push(
  { id: "steer-contact", bone: "pallet-jack", x: 0, y: -0.1, kind: "contact", title: t("Steering wheels on the floor", "Lenkräder am Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.03, footprintWidthM: 0.1, frictionCoefficient: 0.5 },
  { id: "roller-contact", bone: "pallet-jack", x: -1.2, y: -0.1, kind: "contact", title: t("Load rollers on the floor", "Lastrollen am Boden"), support: "contact", normalDeg: 90, footprintLengthM: 0.02, footprintWidthM: 0.3, frictionCoefficient: 0.5 },
  { id: "pallet-feet", bone: "load", x: -0.605, y: 0, kind: "contact", title: t("Pallet blocks on the floor", "Palettenklötze am Boden"), support: "contact", normalDeg: 90, footprintLengthM: 1.2, footprintWidthM: 0.8, frictionCoefficient: 0.5 },
  { id: "tiller-grip", bone: "tiller", x: TILLER, y: 0, z: GRIP_W, kind: "attachment", title: t("Grip of the loop handle, right of the tiller", "Griff des Bügels, rechts der Deichsel") },
  { id: "tiller-grip-2", bone: "tiller", x: TILLER, y: 0, z: -GRIP_W, kind: "attachment", title: t("Grip of the loop handle, left of the tiller", "Griff des Bügels, links der Deichsel") },
  { id: "load-cog", bone: "load", x: -0.605, y: 0.49, kind: "load", title: t("Centre of gravity of pallet and load", "Schwerpunkt von Palette und Last") },
);

a.materials.push(
  { id: "painted-steel", title: t("Powder-coated steel", "Pulverbeschichteter Stahl"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 7850, conductivityWmK: 50, specificHeatJKgK: 470, source: "literature", reference: "ashrae-2021" },
  { id: "polyurethane", title: t("Polyurethane wheel tread", "Polyurethan-Laufbelag"), emissivity: 0.9, solarAbsorptivity: 0.7, densityKgM3: 1200, conductivityWmK: 0.25, specificHeatJKgK: 1800, source: "authored-estimate" },
  { id: "spruce-pallet", title: t("Spruce pallet boards and blocks", "Palettenbretter und -klötze aus Fichte"), emissivity: 0.9, solarAbsorptivity: 0.6, densityKgM3: 450, conductivityWmK: 0.13, specificHeatJKgK: 1600, source: "literature", reference: "ashrae-2021" },
  { id: "wrapped-cartons", title: t("Cartons under stretch film", "Kartons unter Stretchfolie"), emissivity: 0.9, solarAbsorptivity: 0.5, densityKgM3: 380, conductivityWmK: 0.06, specificHeatJKgK: 1400, source: "authored-estimate" },
);

a.thermal.regions.push(
  { id: "truck-frame", title: t("Pallet truck chassis, forks and tiller", "Hubwagen-Chassis, Gabeln und Deichsel"), kind: "surface", model: "passive", material: "painted-steel", areaM2: 1.43, characteristicLengthM: 0.16, geometry: "horizontal-plate-up", skyViewFactor: 0.3, layers: [], heatCapacityJM2K: 23000, ratedPowerW: 0, ratedPowerBasis: "passive", source: "authored-estimate", reference: "pallet-truck-2500" },
  { id: "truck-wheels", title: t("Steering wheels and load rollers", "Lenkräder und Lastrollen"), kind: "tyre", model: "passive", material: "polyurethane", areaM2: 0.2, characteristicLengthM: 0.2, geometry: "cylinder", skyViewFactor: 0.1, layers: [], heatCapacityJM2K: 30000, source: "authored-estimate", reference: "pallet-truck-2500" },
  { id: "pallet", title: t("Euro pallet", "Europalette"), kind: "surface", model: "passive", material: "spruce-pallet", areaM2: 2.6, characteristicLengthM: 0.8, geometry: "horizontal-plate-up", skyViewFactor: 0.1, layers: [], heatCapacityJM2K: 16000, source: "authored-estimate", reference: "epal-1990" },
  { id: "load", title: t("Stretch-wrapped carton load", "Gewickelte Kartonladung"), kind: "surface", model: "passive", material: "wrapped-cartons", areaM2: 4.8, characteristicLengthM: 0.85, geometry: "vertical-plate", skyViewFactor: 0.4, layers: [], heatCapacityJM2K: 30000, source: "authored-estimate" },
);

/** 🧱️ The truck as parts per scale (bone-local metres). Chassis frame: origin on the steering axle 0.10 m above the floor, u ahead (towards the operative), v up. Fork frame: origin on the floor below the fork heel. Tiller: origin on the pivot, u along the tiller. */
const S50 = [50], S100 = [100], FINE = [50, 100], COARSE = [200, 500], ALL = [50, 100, 200, 500];
const box = (u0: number, u1: number, v0: number, v1: number): [number, number][] => [[u0, v0], [u1, v0], [u1, v1], [u0, v1]];
const CH = "pallet-jack", FK = "forks", LD = "load", TL = "tiller";
const steel = { region: "truck-frame", material: "painted-steel", tone: 0.3 };
const tyre = { region: "truck-wheels", material: "polyurethane", tone: 0.15 };
const wood = { region: "pallet", material: "spruce-pallet", tone: 0.62 };
const cargo = { region: "load", material: "wrapped-cartons", tone: 0.55 };
const parts: Part[] = [
  { id: "steer-wheel-r", bone: CH, ...tyre, scales: FINE, geom: { kind: "wheel", c: [0, 0], r: 0.1, ri: 0.045, w: [-0.07, -0.02] }, z2d: 2.62 },
  { id: "steer-wheel-l", bone: CH, ...tyre, scales: FINE, geom: { kind: "wheel", c: [0, 0], r: 0.1, ri: 0.045, w: [0.02, 0.07] }, views: ["front", "plan"], z2d: 2.4 },
  { id: "steer-hub", bone: CH, ...steel, tone: 0.5, scales: FINE, geom: { kind: "wheel", c: [0, 0], r: 0.046, w: [-0.065, 0.065] }, z2d: 2.63, role2d: "detail" },
  { id: "steer-wheels", bone: CH, ...tyre, scales: [200], geom: { kind: "wheel", c: [0, 0], r: 0.1, w: [-0.07, 0.07] }, z2d: 2.62 },
  { id: "steer-wheels", bone: CH, ...tyre, scales: [500], views: ["elevation", "plan"], geom: { kind: "wheel", c: [0, 0], r: 0.1, w: [-0.07, 0.07] }, z2d: 2.62 },
  { id: "yoke-r", bone: CH, ...steel, scales: S50, geom: { kind: "profile", pts: [[-0.055, -0.02], [0.055, -0.02], [0.07, 0.15], [-0.07, 0.15]], w: [-0.085, -0.075] }, z2d: 2.64, tone2d: 0.32 },
  { id: "yoke-l", bone: CH, ...steel, scales: S50, geom: { kind: "profile", pts: [[-0.055, -0.02], [0.055, -0.02], [0.07, 0.15], [-0.07, 0.15]], w: [0.075, 0.085] }, views: ["front", "plan"], z2d: 2.3 },
  { id: "yoke", bone: CH, ...steel, scales: [100], geom: { kind: "profile", pts: [[-0.06, -0.015], [0.06, -0.015], [0.075, 0.15], [-0.075, 0.15]], w: [-0.085, 0.085] }, z2d: 2.64 },
  { id: "yoke", bone: CH, ...steel, scales: COARSE, geom: { kind: "profile", pts: [[-0.065, 0.03], [0.065, 0.03], [0.075, 0.16], [-0.075, 0.16]], w: [-0.085, 0.085] }, z2d: 2.64 },
  { id: "turntable", bone: CH, ...steel, scales: FINE, geom: { kind: "profile", pts: box(-0.1, 0.09, 0.14, 0.17), w: [-0.1, 0.1] }, z2d: 2.65 },
  { id: "pump", bone: CH, ...steel, tone: 0.25, scales: ALL, geom: { kind: "profile", pts: [[-0.08, 0.16], [0.045, 0.16], [0.045, 0.31], [0.03, 0.335], [-0.065, 0.335], [-0.08, 0.31]], w: [-0.06, 0.06] }, z2d: 2.66 },
  { id: "pivot", bone: CH, ...steel, tone: 0.2, scales: FINE, geom: { kind: "bar", c: [0, PIVOT_V], r: 0.022, w: [-0.055, 0.055] }, z2d: 2.67, role2d: "detail" },
  { id: "roller-r", bone: CH, ...tyre, scales: FINE, geom: { kind: "wheel", c: [-1.2, -0.059], r: 0.041, w: [-0.2, -0.13] }, z2d: 1.9 },
  { id: "roller-l", bone: CH, ...tyre, scales: FINE, geom: { kind: "wheel", c: [-1.2, -0.059], r: 0.041, w: [0.13, 0.2] }, views: ["front", "plan"], z2d: 1.8 },
  { id: "fork-r", bone: FK, ...steel, scales: ALL, geom: { kind: "profile", pts: [[-1.15, 0.07], [-1.1, 0.04], [0, 0.04], [0, 0.1], [-1.15, 0.1]], w: [-0.24, -0.09] }, z2d: 1.95 },
  { id: "fork-l", bone: FK, ...steel, scales: ALL, geom: { kind: "profile", pts: [[-1.15, 0.07], [-1.1, 0.04], [0, 0.04], [0, 0.1], [-1.15, 0.1]], w: [0.09, 0.24] }, views: ["front", "plan"], z2d: 1.85 },
  { id: "fork-heel", bone: FK, ...steel, scales: ALL, geom: { kind: "profile", pts: [[-0.005, 0.04], [0.105, 0.04], [0.105, 0.2], [0.07, 0.27], [-0.005, 0.27]], w: [-0.24, 0.24] }, z2d: 2.5 },
  { id: "pallet-top", bone: LD, ...wood, scales: FINE, geom: { kind: "profile", pts: box(-1.205, -0.005, 0.1, 0.144), w: [-0.4, 0.4] }, z2d: 3.3 },
  { id: "pallet-bottom", bone: LD, ...wood, scales: FINE, geom: { kind: "profile", pts: box(-1.205, -0.005, 0, 0.022), w: [-0.4, 0.4] }, z2d: 3.3 },
  ...[-1.1325, -0.605, -0.0775].map((u, i): Part => ({ id: `pallet-block-${i}`, bone: LD, ...wood, tone: 0.5, scales: FINE, geom: { kind: "profile", pts: box(u - 0.0725, u + 0.0725, 0.02, 0.102), w: [-0.4, 0.4] }, z2d: 3.31 })),
  { id: "pallet", bone: LD, ...wood, scales: [200], geom: { kind: "profile", pts: [[-1.205, 0], [-1.06, 0], [-1.06, 0.022], [-0.6775, 0.022], [-0.6775, 0], [-0.5325, 0], [-0.5325, 0.022], [-0.15, 0.022], [-0.15, 0], [-0.005, 0], [-0.005, 0.144], [-1.205, 0.144]], w: [-0.4, 0.4] }, z2d: 3.3 },
  { id: "pallet", bone: LD, ...wood, scales: [500], geom: { kind: "profile", pts: box(-1.205, -0.005, 0, 0.144), w: [-0.4, 0.4] }, z2d: 3.3 },
  { id: "cartons", bone: LD, ...cargo, scales: ALL, geom: { kind: "profile", pts: box(-1.195, -0.015, 0.144, 0.994), w: [-0.39, 0.39] }, z2d: 3.2 },
  { id: "tiller-tube", bone: TL, ...steel, scales: [50, 100, 200], geom: { kind: "tube", a: [-0.03, 0], b: [0.74, 0], r: 0.0175, w: 0 }, z2d: 2.7 },
  { id: "tiller-tube", bone: TL, ...steel, scales: [500], geom: { kind: "tube", a: [-0.03, 0], b: [0.86, 0], r: 0.0175, w: 0 }, slender: true, draw2d: [[-0.05, -0.05], [0.86, -0.05], [0.86, 0.05], [-0.05, 0.05]], drawW: [-0.05, 0.05], z2d: 2.7 },
  ...[1, -1].map((s): Part => ({ id: `tiller-bow-${s > 0 ? "l" : "r"}`, bone: TL, ...steel, scales: FINE, geom: { kind: "tubeW", a: [0.725, 0.02 * s], b: [TILLER, 0.16 * s], r: 0.015, v: 0 }, z2d: 2.71 })),
  { id: "tiller-bow", bone: TL, ...steel, scales: [200], geom: { kind: "profile", pts: [[0.72, -0.02], [TILLER, -0.02], [TILLER, 0.02], [0.72, 0.02]], w: [-0.16, 0.16] }, draw2d: [[0.72, -0.025], [TILLER + 0.02, -0.025], [TILLER + 0.02, 0.025], [0.72, 0.025]], drawW: [-0.17, 0.17], z2d: 2.71 },
  { id: "grip", bone: TL, ...steel, tone: 0.15, scales: [50, 100, 200], geom: { kind: "bar", c: [TILLER, 0], r: 0.018, w: [-0.17, 0.17] }, z2d: 2.72 },
  { id: "grip", bone: TL, ...steel, tone: 0.15, scales: [500], geom: { kind: "bar", c: [TILLER, 0], r: 0.0375, w: [-0.13, 0.13] }, z2d: 2.72 },
  { id: "lift-lever", bone: TL, ...steel, tone: 0.15, scales: S50, geom: { kind: "tube", a: [0.74, 0.005], b: [0.83, 0.05], r: 0.008, w: 0 }, z2d: 2.73, role2d: "detail" },
];

const marker = (id: string, bone: string, c: P3, r: number, part: "mass" | "joint"): Json => ({ id, kind: "ellipsoid", bone, material: part === "mass" ? "wrapped-cartons" : "painted-steel", role: "detail", tone: 0.4, radii: [r, r, r], transform: { translation: c }, member: bone, part });
for (const s of [50, 100, 200, 500]) {
  const extra = [marker("mass-load", LD, [-0.605, 0.49, 0], 0.08, "mass")];
  if (s <= 100) extra.push(marker("tiller-joint", TL, [0, 0, 0], s === 50 ? 0.024 : 0.03, "joint"), marker("mass-forks", FK, [-0.55, 0.07, 0], 0.04, "mass"));
  addParts(a, s, parts, extra);
}

/** 🦾️ Poses: legs, torso and head from the courier's gait keys and stances; the truck placed behind the operative with its tiller lowered towards the right hand, which is solved onto the grip. */
const legs = (o: Json): Json => Object.fromEntries(Object.entries(o).filter(([k]) => /^(thigh|shank|foot|toe)-/u.test(k)));
const GROUND_CONTACTS = ["heel-l", "ball-l", "heel-r", "ball-r"];

/** 🧭️ Places the truck (heading, reach, float, tiller angle, fork lift) for the body offsets and solves the given arms onto their grips; with `tillerFree` the tiller angle is chosen within that many degrees so that the right shoulder stays `SHOULDER_TO_GRIP` from the grip. */
function place(body: Json, o: { yaw?: number; reach: number; tillerDeg: number; lift: number; contacts: string[]; arms: [Json, string, Json][]; free?: Json; torso?: number; tillerFree?: number }): { bones: Json; residual: number } {
  const bones: Json = { ...body, ...(o.torso !== undefined ? { torso: { angleDeg: o.torso } } : {}), "jack-yaw": { twistDeg: o.yaw ?? 0 }, "jack-reach": { dy: o.reach }, "jack-lift": { dx: 0 }, tiller: { angleDeg: o.tillerDeg }, forks: { dy: o.lift }, ...(o.free ?? {}) };
  const feet = lowest(a, chain(a, bones), o.contacts.filter((c) => GROUND_CONTACTS.includes(c)));
  const steer = anchorWorld(a, chain(a, bones), "steer-contact")[1];
  bones["jack-lift"] = { dx: r4(feet - steer) };
  if (o.tillerFree) {
    let best = [Infinity, o.tillerDeg];
    for (let d = -o.tillerFree; d <= o.tillerFree; d += 0.25) {
      const fr = chain(a, { ...bones, tiller: { angleDeg: o.tillerDeg + d } });
      const sh = fr["arm-r"].frame.o, g = anchorWorld(a, fr, "tiller-grip");
      const c = Math.abs(Math.hypot(g[0] - sh[0], g[1] - sh[1], g[2] - sh[2]) - SHOULDER_TO_GRIP) + 0.002 * Math.abs(d);
      if (c < best[0]!) best = [c, o.tillerDeg + d];
    }
    bones.tiller = { angleDeg: r4(best[1]!) };
  }
  let residual = 0;
  for (const [names, grip, opts] of o.arms) {
    const target = anchorWorld(a, chain(a, bones), grip);
    const arm = solveArm(a, bones, names, names.hand === "hand-r" ? "grip-r" : "grip-l", target, opts);
    Object.assign(bones, arm.bones);
    residual = Math.max(residual, arm.residual);
  }
  return { bones, residual };
}

/** 📏️ Distance from the right shoulder joint to the grip that leaves the elbow slightly bent. */
const SHOULDER_TO_GRIP = 0.6;
const BEHIND = { prefer: [-25, -4, 0, 20, 10], handDir: [-0.45, -0.89, 0], handWeight: 0.01 };
const AHEAD_R = { prefer: [35, 6, 0, 55, 10] };
const AHEAD_L = { prefer: [35, -6, 0, 55, 10] };

const swing = (deg: number, elbow: number): Json => ({ "arm-l": { angleDeg: r4(deg), abductionDeg: -4 }, "forearm-l": { angleDeg: r4(elbow) }, "hand-l": { angleDeg: 4 } });
const walkKey = (k: Json): Json => {
  const thighL = k.bones["thigh-l"].angleDeg, thighR = k.bones["thigh-r"].angleDeg;
  return swing(0.6 * (thighR - thighL) + 2, 14 + 0.25 * Math.max(0, thighR - thighL));
};

const waiting = base.poses.find((p: Json) => p.id === "waiting");
const contact = base.poses.find((p: Json) => p.id === "walking-contact");
const pushWalk = base.clips.find((c: Json) => c.id === "push-walk");
const REACH = -0.42, PULL_TILLER = -45, PULL_LIFT = 0.06, FACE_REACH = -0.52;
const facing = (tillerDeg: number, lift: number, torso: number, reach = FACE_REACH): { bones: Json; residual: number } => place({ ...legs(waiting.bones), neck: { angleDeg: -2 }, head: { angleDeg: -8 } }, { yaw: 180, reach, tillerDeg, lift, contacts: GROUND_CONTACTS, arms: [[ARM_R, "tiller-grip-2", AHEAD_R], [ARM_L, "tiller-grip", AHEAD_L]], torso });

const PULL_KEY = pushWalk.keys[21];
const pull = place({ ...legs(PULL_KEY.bones), neck: { angleDeg: 4 }, head: { angleDeg: 2 } }, { reach: REACH, tillerDeg: PULL_TILLER, lift: PULL_LIFT, contacts: GROUND_CONTACTS, arms: [[ARM_R, "tiller-grip", BEHIND]], free: walkKey(PULL_KEY), torso: PULL_KEY.bones.torso.angleDeg - 3, tillerFree: 10 });
const pullKey0 = place({ ...legs(contact.bones), neck: { angleDeg: 4 }, head: { angleDeg: 2 } }, { reach: REACH, tillerDeg: PULL_TILLER, lift: PULL_LIFT, contacts: GROUND_CONTACTS, arms: [[ARM_R, "tiller-grip", BEHIND]], free: walkKey(contact), torso: -9, tillerFree: 10 });
const pump = facing(-30, 0.03, -10);
const parked = facing(0, 0, -3, -0.75);
console.log("[DEBUG] residuals pull", pull.residual.toFixed(5), "pump", pump.residual.toFixed(5), "parked", parked.residual.toFixed(5));

const loads = (factor: number): Json[] => [{ id: "pallet-load-weight", kind: "carried", title: t("Weight of the pallet and its load", "Gewicht der Palette und ihrer Last"), anchor: "load-cog", fxN: 0, fyN: r4(-(LOAD_KG + PALLET_KG) * 9.80665 * factor), source: "authored-estimate" }];
const over = (sun = 0): Json => ({ boots: { exposedFraction: 1, sunlitFraction: sun, contact: { fraction: 0.3, tempC: 14 } } });
a.poses = [
  { id: "pulling", title: TEXTS.poses.pulling.title, description: TEXTS.poses.pulling.description, bones: pull.bones, ground: "auto", contacts: ["heel-r", "ball-l", "steer-contact", "roller-contact"], thermal: binding(a, 3.0, "pulling-walk", over()), loads: loads(1) },
  { id: "pumping", title: TEXTS.poses.pumping.title, description: TEXTS.poses.pumping.description, bones: pump.bones, ground: "auto", contacts: [...GROUND_CONTACTS, "steer-contact", "roller-contact"], thermal: binding(a, 2.4, "pumping-effort", over()), loads: loads(1) },
  { id: "parked", title: TEXTS.poses.parked.title, description: TEXTS.poses.parked.description, bones: parked.bones, ground: "auto", contacts: [...GROUND_CONTACTS, "steer-contact", "roller-contact", "pallet-feet"], thermal: binding(a, 1.4, "rest-standing", over()), loads: loads(1) },
  { id: "pull-contact", key: true, title: TEXTS.poses.contact.title, description: TEXTS.poses.contact.description, bones: pullKey0.bones, ground: "auto", contacts: ["heel-l", "ball-r", "steer-contact", "roller-contact"], thermal: binding(a, 3.0, "pulling-walk", over()), loads: loads(1) },
];

/** 🎞️ Clips: the pulling walk on the courier's planted gait keys (torso leaning 9 degrees into the pull, right arm solved on the grip at every key, left arm swinging against the legs) and the pumping clip that raises the forks in three strokes. */
let worst = 0;
const walkKeys = pushWalk.keys.map((k: Json) => {
  const tm = k.t;
  const torso = k.bones.torso.angleDeg - 3;
  const p = place({ ...legs(k.bones), neck: { angleDeg: 4 }, head: { angleDeg: 2 } }, { reach: REACH, tillerDeg: PULL_TILLER, lift: PULL_LIFT, contacts: GROUND_CONTACTS, arms: [[ARM_R, "tiller-grip", BEHIND]], free: walkKey(k), torso, tillerFree: 10 });
  worst = Math.max(worst, p.residual);
  if (p.residual > 0.002) console.log("[DEBUG] key", tm ?? k?.t, p.residual.toFixed(4));
  return { t: k.t, pose: "pull-contact", ease: "smooth", bones: p.bones };
});
const strokes = [0, 0.6, 1.2, 1.8, 2.4, 3.0, 3.6, 4.4];
const tillerAt = [-15, -40, -15, -40, -15, -40, -15, -40];
const liftAt = [0, 0, 0.02, 0.02, 0.04, 0.04, 0.06, 0.06];
const pumpKeys = strokes.map((tm, i) => {
  const k = undefined as Json;
  const p = facing(tillerAt[i]!, liftAt[i]!, tillerAt[i]! > -25 ? -12 : -6);
  worst = Math.max(worst, p.residual);
  if (p.residual > 0.002) console.log("[DEBUG] key", tm ?? k?.t, p.residual.toFixed(4));
  return { t: tm, pose: "pumping", ease: "smooth", bones: p.bones };
});
console.log("[DEBUG] worst clip residual", worst.toFixed(5));
a.clips = [
  { id: "pull-walk", title: TEXTS.clips.walk.title, description: TEXTS.clips.walk.description, durationS: pushWalk.durationS, fps: 30, loop: true, rootMotion: "plant", advanceM: pushWalk.advanceM, keys: walkKeys, contacts: pushWalk.contacts, phases: pushWalk.phases, events: pushWalk.events, tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 3.0 }, { t: 1.2, v: 3.0 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }, { t: 1.2, v: 1 }] }, { target: "externalWorkW", ease: "smooth", keys: [{ t: 0, v: 55 }, { t: 1.2, v: 55 }] }] },
  { id: "pump-lift", title: TEXTS.clips.pump.title, description: TEXTS.clips.pump.description, durationS: 4.4, fps: 30, loop: false, rootMotion: "none", keys: pumpKeys, contacts: [...GROUND_CONTACTS, "steer-contact", "roller-contact"].map((anchor) => ({ anchor, fromS: 0, toS: 4.4 })), phases: [{ id: "pumping", fromS: 0, toS: 3.6, title: t("Pumping the tiller", "Deichsel pumpen") }, { id: "ready", fromS: 3.6, toS: 4.4, title: t("Tiller lowered to pull", "Deichsel zum Ziehen gesenkt") }], events: [{ t: 1.2, kind: "motion", title: t("First stroke: forks 20 mm up", "Erster Hub: Gabeln 20 mm oben"), target: "forks" }, { t: 3.6, kind: "load-on", title: t("Pallet lifted clear of the floor", "Palette vom Boden abgehoben"), target: "pallet-load-weight" }], tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 2 }, { t: 3.6, v: 0 }, { t: 4.4, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 2.4 }, { t: 3.6, v: 2.4 }, { t: 4.4, v: 1.6 }] }, { target: "externalWorkW", ease: "smooth", keys: [{ t: 0, v: 20 }, { t: 3.6, v: 20 }, { t: 4.4, v: 0 }] }] },
];

/** 🌡️ Indoor warehouse environment, respiration patterns of pulling and pumping, plume kept. */
a.thermal.environment = { airTempC: 15, meanRadiantTempC: 14, skyTempC: 14, groundTempC: 14, relativeHumidity: 0.5, windMS: 0.2, windDirectionDeg: 180, solarWM2: 0, diffuseFraction: 1, pressurePa: 101325 };
a.thermal.baseMet = 1.4;
a.thermal.body.basalMetabolismW = 85.1;
a.thermal.respiration.patterns = TEXTS.patterns;
a.thermal.assumptions = TEXTS.thermalAssumptions;
a.structure.assumptions = TEXTS.structureAssumptions;

Object.assign(a, { id: "worker-with-pallet-jack", title: TEXTS.title, summary: TEXTS.summary, tags: ["human", "adult", "man", "warehouse", "logistics", "pallet-truck", "pulling", "load", "workwear", "walking"], provenance: TEXTS.provenance(a.provenance), presentation: { defaultPose: "pulling", defaultClip: "pull-walk", defaultScale: 100 } });
for (const r of a.thermal.regions) if (TEXTS.regionTitles[r.id]) r.title = TEXTS.regionTitles[r.id];
for (const s of ["50", "100", "200", "500"]) {
  const rep = a.representations[s];
  rep.decisions = TEXTS.repDecisions[s];
  const vol = a.volumes[s];
  vol.decisions = [...vol.decisions.filter((d: Json) => !/truck|Karre|parcel|Paket|handle|Griff|carton|Karton/iu.test(d.en + d.de)), ...TEXTS.volDecisions[s]];
}
for (const s of SCALES_FRONT) for (const shape of a.representations[s].geometry.front) if (TRUCK_BONES.has(shape.bone)) shape.z = Math.round((0.2 + shape.z * 0.2) * 1000) / 1000;
for (const s of SCALES_FRONT) a.representations[s].geometry.front.sort((p: Json, q: Json) => p.z - q.z);
pruneMaterials(a);
setDimensions(a, rigMass(a));
console.log("[DEBUG] dimensions", JSON.stringify(a.dimensions), "extents", JSON.stringify(extents(a)));
console.log("[DEBUG] wrote", writeHuman(a));
