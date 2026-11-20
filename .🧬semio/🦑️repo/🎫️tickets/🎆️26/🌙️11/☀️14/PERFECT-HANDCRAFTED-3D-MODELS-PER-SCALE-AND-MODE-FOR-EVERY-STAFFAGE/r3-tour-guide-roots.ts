/** 🚩 Root fixes of `humans/tour-guide` outside `volumes` and `representations`: the rig after Winter 2009 for a stature of 1.75 m (1.765 m in shoes with a 15 mm sole), the shoulder joints 0.35 m apart, the hip, knee and ankle heights, a foot of a 0.30 m shoe, the pointer held along the hand and the flag pole in the palm of the fist; the anchors; and the stance of every pose (knees almost straight, the free leg a little forward, both soles flat on the ground).
 *
 * Usage: `bun r3-tour-guide-roots.ts` writes `🗑️generated/r3-tour-guide/roots.json` (members `rig`, `anchors`, `poses`; spliced by `r3-tour-guide-apply.py`). Every value is absolute, so the script may run on the original or on the corrected asset.
 * @see ./r3-humans-tour-guide.md */
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { rigAt } from "./r3-lib.ts";
import { entryOf, type Json } from "./volumes-lib.ts";

const { asset: catalog } = entryOf("humans/tour-guide");
const asset = JSON.parse(JSON.stringify(catalog)) as Json;
const r4 = (x: number): number => Math.round(x * 1e4) / 1e4;
const r2 = (x: number): number => Math.round(x * 100) / 100;
const bone = (id: string): Json => asset.rig.bones.find((b: Json) => b.id === id);

/** 📏 Stature 1.75 m barefoot after Winter 2009 (hip 0.52 H, knee 0.285 H, ankle 0.039 H, shoulder 0.818 H, upper arm 0.186 H, forearm 0.146 H, hand 0.108 H), plus a 15 mm sole. */
export const LIMB = { H: 1.75, sole: 0.015, ankle: 0.083, shank: 0.431, thigh: 0.411, shoulderX: 0.4221, shoulderZ: 0.175, upper: 0.3255, fore: 0.2555, hand: 0.189, head: 0.2826, foot: 0.235, ball: 0.14, toe: 0.095, grip: 0.08, palm: 0.026 } as const;
const scaleMass = (b: Json, length: number): void => {
  if (b.mass) b.mass.comX = r4((b.mass.comX / b.lengthM) * length);
  b.lengthM = length;
};
for (const side of ["l", "r"]) {
  const s = side === "l" ? 1 : -1;
  Object.assign(bone(`upper-arm-${side}`), { x: LIMB.shoulderX, z: s * LIMB.shoulderZ });
  scaleMass(bone(`upper-arm-${side}`), LIMB.upper);
  bone(`forearm-${side}`).x = LIMB.upper;
  scaleMass(bone(`forearm-${side}`), LIMB.fore);
  bone(`hand-${side}`).x = LIMB.fore;
  scaleMass(bone(`hand-${side}`), LIMB.hand);
  scaleMass(bone(`thigh-${side}`), LIMB.thigh);
  bone(`shank-${side}`).x = LIMB.thigh;
  scaleMass(bone(`shank-${side}`), LIMB.shank);
  bone(`foot-${side}`).x = LIMB.shank;
  scaleMass(bone(`foot-${side}`), LIMB.foot);
  bone(`foot-${side}`).mass.comY = r4(-LIMB.ankle / 2);
  Object.assign(bone(`toe-${side}`), { x: LIMB.ball, y: -LIMB.ankle, lengthM: LIMB.toe });
}
scaleMass(bone("head"), LIMB.head);
Object.assign(bone("pointer"), { x: 0.07, y: 0.02, z: 0.024 });
Object.assign(bone("flag-pole"), { x: LIMB.grip, y: 0, z: -LIMB.palm });

/* ---------------------------------------------------------------- anchors */
const anchor = (id: string): Json => asset.anchors.find((a: Json) => a.id === id);
for (const side of ["l", "r"]) {
  Object.assign(anchor(`contact-${side}`), { x: LIMB.ball, y: -LIMB.ankle, footprintLengthM: 0.2, footprintWidthM: 0.1 });
  Object.assign(anchor(`heel-${side}`), { x: -0.03, y: -LIMB.ankle, footprintLengthM: 0.08, footprintWidthM: 0.075 });
  anchor(`grip-${side}`).x = LIMB.grip;
}
Object.assign(anchor("mouth"), { x: 0.1, y: -0.104 });
Object.assign(anchor("nostrils"), { x: 0.122, y: -0.112 });
Object.assign(anchor("crown"), { x: 0.297, y: 0.005 });
anchor("pointer-tip").x = 0.55;

/* ---------------------------------------------------------------- stance */
/** 🦵 The legs of a pose: the standing leg with its hip flexion and a slightly flexed knee, the free leg with its hip flexion; the knee of the free leg follows so that both soles lie on one ground (hip to sole = thigh cos t + shank cos(t + s) + ankle, the pelvis stays level), the shank leaning back, and both feet are set flat. */
const STANCE: Record<string, { stand: "l" | "r"; t: number; s: number; free: number }> = {
  "standing-pointing": { stand: "r", t: 2, s: -7, free: 4 },
  "gesturing-to-group": { stand: "r", t: 2, s: -7, free: 4 },
  "holding-flag": { stand: "r", t: 1, s: -5, free: 3 },
  waving: { stand: "r", t: 2, s: -6, free: 3 },
  "wave-swing-a": { stand: "r", t: 2, s: -6, free: 3 },
  "wave-swing-b": { stand: "r", t: 2, s: -6, free: 3 },
  "turning-to-group": { stand: "l", t: -4, s: -9, free: 11 },
};
const pose = (id: string): Json => asset.poses.find((p: Json) => p.id === id);
const cos = (d: number): number => Math.cos((d * Math.PI) / 180);
for (const [id, st] of Object.entries(STANCE)) {
  const b = pose(id).bones as Json;
  const drop = LIMB.thigh * cos(st.t) + LIMB.shank * cos(st.t + st.s);
  const k = (drop - LIMB.thigh * cos(st.free)) / LIMB.shank;
  if (k > 1) throw new Error(`${id}: free leg cannot reach the ground`);
  const fs = -((Math.acos(k) * 180) / Math.PI) - st.free;
  const other = st.stand === "l" ? "r" : "l";
  b[`thigh-${st.stand}`] = { angleDeg: st.t };
  b[`shank-${st.stand}`] = { angleDeg: st.s };
  b[`foot-${st.stand}`] = { angleDeg: -(st.t + st.s) };
  b[`thigh-${other}`] = { angleDeg: st.free };
  b[`shank-${other}`] = { angleDeg: r2(fs) };
  b[`foot-${other}`] = { angleDeg: r2(-(st.free + fs)) };
  const f = rigAt(asset, 50, { kind: "pose", pose: id }).frames;
  const sole = (s: string): string => (f[`foot-${s}`]!.o[1] - LIMB.ankle).toFixed(4);
  console.log(id.padEnd(20), "sole l/r", sole("l"), sole("r"), JSON.stringify(Object.fromEntries(["thigh-l", "shank-l", "foot-l", "thigh-r", "shank-r", "foot-r"].map((k2) => [k2, b[k2].angleDeg]))));
}

if (import.meta.main) {
  const out = join(import.meta.dir, "🗑️generated", "r3-tour-guide");
  mkdirSync(out, { recursive: true });
  writeFileSync(join(out, "roots.json"), JSON.stringify({ rig: asset.rig, anchors: asset.anchors, poses: asset.poses }));
}
