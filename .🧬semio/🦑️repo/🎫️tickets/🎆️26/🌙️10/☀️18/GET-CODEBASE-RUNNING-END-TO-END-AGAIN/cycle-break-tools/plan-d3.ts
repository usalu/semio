/** 🔌️ D3: the server ports are the library's own adapter layer; they move under the library so the library no longer imports the coordinator. */
import { Plan } from "./plan.ts";
import { planMoves } from "./moves.ts";
import { finish } from "./direct.ts";
import { scan } from "../exports-stage-tools/scan.ts";
if (process.env.REFRESH) scan(true);
const S = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server";
const W = "d3-server-ports";
const plan = new Plan();
planMoves(plan, W, "ports are the adapter layer of the server library: the coordinator consumes them", [{ from: `${S}/🎛️coordinator/🔌️ports`, to: `${S}/📚️library/🔌️ports` }]);
plan.edit({ file: `${S}/🎛️coordinator/🔌️ports/🟦️.ts`, from: `new URL("../📦️packages/🟦️typescript/package.json", import.meta.url)`, to: `new URL("../../🎛️coordinator/📦️packages/🟦️typescript/package.json", import.meta.url)`, mode: "text", wave: W, why: "driver resolution stays anchored at the manifest that declares pg until the library package declares it" });
finish(plan, "d3", process.argv);
