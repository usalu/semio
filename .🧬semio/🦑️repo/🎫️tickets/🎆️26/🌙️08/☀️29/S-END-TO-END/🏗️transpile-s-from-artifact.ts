// 🏗️ Transpiles an already-built `s` component into `🔌️plugin-modules/s/`, skipping the cargo step.
//
// The dev pipeline always rebuilds before materializing, and that rebuild spends its entire 20-minute
// budget blocked on the shared cargo target lock while other sessions build — so it is killed
// (`spawnSync cargo ETIMEDOUT`) and the transpile never runs, leaving a stale component on disk.
// This reuses the pipeline's OWN `transpilePluginComponentAsync`, so every post-processing step
// (async-result lifting, asset URL rewriting, core-module optimization, preview2 shim imports) runs
// exactly as it would in `materializePlugin` — only the redundant rebuild is skipped.
import { join } from "node:path";
import { writeFileSync } from "node:fs";
import { transpilePluginComponentAsync, ensurePreview2ShimVendorAt, pluginComponentBridgeSource, hostShimSource } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const artifact = process.argv[2];
const outDir = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/s");
const preview2VendorDir = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/_vendor/@bytecodealliance/preview2-shim");

ensurePreview2ShimVendorAt(preview2VendorDir, repoRoot);
await transpilePluginComponentAsync(artifact, outDir, "semio_s_plugin_space_component", { repoRoot, preview2VendorDir });
// 🔗️ `materializePlugin` writes these two alongside the transpile; the runtime fetches
// `<wasmOut>.js` as the plugin's entry, so a stale one silently loads the previous component.
writeFileSync(join(outDir, "🟨️.js"), hostShimSource());
writeFileSync(join(outDir, "semio_s_plugin_space.js"), pluginComponentBridgeSource("semio_s_plugin_space_component", "semio_s_plugin_space.wasm"));
console.log("transpiled + bridge/shim rewritten:", artifact, "->", outDir);
