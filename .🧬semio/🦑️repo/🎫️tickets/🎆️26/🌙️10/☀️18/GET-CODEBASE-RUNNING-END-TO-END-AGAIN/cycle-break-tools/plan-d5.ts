/** 🧬 D5: shared contracts move down to where both sides can reach them (effect vocabulary into one module, ui contract types out of the manifest). */
import { Plan } from "./plan.ts";
import { planMoves } from "./moves.ts";
import { finish } from "./direct.ts";
import { scan } from "../exports-stage-tools/scan.ts";
if (process.env.REFRESH) scan(true);
const M = "🧰️framework/🔨️modules";
const W = "d5-shared-contracts";
const plan = new Plan();
const r = planMoves(plan, W, "contract extraction", [
  { from: `${M}/🎠️kernel/🛠️spawned-job`, to: `${M}/🎠️kernel/⚡️effect/🛠️spawned-job` },
  { from: `${M}/🎠️kernel/🎥️video-render-program`, to: `${M}/🎠️kernel/⚡️effect/🎥️video-render-program` },
  { from: `${M}/🛂️manifest/🏷️ui-contract-and-canvas-types`, to: `${M}/🖱️ui/🧬️contract/🏷️ui-contract-and-canvas-types` },
  { from: `${M}/🛂️manifest/🗺️window-layout`, to: `${M}/🖱️ui/🧬️contract/🗺️window-layout` },
  { from: `${M}/🖱️ui/🎬️scene/📋️context-menu`, to: `${M}/🖱️ui/🧬️contract/📋️context-menu` },
]);
for (const x of r.reports) console.log(x);
const P = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript/📋️project.json";
plan.edit({ file: P, from: `"{workspaceRoot}/🧰️framework/🔨️modules/🎠️kernel/🎥️video-render-program/🟦️.ts",`, to: `"{workspaceRoot}/🧰️framework/🔨️modules/🎠️kernel/⚡️effect/🎥️video-render-program/🟦️.ts",`, mode: "text", wave: W, why: "project input follows the moved module" });
plan.edit({ file: P, from: `"{workspaceRoot}/🧰️framework/🔨️modules/🎠️kernel/🛠️spawned-job/🟦️.ts"`, to: `"{workspaceRoot}/🧰️framework/🔨️modules/🎠️kernel/⚡️effect/🛠️spawned-job/🟦️.ts"`, mode: "text", wave: W, why: "project input follows the moved module" });
finish(plan, "d5", process.argv);
