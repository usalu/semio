/** 🤟️ Generator of `person-signing`: derived from the rig, outlines and volumes of `standing-adult`; a person communicating in sign language with sign poses (HOUSE, THANK YOU, YOU) solved by arm inverse kinematics and hand shapes, a looping signing phrase and idle breathing.
 *  bun gen-signing.ts
 * @see ./sign-design.ts
 * @see ./sign-texts.ts
 */
import { load, save, clone } from "./lib.ts";
import { solveArm } from "./ik.ts";
import { hand, SIGN_POSES, PHRASE, PHRASE_S } from "./sign-design.ts";
import { signTexts } from "./sign-texts.ts";

const doc = await load("👤️humans", "standing-adult");
const a = doc.asset;
a.id = "person-signing";

const relaxed = a.poses.find((p: any) => p.id === "relaxed");
a.poses = [relaxed];
a.clips = a.clips.filter((c: any) => c.id === "idle-breathing");
a.thermal.respiration.patterns = a.thermal.respiration.patterns.filter((p: any) => ["quiet-standing", "talking"].includes(p.id));

const log: string[] = [];
for (const s of SIGN_POSES) {
  const base: Record<string, any> = clone(relaxed.bones);
  const add = (bone: string, d?: number) => { if (d) base[bone] = { ...(base[bone] ?? {}), angleDeg: (base[bone]?.angleDeg ?? 0) + d }; };
  add("head", s.head); add("neck", s.neck); add("torso", s.torso);
  Object.assign(base, hand("r", s.hands[0] as any), hand("l", s.hands[1] as any));
  const r = solveArm(a, base, "r", s.right);
  Object.assign(base, r.offsets);
  const l = solveArm(a, base, "l", s.left);
  Object.assign(base, l.offsets);
  log.push(`${s.id}: r ${r.error.wristM.toFixed(3)} m ${r.error.fingersDeg.toFixed(0)}° ${r.error.palmDeg.toFixed(0)}° | l ${l.error.wristM.toFixed(3)} m ${l.error.fingersDeg.toFixed(0)}° ${l.error.palmDeg.toFixed(0)}° | ${JSON.stringify({ ...r.offsets, ...l.offsets })}`);
  const thermal = clone(relaxed.thermal);
  thermal.activityMet = s.met;
  a.poses.push({ id: s.id, title: { en: s.id, de: s.id }, description: { en: s.id, de: s.id }, ...(s.key ? { key: true } : {}), bones: base, ground: "auto", contacts: clone(relaxed.contacts), thermal, loads: [] });
}
a.clips.unshift({
  id: "signing-phrase", title: { en: "", de: "" }, description: { en: "", de: "" }, durationS: PHRASE_S, fps: 30, loop: true, rootMotion: "none",
  keys: PHRASE.map(([t, pose]) => ({ t, pose, ease: "smooth" })),
  contacts: ["heel-l", "ball-l", "heel-r", "ball-r"].map((anchor) => ({ anchor, fromS: 0, toS: PHRASE_S })),
  phases: [], events: [],
  tracks: [{ target: "activityMet", ease: "smooth", keys: [{ t: 0, v: 1.5 }, { t: PHRASE_S, v: 1.5 }] }, { target: "breathing", ease: "step", keys: [{ t: 0, v: 0 }, { t: PHRASE_S, v: 0 }] }],
});
a.presentation.defaultPose = "signing-ready";
a.presentation.defaultClip = "signing-phrase";
signTexts(a);
console.log(log.join("\n"));
console.log(save(doc));
