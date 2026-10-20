/** 🧪 Loads one Vite config exactly as the product's serve/build script does (same loader) and exits 0 when it resolves.
 * Usage: `bun resolve-vite-config.mjs <config> <serve|build> <loader: runner|native> [root]`; run from the repository root.
 * The factory is evaluated with a dummy bounded cache directory inside the products partition (nothing is created). `serve` needs a real activation. */
import { createRequire } from "node:module";
import { join, resolve } from "node:path";
const root = resolve(process.argv[5] ?? process.cwd());
const [file, command, loader] = process.argv.slice(2);
if (!file || !command || !["runner", "native"].includes(loader)) { console.error("usage: resolve-vite-config.mjs <config> <serve|build> <runner|native> [root]"); process.exit(2); }
process.env.SEMIO_BUILD_MODE ??= "prototype";
process.env.SEMIO_VITE_CACHE_DIRECTORY ??= join(root, ".cache", "b", "p", "m", "vite-probe-resolve-config");
const { resolveConfig } = await import(createRequire(join(root, "package.json")).resolve("vite"));
const started = Date.now();
try {
  const config = await resolveConfig({ configFile: resolve(root, file), mode: process.env.SEMIO_BUILD_MODE, configLoader: loader, logLevel: "error" }, command, process.env.SEMIO_BUILD_MODE);
  console.log(`RESOLVED ${file} ${command}/${loader} in ${Date.now() - started} ms: plugins=${config.plugins.length} port=${config.server?.port}`);
  process.exit(0);
} catch (error) {
  console.log(`FAILED ${file} ${command}/${loader} in ${Date.now() - started} ms: ${String(error?.stack ?? error).split("\n").slice(0, 6).join("\n")}`);
  process.exit(1);
}
