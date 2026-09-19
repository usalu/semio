/** 🕸️ dag interaction probe (variant `dag`, port 6017).
 * Bar: the Demo example renders on the node-graph canvas, `addNode` appends an applied ledger entry,
 * undo retires it.
 */
import { runInteractionProbe } from "./🐍️b2b-interaction-probe.mjs";

await runInteractionProbe({ plugin: "dag", variant: "dag", port: 6017, action: "addNode", args: { kind: "computation" } });
