/** 🕸️ dag react playground boot probe (variant `dag`, port 6017).
 * Interaction: pick the Demo example, then execute `addNode` from the DAG window's Actions pane.
 */
import { runBootProbe } from "./🐍️b1b-boot-probe.mjs";

await runBootProbe({ plugin: "dag", variant: "dag", port: 6017, readyId: "dag", exampleLabel: "Demo", action: "addNode", args: {} });
