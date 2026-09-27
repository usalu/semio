/** 🌐️ WG9 (ticket-local, from WG7) — serves `wg9-vite.config.ts` on one port for one playground variant whose plugin module is a
 * catalog-exact durable root. Usage: WG9_VARIANT=<variant> WG9_PLUGIN=<pluginId> WG9_MODULE_ROOT=<name> bun serve/wg9-serve.ts <port> */
import { join, resolve } from "node:path";
import { serveVite } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
const port = Number(process.argv[2] ?? 6552);
process.env.SEMIO_PLUGIN = process.env.WG9_PLUGIN ?? "note";
process.env.SEMIO_RENDERER = "wgpu";
process.env.SEMIO_BUILD_MODE = "ship";
const root = join("/Users/ueli/Documents/semio", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server");
await serveVite({ root, config: join(import.meta.dirname, "wg9-vite.config.ts"), configLoader: "native", host: "127.0.0.1", port, signal: new AbortController().signal, ready: (url) => console.log("WG9 catalog serve ready: " + url + "?plugin=" + process.env.SEMIO_PLUGIN) });
