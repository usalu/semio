/** 🔎 Prints the world position of the cane tip, the grip and the feet for every pose and along every clip of elderly-with-cane. */
import { load, rigAt, type Query } from "./r3-lib.ts";

const { asset } = load("humans/elderly-with-cane", "r3-elderly-with-cane-probe.ts");
const f = (n: number): string => n.toFixed(3);
function line(label: string, q: Query): void {
  const rig = rigAt(asset, 50, q);
  const b = Object.fromEntries(rig.bones.map((x) => [x.id, x]));
  const p = (id: string, tip = true): string => { const v = tip ? b[id]!.tip : b[id]!.o; return `(${f(v[0])},${f(v[1])},${f(v[2])})`; };
  console.log(`${label.padEnd(26)} caneTip=${p("cane")} grip=${p("cane", false)} elbowR=${p("arm-r")} toeL=${p("toe-l")} toeR=${p("toe-r")} ankleL=${p("shank-l")} ankleR=${p("shank-r")} hip=${p("pelvis", false)} head=${p("head")}`);
}
for (const pose of asset.poses) line(`pose ${pose.id}`, { kind: "pose", pose: pose.id });
for (const clip of asset.clips) for (let i = 0; i <= 17; i++) { const t = (clip.durationS * i) / 17; line(`${clip.id} t=${t.toFixed(2)}`, { kind: "clip", clip: clip.id, timeS: Math.min(t, clip.durationS - 1e-6) }); }
