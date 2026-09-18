/** 📕️ norm react playground boot probe (variant `din4108`, port 6091 — one of the plugin's fifteen
 * standard variants, all served by the same component). Interaction: execute `evaluate` from the
 * Inputs window's Actions pane and watch the Results pane's check rows change.
 */
import { runBootProbe } from "./🐍️b1b-boot-probe.mjs";

await runBootProbe({ plugin: "norm", variant: "din4108", port: 6091, readyId: "din4108", exampleLabel: "Demo", action: "evaluate", args: {} });
