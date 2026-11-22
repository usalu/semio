/** 🏗️ Computes the exact lifting forces of a lifted part: the product's own solver run on a copy of the candidate in which only the subtree of the lifted bone has mass and only the lifting anchors are supports, so the sling forces are the statics of the lifted body alone.
 *
 * `$BUN lift-loads.ts <slug> <pose> <lifted bone> <lifting anchor> [<lifting anchor> …]` prints the weight of the subtree and the force per anchor; the author writes these numbers into the pose as applied loads (pattern of `used-playground-roundabout`).
 *
 * @see ./requests.md — why lifting supports beside footing supports share the weight wrongly */
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { evaluate } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🧮️evaluation/🟦️.ts";
import { solveStructure } from "../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🏗️structure/🟦️.ts";

const [slug = "", poseId = "", lifted = "", ...lifts] = process.argv.slice(2);
const asset = JSON.parse(readFileSync(join(import.meta.dir, "assets", slug, "🔣️.json"), "utf8")).asset;
const parentOf = new Map<string, string | null>(asset.rig.bones.map((bone: { id: string; parent: string | null }) => [bone.id, bone.parent]));
const inSubtree = (id: string | null | undefined): boolean => (id === null || id === undefined ? false : id === lifted || inSubtree(parentOf.get(id)));
let kg = 0;
for (const bone of asset.rig.bones) {
  if (inSubtree(bone.id)) kg += bone.mass?.kg ?? 0;
  else if (bone.mass) bone.mass = { ...bone.mass, kg: 0 };
}
for (const anchor of asset.anchors) if (lifts.includes(anchor.id)) Object.assign(anchor, { kind: "support", support: "pin" });
asset.structure.supports = [];
asset.structure.dynamics = "quasi-static";
const pose = asset.poses.find((candidate: { id: string }) => candidate.id === poseId);
pose.contacts = lifts;
pose.loads = [];
const result = solveStructure(evaluate(asset, { kind: "pose", pose: poseId }));
console.log(`${slug} ${poseId}: subtree of ${lifted} ${kg.toFixed(3)} kg = ${(kg * result.gravityMS2).toFixed(1)} N; residual fy ${result.residual.fyN.toFixed(3)} N`);
for (const reaction of result.reactions) if (reaction.active) console.log(`  ${reaction.anchor}: fxN ${reaction.fxN.toFixed(1)} fyN ${reaction.fyN.toFixed(1)} fzN ${reaction.fzN.toFixed(1)}`);
