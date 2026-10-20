/** 🧬 D5b: layering of the manifest foundation: onboarding < manifest < ui-contract (window layout down into onboarding, context menu into the manifest, canvas types stay with their generated sources). */
import { Plan } from "./plan.ts";
import { planMoves } from "./moves.ts";
import { finish } from "./direct.ts";
const M = "🧰️framework/🔨️modules";
const W = "d5b-manifest-layering";
const plan = new Plan();
const r = planMoves(plan, W, "layering of onboarding, manifest and ui-contract", [
  { from: `${M}/🖱️ui/🧬️contract/🏷️ui-contract-and-canvas-types`, to: `${M}/🛂️manifest/🏷️ui-contract-and-canvas-types` },
  { from: `${M}/🖱️ui/🧬️contract/🗺️window-layout`, to: `${M}/🎓️onboarding/🗺️window-layout` },
  { from: `${M}/🖱️ui/🧬️contract/📋️context-menu`, to: `${M}/🛂️manifest/📋️context-menu` },
]);
for (const x of r.reports) console.log(x);
finish(plan, "d5b", process.argv);
