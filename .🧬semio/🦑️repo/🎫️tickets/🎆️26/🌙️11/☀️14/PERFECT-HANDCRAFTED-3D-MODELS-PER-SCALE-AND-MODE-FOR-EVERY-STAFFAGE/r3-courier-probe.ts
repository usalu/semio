/** 🔬️ Prints the bone frames (origin, tip, world angle) and the anchors of `humans/courier-with-hand-truck` in a pose: `bun r3-courier-probe.ts [pose|rest] [--asset file.json]`. */
import { readFileSync } from "node:fs";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { entryOf } from "./volumes-lib.ts";
const args = process.argv.slice(2);
const file = args.indexOf("--asset") < 0 ? "" : args[args.indexOf("--asset") + 1]!;
const asset = file === "" ? entryOf("humans/courier-with-hand-truck").asset : JSON.parse(readFileSync(file, "utf8")).asset;
const pose = args[0] !== undefined && !args[0].startsWith("--") ? args[0] : asset.presentation.defaultPose;
const rest = pose === "rest";
const probe = rest ? { ...asset, poses: [...asset.poses, { ...asset.poses[0], id: "rest", bones: {}, ground: "authored" }] } : asset;
const state = evaluate(probe, { kind: "pose", pose } as never) as any;
const r = (a: number[]) => a.map((x) => x.toFixed(4).padStart(8)).join(" ");
for (const bone of asset.rig.bones) {
  const b = state.bones[bone.id];
  const f = b.frame;
  console.log(bone.id.padEnd(11), "o", r([f.o.x, f.o.y, f.o.z]), "tip", r([b.tip.x, b.tip.y, b.tip.z ?? 0]), "deg", ((Math.atan2(f.u.y, f.u.x) * 180) / Math.PI).toFixed(2));
}
for (const a of asset.anchors) {
  const f = state.bones[a.bone].frame;
  console.log("anchor", a.id.padEnd(14), r(["x", "y", "z"].map((k) => f.o[k] + a.x * f.u[k] + a.y * f.v[k] + (a.z ?? 0) * f.w[k])));
}
console.log("bounds", JSON.stringify(state.bounds), "root", JSON.stringify(state.root ?? null));
