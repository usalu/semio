/** 🧊️ Checks the 3D fields of the evaluated state against solveChain: bone frame/tip, anchor z, root, kinematics. Usage: bun state3d.ts slug pose */
import { readFileSync, readdirSync, existsSync } from "node:fs";
import { join } from "node:path";
import type { Asset } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";
import { evaluate, sampleOf } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveChain } from "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🦴️rig/🟦️.ts";

const ROOT = join(import.meta.dirname, "../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog");
const [slug, pose] = process.argv.slice(2);
let asset: Asset | undefined;
for (const g of readdirSync(ROOT, { withFileTypes: true }).filter((e) => e.isDirectory())) {
  const f = join(ROOT, g.name, slug!, "🔣️.json");
  if (existsSync(f)) asset = (JSON.parse(readFileSync(f, "utf8")) as { asset: Asset }).asset;
}
const query = { kind: "pose", pose: pose ?? asset!.presentation.defaultPose } as const;
const state = evaluate(asset!, query);
const sample = sampleOf(asset!, query);
const chain = solveChain(asset!.rig, sample.offsets, sample.root);
let worst = 0;
for (const bone of asset!.rig.bones) {
  const f = chain[bone.id]!.frame;
  const b = state.bones[bone.id]!;
  const tipZ = f.o[2] + f.u[2] * bone.lengthM;
  worst = Math.max(worst, Math.abs(b.frame.o.z - f.o[2]), Math.abs(b.tip.z - tipZ), Math.abs(b.frame.u.z - f.u[2]), Math.abs(b.frame.w.z - f.w[2]));
}
console.log(JSON.stringify({ asset: slug, worstFrameError: worst, root: state.root, anchorsZ: Object.fromEntries(Object.entries(state.anchors).map(([k, v]) => [k, +v.z.toFixed(3)])), com: state.kinematics.comVelocity }));
