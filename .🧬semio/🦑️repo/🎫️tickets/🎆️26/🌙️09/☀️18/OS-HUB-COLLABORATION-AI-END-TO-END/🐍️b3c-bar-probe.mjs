/** 🩺️ B3c bar runner — thin wrapper over the SHARED `🐍️b3a-interaction-probe.mjs` (B3a2's fixed
 * version: UiNode surfaces, app panels in `panelRows`, docked-panel retirement, settle grace).
 *
 * The probe itself is not copied or changed; only the per-variant configuration is supplied here,
 * from the environment, so a verb can be re-picked without editing a file.
 *
 * Usage:
 *   B3C_VARIANT=generation2d B3C_PORT=6021 B3C_ACTION=addGeneration \
 *   [B3C_PLUGIN=generation2d] [B3C_ARGS='{"name":"x"}'] [B3C_SETUP='a,b'] [B3C_PANELS='x,y'] \
 *   bun 🐍️b3c-bar-probe.mjs
 */
import { runInteractionProbe } from "./🐍️b3a-interaction-probe.mjs";

const variant = process.env.B3C_VARIANT;
const port = Number(process.env.B3C_PORT);
const action = process.env.B3C_ACTION ?? "undo";
const plugin = process.env.B3C_PLUGIN ?? variant;
const args = process.env.B3C_ARGS === undefined ? {} : JSON.parse(process.env.B3C_ARGS);
const setup = process.env.B3C_SETUP === undefined ? [] : process.env.B3C_SETUP.split(",").filter(Boolean);
const panels = process.env.B3C_PANELS === undefined ? undefined : process.env.B3C_PANELS.split(",").filter(Boolean);

await runInteractionProbe({ plugin, variant, port, action, args, setup, ...(panels === undefined ? {} : { panels }) });
