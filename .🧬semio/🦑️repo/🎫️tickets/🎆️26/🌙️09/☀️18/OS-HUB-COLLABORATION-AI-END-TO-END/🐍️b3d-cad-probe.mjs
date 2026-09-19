/** 📐️ cad interaction probe (variant `cad`, port 6020) — root plugin + its 4 extensions.
 * Bar: the default model renders, one Actions-panel row appends an applied ledger entry, undo retires it.
 */
import { runInteractionProbe } from "./🐍️b3d-interaction-probe.mjs";

await runInteractionProbe({ plugin: "cad", variant: "cad", port: 6020, action: process.env.B3D_ACTION ?? "addObject", args: {} });
