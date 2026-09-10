import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
const workspace = "/Users/ueli/Documents/semio";
const require = createRequire(import.meta.url);
const packageRoot = `${workspace}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust`;
const entry = `${workspace}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts`;
const project = JSON.parse(readFileSync(`${packageRoot}/📋️project.json`, "utf8"));
const { cacheInternals } = await import(`${workspace}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🟨️.mjs`);
const declared: unknown[] = cacheInternals.declaredSourceInputs(project, workspace).browserBootSources;
const sourceFiles = declared.filter((i): i is string => typeof i === "string").map((i) => i.replace("{workspaceRoot}/", ""));
const esbuild = await require("esbuild").build({ entryPoints: [entry], absWorkingDir: workspace, bundle: true, write: false, platform: "browser", format: "esm", define: { "import.meta.vitest": "undefined" }, metafile: true, logLevel: "silent" });
const untracked = Object.keys(esbuild.metafile.inputs).filter((i) => !i.endsWith("🤖️generated/🎮️playgrounds.ts") && !sourceFiles.includes(i));
console.log(JSON.stringify({
  declaredCount: sourceFiles.length,
  graphCount: Object.keys(esbuild.metafile.inputs).length,
  bootLivenessDeclared: sourceFiles.some((f) => f.includes("🫀️boot-liveness")),
  bootLivenessInGraph: Object.keys(esbuild.metafile.inputs).some((f) => f.includes("🫀️boot-liveness")),
  wasmCacheInBootGraph: Object.keys(esbuild.metafile.inputs).some((f) => f.includes("🗄️wasm-module-cache")),
  untracked,
}, null, 1));
