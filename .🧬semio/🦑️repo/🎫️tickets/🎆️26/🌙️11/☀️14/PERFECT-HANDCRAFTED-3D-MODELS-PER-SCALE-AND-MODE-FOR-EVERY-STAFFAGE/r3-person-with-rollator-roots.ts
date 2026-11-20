/** 🛒 Root fixes of `humans/person-with-rollator` outside `volumes` and `representations`: the rollator fitted to its user (grips at the wrist crease, the user walking inside the frame), a turning bone so that she sits on the seat the right way round, the grip, seat and basket anchors, and the arms of every pose and clip key re-solved onto the grips.
 *
 * Usage: `bun r3-person-with-rollator-roots.ts` writes `🗑️generated/r3-person-with-rollator/roots.json` (members `rig`, `anchors`, `poses`, `clips`, `provenance`), spliced by `r3-person-with-rollator-candidate.py`.
 * @see ./r3-humans-person-with-rollator.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { rigAt, type Query } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const { entry } = entryOf("humans/person-with-rollator");
const doc = JSON.parse(readFileSync((entry as Json).path, "utf8")) as Json;
const asset = doc.asset as Json;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);
const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;

/** 📐 The rollator in the frame of its bone (u ahead from the rear tread patch, v up, w left). */
export const AID = { turnU: 0.295, gripU: 0.15, gripV: 0.775, gripW: 0.235, seatU: 0.295, seatV: 0.52, basketU: 0.53, basketV: 0.4, frameX: 0.19 };
/** ✊ The centre of the grip tube in the frame of the closed hand, and the slope of the hand on the grip in the side elevation. */
export const HAND = { gripX: 0.055, gripY: -0.03, slopeDeg: -15 };

const bones = asset.rig.bones as Json[];
if (!bone("aid-turn")) {
  bones.splice(bones.indexOf(bone("frame")), 0, { id: "aid-turn", parent: "aid-slide", title: { en: "Aid mount, turn about the vertical", de: "Hilfsmittelaufnahme, Drehung um die Senkrechte" }, x: r4(AID.frameX + AID.turnU), y: -0.8494, angleDeg: 90, lengthM: 0, joint: { type: "revolute", minDeg: 0, maxDeg: 0, twistMinDeg: -180, twistMaxDeg: 180 } });
}
Object.assign(bone("frame"), { parent: "aid-turn", x: 0, y: AID.turnU, angleDeg: -90 });
bone("frame").mass.comX = 0.3;
bone("frame").mass.comY = 0.505;
for (const side of ["l", "r"]) bone(`hand-${side}`).mass.comX = 0.0531;
for (const side of ["l", "r"]) Object.assign(anchor(`grip-${side}`), { x: HAND.gripX, y: HAND.gripY });
Object.assign(anchor("seat"), { x: AID.seatU, y: AID.seatV });
Object.assign(anchor("basket"), { x: AID.basketU, y: AID.basketV });
anchor("wheel-rear-contact").footprintWidthM = 0.036;
anchor("wheel-front-contact").footprintWidthM = 0.036;

const withTurn = (b: Json): Json => {
  const out: Json = {};
  for (const [k, v] of Object.entries(b)) {
    if (k === "aid-turn") continue;
    out[k] = v;
    if (k === "aid-slide") out["aid-turn"] = { angleDeg: 0 };
  }
  return out;
};
for (const p of asset.poses as Json[]) p.bones = withTurn(p.bones);
for (const c of asset.clips as Json[]) for (const k of c.keys as Json[]) if (k.bones) k.bones = withTurn(k.bones);

type V3 = [number, number, number];
const at = (f: { o: V3; u: V3; v: V3; w: V3 }, p: V3): V3 => [0, 1, 2].map((k) => f.o[k]! + p[0] * f.u[k]! + p[1] * f.v[k]! + p[2] * f.w[k]!) as V3;
/** 🤝 Solves one arm of the offsets `b` (a pose or a clip key, evaluated through `q`) so that the grip anchor of the hand lies on the centre of its grip tube and the hand slopes as a fist on the grip does. */
function grasp(b: Json, q: Query, side: "l" | "r"): { elbow: number; miss: number } {
  const k = side === "l" ? 1 : -1;
  const ids = [`arm-${side}`, `forearm-${side}`, `hand-${side}`];
  const get = (): number[] => [b[ids[0]!].angleDeg, b[ids[0]!].abductionDeg ?? 0, b[ids[1]!].angleDeg, b[ids[2]!].angleDeg];
  const set = (x: number[]): void => {
    b[ids[0]!] = { angleDeg: x[0], abductionDeg: x[1] };
    b[ids[1]!] = { angleDeg: x[2] };
    b[ids[2]!] = { angleDeg: x[3] };
  };
  const err = (x: number[]): number[] => {
    set(x);
    const rig = rigAt(asset, 50, q);
    const target = at(rig.frames.frame as never, [AID.gripU, AID.gripV, k * AID.gripW]);
    const hand = rig.frames[ids[2]!]!;
    const have = at(hand as never, [HAND.gripX, HAND.gripY, 0]);
    const slope = (Math.atan2(hand.u[1], hand.u[0]) * 180) / Math.PI;
    return [have[0] - target[0], have[1] - target[1], have[2] - target[2], ((slope - HAND.slopeDeg) * Math.PI) / 180 / 10];
  };
  let x = get();
  x[2] = Math.min(x[2]!, 40);
  for (let it = 0; it < 40; it++) {
    const e = err(x);
    if (Math.hypot(...e) < 2e-6) break;
    const J: number[][] = [];
    for (let j = 0; j < 4; j++) {
      const y = [...x];
      y[j]! += 0.01;
      const ej = err(y);
      J.push(ej.map((v, i) => (v - e[i]!) / 0.01));
    }
    const A = [0, 1, 2, 3].map((i) => [...[0, 1, 2, 3].map((j) => J[j]![i]!), -e[i]!]);
    for (let c = 0; c < 4; c++) {
      let p = c;
      for (let r = c + 1; r < 4; r++) if (Math.abs(A[r]![c]!) > Math.abs(A[p]![c]!)) p = r;
      [A[c], A[p]] = [A[p]!, A[c]!];
      for (let r = 0; r < 4; r++) if (r !== c) { const f = A[r]![c]! / A[c]![c]!; for (let m = c; m < 5; m++) A[r]![m]! -= f * A[c]![m]!; }
    }
    const step = [0, 1, 2, 3].map((i) => A[i]![4]! / A[i]![i]!);
    const lim = Math.min(1, 12 / Math.max(...step.map(Math.abs)));
    x = x.map((v, i) => v + step[i]! * lim);
    x[2] = Math.max(2, x[2]!);
  }
  const miss = Math.hypot(...err(x).slice(0, 3));
  set(x.map(r4));
  return { elbow: x[2]!, miss };
}
const report = (label: string, b: Json, q: Query): void => {
  const l = grasp(b, q, "l");
  const r = grasp(b, q, "r");
  console.log(label.padEnd(30), `elbow l ${l.elbow.toFixed(1)} r ${r.elbow.toFixed(1)}  arm-l ${JSON.stringify(b["arm-l"])} hand-l ${b["hand-l"].angleDeg}  miss ${(Math.max(l.miss, r.miss) * 1000).toFixed(3)} mm`);
  if (Math.max(l.miss, r.miss) > 1e-4) throw new Error(`${label}: grip not reached`);
};

/** 🚶 While walking the rollator rolls this much farther ahead of the pelvis than at rest, so the elbows stay open although the pelvis rides lower. */
const WALK_AHEAD = 0.03;
const ahead = (b: Json): void => { b["aid-slide"] = { dx: r4(b["aid-slide"].dx + WALK_AHEAD), dy: 0 }; };
ahead(pose("walking-midstance").bones);
for (const k of asset.clips.find((c: Json) => c.id === "rollator-walk").keys as Json[]) ahead(k.bones);
for (const k of asset.clips.find((c: Json) => c.id === "rest-at-the-grips").keys as Json[]) if (k.bones["aid-slide"].dx > 0.02) k.bones["aid-slide"] = { dx: 0.02, dy: 0 };
for (const id of ["stand-at-grips", "walking-midstance"]) report(`pose ${id}`, pose(id).bones, { kind: "pose", pose: id });
for (const c of asset.clips as Json[]) {
  const keys = c.keys as Json[];
  for (const k of keys.slice(0, -1)) report(`clip ${c.id} t=${k.t}`, k.bones, { kind: "clip", clip: c.id, timeS: k.t });
  const [first, last] = [keys[0]!, keys[keys.length - 1]!];
  for (const id of ["arm-l", "forearm-l", "hand-l", "arm-r", "forearm-r", "hand-r"]) last.bones[id] = { ...first.bones[id] };
}

const seated = pose("seated-rest");
seated.bones["aid-turn"] = { angleDeg: 0, twistDeg: 180 };
seated.bones["aid-slide"] = { dx: r4(0.02 - AID.frameX - AID.turnU), dy: 0 };
seated.description = { en: "Sitting on the seat with her back to the front of the rollator and against the backrest strap, hands in the lap, feet flat on the ground, brakes locked; the grips stand beside her hips.", de: "Sie sitzt auf dem Sitz, den Rücken zur Vorderseite des Rollators und am Rückengurt, die Hände im Schoß, die Füße flach am Boden, Bremsen festgestellt; die Griffe stehen neben ihren Hüften." };
pose("stand-at-grips").description = { en: "Both hands closed on the grips at wrist height, the elbows a little bent, the back rounded, the toes at the rear axle inside the frame, weight evenly on both feet, brakes locked; the shopping rides in the basket.", de: "Beide Hände umfassen die Griffe auf Handgelenkhöhe, die Ellbogen leicht gebeugt, runder Rücken, die Fußspitzen an der Hinterachse im Rahmen, Gewicht gleichmäßig auf beiden Füßen, Bremsen angezogen; der Einkauf liegt im Korb." };
asset.clips.find((c: Json) => c.id === "rollator-walk").description = { en: "Slow stride of 0.63 m in 1.3 s (0.48 m/s) inside the frame of a rollator whose 0.2 m wheels turn exactly once per stride; the trunk stays stooped, the hands stay closed on the grips and the elbows give with every step.", de: "Langsamer Doppelschritt von 0,63 m in 1,3 s (0,48 m/s) im Rahmen eines Rollators, dessen 0,2-m-Räder sich genau einmal pro Doppelschritt drehen; der Rumpf bleibt gebeugt, die Hände bleiben um die Griffe geschlossen und die Ellbogen geben bei jedem Schritt nach." };

/** 🧺 The hands in the basket: the wrists come to the rim of the basket, a hand's breadth inside its side walls. */
const stoop = pose("stoop-to-basket");
stoop.bones["aid-slide"] = { dx: -0.12, dy: 0 };
{
  const q: Query = { kind: "pose", pose: "stoop-to-basket" };
  const saved = { ...AID };
  Object.assign(AID, { gripU: 0.5, gripV: 0.64, gripW: 0.06 });
  const savedSlope = HAND.slopeDeg;
  HAND.slopeDeg = -70;
  report("pose stoop-to-basket", stoop.bones, q);
  Object.assign(AID, saved);
  HAND.slopeDeg = savedSlope;
}

const notes = asset.provenance.notes;
notes.en = notes.en.replace("wheelbase 0.52 m, grip height 0.85 m, seat height 0.52 m", "wheelbase 0.52 m, grips 0.79 m high at the wrist crease of the user, seat height 0.52 m, overall width 0.60 m");
notes.de = notes.de.replace("Radstand 0,52 m, Griffhöhe 0,85 m, Sitzhöhe 0,52 m", "Radstand 0,52 m, Griffe 0,79 m hoch auf Höhe der Handgelenkfalte der Nutzerin, Sitzhöhe 0,52 m, Gesamtbreite 0,60 m");

const out = join(import.meta.dir, "🗑️generated", "r3-person-with-rollator");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "roots.json"), JSON.stringify({ rig: asset.rig, anchors: asset.anchors, poses: asset.poses, clips: asset.clips, provenance: asset.provenance }));
console.log("written roots.json");
