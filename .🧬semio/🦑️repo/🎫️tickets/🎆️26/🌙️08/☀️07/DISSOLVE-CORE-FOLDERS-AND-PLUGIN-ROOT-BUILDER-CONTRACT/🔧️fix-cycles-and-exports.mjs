import { readFileSync, writeFileSync } from "fs";
import { join } from "path";

const MODULES="/Users/ueli/Documents/semio/🧰️framework/🔨️modules";
const manifestPath = join(MODULES, "🛂️manifest/🟦️component.ts");
let manifest = readFileSync(manifestPath, "utf8");

// Extract tests region
const testStart = manifest.indexOf("//#region 🧪️Tests");
const testEnd = manifest.indexOf("//#endregion 🧪️Tests");
if (testStart < 0 || testEnd < 0) throw new Error("tests region missing");
const testsBlock = manifest.slice(testStart, testEnd + "//#endregion 🧪️Tests".length);

// Remove test imports and tests from manifest
manifest = manifest.replace(/\nimport \{\n  organizeContextMenu,\n\} from "\.\.\/🔺️mesh\/🟦️component\.ts";\nimport \{\n  createMemoryStoragePort,\n  emptyPaneState,\n  emptySkeleton,\n  emptyUiState,\n\} from "\.\.\/🖥️platform\/🟦️component\.ts";\nimport \{\n  createDevPluginSource,\n  createExtensionSource,\n  multiplexPluginSources,\n  pluginWorkerUrl,\n  resolvePlaygroundBoot,\n  resolvePluginHostConfig,\n  resolvePluginRegistryId,\n  acquirePluginModule,\n  evictPluginModule,\n  createLeasePool,\n\} from "\.\.\/🎠️kernel\/🟦️component\.ts";\n/, "\n");

manifest = manifest.replace(testsBlock + "\n", "");
writeFileSync(manifestPath, manifest);
console.log("manifest lines now", manifest.split("\n").length);

// Write tests into kernel (most subjects) with imports from siblings — OR into package glue
// Prefer package glue so vitest picks them up via re-export chain

const PKG = "/Users/ueli/Documents/semio/🧰️framework/📦️packages/🟦️typescript";
const gluePath = join(PKG, "🟦️glue.ts");
const oldGlue = readFileSync(gluePath, "utf8");
console.log("old glue:", oldGlue);

const newGlue = `/** @emoji 📦️ \`@semio-tech/framework\` — package glue (reexports only). */
export * from "../../🔨️modules/🎯️action-bus/🟦️component.ts";
export * from "../../🔨️modules/🖥️platform/🟦️component.ts";
export * from "../../🔨️modules/🔺️mesh/🟦️component.ts";
export * from "../../🔨️modules/🛂️manifest/🟦️component.ts";
export * from "../../🔨️modules/🎠️kernel/🟦️component.ts";

${testsBlock.replace(/^/gm, "")}
`;

// Tests need the functions in scope — originally they were same-file. With export * they are NOT in local scope.
// Rewrite tests to import from the re-exported modules explicitly at top of test region.

const testPreamble = `
import {
  organizeContextMenu,
} from "../../🔨️modules/🔺️mesh/🟦️component.ts";
import {
  createMemoryStoragePort,
  emptyPaneState,
  emptySkeleton,
  emptyUiState,
  DockLayoutStore,
  DockUiStateStore,
  WindowPaneStateStore,
} from "../../🔨️modules/🖥️platform/🟦️component.ts";
import {
  createDevPluginSource,
  createExtensionSource,
  multiplexPluginSources,
  pluginWorkerUrl,
  resolvePlaygroundBoot,
  resolvePluginHostConfig,
  resolvePluginRegistryId,
  acquirePluginModule,
  evictPluginModule,
  createLeasePool,
} from "../../🔨️modules/🎠️kernel/🟦️component.ts";
`;

// Actually looking at tests - they use symbols directly. Better put tests at end of kernel file with imports.

const kernelPath = join(MODULES, "🎠️kernel/🟦️component.ts");
let kernel = readFileSync(kernelPath, "utf8");
// Ensure bad paths fixed
kernel = kernel.replaceAll("../�️️manifest/", "../🛂️manifest/");

const kernelTestImports = `
import {
  organizeContextMenu,
} from "../🔺️mesh/🟦️component.ts";
import {
  createMemoryStoragePort,
  emptyPaneState,
  emptySkeleton,
  emptyUiState,
} from "../🖥️platform/�📦component.ts";
`;
