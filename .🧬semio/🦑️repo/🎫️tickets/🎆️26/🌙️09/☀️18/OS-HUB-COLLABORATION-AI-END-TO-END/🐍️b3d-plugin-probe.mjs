/** 🎛️ Slice-B3d per-plugin interaction probe launcher.
 * `bun 🐍️b3d-plugin-probe.mjs <plugin> <variant> <port> <action> [argsJson]`
 */
import { runInteractionProbe } from "./🐍️b3d-interaction-probe.mjs";

const [plugin, variant, port, action, argsJson] = process.argv.slice(2);
await runInteractionProbe({ plugin, variant, port: Number(port), action, args: argsJson ? JSON.parse(argsJson) : {} });
