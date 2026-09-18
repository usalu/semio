/** 💡️ reasoning react playground boot probe (variant `reasoning-wires`, port 6015).
 * Interaction: pick the Demo example, then execute `addNode` from the Canvas window's Actions pane.
 */
import { runBootProbe } from "./🐍️b1b-boot-probe.mjs";

await runBootProbe({ plugin: "reasoning", variant: "reasoning-wires", port: 6015, readyId: "reasoning-wires", exampleLabel: "Demo", action: "addNode", args: {} });
