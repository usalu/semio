/** 📜️ imperative interaction probe (variant `imperative`, port 6076).
 * Bar: the procedure document renders, `addStep` (staged `kind` argument) appends an applied ledger
 * entry, undo retires it, redo reapplies it.
 */
import { runInteractionProbe } from "./🐍️b2b-interaction-probe.mjs";

await runInteractionProbe({ plugin: "imperative", variant: "imperative", port: 6076, action: "addStep", args: { kind: "log.print" } });
