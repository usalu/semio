/** 🧱️ Root fixes of `humans/courier-with-hand-truck` (rig, anchors, poses, clips, texts) written as a candidate document to `🗑️generated/r3-courier-with-hand-truck/🔣️.json`: shoulders at the glenohumeral width, a real hand truck (axle 0.14 m behind the rail plane, toe plate at ground level, bow handle at 1.15 m, balanced at 45 degrees), a horizontal reach joint for the truck, hands solved onto the handle bar in every pose and clip key. Every value is absolute, so the script is re-runnable on its own result.
 * @see ./r3-humans-courier-with-hand-truck.md */
import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { GENERATED, entryOf } from "./volumes-lib.ts";

const S = "/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/👤️humans/courier-with-hand-truck/🔣️.json";
const doc = JSON.parse(readFileSync(S, "utf8"));
const asset = doc.asset;
void entryOf;
const SHOULDER_Z = 0.185;
const AXLE_X = 0.95;
const HANDLE = { x: 1.0065, y: -0.14 };
const GRIP = { x: 0.082, y: -0.032 };
const GRIP_Z = 0.07;
const WRIST = 12;
const PARCELS_COM = { x: 0.2909, y: -0.3415 };
const TRUCK_COM = { x: 0.14, y: -0.12 };

const bones: any[] = asset.rig.bones;
const bone = (id: string): any => bones.find((b) => b.id === id);
bone("arm-l").z = SHOULDER_Z;
bone("arm-r").z = -SHOULDER_Z;
if (bone("truck-reach") === undefined)
  bones.splice(bones.indexOf(bone("truck-lift")), 0, { id: "truck-reach", parent: "pelvis", title: { en: "Hand truck horizontal reach ahead of the courier", de: "Waagerechter Abstand der Sackkarre vor dem Kurier" }, x: -0.82745, y: -AXLE_X, angleDeg: 0, lengthM: 0, joint: { type: "prismatic", axisDeg: 90, minM: -0.1, maxM: 0.6 } });
Object.assign(bone("truck-reach"), { x: -0.82745, y: -AXLE_X });
Object.assign(bone("truck-lift"), { parent: "truck-reach", x: 0, y: 0 });
Object.assign(bone("hand-truck"), { angleDeg: 135, lengthM: 1.02, joint: { type: "revolute", minDeg: -45, maxDeg: 45 } });
bone("hand-truck").title = { en: "Hand truck frame, balanced on its axle at 45 degrees", de: "Sackkarrenrahmen, bei 45 Grad auf der Achse ausbalanciert" };
Object.assign(bone("hand-truck").mass, { comX: TRUCK_COM.x, comY: TRUCK_COM.y });
bone("parcels").lengthM = 0.812;
Object.assign(bone("parcels").mass, { comX: PARCELS_COM.x, comY: PARCELS_COM.y });
const anchor = (id: string): any => asset.anchors.find((a: any) => a.id === id);
Object.assign(anchor("handle-grip"), HANDLE);
Object.assign(anchor("parcels-cog"), PARCELS_COM);
Object.assign(anchor("grip-l"), GRIP);
Object.assign(anchor("grip-r"), GRIP);
Object.assign(anchor("nostrils"), { x: -0.001, y: -0.114 });
Object.assign(anchor("mouth"), { x: -0.019, y: -0.103 });
Object.assign(anchor("crown"), { x: 0.18, y: -0.002 });
Object.assign(asset.thermal.regions.find((r: any) => r.id === "tyres"), { areaM2: 0.31, characteristicLengthM: 0.26 });
anchor("handle-grip").title = { en: "Bow handle bar", de: "Bügelgriff" };
if (!asset.provenance.references.some((r: any) => r.id === "fetra-b1125l")) asset.provenance.references.push({ id: "fetra-b1125l", citation: "fetra Stahlrohrkarre B1125L, manufacturer data sheet: overall 570 x 533 x 1150 mm (width x depth x height), toe plate 320 x 250 mm, pneumatic wheels 260 x 85 mm, load capacity 200 kg, own weight 11 kg." });

const world = (state: any, a: any): [number, number, number] => {
  const f = state.bones[a.bone].frame;
  return (["x", "y", "z"] as const).map((k) => f.o[k] + a.x * f.u[k] + a.y * f.v[k] + (a.z ?? 0) * f.w[k]) as [number, number, number];
};
const round = (v: number): number => Math.round(v * 1e4) / 1e4;

/** 🤲️ Shoulder flexion, elbow flexion and adduction of one arm that put its grip anchor on the handle bar at lateral `z`; the wrist keeps `WRIST` degrees of extension. */
function solveArm(base: Record<string, any>, side: "l" | "r"): void {
  const z = side === "l" ? GRIP_Z : -GRIP_Z;
  let q = [-10, 45, side === "l" ? -12 : 12];
  const residual = (p: number[]): number[] => {
    const b = { ...base, [`arm-${side}`]: { angleDeg: p[0], abductionDeg: p[2] }, [`forearm-${side}`]: { angleDeg: p[1] }, [`hand-${side}`]: { angleDeg: WRIST } };
    const st = evaluate({ ...asset, poses: [{ ...asset.poses[0], id: "ik", bones: b, ground: "authored" }] } as never, { kind: "pose", pose: "ik" } as never) as any;
    const g = world(st, anchor(`grip-${side}`));
    const h = world(st, anchor("handle-grip"));
    return [g[0] - h[0], g[1] - h[1], g[2] - z];
  };
  for (let it = 0; it < 40; it++) {
    const r = residual(q);
    if (Math.hypot(...r) < 1e-7) break;
    const J = [0, 1, 2].map((k) => {
      const p = [...q];
      p[k]! += 0.01;
      return residual(p).map((v, i) => (v - r[i]!) / 0.01);
    });
    const m = [0, 1, 2].map((i) => [J[0]![i]!, J[1]![i]!, J[2]![i]!, -r[i]!]);
    for (let c = 0; c < 3; c++) {
      let piv = c;
      for (let i = c + 1; i < 3; i++) if (Math.abs(m[i]![c]!) > Math.abs(m[piv]![c]!)) piv = i;
      [m[c], m[piv]] = [m[piv]!, m[c]!];
      for (let i = 0; i < 3; i++) if (i !== c) { const f = m[i]![c]! / m[c]![c]!; for (let k = c; k < 4; k++) m[i]![k]! -= f * m[c]![k]!; }
    }
    q = q.map((v, i) => v + Math.max(-15, Math.min(15, m[i]![3]! / m[i]![i]!)));
  }
  const rest = Math.hypot(...residual(q));
  if (!(rest < 1e-5) || q[1]! < 2) throw new Error(`arm-${side} does not reach the handle: residual ${rest}, q ${q}`);
  base[`arm-${side}`] = { angleDeg: round(q[0]!), abductionDeg: round(q[2]!) };
  base[`forearm-${side}`] = { angleDeg: round(q[1]!) };
  base[`hand-${side}`] = { angleDeg: WRIST };
}

const TILT: Record<string, number> = { waiting: 0, "tilted-back": 6, "parked-upright": -45, "walking-contact": 0 };
for (const pose of asset.poses) {
  pose.bones["hand-truck"] = { angleDeg: TILT[pose.id] };
  if (pose.id === "parked-upright") pose.bones["truck-reach"] = { dy: 0.53 };
  else delete pose.bones["truck-reach"];
  solveArm(pose.bones, "r");
  if (pose.id !== "parked-upright") solveArm(pose.bones, "l");
}
const poseOf = (id: string): any => asset.poses.find((p: any) => p.id === id);
for (const clip of asset.clips)
  for (const key of clip.keys) {
    if (key.bones === undefined) continue;
    if (clip.id === "tilt-and-settle") key.bones["hand-truck"] = { angleDeg: key.t === 1.2 ? 3.3 : key.t === 2.4 ? 6 : 2 };
    const merged = { ...poseOf(key.pose).bones, ...key.bones };
    delete merged["truck-reach"];
    solveArm(merged, "r");
    solveArm(merged, "l");
    for (const id of ["arm-r", "forearm-r", "hand-r", "arm-l", "forearm-l", "hand-l"]) key.bones[id] = merged[id];
  }

const say = (target: any, en: string, de: string): void => void Object.assign(target, { en, de });
say(asset.summary, "A 34-year-old parcel courier of 1.82 m in a high-visibility jacket, cargo trousers, work boots and a cap, wheeling a steel hand truck (1.15 m high, 0.57 m wide, pneumatic wheels 260 x 85 mm) loaded with three parcels (38 kg), balanced on its wheels at 45 degrees. Poses: waiting, tilting the load, parked upright; clips: walking with the load and rocking it back over the wheels.", "Ein 34-jähriger Paketkurier von 1,82 m in Warnschutzjacke, Cargohose, Arbeitsstiefeln und Mütze, der eine Stahlrohr-Sackkarre (1,15 m hoch, 0,57 m breit, Lufträder 260 x 85 mm) mit drei Paketen (38 kg) bei 45 Grad auf den Rädern ausbalanciert schiebt. Posen: Warten, Last kippen, aufrecht abgestellt; Clips: Gehen mit Last und Zurückwiegen der Last über die Räder.");
say(poseOf("waiting").description, "Feet together, truck balanced at 45 degrees on its wheels (centre of gravity of truck and load just ahead of the axle), both gloved hands on the bow handle, parcels standing on the toe plate against the rails.", "Füße zusammen, Karre bei 45 Grad auf den Rädern ausbalanciert (Schwerpunkt von Karre und Last knapp vor der Achse), beide behandschuhten Hände am Bügelgriff, Pakete stehen auf der Schaufel an den Holmen.");
say(poseOf("tilted-back").description, "The courier pulls the handle towards the body past the balance point, truck tilted back 51 degrees, arms nearly straight.", "Der Kurier zieht den Griff über den Gleichgewichtspunkt zum Körper, Karre 51 Grad geneigt, Arme fast gestreckt.");
say(poseOf("parked-upright").description, "Truck stood upright on toe plate and wheels close in front of the courier, the right hand resting on the bow handle, the left arm hanging.", "Karre steht aufrecht auf Schaufel und Rädern dicht vor dem Kurier, die rechte Hand ruht auf dem Bügelgriff, der linke Arm hängt.");
say(poseOf("walking-contact").description, "Frame 0 of the push-walk cycle: left heel strikes, right foot pushes off, the balanced truck rolls ahead of the courier.", "Bild 0 des Schiebe-Gehzyklus: Der linke Fuß setzt mit der Ferse auf, der rechte stößt ab, die ausbalancierte Karre rollt vor dem Kurier.");
const clipOf = (id: string): any => asset.clips.find((c: any) => c.id === id);
say(clipOf("push-walk").description, "Level walking at 0.95 m/s with a 49 kg load, 100 steps per minute; feet planted without sliding, torso leaning 6 degrees forward, both hands on the bow handle, the truck balanced at 45 degrees rolling ahead of the courier with the axle height held on the ground.", "Gehen in der Ebene mit 0,95 m/s und 49 kg Last, 100 Schritte pro Minute; Füße gleiten nicht, Oberkörper 6 Grad nach vorn geneigt, beide Hände am Bügelgriff, die bei 45 Grad ausbalancierte Karre rollt vor dem Kurier, die Achshöhe bleibt am Boden.");
say(clipOf("tilt-and-settle").description, "Non-looping, five seconds: from waiting the courier pulls the handle in, the truck tilts from 45 to 51 degrees past its balance point with a short effort breath, then settles back; feet and wheel stay put.", "Ohne Wiederholung, fünf Sekunden: Aus dem Warten zieht der Kurier den Griff heran, die Karre kippt von 45 auf 51 Grad über den Gleichgewichtspunkt bei kurzer Anstrengungsatmung und setzt sich dann zurück; Füße und Rad bleiben stehen.");
const out = join(GENERATED, "r3-courier-with-hand-truck");
mkdirSync(out, { recursive: true });
writeFileSync(join(out, "🔣️.json"), JSON.stringify(doc));
for (const p of asset.poses) console.log(p.id, JSON.stringify(Object.fromEntries(["arm-l", "forearm-l", "arm-r", "forearm-r"].map((k) => [k, p.bones[k]]))));
console.log(join(out, "🔣️.json"));
