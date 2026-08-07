import { readFileSync, writeFileSync, existsSync, readdirSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const MODULES = join(ROOT, "🧰️framework", "🔨️modules");
const PKG = join(ROOT, "🧰️framework", "📦️packages", "🟦️typescript");

function findDir(parent, needle) {
  return readdirSync(parent).find((n) => n.includes(needle));
}

const manifestDir = findDir(MODULES, "manifest");
const kernelDir = findDir(MODULES, "kernel");
const platformDir = findDir(MODULES, "platform");
const meshDir = findDir(MODULES, "mesh");
const actionDir = findDir(MODULES, "action-bus");
const coreDir = findDir(MODULES, "core");

console.log({ manifestDir, kernelDir, platformDir, meshDir, actionDir, coreDir });

const manifestPath = join(MODULES, manifestDir, "🟦️component.ts");
let manifest = readFileSync(manifestPath, "utf8");

const markerStart = "//#region 🧪️Tests";
const markerEnd = "//#endregion 🧪️Tests";
const testStart = manifest.indexOf(markerStart);
const testEnd = manifest.indexOf(markerEnd);
if (testStart < 0 || testEnd < 0) throw new Error("tests markers missing");
const testsBlock = manifest.slice(testStart, testEnd + markerEnd.length);

// Remove circular imports + tests from manifest
manifest = manifest.replace(/import \{\n  organizeContextMenu,\n\} from "\.\.\/[^"]+";\n/g, "");
manifest = manifest.replace(/import \{\n  createMemoryStoragePort,\n  emptyPaneState,\n  emptySkeleton,\n  emptyUiState,\n\} from "\.\.\/[^"]+";\n/g, "");
manifest = manifest.replace(/import \{\n  createDevPluginSource,\n  createExtensionSource,\n  multiplexPluginSources,\n  pluginWorkerUrl,\n  resolvePlaygroundBoot,\n  resolvePluginHostConfig,\n  resolvePluginRegistryId,\n  acquirePluginModule,\n  evictPluginModule,\n  createLeasePool,\n\} from "\.\.\/[^"]+";\n/g, "");
manifest = manifest.slice(0, testStart) + manifest.slice(testEnd + markerEnd.length);
manifest = manifest.replace(/\n{3,}/g, "\n\n");
writeFileSync(manifestPath, manifest);
console.log("manifest lines", manifest.split("\n").length);

// Fix bad paths in all module TS files
for (const dir of [manifestDir, kernelDir, platformDir, meshDir, actionDir]) {
  const p = join(MODULES, dir, "🟦️component.ts");
  let t = readFileSync(p, "utf8");
  const before = t;
  // normalize any wrong manifest relative imports to correct dir name
  t = t.replace(/\.\.\/[^"'/\n]*manifest\//g, `../${manifestDir}/`);
  if (t !== before) {
    writeFileSync(p, t);
    console.log("normalized paths in", dir);
  }
}

// Write package glue with reexports + tests that import from modules
const gluePath = join(PKG, "🟦️glue.ts");
const glue = `/** @emoji 📦️ \`@semio-tech/framework\` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/${actionDir}/🟦️component.ts";
export * from "../../🔨️modules/${platformDir}/🟦️component.ts";
export * from "../../🔨️modules/${meshDir}/🟦️component.ts";
export * from "../../🔨️modules/${manifestDir}/�📦component.ts";
export * from "../../🔨️modules/${kernelDir}/�📦component.ts";
`;
// Fix emoji for component - use correct
const glueFixed = `/** @emoji 📦️ \`@semio-tech/framework\` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/${actionDir}/