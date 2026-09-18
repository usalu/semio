/** 📖️ playbook react playground boot probe (variant `playbook`, port 6085).
 * Interaction: pick the Demo example, then execute `addStep` from the Builder window's Actions pane.
 */
import { runBootProbe } from "./🐍️b1b-boot-probe.mjs";

await runBootProbe({ plugin: "playbook", variant: "playbook", port: 6085, readyId: "playbook", exampleLabel: "Demo", action: "addStep", args: {} });
