/** 🌊️ flow interaction probe (variant `flow`, port 6016) + the 9 extension plugins' install rows. */
import { runInteractionProbe } from "./🐍️b3c-interaction-probe.mjs";

const extensions = ["bim", "brep", "dictionary", "draw", "list", "logic", "math", "primitive", "text"]
  .map((name) => ({ id: `flow-extension-${name}`, actions: ["runExtensionAction"] }));

await runInteractionProbe({ plugin: "flow", variant: "flow", port: 6016, action: ["addWidget", "reorganize", "selectAll"], args: {}, extensions });
