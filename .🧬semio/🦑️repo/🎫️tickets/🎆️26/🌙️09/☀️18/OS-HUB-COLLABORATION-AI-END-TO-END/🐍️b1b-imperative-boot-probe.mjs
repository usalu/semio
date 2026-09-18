/** 📜️ imperative react playground boot probe (variant `imperative`, port 6076).
 * Interaction: pick the Demo example, then execute `addStep` from the Imperative window's Actions pane.
 */
import { runBootProbe } from "./🐍️b1b-boot-probe.mjs";

await runBootProbe({ plugin: "imperative", variant: "imperative", port: 6076, readyId: "imperative", exampleLabel: "Demo", action: "addStep", args: {} });
