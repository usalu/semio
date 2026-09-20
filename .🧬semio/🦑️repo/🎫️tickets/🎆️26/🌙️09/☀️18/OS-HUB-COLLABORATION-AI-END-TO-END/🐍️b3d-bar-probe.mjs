/** 🩺️ Slice-B3d bar runner over the SHARED probe `🐍️b3a-interaction-probe.mjs`.
 *
 * One wrapper for all fourteen B3d variants instead of fourteen one-line files: the variant, port
 * and verb are CLI arguments, so a census run (no verb → the `open-actions` step still dumps the
 * live Actions rail into `report.json`) and a bar run differ only in the third argument.
 *
 * Usage: bun 🐍️b3d-bar-probe.mjs <plugin> <variant> <port> [action] [key=value …]
 */
import { runInteractionProbe } from "./🐍️b3a-interaction-probe.mjs";

const [plugin, variant, port, action = "__census__", ...rest] = process.argv.slice(2);
const args = Object.fromEntries(rest.filter((pair) => pair.includes("=")).map((pair) => {
  const at = pair.indexOf("=");
  return [pair.slice(0, at), pair.slice(at + 1)];
}));
const rowSelector = process.env.SEMIO_PROBE_ROW ?? undefined;
await runInteractionProbe({ plugin, variant, port: Number(port), action, args, rowSelector });
