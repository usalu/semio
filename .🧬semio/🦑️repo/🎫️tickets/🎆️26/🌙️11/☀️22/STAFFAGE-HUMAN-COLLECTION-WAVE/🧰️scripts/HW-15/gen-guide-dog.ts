/** 🦮️ Generator of `person-with-guide-dog`: a blind woman (rig, clothing, outlines and volumes of `person-with-white-cane`, the cane removed) walking with a guide dog (rig, outlines, volumes and physiology of `labrador-dog`) in a harness on her left; the left hand holds the rigid harness handle, the right arm swings, the dog walks on its own planted paws two strides for each of hers.
 *  bun gen-guide-dog.ts
 * @see ./dog-merge.ts
 * @see ./harness-design.ts
 * @see ./dog-texts.ts
 */
import { join } from "node:path";
import { S, load, save, clone, removeBones, removeShapes, removeRegion, frames, v3, toLocal, toWorld, sub, unit, add, mul, type V3 } from "./lib.ts";
import { mergeDog, dogOffsets, P, DOG, type Rest } from "./dog-merge.ts";
import { solveArm } from "./ik.ts";
import { harness, HARNESS } from "./harness-design.ts";
import { dogTexts } from "./dog-texts.ts";
import { mirrorRightHand } from "./right-hand.ts";

const { evaluate } = await import(join(S, "🔨️modules/🧮️evaluation/🟦️.ts"));
const { sampleClip, clipOf } = await import(join(S, "🔨️modules/🎞️animation/🟦️.ts"));

const doc = await load("👤️humans", "person-with-white-cane");
const lab = (await load("🐕️animals", "labrador-dog")).asset;
const a = doc.asset;
a.id = "person-with-guide-dog";

const CANE = ["cane-alu-white", "grip-eva", "nylon-roller"];
removeBones(a, ["cane"]);
a.anchors = a.anchors.filter((x: any) => x.id !== "grip-r");
removeShapes(a, (s) => CANE.includes(s.material));
for (const r of a.thermal.regions.filter((r: any) => CANE.includes(r.material)).map((r: any) => r.id)) removeRegion(a, r);
a.materials = a.materials.filter((m: any) => !CANE.includes(m.id));
a.structure.supports = [];
mirrorRightHand(a);

const W = 1.2, DOG_CYCLE = 0.6, LAB_WALK = 0.7, ADVANCE = 2 * 0.5212;
const rename = (id: string, to: string) => {
  for (const p of a.poses) if (p.id === id) p.id = to;
  for (const c of a.clips) for (const k of c.keys) if (k.pose === id) k.pose = to;
  if (a.presentation.defaultPose === id) a.presentation.defaultPose = to;
};
a.poses = a.poses.filter((p: any) => ["constant-contact-stance", "walking-midstance"].includes(p.id));
rename("constant-contact-stance", "standing-with-guide-dog");
rename("walking-midstance", "walking-with-guide-dog");
a.clips = a.clips.filter((c: any) => c.id === "constant-contact-walk");
const walk = a.clips[0];
walk.id = "guided-walk";
a.presentation.defaultClip = "guided-walk";

const mirrorArm = (from: Record<string, any>) => {
  const o: Record<string, any> = {};
  for (const [s, d] of [["arm-l", "arm-r"], ["forearm-l", "forearm-r"], ["hand-l", "hand-r"]]) {
    const v = from[s!] ?? {};
    o[d!] = { angleDeg: v.angleDeg ?? 0, ...(v.abductionDeg ? { abductionDeg: -v.abductionDeg } : {}), ...(v.twistDeg ? { twistDeg: -v.twistDeg } : {}) };
  }
  return o;
};
for (const p of a.poses) Object.assign(p.bones, mirrorArm(p.bones));
const T0 = walk.durationS, keys0 = clone(walk.keys);
const armAt = (t: number) => keys0.reduce((best: any, k: any) => (Math.abs(((k.t - t) % T0 + T0) % T0) < Math.abs(((best.t - t) % T0 + T0) % T0) ? k : best), keys0[0]).bones;
for (const k of walk.keys) Object.assign(k.bones, mirrorArm(armAt(k.t + T0 / 2)));

const LEGS = ["thigh-l", "shank-l", "foot-l", "toe-l", "thigh-r", "shank-r", "foot-r", "toe-r"];
const gait = clone(walk.keys.map((k: any) => k.bones));
const retime = (s: number) => {
  walk.keys.forEach((k: any, i: number) => {
    k.t = Math.round(keys0[i].t * W / T0 * 1e4) / 1e4;
    for (const b of LEGS) if (gait[i][b]) k.bones[b] = { ...gait[i][b], angleDeg: Math.round(gait[i][b].angleDeg * s * 1e4) / 1e4 };
  });
  walk.durationS = W;
  walk.contacts = walk.contacts.map((c: any, i: number) => ({ ...c, fromS: Math.round(c.fromS * W / T0 * 1e4) / 1e4, toS: Math.round(c.toS * W / T0 * 1e4) / 1e4 }));
  const travel = sampleClip(a, walk, W - 1e-9).root.x - sampleClip(a, walk, 0).root.x;
  return travel;
};
const contacts0 = clone(walk.contacts);
let lo = 0.85, hi = 1.05;
for (let i = 0; i < 30; i++) { walk.contacts = clone(contacts0); const m = (lo + hi) / 2; if (retime(m) > ADVANCE) hi = m; else lo = m; }
walk.contacts = clone(contacts0);
const travel = retime((lo + hi) / 2);
walk.advanceM = Math.round(travel * 1e5) / 1e5;
for (const e of walk.events) e.t = Math.round(e.t * W / T0 * 1e4) / 1e4;
for (const ph of walk.phases) { ph.fromS = Math.round(ph.fromS * W / T0 * 1e4) / 1e4; ph.toS = Math.round(ph.toS * W / T0 * 1e4) / 1e4; }
for (const tr of walk.tracks) for (const k of tr.keys) k.t = Math.round(k.t * W / T0 * 1e4) / 1e4;
console.log(`[DEBUG] gait scale ${((lo + hi) / 2).toFixed(4)} travel ${travel.toFixed(4)}`);

const human = clone(a);
const REST: Rest = { x: HARNESS.dogX, y: 0.5, z: HARNESS.dogZ };
mergeDog(a, lab, REST);
a.thermal.passengers[0].physiology = clone(lab.thermal.body);

/** 🧭️ Carrier offsets that put the dog pelvis where the labrador state has it, shifted by `dx` ahead and `REST.z` to the left, below a human pelvis frame. */
function placeDog(hp: any, dogPelvis: any, dx: number, pelvisOffset: number): Record<string, any> {
  const o = v3(hp.o), u = v3(hp.u), v = v3(hp.v), w = v3(hp.w);
  const target: V3 = [v3(dogPelvis.o)[0] + dx, v3(dogPelvis.o)[1], REST.z];
  const rel = sub(sub(target, o), mul(w, REST.z));
  const lead = a.rig.bones.find((b: any) => b.id === P("lead-x"));
  const s2 = rel[0] * u[0] + rel[1] * u[1] + rel[2] * u[2] - lead.x;
  const s1 = rel[0] * v[0] + rel[1] * v[1] + rel[2] * v[2] - lead.y;
  const thp = Math.atan2(u[1], u[0]) * 180 / Math.PI;
  const dogU = v3(dogPelvis.u);
  const phi = Math.atan2(dogU[1], dogU[0]) * 180 / Math.PI;
  const pelvisRest = lab.rig.bones.find((b: any) => b.id === "pelvis").angleDeg;
  const alpha = phi - thp + 90 - pelvisRest - pelvisOffset;
  const r = (x: number) => Math.round(x * 1e5) / 1e5;
  return { [P("lead-x")]: { dy: r(s1) }, [P("lead-y")]: { dx: r(s2) }, [P("lead")]: { angleDeg: r(alpha) } };
}

const pelvisOf = (asset: any, query: any) => evaluate(asset, query).bones["pelvis"].frame;
const labPose = (id: string) => ({ frame: evaluate(lab, { kind: "pose", pose: id }).bones["pelvis"].frame, offsets: lab.poses.find((p: any) => p.id === id).bones, thermal: lab.poses.find((p: any) => p.id === id).thermal });
const labWalk = (t: number) => {
  const tl = t * LAB_WALK / DOG_CYCLE;
  const sm = sampleClip(lab, clipOf(lab, "walk-cycle"), tl);
  return { frame: evaluate(lab, { kind: "clip", clip: "walk-cycle", timeS: tl }).bones["pelvis"].frame, offsets: sm.offsets };
};

harness(a, lab, REST);
const handleTip = (offsets: Record<string, any>) => toWorld(frames(a, offsets)[P("harness-handle")].frame, [HARNESS.handleLength, 0, 0]);

/** ✋️ Left arm offsets that put the hand on the handle grip. */
function grip(offsets: Record<string, any>, start?: Record<string, any>) {
  const f = frames(a, offsets);
  const want = add(v3(f["pelvis"].frame.o), HARNESS.gripFromPelvis);
  let best = 0, bestD = Infinity;
  for (let ang = -45; ang <= 80; ang += 0.5) {
    const d = Math.hypot(...sub(handleTip({ ...offsets, [P("harness-handle")]: { angleDeg: ang } }), want));
    if (d < bestD) { bestD = d; best = ang; }
  }
  offsets = { ...offsets, [P("harness-handle")]: { angleDeg: best } };
  const g = handleTip(offsets);
  const fingers = unit(HARNESS.fingers), palm = unit(HARNESS.palm);
  const wrist = sub(sub(g, mul(fingers, HARNESS.gripAlongHand)), mul(palm, -HARNESS.gripBelowPalm));
  const r = solveArm(a, offsets, "l", { wrist, fingers, palm }, start);
  if (r.error.wristM > 0.01) console.log(`[DEBUG] grip error ${r.error.wristM.toFixed(3)} m`);
  return { ...r.offsets, [P("harness-handle")]: { angleDeg: best } };
}

const DOG_SHIFT = HARNESS.dogShift;
const thermalDog = (src: any, met: number, breathing: string) => ({ regions: Object.fromEntries(Object.entries(src.regions).map(([k, v]) => [P(k), v])), emitters: Object.fromEntries(Object.entries(src.emitters).map(([k, v]) => [P(k), v])), passengers: { [DOG]: { activityMet: met, breathing } } });
const addThermal = (p: any, t: any) => {
  Object.assign(p.thermal.regions, t.regions);
  Object.assign(p.thermal.emitters, t.emitters);
  p.thermal.passengers = t.passengers;
};

const dogContacts = ["contact-hn", "contact-hf", "contact-fn", "contact-ff"].map(P);
const stand = a.poses.find((p: any) => p.id === "standing-with-guide-dog");
const walkPose = a.poses.find((p: any) => p.id === "walking-with-guide-dog");
const sitting = clone(stand);
sitting.id = "waiting-dog-sitting";
sitting.bones.head = { angleDeg: (stand.bones.head?.angleDeg ?? 0) - 4 };
a.poses.push(sitting);

const ls = labPose("standing-alert"), lsit = labPose("sitting-waiting");
for (const [p, l, extra] of [[stand, ls, []], [sitting, lsit, [P("contact-rump")]]] as const) {
  const hp = pelvisOf(human, { kind: "pose", pose: p.id === "waiting-dog-sitting" ? "standing-with-guide-dog" : p.id });
  Object.assign(p.bones, dogOffsets(l.offsets), placeDog(hp, l.frame, DOG_SHIFT, l.offsets["pelvis"]?.angleDeg ?? 0));
  Object.assign(p.bones, grip(p.bones));
  p.contacts = [...p.contacts, ...dogContacts, ...extra];
  addThermal(p, thermalDog(l.thermal, l.thermal.activityMet, l.thermal.breathing));
}

const walkThermal = lab.poses.find((p: any) => p.id === "walk-phase-0").thermal;
const l0 = labWalk(0), h0 = pelvisOf(human, { kind: "clip", clip: "guided-walk", timeS: 0 });
const c = v3(h0.o)[0] + DOG_SHIFT - v3(l0.frame.o)[0];
let prevArm: Record<string, any> | undefined;
for (const k of walk.keys) {
  const hp = pelvisOf(human, { kind: "clip", clip: "guided-walk", timeS: Math.min(k.t, W - 1e-6) });
  const l = labWalk(k.t);
  const dx = c;
  const full = { ...walkPose.bones, ...k.bones };
  const dogBones = { ...dogOffsets(l.offsets), ...placeDog(hp, l.frame, dx, l.offsets["pelvis"]?.angleDeg ?? 0) };
  const arm = grip({ ...full, ...dogBones }, prevArm);
  prevArm = arm;
  Object.assign(k.bones, dogBones, arm);
}
{
  const l = labWalk(0.25);
  const hp = pelvisOf(human, { kind: "pose", pose: "walking-with-guide-dog" });
  Object.assign(walkPose.bones, dogOffsets(l.offsets), placeDog(hp, l.frame, DOG_SHIFT, l.offsets["pelvis"]?.angleDeg ?? 0));
  Object.assign(walkPose.bones, grip(walkPose.bones));
  walkPose.contacts = [...walkPose.contacts, ...dogContacts.filter((x) => sampleClip(lab, clipOf(lab, "walk-cycle"), 0.25 * LAB_WALK / DOG_CYCLE).contacts.includes(x.slice(4)))];
  addThermal(walkPose, thermalDog(walkThermal, walkThermal.activityMet, walkThermal.breathing));
}
walk.keys[walk.keys.length - 1].bones = clone(walk.keys[0].bones);

const KERB: [number, number, string][] = [[0, 0, "standing-with-guide-dog"], [0.8, 0.75, "standing-with-guide-dog"], [1.6, 1.5, "waiting-dog-sitting"], [3.4, 3.2, "waiting-dog-sitting"], [5, 5, "waiting-dog-sitting"]];
const kerb: any = { id: "kerb-stop-and-sit", title: { en: "", de: "" }, description: { en: "", de: "" }, durationS: 5, fps: 30, loop: false, rootMotion: "none", keys: [], contacts: [], phases: [], events: [], tracks: [] };
const sitClip = clipOf(lab, "sit-and-pant");
const standFn = evaluate(lab, { kind: "pose", pose: "standing-alert" }).anchors["contact-fn"].at.x;
for (const [t, tl, pose] of KERB) {
  const hp = pelvisOf(human, { kind: "pose", pose: "standing-with-guide-dog" });
  const sm = sampleClip(lab, sitClip, tl);
  const ev = evaluate(lab, { kind: "clip", clip: "sit-and-pant", timeS: tl });
  const fr = ev.bones["pelvis"].frame;
  const keep = standFn - ev.anchors["contact-fn"].at.x;
  const base = a.poses.find((p: any) => p.id === pose).bones;
  const dogBones = { ...dogOffsets(sm.offsets), ...placeDog(hp, fr, DOG_SHIFT + keep, sm.offsets["pelvis"]?.angleDeg ?? 0) };
  kerb.keys.push({ t, pose, ease: "smooth", bones: { ...dogBones, ...grip({ ...base, ...dogBones }) } });
}
kerb.contacts = [...stand.contacts.map((anchor: string) => ({ anchor, fromS: 0, toS: 5 })), { anchor: P("contact-rump"), fromS: 1.5, toS: 5 }];
a.clips.push(kerb);
const sitKey = kerb.keys[kerb.keys.length - 1].bones;
for (const id of Object.keys(sitKey)) if (id.startsWith("dog-") || ["arm-l", "forearm-l", "hand-l"].includes(id)) sitting.bones[id] = clone(sitKey[id]);

const spans = clipOf(lab, "walk-cycle").contacts.flatMap((s: any) => [0, 1].map((n) => ({ anchor: P(s.anchor), fromS: Math.round((s.fromS * DOG_CYCLE / LAB_WALK + n * DOG_CYCLE) * 1e4) / 1e4, toS: Math.round((s.toS * DOG_CYCLE / LAB_WALK + n * DOG_CYCLE) * 1e4) / 1e4 })));
const starts = [...new Set([...walk.contacts, ...spans].map((c: any) => c.fromS))].sort((p: number, q: number) => p - q);
const chop = (s: any) => {
  const cuts = [s.fromS, ...starts.filter((x: number) => x > s.fromS + 1e-6 && x < s.toS - 1e-6), s.toS];
  return cuts.slice(0, -1).map((f: number, i: number) => ({ anchor: s.anchor, fromS: f, toS: cuts[i + 1] }));
};
walk.contacts = [...walk.contacts.flatMap(chop), ...spans.flatMap(chop)];
{ const m = structuredClone(a); const mw = m.clips.find((c: any) => c.id === "guided-walk"); walk.advanceM = Math.round((sampleClip(m, mw, W - 1e-9).root.x - sampleClip(m, mw, 0).root.x) * 1e5) / 1e5; }

dogTexts(a, lab);
console.log(save(doc));
