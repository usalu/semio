import { mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";
import { nativeTranspileArgsV1, nativeTranspileBinaryV1, PLUGIN_COMPONENT_IMPORT_MAPS_V1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/🟦️.ts";
import { stripWasmDebugSectionsV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️strip-debug/🟦️.ts";
import { rewriteJcoAsyncResultLifting, rewriteJcoComponentFactory } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧬️jco-module-rewrites/🟦️.ts";
import { PREVIEW2_VENDOR_RELATIVE, rewritePreview2ShimImportSource } from "@semio-tech/framework-plugin/browser-bundle/imports";
import { MODULE_PLUGIN_ROUTE } from "@semio-tech/plugin-registry/deployment";

const base = "semio_s_plugin_flow_math_component";
async function prepareModule(output: string) {
  const modulePath = join(output, base + ".js");
  const source = await readFile(modulePath, "utf8");
  const factory = rewriteJcoComponentFactory(rewriteJcoAsyncResultLifting(source));
  const specifiers = rewritePreview2ShimImportSource(factory, `../..${MODULE_PLUGIN_ROUTE}/${PREVIEW2_VENDOR_RELATIVE}/`);
  if (specifiers !== source) await writeFile(modulePath, specifiers);
}

if (process.argv[2] === "--prepare") {
  await prepareModule(resolve(process.argv[3]));
} else {
  const source = resolve(process.argv[2]);
  const output = resolve(process.argv[3]);
  await mkdir(output, { recursive: true });
  const unstripped = await readFile(source);
  const bytes = stripWasmDebugSectionsV1(unstripped);
  const input = bytes.length === unstripped.length ? source : join(dirname(output), "math-transpile-input.wasm");
  if (input !== source) await writeFile(input, bytes);
  console.log("[DEBUG] Math component transpiler input", bytes.length);
  const child = Bun.spawn([
    nativeTranspileBinaryV1(process.cwd(), "prototype"),
    ...nativeTranspileArgsV1(input, output, base, PLUGIN_COMPONENT_IMPORT_MAPS_V1),
  ], { stdout: "inherit", stderr: "inherit" });
  const code = await child.exited;
  console.log("[DEBUG] Math native transpile", code, child.signalCode);
  process.exitCode = code;
  if (code === 0) await prepareModule(output);
}
