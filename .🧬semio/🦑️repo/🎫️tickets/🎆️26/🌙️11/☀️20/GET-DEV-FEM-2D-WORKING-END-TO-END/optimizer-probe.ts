import { mkdirSync } from "node:fs";
import { join, resolve } from "node:path";
import { normalizePath, optimizeDeps, resolveConfig } from "vite";

const [selection] = process.argv.slice(2);
if (selection !== "configured" && selection !== "entry") throw Error("optimizer-probe <configured|entry>");
const workspace = resolve(import.meta.dirname, "../../../../../../..");
const cacheDir = join(import.meta.dirname, "🗑️generated", `optimizer-${selection}`);
mkdirSync(cacheDir, { recursive: true });
process.env.SEMIO_PLUGIN = "fem2d";
process.env.SEMIO_RENDERER = "react";
process.env.SEMIO_BUILD_MODE = "prototype";
process.env.SEMIO_VITE_CACHE_DIRECTORY = join(workspace, ".cache/b/p/m/vite");
const variant = join(workspace, "✏️s/🧑‍💻dev/🎭️variants/🏗️fem");
const config = await resolveConfig({
  configFile: join(variant, "🏗️builder/🌐️vite/🟦️.ts"),
  configLoader: "native",
  cacheDir, mode: "prototype", logLevel: "silent",
  ...(selection === "entry" && { optimizeDeps: { entries: [join(variant, "🚀️entry/🟦️.ts").replaceAll("\\", "/")] } }),
}, "serve");
if (config.cacheDir !== normalizePath(cacheDir)) throw Error("Optimizer probe requires its isolated cache directory");
const result = await optimizeDeps(config, true);
console.log(JSON.stringify({ entries: config.optimizeDeps.entries, optimized: Object.keys(result.optimized).sort() }, null, 2));
