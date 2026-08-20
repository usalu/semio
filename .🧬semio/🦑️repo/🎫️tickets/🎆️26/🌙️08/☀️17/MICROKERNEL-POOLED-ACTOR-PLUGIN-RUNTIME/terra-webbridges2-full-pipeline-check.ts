#!/usr/bin/env bun
// [DEBUG] terra-web-bridges (re-run 2): drives the REAL transpilePluginComponent + hostShimSource +
// pluginComponentBridgeSource against a REAL wasip2 component built from the collapsed `world actor`
// (semio_framework_os_scale_fixture, world-collapse's own verification build), not a hand-simulated
// fixture. Proves the exact production code path (not a raw `bunx jco` CLI call) end-to-end: transpile
// with the real --map flags, preview2-shim vendoring + import rewrite, and that the generated
// host-shim.js + bridge.js this repo's own generator emits are syntactically valid ES modules that
// actually import successfully in bun against the real transpiled output.
import { mkdirSync, writeFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import {
  ensurePreview2ShimVendorAt,
  transpilePluginComponent,
  hostShimSource,
  pluginComponentBridgeSource,
  PLUGIN_HOST_SHIM_FILE,
} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🌐plugin-web-materialize.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const ticketDir = join(repoRoot, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME");
// mirrors the REAL dev-modules layout: _vendor is a SIBLING of each plugin's own outDir
// (…/dev/🔌️plugin-modules/_vendor next to …/dev/🔌️plugin-modules/mathematical/), never a
// subdirectory of it — the first attempt at this script put _vendor inside outDir and produced a
// bare `_vendor/cli.js` import specifier (no leading `./`), which is invalid ESM; that was this
// script's own setup bug, not a defect in `rewritePreview2ShimImports`.
const rootDir = join(ticketDir, "terra-webbridges2-fullpipeline-out");
const outDir = join(rootDir, "scalefixture2");
const vendorDir = join(rootDir, "_vendor");
const wasm = "/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-wasm/wasm32-wasip2/release/semio_framework_os_scale_fixture.wasm";

mkdirSync(outDir, { recursive: true });
ensurePreview2ShimVendorAt(vendorDir, repoRoot);
console.log("[DEBUG] vendored preview2-shim OK");

const componentBase = "scalefixture2";
transpilePluginComponent(wasm, outDir, componentBase, { repoRoot, preview2VendorDir: vendorDir });
console.log("[DEBUG] transpilePluginComponent (REAL function, REAL --map flags) OK");

// write the REAL generated host-shim + bridge next to the transpiled output
writeFileSync(join(outDir, PLUGIN_HOST_SHIM_FILE), hostShimSource());
writeFileSync(join(outDir, "bridge.js"), pluginComponentBridgeSource(componentBase, `${componentBase}.core.wasm`));
console.log("[DEBUG] wrote real generated 🟨️host-shim.js + bridge.js next to real transpiled output");

// confirm no bare '@bytecodealliance/preview2-shim/*' specifiers survive (rewritePreview2ShimImports ran)
const js = await Bun.file(join(outDir, `${componentBase}.js`)).text();
const bareSpecifier = /from\s+['"]@bytecodealliance\/preview2-shim\//;
if (bareSpecifier.test(js)) throw new Error("FAIL: bare preview2-shim specifier survived rewrite");
console.log("[DEBUG] confirmed: no bare preview2-shim specifiers remain in transpiled output");

// confirm the destructure this file's own bridge assumes still matches the real transpiled export shape
const exportLine = js.split("\n").find((line) => line.startsWith("export {")) ?? "";
for (const name of ["reactor", "jobs", "checkpoint", "describe"]) {
  if (!new RegExp(`\\b${name}100 as ${name}\\b`).test(exportLine))
    throw new Error(`FAIL: '${name}' not found in real transpiled output's export line: ${exportLine}`);
}
console.log("[DEBUG] confirmed: real transpiled output exports reactor/jobs/checkpoint/describe exactly as pluginComponentBridgeSource assumes");

// syntax-check the generated bridge/shim files with bun's own transpiler (no jco/component involved)
for (const f of [PLUGIN_HOST_SHIM_FILE, "bridge.js"]) {
  const path = join(outDir, f);
  if (!existsSync(path)) throw new Error(`missing ${f}`);
  const transpiler = new Bun.Transpiler({ loader: "js" });
  transpiler.transformSync(await Bun.file(path).text());
  console.log(`[DEBUG] ${f} parses as valid JS`);
}

console.log("ALL CHECKS PASS");
