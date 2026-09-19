/** 🏭️ process process3d interaction probe (variant `process3d`, port 6022) + its 4 material extensions. */
import { runInteractionProbe } from "./🐍️b3c-interaction-probe.mjs";

const extensions = ["concrete", "metal", "robotic", "wood"].map((name) => ({ id: `process-extension-${name}`, actions: ["addStep"] }));

await runInteractionProbe({ plugin: "process3d", variant: "process3d", port: 6022, action: ["addStep", "redo"], args: {}, extensions });
