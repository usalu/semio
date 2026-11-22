/** 🔎 Author 04 probe: prints bone origins and tips of a candidate in a pose. Usage: bun rework-04-probe.ts <slug> <pose|-> <bone-regex> */
import { readJsonFile, candidatePath } from "./rework-04-kit.ts";
import { rigAt } from "./r3-lib.ts";
const [slug, pose, re] = process.argv.slice(2);
const asset = readJsonFile(candidatePath(slug!)).asset;
const rig = rigAt(asset, 50, { kind: "pose", pose: pose === "-" || pose === undefined ? asset.presentation.defaultPose : pose });
for (const b of rig.bones) if (new RegExp(re ?? ".").test(b.id)) console.log(b.id.padEnd(22), b.o.map((v) => v.toFixed(3)).join(" "), " -> ", b.tip.map((v) => v.toFixed(3)).join(" "));
