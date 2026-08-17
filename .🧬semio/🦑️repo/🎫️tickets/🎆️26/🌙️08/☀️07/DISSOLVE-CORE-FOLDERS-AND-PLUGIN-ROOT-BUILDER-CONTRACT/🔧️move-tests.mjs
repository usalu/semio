import { readFileSync, writeFileSync, existsSync } from "fs";
import { join } from "path";

const MODULES="/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const CORE_TS = join(MODULES, "🧩core/🟦️component.ts");
// We already stripped? re-read manifest
const manifestPath = join(MODULES, "🛂️manifest/🟦️component.ts");
let manifest = readFileSync(manifestPath, "utf8");

const markerStart = "//#region 🧪️Tests";
const markerEnd = "//#endregion 🧪️Tests";
let testsBlock = null;
if (manifest.includes(markerStart)) {
  const testStart = manifest.indexOf(markerStart);
  const testEnd = manifest.indexOf(markerEnd) + markerEnd.length;
  testsBlock = manifest.slice(testStart, testEnd);
  // remove imports added for tests
  manifest = manifest.replace(/import \{\n  organizeContextMenu,\n\} from "\.\.\/🔺️mesh\/🟦️component\.ts";\n/g, "");
  manifest = manifest.replace(/import \{\n  createMemoryStoragePort,\n  emptyPaneState,\n  emptySkeleton,\n  emptyUiState,\n\} from "\.\.\/🖥️platform\/🟦️component\.ts";\n/g, "");
  manifest = manifest.replace(/import \{\n  createDevPluginSource,\n  createExtensionSource,\n  multiplexPluginSources,\n  pluginWorkerUrl,\n  resolvePlaygroundBoot,\n  resolvePluginHostConfig,\n  resolvePluginRegistryId,\n  acquirePluginModule,\n  evictPluginModule,\n  createLeasePool,\n\} from "\.\.\/🎠️kernel\/🟦️component\.ts";\n/g, "");
  manifest = manifest.slice(0, testStart) + manifest.slice(testEnd);
  // clean extra blank lines near end
  writeFileSync(manifestPath, manifest.replace(/\n{3,}/g, "\n\n"));
  console.log("stripped tests from manifest, lines", manifest.split("\n").length);
} else {
  console.log("tests already stripped from manifest");
}

// Get tests from original if needed - we may have lost them. Check core barrel - tests gone.
// Recover from git? Can't use git mutating. Check if testsBlock null - read from split backup?
// The temporary barrel overwrote core component.ts! Tests only in manifest if still there.

if (!testsBlock) {
  // try read from ticket if we saved? Otherwise reconstruct from nothing.
  console.log("ERROR: tests block lost?");
  // Check manifest once more
  console.log("manifest has Tests?", readFileSync(manifestPath,"utf8").includes("🧪️Tests"));
  process.exit(1);
}

// Append tests to glue with proper imports so symbols resolve; vitest include may need updating
const PKG = "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript";
const glue = `/** @emoji 📦️ \`@semio-tech/framework\` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/🎯️action-bus/🟦️component.ts";
export * from "../../🔨️modules/🖥️platform/🟦️component.ts";
export * from "../../🔨️modules/🔺️mesh/🟦️component.ts";
export * from "../../🔨️modules/🛂️manifest/🟦️component.ts";
export * from "../../🔨️modules/🎠️kernel/