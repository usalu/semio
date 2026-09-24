/** 🌐️ WG7 (ticket-local) — serves `wg7-vite.config.ts` on one port. Usage: bun serve/wg7-serve.ts <port> */
import { join, resolve } from "node:path";
import { serveVite } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts";
const port = Number(process.argv[2] ?? 6552);
process.env.SEMIO_PLUGIN = "note";
process.env.SEMIO_RENDERER = "wgpu";
process.env.SEMIO_BUILD_MODE = "ship";
const root = join("/Users/ueli/Documents/semio", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server");
await serveVite({ root, config: join(import.meta.dirname, "wg7-vite.config.ts"), configLoader: "native", host: "127.0.0.1", port, signal: new AbortController().signal, ready: (url) => console.log("WG7 catalog-note serve ready: " + url + "?plugin=note") });
