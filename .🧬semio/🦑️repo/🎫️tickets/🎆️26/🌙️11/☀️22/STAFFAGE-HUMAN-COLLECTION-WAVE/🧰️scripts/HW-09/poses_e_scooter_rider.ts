/** 🛴️ HW-09: solves the poses and clips of `e-scooter-rider` (soles on the deck mat or the ground, gloves on the grips) with the
 * damped least-squares IK of kin.ts, writes them and the default-pose dimensions into the asset. Run after build_e_scooter_rider.py.
 * @see ./kin.ts */
import { loadHuman, saveHuman, solve, stateOf, pointOf, tidy, type PoseBones, type Var, type Target } from "./kin.ts";
import { evaluate, viewExtent, rigBounds } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const SLUG = "e-scooter-rider";
const { doc, asset } = await loadHuman(SLUG);
asset.poses = [];
asset.clips = [];
const MAT = 0.1515;
const GRIP = (side: 1 | -1) => ({ bone: "stem", local: [0.708, -0.024, side * 0.165] as [number, number, number] });

const arm = (s: "l" | "r"): Var[] => [
  { bone: `arm-${s}`, field: "angleDeg", reg: 0.02 }, { bone: `arm-${s}`, field: "abductionDeg", reg: 0.05, min: -30, max: 30 }, { bone: `arm-${s}`, field: "twistDeg", reg: 0.2, min: -25, max: 25 },
  { bone: `forearm-${s}`, field: "angleDeg", reg: 0.02, min: 0, max: 140 }, { bone: `hand-${s}`, field: "angleDeg", reg: 0.1, min: -40, max: 40 }, { bone: `hand-${s}`, field: "abductionDeg", reg: 0.1, min: -20, max: 20 },
];
const leg = (s: "l" | "r"): Var[] => [
  { bone: `thigh-${s}`, field: "angleDeg", reg: 0.01 }, { bone: `thigh-${s}`, field: "abductionDeg", reg: 0.05, min: -30, max: 30 }, { bone: `thigh-${s}`, field: "twistDeg", reg: 0.3, min: -20, max: 20 },
  { bone: `shank-${s}`, field: "angleDeg", reg: 0.01, min: -140, max: 0 }, { bone: `foot-${s}`, field: "angleDeg", reg: 0.01, min: -55, max: 40 },
];
const sole = (s: "l" | "r", ballX: number, z: number, y = MAT, heelX?: number): Target[] => [
  { point: { anchor: `ball-${s}` }, at: { x: ballX, y, z } },
  { point: { anchor: `heel-${s}` }, at: { y, z, ...(heelX === undefined ? {} : { x: heelX }) } },
];
const grips = (): Target[] => [
  { point: { anchor: "grip-l" }, at: pointAtRest(GRIP(1)) },
  { point: { anchor: "grip-r" }, at: pointAtRest(GRIP(-1)) },
];
function pointAtRest(p: { bone: string; local: [number, number, number] }) {
  return pointOf(stateOf(asset, {}), p);
}

type Spec = { trunk: PoseBones; hip: { dy: number; dx: number }; feet: Target[]; legs: Var[]; hands?: Target[]; extra?: Var[]; start?: PoseBones };
let n = 0;
function pose(spec: Spec): PoseBones {
  n++;
  let bones: PoseBones = { ...spec.start, ...spec.trunk, "hip-lift": { dy: spec.hip.dy }, "hip-shift": { dx: spec.hip.dx } };
  for (const s of ["l", "r"]) bones[`shank-${s}`] ??= { angleDeg: -12 }, bones[`foot-${s}`] ??= { angleDeg: 6 };
  bones = JSON.parse(JSON.stringify(bones));
  const a = solve(asset, bones, [...spec.legs, ...(spec.extra ?? [])], spec.feet.map((t) => ({ ...t, weight: 10 })), {}, 300);
  if (a.worst > 0.002) console.log(`[DEBUG] pose ${n} legs residual ${a.worst.toFixed(4)}`);
  bones = a.bones;
  if (spec.hands) {
    bones["arm-l"] ??= { angleDeg: 25, abductionDeg: 8 }; bones["arm-r"] ??= { angleDeg: 25, abductionDeg: -8 };
    bones["forearm-l"] ??= { angleDeg: 35 }; bones["forearm-r"] ??= { angleDeg: 35 };
    const h = solve(asset, bones, [...arm("l"), ...arm("r")], spec.hands.map((t) => ({ ...t, weight: 10 })), {}, 300);
    if (h.worst > 0.002) console.log(`[DEBUG] pose ${n} hands residual ${h.worst.toFixed(4)}`);
    bones = h.bones;
  }
  return tidy(bones);
}

const REGIONS = asset.thermal.regions.map((r: any) => r.id);
const SUN: Record<string, number> = { "face-skin": 0.5, hair: 0.3, helmet: 0.9, gloves: 0.5, jacket: 0.45, trousers: 0.35, shoes: 0.3, "deck-battery": 0.2, "deck-grip": 0.4, "bms-controller": 0, "hub-motor": 0.2, "stem-bars": 0.5, fenders: 0.5, tyres: 0.2, "brake-disc": 0.1, "rear-rim": 0.2 };
const EXPOSED: Record<string, number> = { "face-skin": 1, hair: 0.3, helmet: 1, gloves: 0.8, jacket: 0.95, trousers: 0.95, shoes: 0.85, "deck-battery": 0.85, "deck-grip": 0.45, "bms-controller": 0.1, "hub-motor": 0.5, "stem-bars": 0.95, fenders: 1, tyres: 1, "brake-disc": 0.7, "rear-rim": 1 };
function thermal(met: number, breathing: string, op: Record<string, number>, plume: number, footDown: boolean) {
  const regions: Record<string, any> = {};
  for (const id of REGIONS) {
    const b: any = { exposedFraction: EXPOSED[id] ?? 1, sunlitFraction: SUN[id] ?? 0.3 };
    if (id === "tyres") b.contact = { fraction: 0.003, tempC: 21 };
    if (id === "gloves") b.contact = { fraction: 0.2, tempC: 18 };
    if (id === "shoes") b.contact = { fraction: footDown ? 0.12 : 0.2, tempC: footDown ? 21 : 19 };
    regions[id] = b;
  }
  return { activityMet: met, externalWorkW: 0, operating: { drive: 0, rolling: 0, braking: 0, charging: 0, ...op }, breathing, regions, emitters: { plume } };
}
const named = (id: string, en: string, de: string, dEn: string, dDe: string, bones: PoseBones, contacts: string[], th: any, key = false) => ({
  id, title: { en, de }, description: { en: dEn, de: dDe }, ...(key ? { key: true } : {}), bones, root: { x: 0, y: 0, angleDeg: 0 }, ground: "auto", contacts, thermal: th, loads: [],
});

const UPRIGHT: PoseBones = { pelvis: { angleDeg: 6 }, torso: { angleDeg: 8 }, neck: { angleDeg: 12 }, head: { angleDeg: -13 } };
const legsBoth = [...leg("l"), ...leg("r"), { bone: "hip-lift", field: "dy", reg: 0 } as Var];
const legL = [...leg("l"), ...leg("r"), { bone: "hip-lift", field: "dy", reg: 0 } as Var, { bone: "hip-shift", field: "dx", reg: 0.5 } as Var];
const tyres = ["contact-f", "contact-r"];

const cruising = pose({ trunk: UPRIGHT, hip: { dy: -0.045, dx: 0.02 }, feet: [...sole("l", 0.16, 0.022), ...sole("r", -0.1, -0.022)], legs: legsBoth, hands: grips() });
const swayL = pose({ trunk: { ...UPRIGHT, torso: { angleDeg: 9.5 } }, hip: { dy: -0.05, dx: 0.035 }, feet: [...sole("l", 0.16, 0.022), ...sole("r", -0.1, -0.022)], legs: legsBoth, hands: grips() });
const swayR = pose({ trunk: { ...UPRIGHT, torso: { angleDeg: 6.5 }, head: { angleDeg: -11 } }, hip: { dy: -0.04, dx: 0.005 }, feet: [...sole("l", 0.16, 0.022), ...sole("r", -0.1, -0.022)], legs: legsBoth, hands: grips() });
const braking = pose({ trunk: { pelvis: { angleDeg: 2 }, torso: { angleDeg: 4 }, neck: { angleDeg: 14 }, head: { angleDeg: -8 } }, hip: { dy: -0.07, dx: -0.06 }, feet: [...sole("l", 0.17, 0.024), ...sole("r", -0.11, -0.024)], legs: legL, hands: grips() });
const waiting = pose({ trunk: { pelvis: { angleDeg: 8 }, torso: { angleDeg: 8 }, neck: { angleDeg: 8 }, head: { angleDeg: -10 } }, hip: { dy: -0.06, dx: 0.0 }, feet: [...sole("l", 0.14, 0.03), ...sole("r", 0.02, -0.27, 0)], legs: legL, hands: grips(), start: { "thigh-r": { abductionDeg: -14 }, "shank-r": { angleDeg: -4 }, "thigh-l": { angleDeg: 4 }, "shank-l": { angleDeg: -25 } } });
const kickPlant = pose({ trunk: { pelvis: { angleDeg: 6 }, torso: { angleDeg: 6 }, neck: { angleDeg: 6 }, head: { angleDeg: -6 } }, hip: { dy: -0.1, dx: 0.06 }, feet: [...sole("l", 0.16, 0.03), ...sole("r", 0.0, -0.13, 0)], legs: legL, hands: grips(), start: { "thigh-r": { abductionDeg: -6 }, "shank-r": { angleDeg: -4 }, "thigh-l": { angleDeg: 20 }, "shank-l": { angleDeg: -40 } } });
const kickPush = pose({ trunk: { pelvis: { angleDeg: 4 }, torso: { angleDeg: 4 }, neck: { angleDeg: 10 }, head: { angleDeg: -6 } }, hip: { dy: -0.1, dx: 0.08 }, feet: [...sole("l", 0.16, 0.03), { point: { anchor: "ball-r" }, at: { x: -0.36, y: 0, z: -0.13 } }, { point: { anchor: "heel-r" }, at: { x: -0.5, z: -0.13 } }, { point: { bone: "toe-r", local: [0.0683, 0, 0] }, at: { y: 0.016 } }], legs: [...legL, { bone: "toe-r", field: "angleDeg", reg: 0.01, min: 0, max: 60 } as Var], hands: grips(), start: { "thigh-r": { angleDeg: -15, abductionDeg: -5 }, "shank-r": { angleDeg: -10 }, "foot-r": { angleDeg: 25 }, "thigh-l": { angleDeg: 25 }, "shank-l": { angleDeg: -45 } } });
const kickLift = pose({ trunk: { pelvis: { angleDeg: 7 }, torso: { angleDeg: 8 }, neck: { angleDeg: 6 }, head: { angleDeg: -7 } }, hip: { dy: -0.05, dx: 0.05 }, feet: [...sole("l", 0.16, 0.025), { point: { anchor: "ball-r" }, at: { x: -0.2, y: 0.24, z: -0.06 } }, { point: { anchor: "heel-r" }, at: { x: -0.37, y: 0.25, z: -0.06 } }], legs: legL, hands: grips() });

const RIDE = { drive: 0.4, rolling: 0.8 };
const D = (en: string, de: string) => [en, de] as const;
asset.poses.push(
  named("cruising", "Cruising at 20 km/h", "Fahrt mit 20 km/h", "Standing upright on the deck, left foot ahead and right foot behind on the grip mat, knees slightly soft, both gloved hands on the grips; the hub motor drives at cruising power.", "Aufrecht auf dem Trittbrett, linker Fuß vorn und rechter Fuß hinten auf der Trittmatte, Knie leicht gebeugt, beide behandschuhten Hände an den Griffen; der Nabenmotor treibt mit Reiseleistung an.", cruising, tyres, thermal(1.8, "cruising", RIDE, 0.25, false)),
  named("kicking-off", "Kicking off", "Abstoßen", "The right foot pushes back against the road beside the rear wheel while the left stands on the deck; the motor engages only above 5 km/h, so the start is a human push.", "Der rechte Fuß stößt neben dem Hinterrad gegen die Fahrbahn, während der linke auf dem Trittbrett steht; der Motor setzt erst über 5 km/h ein, daher ist der Start ein menschlicher Abstoß.", kickPush, [...tyres, "ball-r"], thermal(3.0, "kicking", { drive: 0, rolling: 0.15 }, 0.6, true)),
  named("waiting-foot-down", "Waiting at a red light", "Warten an der Ampel", "Standstill with the left foot on the deck and the right foot on the road beside it, hands on the grips, the scooter upright in standby.", "Stillstand mit dem linken Fuß auf dem Trittbrett und dem rechten Fuß daneben auf der Fahrbahn, Hände an den Griffen, der Roller aufrecht in Bereitschaft.", waiting, [...tyres, "heel-r", "ball-r"], thermal(1.3, "waiting", { drive: 0.05 }, 0.8, true)),
  named("braking", "Braking", "Bremsen", "Weight shifted back and down over the rear foot, arms straightened against the bar while the rear disc brake slows the scooter.", "Gewicht nach hinten und unten über den hinteren Fuß verlagert, Arme gegen den Lenker gestreckt, während die hintere Scheibenbremse den Roller verzögert.", braking, tyres, thermal(2.0, "cruising", { rolling: 0.5, braking: 1 }, 0.3, false)),
);
asset.poses.push(
  named("cruise-sway-forward", "Sway forward", "Pendeln nach vorn", "Key of the ride loop: the body drifts a little forward over the front foot.", "Schlüssel der Fahrschleife: der Körper wandert etwas über den vorderen Fuß.", swayL, tyres, thermal(1.8, "cruising", RIDE, 0.25, false), true),
  named("cruise-sway-back", "Sway back", "Pendeln nach hinten", "Key of the ride loop: the body settles a little back over the rear foot.", "Schlüssel der Fahrschleife: der Körper setzt sich etwas über den hinteren Fuß.", swayR, tyres, thermal(1.8, "cruising", RIDE, 0.25, false), true),
  named("kick-plant", "Foot planted to kick", "Fuß zum Abstoßen aufgesetzt", "Key of the kick start: the right foot is set down beside the deck under the hip.", "Schlüssel des Anfahrens: der rechte Fuß steht neben dem Trittbrett unter der Hüfte.", kickPlant, [...tyres, "heel-r", "ball-r"], thermal(3.0, "kicking", { drive: 0, rolling: 0 }, 0.6, true), true),
  named("kick-lift", "Kicking foot lifted", "Stoßfuß angehoben", "Key of the kick start: the right foot swings forward to its place on the deck.", "Schlüssel des Anfahrens: der rechte Fuß schwingt nach vorn auf seinen Platz auf dem Trittbrett.", kickLift, tyres, thermal(2.4, "kicking", { drive: 0.6, rolling: 0.3 }, 0.4, false), true),
);

const R = 0.108, V = 20 / 3.6, LOOP = 2.9292;
const turns = (t: number) => (-(V * t) / R) * (180 / Math.PI);
const wheelKey = (t: number) => ({ "wheel-f": { angleDeg: Number(turns(t).toFixed(3)) }, "wheel-r": { angleDeg: Number(turns(t).toFixed(3)) } });
const quarter = LOOP / 4;
asset.clips.push({
  id: "ride-cruise", title: { en: "Cruising loop at 20 km/h", de: "Fahrschleife mit 20 km/h" },
  description: { en: "Four seconds of steady riding: the wheels roll without slip (theta = -x / r, 24 turns per loop of 16.3 m) and the rider sways gently over the front and the rear foot; the scooter stays in place and the road runs by.", de: "Knapp drei Sekunden gleichmäßiges Fahren: die Räder rollen ohne Schlupf (theta = -x / r, 24 Umdrehungen je Schleife von 16,3 m) und die Person pendelt leicht über den vorderen und den hinteren Fuß; der Roller bleibt stehen und die Fahrbahn läuft vorbei." },
  durationS: LOOP, fps: 30, loop: true, rootMotion: "none",
  keys: [
    { t: 0, pose: "cruising", ease: "smooth", bones: wheelKey(0) },
    { t: Number(quarter.toFixed(4)), pose: "cruise-sway-forward", ease: "smooth", bones: wheelKey(quarter) },
    { t: Number((2 * quarter).toFixed(4)), pose: "cruising", ease: "smooth", bones: wheelKey(2 * quarter) },
    { t: Number((3 * quarter).toFixed(4)), pose: "cruise-sway-back", ease: "smooth", bones: wheelKey(3 * quarter) },
    { t: LOOP, pose: "cruising", ease: "smooth", bones: { "wheel-f": { angleDeg: -8640 }, "wheel-r": { angleDeg: -8640 } } },
  ],
  contacts: [{ anchor: "contact-f", fromS: 0, toS: LOOP }, { anchor: "contact-r", fromS: 0, toS: LOOP }],
  phases: [{ id: "cruise", fromS: 0, toS: LOOP, title: { en: "Steady cruise", de: "Gleichmäßige Fahrt" } }],
  events: [{ t: 0, kind: "operating", title: { en: "Motor at cruising power", de: "Motor mit Reiseleistung" }, target: "drive" }],
  tracks: [{ target: "operating:drive", ease: "smooth", keys: [{ t: 0, v: 0.4 }, { t: LOOP, v: 0.4 }] }, { target: "operating:rolling", ease: "smooth", keys: [{ t: 0, v: 0.8 }, { t: LOOP, v: 0.8 }] }, { target: "emitter:plume", ease: "smooth", keys: [{ t: 0, v: 0.25 }, { t: LOOP, v: 0.25 }] }],
});
const kickWheel = (x: number) => ({ "wheel-f": { angleDeg: Number(((-x / R) * 180 / Math.PI).toFixed(3)) }, "wheel-r": { angleDeg: Number(((-x / R) * 180 / Math.PI).toFixed(3)) } });
asset.clips.push({
  id: "kick-start", title: { en: "Kick start from the kerb", de: "Anfahren mit Abstoß" },
  description: { en: "From waiting with one foot down the rider sets the right foot under the hip, pushes once against the road, swings the foot onto the deck and the motor takes over; the wheels roll by the distance travelled.", de: "Aus dem Warten mit einem Fuß am Boden setzt die Person den rechten Fuß unter die Hüfte, stößt sich einmal von der Fahrbahn ab, schwingt den Fuß auf das Trittbrett und der Motor übernimmt; die Räder rollen um die gefahrene Strecke." },
  durationS: 3.2, fps: 30, loop: false, rootMotion: "none",
  keys: [
    { t: 0, pose: "waiting-foot-down", ease: "smooth", bones: kickWheel(0) },
    { t: 0.8, pose: "waiting-foot-down", ease: "smooth", bones: kickWheel(0) },
    { t: 1.3, pose: "kick-plant", ease: "smooth", bones: kickWheel(0.02) },
    { t: 1.75, pose: "kicking-off", ease: "smooth", bones: kickWheel(0.4) },
    { t: 2.2, pose: "kick-lift", ease: "smooth", bones: kickWheel(1.1) },
    { t: 3.2, pose: "cruising", ease: "smooth", bones: kickWheel(3.2) },
  ],
  contacts: [{ anchor: "contact-f", fromS: 0, toS: 3.2 }, { anchor: "contact-r", fromS: 0, toS: 3.2 }, { anchor: "heel-r", fromS: 0, toS: 1.45 }, { anchor: "ball-r", fromS: 0, toS: 1.9 }],
  phases: [{ id: "waiting", fromS: 0, toS: 0.8, title: { en: "Waiting", de: "Warten" } }, { id: "kick", fromS: 0.8, toS: 1.9, title: { en: "Kick against the road", de: "Abstoß von der Fahrbahn" } }, { id: "glide", fromS: 1.9, toS: 3.2, title: { en: "Foot onto the deck, motor engages", de: "Fuß aufs Trittbrett, Motor setzt ein" } }],
  events: [{ t: 1.9, kind: "contact-off", title: { en: "Right foot leaves the road", de: "Rechter Fuß verlässt die Fahrbahn" }, target: "ball-r" }, { t: 2.2, kind: "operating", title: { en: "Motor engages", de: "Motor setzt ein" }, target: "drive" }],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 1.3, v: 2 }] }],
});
asset.clips.push({
  id: "idle-breathing", title: { en: "Waiting and breathing", de: "Warten und atmen" },
  description: { en: "Waiting at a red light with the right foot on the road: calm breathing at 13 breaths per minute moves the chest, the head turns slightly to watch the signal.", de: "Warten an der Ampel mit dem rechten Fuß auf der Fahrbahn: ruhige Atmung mit 13 Atemzügen pro Minute bewegt den Brustkorb, der Kopf dreht sich leicht zur Ampel." },
  durationS: 4.6154, fps: 30, loop: true, rootMotion: "none",
  keys: [
    { t: 0, pose: "waiting-foot-down", ease: "smooth" },
    { t: 2.3077, pose: "waiting-foot-down", ease: "smooth", bones: { head: { ...(waiting.head ?? {}), angleDeg: (waiting.head?.angleDeg ?? 0) + 4 } } },
    { t: 4.6154, pose: "waiting-foot-down", ease: "smooth" },
  ],
  contacts: [{ anchor: "contact-f", fromS: 0, toS: 4.6154 }, { anchor: "contact-r", fromS: 0, toS: 4.6154 }, { anchor: "heel-r", fromS: 0, toS: 4.6154 }, { anchor: "ball-r", fromS: 0, toS: 4.6154 }],
  phases: [{ id: "waiting", fromS: 0, toS: 4.6154, title: { en: "Waiting at the light", de: "Warten an der Ampel" } }],
  events: [],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }] }],
});

const s = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
const fr = viewExtent(s, "front"), pl = viewExtent(s, "plan");
const b = s.bounds;
console.log("[DEBUG] elevation", (b.maxX - b.minX).toFixed(3), (b.maxY - b.minY).toFixed(3), "front", fr?.horizontal.toFixed(3), fr?.vertical.toFixed(3), "plan", pl?.horizontal.toFixed(3), pl?.vertical.toFixed(3));
const mass = asset.rig.bones.reduce((m: number, x: any) => m + (x.mass?.kg ?? 0), 0);
asset.dimensions = { heightM: Number((b.maxY - b.minY).toFixed(3)), widthM: Number((b.maxX - b.minX).toFixed(3)), depthM: Number((fr?.horizontal ?? 0).toFixed(3)), massKg: Number(mass.toFixed(2)) };
console.log("[DEBUG] dimensions", JSON.stringify(asset.dimensions));
await saveHuman(SLUG, doc);
