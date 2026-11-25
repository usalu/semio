import { mkdir, copyFile, cp, stat, writeFile } from "node:fs/promises";
import { constants } from "node:fs";
import { join, resolve } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const root = process.cwd();
const materialize = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization";
const source = name => pathToFileURL(resolve(root, materialize, name)).href;
const [
  { rewriteJcoAsyncResultLiftingAt, rewriteJcoComponentFactoryAt },
  { rewritePreview2ShimImports },
  { hostShimSource },
  { PLUGIN_HOST_SHIM_FILE },
  { PREVIEW2_VENDOR_RELATIVE },
  { PLUGIN_DESCRIPTOR_PROBE_SOURCE },
  { builtPluginModulesRootIn },
] = await Promise.all([
  import(source("🧬️jco-module-rewrites/🟦️.ts")),
  import(source("🟦️.ts")),
  import(source("🪢️host-shim-source/🟦️.ts")),
  import(source("🧾️contract/🟦️.ts")),
  import(source("../🕸️imports/🟦️.ts")),
  import(source("../🛂️descriptor/🟦️.ts")),
  import(source("../../../🧑‍💻dev/📦️packages/🟦️typescript/♻️activation/🟦️.ts")),
]);
const generated = fileURLToPath(new URL("./🗑️generated/", import.meta.url));
const original = join(generated, "native-transpile-proof");
const scratch = join(generated, "gis-descriptor-proof");
const moduleDir = join(scratch, "module");
const vendor = join(scratch, PREVIEW2_VENDOR_RELATIVE);
const selected = join(builtPluginModulesRootIn(root, "prototype"), PREVIEW2_VENDOR_RELATIVE);
await stat(join(selected, "cli.js"));
await mkdir(moduleDir, { recursive: true });
const base = "semio_hub_gis_component";
await Promise.all([
  copyFile(join(original, base + ".js"), join(moduleDir, base + ".js"), constants.COPYFILE_FICLONE),
  copyFile(join(original, base + ".core.wasm"), join(moduleDir, base + ".core.wasm"), constants.COPYFILE_FICLONE),
  cp(selected, vendor, { recursive: true }),
  writeFile(join(moduleDir, PLUGIN_HOST_SHIM_FILE), hostShimSource()),
  writeFile(join(scratch, "probe.eval"), PLUGIN_DESCRIPTOR_PROBE_SOURCE),
]);
const module = join(moduleDir, base + ".js");
rewriteJcoAsyncResultLiftingAt(module);
rewriteJcoComponentFactoryAt(module);
rewritePreview2ShimImports(module, vendor);
console.log("[DEBUG] Isolated module:", module);
console.log("[DEBUG] Canonical prototype vendor:", selected);
