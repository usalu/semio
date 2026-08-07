import { readFileSync, writeFileSync, readdirSync, rmSync, existsSync } from "fs";
import { join } from "path";

const ROOT = "/Users/ueli/Documents/semio";
const MODULES = join(ROOT, "🧰️framework", "🔨️modules");
const PKG = join(ROOT, "🧰️framework", "📦️packages", "🟦️typescript");

function findDir(parent, needle) {
  const hit = readdirSync(parent).find((n) => n.includes(needle));
  if (!hit) throw new Error(`missing dir ${needle} in ${parent}`);
  return hit;
}

const manifestDir = findDir(MODULES, "manifest");
const kernelDir = findDir(MODULES, "kernel");
const platformDir = findDir(MODULES, "platform");
const meshDir = findDir(MODULES, "mesh");
const actionDir = findDir(MODULES, "action-bus");
const coreDir = findDir(MODULES, "core");
const comp = "🟦️component.ts";

console.log({ manifestDir, kernelDir, platformDir, meshDir, actionDir, coreDir });

const manifestPath = join(MODULES, manifestDir, comp);
let manifest = readFileSync(manifestPath, "utf8");

const markerStart = "//#region 🧪️Tests";
const markerEnd = "//#endregion 🧪️Tests";
const testStart = manifest.indexOf(markerStart);
const testEnd = manifest.indexOf(markerEnd);
let testsBlock = null;
if (testStart >= 0 && testEnd >= 0) {
  testsBlock = manifest.slice(testStart, testEnd + markerEnd.length);
  manifest = manifest.replace(/import \{[\s\S]*?\} from "\.\.\/[^"]+";\n/g, (m) => {
    if (/organizeContextMenu|createMemoryStoragePort|createDevPluginSource/.test(m)) return "";
    return m;
  });
  manifest = manifest.slice(0, testStart) + manifest.slice(testEnd + markerEnd.length);
  manifest = manifest.replace(/\n{3,}/g, "\n\n");
  if (!manifest.includes("// #endregion")) {
    manifest = manifest.trimEnd() + "\n// #endregion 🛂️Manifest\n";
  }
  writeFileSync(manifestPath, manifest.endsWith("\n") ? manifest : manifest + "\n");
  console.log("stripped tests from manifest", manifest.split("\n").length);
} else {
  console.log("tests already stripped from manifest");
}

for (const dir of [manifestDir, kernelDir, platformDir, meshDir, actionDir]) {
  const p = join(MODULES, dir, comp);
  let t = readFileSync(p, "utf8");
  const next = t.replace(/\.\.\/[^"'/\n]*manifest\//g, `../${manifestDir}/`);
  if (next !== t) {
    writeFileSync(p, next);
    console.log("normalized", dir);
  }
}

if (!testsBlock) {
  console.error("FATAL: tests block missing");
  process.exit(1);
}

const gluePath = join(PKG, "🟦️glue.ts");
const glue = `/** @emoji 📦️ \`@semio-tech/framework\` — package glue (reexports + inline vitest). */
export * from "../../🔨️modules/${actionDir}/${comp}";
export * from "../../🔨️modules/${platformDir}/${comp}";
export * from "../../🔨️modules/${meshDir}/${comp}";
export * from "../../🔨️modules/${manifestDir}/${comp}";
export * from "../../🔨️modules/${kernelDir}/${comp}";

import {
  organizeContextMenu,
} from "../../🔨️modules/${meshDir}/${comp}";
import {
  createMemoryStoragePort,
  emptyPaneState,
  emptySkeleton,
  emptyUiState,
} from "../../🔨️modules/${platformDir}/${comp}";
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
} from "../../🔨️modules/${kernelDir}/${comp}";

${testsBlock}
`;
writeFileSync(gluePath, glue);
console.log("wrote glue", glue.split("\n").length);

const vitestPath = join(PKG, "🧪️vitest.config.ts");
let vitest = readFileSync(vitestPath, "utf8");
vitest = vitest
  .replaceAll("@semio-tech/framework-core", "@semio-tech/framework")
  .replaceAll("📦️index.ts", "🟦️glue.ts");
writeFileSync(vitestPath, vitest);
console.log("updated vitest config");

const pkgJsonPath = join(PKG, "package.json");
let pkgJson = readFileSync(pkgJsonPath, "utf8");
pkgJson = pkgJson
  .replaceAll("@semio-tech/framework-core", "@semio-tech/framework")
  .replace("framework · render-independent shared core", "framework · render-independent shared modules");
writeFileSync(pkgJsonPath, pkgJson);
console.log("renamed TS package");

const corePath = join(MODULES, coreDir);
if (existsSync(corePath)) {
  rmSync(corePath, { recursive: true, force: true });
  console.log("deleted", coreDir);
}

console.log("modules:", readdirSync(MODULES).filter((n) => /manifest|kernel|action|platform|mesh|core/.test(n)));
