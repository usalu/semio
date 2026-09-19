/** 📖️ playbook interaction probe (variant `playbook`, port 6085).
 * Bar: the Builder window renders the step/block palette, `addStep` (an argument-less verb, so its
 * Actions row IS its trigger) appends an applied ledger entry, undo retires it, redo reapplies it.
 */
import { runInteractionProbe } from "./🐍️b2b-interaction-probe.mjs";

await runInteractionProbe({ plugin: "playbook", variant: "playbook", port: 6085, action: "addStep", args: {} });
