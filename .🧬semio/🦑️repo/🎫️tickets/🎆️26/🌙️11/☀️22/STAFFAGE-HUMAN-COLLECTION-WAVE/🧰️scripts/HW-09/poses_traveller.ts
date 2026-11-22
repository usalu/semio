/** 🧳️ HW-09: poses and clips of `traveller-with-rolling-suitcase`. The walk keys of walking-commuter drive legs, trunk and right arm;
 * for every key the left arm and the trolley (lift, slide, tilt) are solved together so the left hand holds the grip and the wheel
 * patches stay on the ground; the wheels roll by the planted travel. Run after build_traveller.py.
 * @see ./kin.ts */
import { loadHuman, saveHuman, solve, stateOf, pointOf, tidy, type PoseBones, type Var, type Target } from "./kin.ts";
import { evaluate, viewExtent } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript/🟦️.ts";

const SLUG = "traveller-with-rolling-suitcase";
const { doc, asset } = await loadHuman(SLUG);
const com = (await loadHuman("walking-commuter")).asset;
const R = asset.rig.bones.find((b: any) => b.id === "case-wheels").lengthM;
const CASE = ["case-lift", "case-slide", "suitcase", "case-wheels"];
const FREE = new Set(["arm-l", "forearm-l", "hand-l", ...CASE]);
const strip = (b: PoseBones): PoseBones => Object.fromEntries(Object.entries(b).filter(([k]) => !FREE.has(k)));

const ARM: Var[] = [
  { bone: "arm-l", field: "angleDeg", reg: 0.08, min: -50, max: 60 }, { bone: "forearm-l", field: "angleDeg", reg: 0.05, min: 0, max: 120 }, { bone: "hand-l", field: "angleDeg", reg: 0.15, min: -50, max: 50 },
];
const CASEVARS = (tilt: boolean): Var[] => [{ bone: "case-lift", field: "dx", reg: 0 }, { bone: "case-slide", field: "dx", reg: 0 }, ...(tilt ? [{ bone: "suitcase", field: "angleDeg", reg: 0.02, min: -70, max: 10 } as Var] : [])];

/** 🧲️ The root a pose of the body stands on (ground auto on its contacts). */
function rootOf(bones: PoseBones, contacts: string[]): { x: number; y: number; angleDeg: number } {
  const probe = { id: "__root", title: { en: "r", de: "r" }, description: { en: "r", de: "r" }, bones, ground: "auto", contacts, thermal: asset.poses[0]?.thermal ?? { regions: {}, emitters: {} }, loads: [] };
  return evaluate({ ...asset, poses: [...asset.poses, probe] }, { kind: "pose", pose: "__root" }).root;
}
let count = 0;
/** 🎯️ Left arm and trolley for a body configuration: hand on the grip, wheel patch on the ground, trolley trailing or upright. */
function withCase(body: PoseBones, root: any, mode: { tilt: number; free: boolean; caseX?: number }): PoseBones {
  count++;
  const start: PoseBones = { ...body, "arm-l": { angleDeg: mode.caseX === undefined ? -18 : 15 }, "forearm-l": { angleDeg: mode.caseX === undefined ? 8 : 40 }, "hand-l": { angleDeg: 0 }, suitcase: { angleDeg: mode.tilt } };
  const s0 = stateOf(asset, start, root);
  const targets: Target[] = [{ point: { anchor: "case-wheel-patch" }, at: { y: 0 }, weight: 10 }];
  if (mode.caseX !== undefined) targets.push({ point: { bone: "suitcase", local: [0, 0, 0] }, at: { x: pointOf(s0, { bone: "pelvis", local: [0, 0, 0] }).x + mode.caseX }, weight: 1 });
  const c1 = solve(asset, start, CASEVARS(false), [...targets, { point: { anchor: "case-handle" }, at: { x: pointOf(s0, { anchor: "grip-l" }).x }, weight: mode.caseX === undefined ? 1 : 0 }], root, 100);
  const tries = (mode.free ? [0, 10, -10, 20] : [0]).map((dt) => solve(asset, { ...c1.bones, suitcase: { angleDeg: mode.tilt + dt } }, [...ARM, ...CASEVARS(mode.free)], [...targets, { point: { anchor: "grip-l" }, at: {}, meet: { anchor: "case-handle" }, weight: 10 }], root, 400));
  const out = tries.sort((p, q) => p.worst - q.worst)[0]!.bones;
  const s = stateOf(asset, out, root);
  const d = Math.hypot(...(["x", "y", "z"] as const).map((k) => pointOf(s, { anchor: "grip-l" })[k] - pointOf(s, { anchor: "case-handle" })[k]));
  const g = pointOf(s, { anchor: "case-wheel-patch" }).y;
  if (d > 0.003 || Math.abs(g) > 0.002) console.log(`[DEBUG] ${count} grip ${d.toFixed(4)} patch ${g.toFixed(4)}`);
  return tidy(out);
}

const regionIds = asset.thermal.regions.map((r: any) => r.id);
function thermalOf(src: any, met: number, breathing: string) {
  const regions: Record<string, any> = {};
  for (const id of regionIds) regions[id] = src.thermal.regions[id] ?? { exposedFraction: 1, sunlitFraction: 0 };
  regions["handle-and-wheels"] = { exposedFraction: 0.9, sunlitFraction: 0, contact: { fraction: 0.02, tempC: asset.thermal.environment.groundTempC ?? 10 } };
  regions["hand-skin"] = { ...(regions["hand-skin"] ?? {}), exposedFraction: 0.9, sunlitFraction: 0, contact: { fraction: 0.15, tempC: asset.thermal.environment.airTempC } };
  return { ...src.thermal, activityMet: met, breathing, regions, externalWorkW: met > 3 ? 3 : 0 };
}
const pose = (id: string, title: any, description: any, bones: PoseBones, contacts: string[], thermal: any, key = false) => ({ id, title, description, ...(key ? { key: true } : {}), bones, ground: "auto", contacts, thermal, loads: [] });
const cp = (id: string) => com.poses.find((p: any) => p.id === id);
const T = (en: string, de: string) => ({ en, de });

asset.poses = [];
asset.clips = [];
const walkPose = (id: string) => {
  const src = cp(id);
  const body = strip(src.bones);
  return withCase(body, rootOf(body, src.contacts), { tilt: -45, free: true });
};
const standBody = strip(cp("standing").bones);
const standRoot = rootOf(standBody, cp("standing").contacts);
const upright = withCase(standBody, standRoot, { tilt: 0, free: false, caseX: 0.45 });
const tilting = withCase(standBody, standRoot, { tilt: -22, free: false, caseX: 0.32 });
const ready = withCase(standBody, standRoot, { tilt: -45, free: true });

asset.poses.push(
  pose("pulling-suitcase", T("Pulling the trolley", "Trolley ziehen"), T("Midstance of a steady walk at 1.2 m/s: the left arm reaches back to the grip, the trolley trails tilted about 45 degrees on its two wheels.", "Mittlere Standphase beim gleichmäßigen Gehen mit 1,2 m/s: der linke Arm greift nach hinten zum Griff, der Trolley folgt um etwa 45 Grad gekippt auf seinen zwei Rädern."), walkPose("passing-right"), [...cp("passing-right").contacts, "case-wheel-patch"], thermalOf(cp("passing-right"), 3.3, "walking")),
  pose("standing-with-suitcase", T("Standing with the trolley", "Mit dem Trolley stehen"), T("Waiting with the trolley upright on its wheels and feet beside the left foot, the left hand resting on the extended handle.", "Warten mit dem Trolley aufrecht auf Rädern und Füßen neben dem linken Fuß, die linke Hand auf dem ausgezogenen Griff."), upright, [...cp("standing").contacts, "case-wheel-patch", "case-foot"], thermalOf(cp("standing"), 1.3, "rest-standing")),
  pose("tilting-suitcase", T("Tilting it onto its wheels", "Auf die Räder kippen"), T("The trolley is half tipped back onto its wheels by the left hand, its feet off the ground.", "Der Trolley ist mit der linken Hand halb auf die Räder gekippt, seine Füße sind vom Boden."), tilting, [...cp("standing").contacts, "case-wheel-patch"], thermalOf(cp("standing"), 2.0, "rest-standing")),
  pose("ready-to-walk", T("Ready to walk off", "Bereit zum Losgehen"), T("Standing with the trolley tilted on its wheels behind the left hand, about to walk off.", "Stehen mit dem auf die Räder gekippten Trolley hinter der linken Hand, kurz vor dem Losgehen."), ready, [...cp("standing").contacts, "case-wheel-patch"], thermalOf(cp("standing"), 1.5, "rest-standing")),
);
for (const id of ["contact-left", "loading-left", "passing-left", "contact-right", "loading-right", "passing-right"]) {
  if (id === "passing-right") continue;
  const src = cp(id);
  asset.poses.push(pose(id, src.title, src.description, walkPose(id), [...src.contacts], thermalOf(src, 3.3, "walking"), true));
}
asset.poses.push(pose("passing-right", cp("passing-right").title, cp("passing-right").description, asset.poses[0].bones, [...cp("passing-right").contacts], thermalOf(cp("passing-right"), 3.3, "walking"), true));

const src = com.clips.find((c: any) => c.id === "walk-cycle");
const walk: any = { ...JSON.parse(JSON.stringify(src)), id: "pull-walk", title: T("Pulling walk", "Gehen mit Trolley"), description: T("A steady heel-to-toe walk at 1.2 m/s pulling the cabin trolley: the right arm swings, the left holds the grip behind; the trolley tilts a little with every step and its wheels make five turns per cycle of 1.22 m.", "Gleichmäßiges Gehen von der Ferse zum Ballen mit 1,2 m/s, den Kabinentrolley ziehend: der rechte Arm schwingt, der linke hält hinten den Griff; der Trolley kippt mit jedem Schritt ein wenig und seine Räder drehen sich fünfmal je Zyklus von 1,22 m.") };
walk.tracks = walk.tracks.map((t: any) => (t.target === "activityMet" ? { ...t, keys: t.keys.map((k: any) => ({ ...k, v: 3.3 })) } : t));
walk.keys = walk.keys.map((k: any) => ({ ...k, bones: strip(k.bones ?? {}) }));
asset.clips.push(walk);
const roots = walk.keys.map((k: any) => evaluate(asset, { kind: "clip", clip: "pull-walk", timeS: Math.min(k.t, walk.durationS - 1e-9) }).root);
const x0 = roots[0].x, advance = evaluate(asset, { kind: "clip", clip: "pull-walk", timeS: walk.durationS - 1e-9 }).root.x - x0;
const turns = 5;
console.log(`[DEBUG] advance ${advance.toFixed(5)} wheel radius for ${turns} turns ${(advance / (2 * Math.PI * turns)).toFixed(5)} (rig ${R})`);
const baseKeys = walk.keys.map((k: any) => JSON.parse(JSON.stringify(k.bones)));
let rootsNow = roots;
for (let pass = 0; pass < 4; pass++) {
  const solved: PoseBones[] = [];
  walk.keys.forEach((k: any, i: number) => {
    const merged: PoseBones = { ...strip(cp(k.pose).bones) };
    for (const [b, o] of Object.entries<any>(baseKeys[i])) merged[b] = { ...(merged[b] ?? {}), ...o };
    const sol = i === walk.keys.length - 1 ? solved[0]! : withCase(merged, { ...rootsNow[i], x: 0 }, { tilt: -45, free: true });
    solved.push(sol);
    const roll = i === walk.keys.length - 1 ? -360 * turns : Number(((-(roots[i].x - x0) / R) * 180 / Math.PI).toFixed(3));
    k.bones = { ...baseKeys[i], ...Object.fromEntries(Object.entries(sol).filter(([b]) => FREE.has(b))), "case-wheels": { angleDeg: roll } };
  });
  const next = walk.keys.map((k: any) => evaluate(asset, { kind: "clip", clip: "pull-walk", timeS: Math.min(k.t, walk.durationS - 1e-9) }).root);
  const drift = Math.max(...next.map((r: any, i: number) => Math.abs(r.y - rootsNow[i].y)));
  console.log(`[DEBUG] pass ${pass} root drift ${drift.toFixed(5)}`);
  rootsNow = next;
  if (drift < 0.0005) break;
}
walk.advanceM = Number(advance.toFixed(6));
for (const k of walk.keys) { const e = evaluate(asset, { kind: "clip", clip: "pull-walk", timeS: Math.min(k.t, walk.durationS - 1e-9) }); const d = Math.abs(e.anchors["case-wheel-patch"].at.y); if (d > 0.003) console.log(`[DEBUG] clip key ${k.t} patch ${d.toFixed(4)}`); }
asset.clips.push({
  id: "tilt-and-go", title: T("Tilting the trolley to go", "Trolley zum Losgehen kippen"), description: T("From standing with the trolley upright the traveller tips it back onto its wheels with the left hand until it rests at about 45 degrees, ready to walk off.", "Aus dem Stand mit aufrechtem Trolley kippt die reisende Person ihn mit der linken Hand auf die Räder, bis er bei etwa 45 Grad ruht, bereit zum Losgehen."),
  durationS: 3, fps: 30, loop: false, rootMotion: "none",
  keys: [{ t: 0, pose: "standing-with-suitcase", ease: "smooth" }, { t: 0.8, pose: "standing-with-suitcase", ease: "smooth" }, { t: 1.6, pose: "tilting-suitcase", ease: "smooth" }, { t: 2.4, pose: "ready-to-walk", ease: "smooth" }, { t: 3, pose: "ready-to-walk", ease: "smooth" }],
  contacts: [...["heel-l", "ball-l", "heel-r", "ball-r", "case-wheel-patch"].map((a) => ({ anchor: a, fromS: 0, toS: 3 })), { anchor: "case-foot", fromS: 0, toS: 1.1 }],
  phases: [{ id: "upright", fromS: 0, toS: 0.8, title: T("Trolley upright", "Trolley aufrecht") }, { id: "tilting", fromS: 0.8, toS: 2.4, title: T("Tipping onto the wheels", "Auf die Räder kippen") }, { id: "ready", fromS: 2.4, toS: 3, title: T("Ready to walk", "Bereit zum Gehen") }],
  events: [{ t: 1.1, kind: "contact-off", title: T("Trolley feet lift off", "Trolleyfüße heben ab"), target: "case-foot" }],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }] }, { target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.3 }, { t: 1.6, v: 2.0 }, { t: 3, v: 1.5 }] }],
});
asset.clips.push({
  id: "idle-breathing", title: T("Waiting and breathing", "Warten und atmen"), description: T("Waiting at a platform with the trolley upright: calm breathing at 13 per minute and a slight lift of the shoulders.", "Warten am Bahnsteig mit aufrechtem Trolley: ruhige Atmung mit 13 Atemzügen pro Minute und ein leichtes Heben der Schultern."),
  durationS: 4, fps: 30, loop: true, rootMotion: "none",
  keys: [{ t: 0, pose: "standing-with-suitcase", ease: "smooth" }, { t: 2, pose: "standing-with-suitcase", ease: "smooth", bones: { "clavicle-r": { ...(upright["clavicle-r"] ?? {}), angleDeg: (upright["clavicle-r"]?.angleDeg ?? 0) + 1.5 } } }, { t: 4, pose: "standing-with-suitcase", ease: "smooth" }],
  contacts: ["heel-l", "ball-l", "heel-r", "ball-r", "case-wheel-patch", "case-foot"].map((a) => ({ anchor: a, fromS: 0, toS: 4 })),
  phases: [{ id: "waiting", fromS: 0, toS: 4, title: T("Waiting", "Warten") }], events: [],
  tracks: [{ target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }] }],
});
const st = evaluate(asset, { kind: "pose", pose: asset.presentation.defaultPose }, { kind: "scale", scale: { denominator: 50 } });
const fr = viewExtent(st, "front");
const bb = st.bounds;
const mass = asset.rig.bones.reduce((m: number, x: any) => m + (x.mass?.kg ?? 0), 0);
asset.dimensions = { heightM: Number((bb.maxY - bb.minY).toFixed(3)), widthM: Number((bb.maxX - bb.minX).toFixed(3)), depthM: Number((fr?.horizontal ?? 0).toFixed(3)), massKg: Number(mass.toFixed(2)) };
console.log("[DEBUG] dimensions", JSON.stringify(asset.dimensions));
await saveHuman(SLUG, doc);
