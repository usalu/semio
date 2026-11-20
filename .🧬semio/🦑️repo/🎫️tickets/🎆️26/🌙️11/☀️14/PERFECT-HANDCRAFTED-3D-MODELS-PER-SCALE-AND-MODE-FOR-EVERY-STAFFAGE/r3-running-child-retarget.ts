/** 🦿️ Retargets the poses and clip keys of `humans/running-child` from the old rig (hip joints at crotch height, short legs) to the corrected rig: the pelvis path is carried over scaled by the ratio of the hip heights (flight arcs keep their absolute rise, because the flight times stay), every foot keeps its world orientation and its place under the pelvis scaled by the ratio of the leg lengths (a planted anchor stays on the ground), and thigh, shank and foot angles are solved again for the new segment lengths. Everything else of a pose stays. Runs on the catalog asset as it was before round 3 (guarded) and writes `r3-running-child-motion.json`, the input of `r3-running-child-roots.py`.
 * Usage: `bun r3-running-child-retarget.ts` */
import { writeFileSync } from "node:fs";
import { join } from "node:path";

import { rigAt, type Query } from "./r3-lib.ts";
import { entryOf } from "./volumes-lib.ts";

type Json = Record<string, any>;
const { asset } = entryOf("humans/running-child");
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
if (Math.abs(bone("pelvis").y - 0.5096) > 1e-6) throw new Error("the catalog asset already carries the corrected rig: the retarget reads the old one");

const OLD_HIP = bone("pelvis").y as number;
const NEW = { hip: 0.5745, thigh: 0.2565, shank: 0.262, ball: [0.081, -0.056] as const };
const KAPPA = NEW.hip / OLD_HIP;
const S = (NEW.thigh + NEW.shank) / (bone("thigh-l").lengthM + bone("shank-l").lengthM);
const DEG = 180 / Math.PI;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const OLD_FOOT_REST = -(90 - bone("foot-l").angleDeg);

type Leg = { thigh: number; shank: number; foot: number };
/** 📐️ The share of the full leg length that the hip to ankle distance may reach: beyond it the knee would lock (a knee of about 14 degrees is the straightest leg of these motions). */
const REACH = 0.992;
/** 🎯️ The ankle of one side for the new rig, relative to the hip joint in x and absolute in y, and the world rotation of the foot. */
function ankleOf(rig: ReturnType<typeof rigAt>, side: string): { x: number; y: number; rho: number } {
  const P = rig.frames.pelvis!.o;
  const foot = rig.frames[`foot-${side}`]!;
  const ball = rig.frames[`toe-${side}`]!.o;
  const rho = Math.atan2(foot.u[1]!, foot.u[0]!) - OLD_FOOT_REST / DEG;
  const [c, s] = [Math.cos(rho), Math.sin(rho)];
  return { x: S * (ball[0]! - P[0]!) - (c * NEW.ball[0] - s * NEW.ball[1]), y: S * ball[1]! - (s * NEW.ball[0] + c * NEW.ball[1]), rho };
}
/** 🪜️ The pelvis height lowered until both legs reach their ankles without locking the knee. */
function fit(rig: ReturnType<typeof rigAt>, pelvisY: number): number {
  let y = pelvisY;
  const far = (): boolean => ["l", "r"].some((side) => Math.hypot(ankleOf(rig, side).x, ankleOf(rig, side).y - y) > REACH * (NEW.thigh + NEW.shank));
  while (far()) y -= 0.0005;
  return y;
}
/** 🦵️ The leg angles of one side for the new rig from the old evaluated rig and the new pelvis height. */
function leg(rig: ReturnType<typeof rigAt>, side: string, pelvisY: number): Leg {
  const { x: tx, y: ay, rho } = ankleOf(rig, side);
  const ty = ay - pelvisY;
  const d = Math.min(Math.hypot(tx, ty), (NEW.thigh + NEW.shank) * 0.9995);
  if (Math.hypot(tx, ty) > (NEW.thigh + NEW.shank) * 0.9995) console.log("[DEBUG] unreachable", side, Math.hypot(tx, ty).toFixed(4));
  const knee = Math.acos(Math.max(-1, Math.min(1, (d * d - NEW.thigh ** 2 - NEW.shank ** 2) / (2 * NEW.thigh * NEW.shank))));
  const thigh = Math.atan2(tx, -ty) + Math.asin(Math.min(1, (NEW.shank * Math.sin(knee)) / d));
  return { thigh: r4(thigh * DEG), shank: r4(-knee * DEG), foot: r4((rho - thigh + knee) * DEG) };
}
const put = (bones: Json, rig: ReturnType<typeof rigAt>, pelvisY: number): void => {
  for (const side of ["l", "r"]) {
    const l = leg(rig, side, pelvisY);
    bones[`thigh-${side}`] = { ...bones[`thigh-${side}`], angleDeg: l.thigh };
    bones[`shank-${side}`] = { ...bones[`shank-${side}`], angleDeg: l.shank };
    bones[`foot-${side}`] = { ...bones[`foot-${side}`], angleDeg: l.foot };
  }
};
const at = (query: Query): ReturnType<typeof rigAt> => rigAt(asset, 50, query);

/** ✈️ The take-off pelvis height (old rig) of the flight a contact-less pose belongs to: the sprint flights leave from the toe-off height, the jump from the planted take-off pose. */
const SPRINT_TAKEOFF = at({ kind: "clip", clip: "sprint-cycle", timeS: 0.1768 });
const JUMP_TAKEOFF = at({ kind: "pose", pose: "jump-takeoff" });
const flightBase = (pose: Json): ReturnType<typeof rigAt> | null => (pose.contacts.length > 0 ? null : pose.id === "jump-apex" ? JUMP_TAKEOFF : SPRINT_TAKEOFF);
/** 🛫️ The new pelvis height of an evaluated old rig: planted, the old height scaled and fitted to the reach of the legs; in a flight, the new take-off height plus the old rise above the old take-off. */
function lifted(rig: ReturnType<typeof rigAt>, base: ReturnType<typeof rigAt> | null): number {
  const y = rig.frames.pelvis!.o[1]!;
  if (base === null) return fit(rig, KAPPA * y);
  return fit(base, KAPPA * base.frames.pelvis!.o[1]!) + (y - base.frames.pelvis!.o[1]!);
}

const poses: Json = {};
for (const pose of asset.poses as Json[]) {
  const rig = at({ kind: "pose", pose: pose.id });
  const y = lifted(rig, flightBase(pose));
  const bones = JSON.parse(JSON.stringify(pose.bones));
  put(bones, rig, y);
  poses[pose.id] = { bones, ...(pose.root === undefined ? {} : { root: { ...pose.root, y: r4(y - NEW.hip) } }) };
}
const clips: Json = {};
for (const clip of asset.clips as Json[]) {
  const flights = (clip.phases as Json[]).filter((p) => p.ballistic === true);
  clips[clip.id] = (clip.keys as Json[]).map((key) => {
    const rig = at({ kind: "clip", clip: clip.id, timeS: key.t });
    const flight = flights.find((p) => key.t >= p.fromS - 1e-9 && key.t <= p.toS + 1e-9);
    const base = flight === undefined ? null : at({ kind: "clip", clip: clip.id, timeS: flight.fromS });
    const bones = JSON.parse(JSON.stringify(key.bones));
    put(bones, rig, lifted(rig, base));
    return { t: key.t, pose: key.pose, bones };
  });
}
writeFileSync(join(import.meta.dir, "r3-running-child-motion.json"), `${JSON.stringify({ poses, clips }, null, 1)}\n`);
console.log("kappa", KAPPA.toFixed(4), "s", S.toFixed(4));
for (const [id, p] of Object.entries(poses) as [string, Json][]) console.log(id.padEnd(15), JSON.stringify(p.root ?? {}), ["thigh-l", "shank-l", "foot-l", "thigh-r", "shank-r", "foot-r"].map((b) => p.bones[b].angleDeg).join(" "));
