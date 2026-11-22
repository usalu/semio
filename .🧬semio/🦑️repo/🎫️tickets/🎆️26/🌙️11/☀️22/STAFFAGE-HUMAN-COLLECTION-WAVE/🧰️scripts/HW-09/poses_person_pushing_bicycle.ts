/** 🚲️ HW-09: poses and clips of `person-pushing-bicycle`. Legs, trunk and pelvis height come from the planted-foot gait keys of
 * parent-with-stroller (scaled by 1.03); the arms are solved so the left hand holds the left grip and the right hand the bar beside
 * the stem. Writes poses, clips, advance and default-pose dimensions. Run after build_person_pushing_bicycle.py.
 * @see ./kin.ts */
import { loadHuman, saveHuman, solve, stateOf, pointOf, tidy, type PoseBones, type Var, type Target } from "./kin.ts";
import { evaluate, viewExtent } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const SLUG = "person-pushing-bicycle";
const K = 1.03;
const { doc, asset } = await loadHuman(SLUG);
const parent = (await loadHuman("parent-with-stroller")).asset;
asset.poses = [];
asset.clips = [];
armSolids();
/** 💪️ The arms reach across to the bar, so their flat drawings get lateral solids: elevation shapes of upper arm, forearm, cuff and hand
 * sweep the rest lateral span of their 3D body solids; the sleeve seams (open texture strokes) are left out. */
function armSolids(): void {
  const rest = stateOf(asset, {});
  for (const [s, rep] of Object.entries<any>(asset.representations)) {
    for (const v of ["elevation", "front", "plan"]) rep.geometry[v] = rep.geometry[v].filter((x: any) => !/^(upper-arm|forearm)-[lr]-seam-/.test(x.id));
    for (const x of rep.geometry.elevation) {
      if (!/^(upper-arm|forearm|hand)-[lr]$/.test(x.bone) || x.solid) continue;
      const ws: number[] = [];
      for (const v of asset.volumes[s].body) {
        if (v.bone !== x.bone) continue;
        if (v.kind === "loft") for (const sec of v.sections) ws.push(sec.at[2] - sec.ry, sec.at[2] + sec.ry);
        if (v.kind === "ellipsoid") ws.push((v.transform?.translation?.[2] ?? 0) - v.radii[2], (v.transform?.translation?.[2] ?? 0) + v.radii[2]);
        if (v.kind === "capsule") ws.push(v.p0[2] - v.r0, v.p0[2] + v.r0, v.p1[2] - v.r1, v.p1[2] + v.r1);
      }
      const z = rest.bones[x.bone].frame.o.z;
      const [lo, hi] = ws.length ? [Math.min(...ws), Math.max(...ws)] : [-0.04, 0.04];
      x.solid = { kind: "extruded", sweep: "w", z0: Number((z + lo).toFixed(5)), z1: Number((z + hi).toFixed(5)) };
    }
  }
}
const hip = asset.rig.bones.find((b: any) => b.id === "walk-lift");
const BODY = ["torso", "neck", "head", "thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r", "pelvis"];
const GRIP_L = { bone: "steer", local: [-0.193, -0.176, 0.276] as [number, number, number] };
const BAR_R = { bone: "steer", local: [-0.114, 0.017, 0.17] as [number, number, number] };

/** 🧍️ The trunk and leg offsets and the pelvis lift of a parent pose, so the scaled walker stands as the parent did. */
function fromParent(id: string): PoseBones {
  const p = parent.poses.find((x: any) => x.id === id);
  const s = evaluate(parent, { kind: "pose", pose: id });
  const bones: PoseBones = {};
  for (const b of BODY) if (p.bones[b]) bones[b] = { ...p.bones[b] };
  const angle = (p.root?.angleDeg ?? 0) + (p.bones.pelvis?.angleDeg ?? 0);
  if (angle) bones.pelvis = { angleDeg: angle };
  bones["walk-lift"] = { dy: K * s.bones.pelvis.frame.o.y - hip.y };
  bones["walk-shift"] = { dx: K * (s.bones.pelvis.frame.o.x) };
  return bones;
}
const arms: Var[] = ["l", "r"].flatMap((s) => [
  { bone: `upper-arm-${s}`, field: "angleDeg", reg: 0.15, min: 0, max: 80 }, { bone: `upper-arm-${s}`, field: "abductionDeg", reg: 0.05, min: -60, max: 60 }, { bone: `upper-arm-${s}`, field: "twistDeg", reg: 0.2, min: -50, max: 50 },
  { bone: `forearm-${s}`, field: "angleDeg", reg: 0.05, min: 5, max: 110 }, { bone: `hand-${s}`, field: "angleDeg", reg: 0.1, min: -50, max: 50 }, { bone: `hand-${s}`, field: "abductionDeg", reg: 0.1, min: -40, max: 40 },
] as Var[]);
let count = 0;
function withHands(bones: PoseBones, root = {}): PoseBones {
  count++;
  const s = stateOf(asset, bones, root);
  const targets: Target[] = [{ point: { anchor: "grip-l" }, at: pointOf(s, GRIP_L), weight: 10 }, { point: { anchor: "grip-r" }, at: pointOf(s, BAR_R), weight: 10 }];
  const start = { ...bones, "upper-arm-l": { angleDeg: 25, abductionDeg: -5 }, "upper-arm-r": { angleDeg: 25, abductionDeg: 10 }, "forearm-l": { angleDeg: 35 }, "forearm-r": { angleDeg: 35 } };
  const r = solve(asset, start, arms, targets, root, 300);
  if (r.worst > 0.002) console.log(`[DEBUG] pose ${count} hands residual ${r.worst.toFixed(4)}`);
  return tidy(r.bones);
}

const REGIONS = asset.thermal.regions.map((r: any) => r.id);
const SUN: Record<string, number> = { "head-skin": 0.5, hair: 0.45, "torso-clothing": 0.42, "arm-clothing": 0.38, "hand-skin": 0.35, "leg-clothing": 0.4, shoes: 0.25, "frame-tubes": 0.5, "steer-assembly": 0.5, mudguards: 0.5, tyres: 0.3, "rims-spokes": 0.4, saddle: 1, drivetrain: 0.4, "hub-brakes": 0.3, lamps: 0.5 };
function thermal(met: number, breathing: string, op: Record<string, number>, plume: number, work = 0) {
  const regions: Record<string, any> = {};
  for (const id of REGIONS) {
    const b: any = { exposedFraction: id === "hair" ? 0.95 : id === "drivetrain" ? 0.7 : id === "hub-brakes" ? 0.8 : 1, sunlitFraction: SUN[id] ?? 0.4 };
    if (id === "tyres") b.contact = { fraction: 0.004, tempC: 14 };
    if (id === "shoes") b.contact = { fraction: 0.175, tempC: 14 };
    if (id === "hand-skin") b.contact = { fraction: 0.3, tempC: 12 };
    regions[id] = b;
  }
  return { activityMet: met, externalWorkW: work, operating: { rolling: 0, pedalling: 0, braking: 0, lighting: 0, ...op }, breathing, regions, emitters: { plume } };
}
const pose = (id: string, en: string, de: string, dEn: string, dDe: string, bones: PoseBones, contacts: string[], th: any, key = false, root = { x: 0, y: 0, angleDeg: 0 }) => ({
  id, title: { en, de }, description: { en: dEn, de: dDe }, ...(key ? { key: true } : {}), bones, root, ground: "auto", contacts, thermal: th, loads: [],
});
const TYRES = ["patch-rear", "patch-front"];
const ROLL = { rolling: 0.3 };

const pushing = withHands(fromParent("pushing-stroller"));
const standing = withHands(fromParent("pausing-with-stroller"));
const parkBones: PoseBones = { ...fromParent("pausing-with-stroller"), stand: { angleDeg: 106.4 } };
const parkRoot = { x: 0, y: 0, angleDeg: -2.02 };
{
  const s = stateOf(asset, parkBones, parkRoot);
  const gy = Math.min(pointOf(s, { anchor: "patch-front" }).y, pointOf(s, { anchor: "stand-foot" }).y);
  const feet: Target[] = ["l", "r"].flatMap((f) => {
    const h = pointOf(s, { anchor: `heel-${f}` }), c = pointOf(s, { anchor: `contact-${f}` });
    return [{ point: { anchor: `heel-${f}` }, at: { x: h.x + 0.04, y: gy, z: h.z }, weight: 10 }, { point: { anchor: `contact-${f}` }, at: { x: c.x + 0.04, y: gy, z: c.z }, weight: 10 }];
  });
  const legs: Var[] = ["l", "r"].flatMap((f) => [{ bone: `thigh-${f}`, field: "angleDeg", reg: 0.02 }, { bone: `shank-${f}`, field: "angleDeg", reg: 0.02, max: 0 }, { bone: `foot-${f}`, field: "angleDeg", reg: 0.02 }] as Var[]);
  parkBones.torso = { angleDeg: -12 };
  const r = solve(asset, { ...parkBones, "thigh-l": { angleDeg: 12 }, "shank-l": { angleDeg: -30 }, "thigh-r": { angleDeg: 4 }, "shank-r": { angleDeg: -8 } }, [...legs, { bone: "walk-lift", field: "dy", reg: 0 }, { bone: "pelvis", field: "angleDeg", reg: 0.2 }], feet, parkRoot, 300);
  if (r.worst > 0.002) console.log(`[DEBUG] park legs residual ${r.worst.toFixed(4)}`);
  Object.assign(parkBones, r.bones);
}
const parked = withHands(parkBones, parkRoot);

const pp = (id: string) => parent.poses.find((x: any) => x.id === id);
asset.poses.push(
  pose("pushing-bicycle", "Pushing the bicycle", "Fahrrad schieben", "Midstance of a steady walk at 1.1 m/s on the left side of the bicycle: left hand on the left grip, right hand on the bar beside the stem, the bicycle rolling on both tyres.", "Mittlere Standphase beim gleichmäßigen Gehen mit 1,1 m/s links neben dem Fahrrad: linke Hand am linken Griff, rechte Hand am Lenker neben dem Vorbau, das Fahrrad rollt auf beiden Reifen.", pushing, [...pp("pushing-stroller").contacts.filter((c: string) => !c.startsWith("wheel")), ...TYRES], thermal(3.0, "pushing", ROLL, 0.8, 1.4)),
  pose("standing-with-bicycle", "Standing with the bicycle", "Mit dem Fahrrad stehen", "Standing still beside the bicycle with both hands on the bar, waiting at a crossing.", "Ruhig neben dem Fahrrad stehen, beide Hände am Lenker, an einer Kreuzung wartend.", standing, ["heel-l", "contact-l", "heel-r", "contact-r", ...TYRES], thermal(1.5, "pausing", {}, 0.9)),
  pose("lifting-onto-stand", "Lifting it onto the stand", "Auf den Ständer bocken", "The centre stand is folded down and the rear wheel lifted clear; the walker stands with knees bent beside the bicycle, hands still on the bar.", "Der Mittelständer ist ausgeklappt und das Hinterrad angehoben; die Person steht mit gebeugten Knien neben dem Fahrrad, die Hände noch am Lenker.", parked, ["heel-l", "contact-l", "heel-r", "contact-r", "patch-front", "stand-foot"], thermal(3.5, "lifting", {}, 0.9, 6), false, parkRoot),
);
const GAIT = ["push-heel-strike-l", "push-toe-off-r", "push-midstance-l", "push-heel-off-l", "push-late-swing-r", "push-heel-strike-r", "push-toe-off-l", "push-midstance-r", "push-heel-off-r", "push-late-swing-l"];
const gaitId = (g: string) => g.replace("push-", "walk-");
for (const g of GAIT) {
  const src = pp(g);
  asset.poses.push(pose(gaitId(g), src.title, src.title.de, `Gait key of the push walk (${src.title.en.toLowerCase()}), hands on the bar.`, `Gangschlüssel des Schiebegangs (${src.title.de}), Hände am Lenker.`, withHands(fromParent(g)), [...src.contacts.filter((c: string) => !c.startsWith("wheel")), ...TYRES], thermal(3.0, "pushing", ROLL, 0.8, 1.4), true));
}
for (const p of asset.poses) if (typeof p.title === "object" && typeof p.title.en === "object") p.title = p.title.en;
/** 🦶️ The swinging toe of the two late-swing keys is lifted by 1°: with the parent's 0° the toe prism of 1:200 is seen exactly edge-on in the front view at the eighths of the loop and the polygon union of the engine fails there (`unionPolygons`, see requests.md of the umbrella ticket ☀️22/GET-STAFFAGE-RUNNING-END-TO-END-…). */
for (const [id, toe] of [["walk-late-swing-l", "toe-l"], ["walk-late-swing-r", "toe-r"]] as const) {
  const key = asset.poses.find((p: any) => p.id === id);
  key.bones[toe] = { ...key.bones[toe], angleDeg: (key.bones[toe]?.angleDeg ?? 0) + 1 };
}

const src = parent.clips.find((c: any) => c.id === "push-walk");
const STRIDES = 21, TURNS = 10, T0 = src.durationS, DUR = Number((STRIDES * T0).toFixed(4));
const keys: any[] = [];
for (let c = 0; c < STRIDES; c++) for (const k of src.keys.slice(0, -1)) {
  const t = Number((c * T0 + k.t).toFixed(4));
  keys.push({ t, pose: gaitId(k.pose), ease: "smooth", bones: { "wheel-f": { angleDeg: Number((-360 * TURNS * t / DUR).toFixed(3)) }, "wheel-r": { angleDeg: Number((-360 * TURNS * t / DUR).toFixed(3)) } } });
}
keys.push({ t: DUR, pose: gaitId(src.keys[0].pose), ease: "smooth", bones: { "wheel-f": { angleDeg: -360 * TURNS }, "wheel-r": { angleDeg: -360 * TURNS } } });
const shiftSpans = (list: any[]) => Array.from({ length: STRIDES }, (_, c) => list.map((x) => ({ ...x, fromS: Number((x.fromS + c * T0).toFixed(4)), toS: Number((x.toS + c * T0).toFixed(4)) }))).flat();
const walk: any = {
  id: "push-walk", title: { en: "Pushing walk", de: "Schiebegang" },
  description: { en: "Twenty-one strides of a steady walk at 1.1 m/s beside the bicycle, hands on the bar; the planted feet carry the figure forward and the wheels roll without slip, ten turns per loop.", de: "Einundzwanzig Schritte gleichmäßigen Gehens mit 1,1 m/s neben dem Fahrrad, Hände am Lenker; die aufgesetzten Füße tragen die Figur voran und die Räder rollen ohne Schlupf, zehn Umdrehungen je Schleife." },
  durationS: DUR, fps: 30, loop: true, rootMotion: "plant", advanceM: 0,
  keys,
  contacts: shiftSpans(src.contacts),
  phases: shiftSpans(src.phases).map((p: any, i: number) => ({ ...p, id: `${p.id}-${Math.floor(i / src.phases.length) + 1}` })),
  events: shiftSpans(src.events.map((e: any) => ({ ...e, fromS: e.t, toS: e.t }))).map(({ fromS, toS, ...e }: any) => ({ ...e, t: fromS })),
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 1 }] }, { target: "emitter:plume", ease: "smooth", keys: [{ t: 0, v: 0.8 }, { t: DUR, v: 0.8 }] }, { target: "operating:rolling", ease: "smooth", keys: [{ t: 0, v: 0.3 }, { t: DUR, v: 0.3 }] }],
};
asset.clips.push(walk);
asset.clips.push({
  id: "park-on-stand", title: { en: "Parking on the centre stand", de: "Auf dem Mittelständer abstellen" },
  description: { en: "Standing beside the bicycle, the walker folds the centre stand down and lifts the rear until the stand feet carry it; the rear tyre leaves the ground.", de: "Neben dem Fahrrad stehend klappt die Person den Mittelständer aus und hebt das Heck an, bis die Ständerfüße es tragen; der Hinterreifen verlässt den Boden." },
  durationS: 4, fps: 30, loop: false, rootMotion: "none",
  keys: [{ t: 0, pose: "standing-with-bicycle", ease: "smooth" }, { t: 1, pose: "standing-with-bicycle", ease: "smooth" }, { t: 3, pose: "lifting-onto-stand", ease: "smooth" }, { t: 4, pose: "lifting-onto-stand", ease: "smooth" }],
  contacts: [{ anchor: "heel-l", fromS: 0, toS: 4 }, { anchor: "contact-l", fromS: 0, toS: 4 }, { anchor: "heel-r", fromS: 0, toS: 4 }, { anchor: "contact-r", fromS: 0, toS: 4 }, { anchor: "patch-front", fromS: 0, toS: 4 }, { anchor: "patch-rear", fromS: 0, toS: 2.6 }, { anchor: "stand-foot", fromS: 2.6, toS: 4 }],
  phases: [{ id: "standing", fromS: 0, toS: 1, title: { en: "Standing", de: "Stehen" } }, { id: "lifting", fromS: 1, toS: 3, title: { en: "Stand down, rear lifted", de: "Ständer aus, Heck angehoben" } }, { id: "parked", fromS: 3, toS: 4, title: { en: "Parked on the stand", de: "Auf dem Ständer abgestellt" } }],
  events: [{ t: 2.6, kind: "contact-on", title: { en: "Stand feet touch the ground", de: "Ständerfüße setzen auf" }, target: "stand-foot" }, { t: 2.6, kind: "contact-off", title: { en: "Rear tyre lifts off", de: "Hinterreifen hebt ab" }, target: "patch-rear" }],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: 1, v: 2 }] }],
});
asset.clips.push({
  id: "idle-breathing", title: { en: "Waiting and breathing", de: "Warten und atmen" },
  description: { en: "Standing beside the bicycle at a crossing: calm breathing at 15 breaths per minute, the head turning a little to watch the traffic.", de: "Neben dem Fahrrad an einer Kreuzung stehen: ruhige Atmung mit 15 Atemzügen pro Minute, der Kopf dreht sich ein wenig zum Verkehr." },
  durationS: 4, fps: 30, loop: true, rootMotion: "none",
  keys: [{ t: 0, pose: "standing-with-bicycle", ease: "smooth" }, { t: 2, pose: "standing-with-bicycle", ease: "smooth", bones: { head: { ...(standing.head ?? {}), angleDeg: (standing.head?.angleDeg ?? 0) - 4 } } }, { t: 4, pose: "standing-with-bicycle", ease: "smooth" }],
  contacts: ["heel-l", "contact-l", "heel-r", "contact-r", ...TYRES].map((a) => ({ anchor: a, fromS: 0, toS: 4 })),
  phases: [{ id: "waiting", fromS: 0, toS: 4, title: { en: "Waiting", de: "Warten" } }],
  events: [],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }] }],
});
const a0 = evaluate(asset, { kind: "clip", clip: "push-walk", timeS: 0 }).root.x;
const a1 = evaluate(asset, { kind: "clip", clip: "push-walk", timeS: DUR - 1e-9 }).root.x;
walk.advanceM = Number((a1 - a0).toFixed(5));
console.log("[DEBUG] advance", walk.advanceM, "wheel travel", (TURNS * 2 * Math.PI * 0.345).toFixed(3));

const s = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
const fr = viewExtent(s, "front");
const b = s.bounds;
const mass = asset.rig.bones.reduce((m: number, x: any) => m + (x.mass?.kg ?? 0), 0);
asset.dimensions = { heightM: Number((b.maxY - b.minY).toFixed(3)), widthM: Number((b.maxX - b.minX).toFixed(3)), depthM: Number((fr?.horizontal ?? 0).toFixed(3)), massKg: Number(mass.toFixed(2)) };
console.log("[DEBUG] dimensions", JSON.stringify(asset.dimensions));
await saveHuman(SLUG, doc);
