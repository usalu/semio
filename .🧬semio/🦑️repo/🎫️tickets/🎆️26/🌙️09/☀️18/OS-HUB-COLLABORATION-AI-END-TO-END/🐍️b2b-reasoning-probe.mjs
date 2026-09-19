/** 💡️ reasoning interaction probe (variant `reasoning-wires`, port 6015).
 * Bar: the Demo example renders on the canvas, `addNode` appends an applied ledger entry, undo retires it.
 */
import { runInteractionProbe } from "./🐍️b2b-interaction-probe.mjs";

await runInteractionProbe({ plugin: "reasoning", variant: "reasoning-wires", port: 6015, action: "addNode", args: {} });
