import { mkdirSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { nativeTranspileArgsV1, nativeTranspileBinaryV1, PLUGIN_COMPONENT_IMPORT_MAPS_V1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🦀️native-transpile/🟦️.ts";
import { stripWasmDebugSectionsV1 } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️strip-debug/🟦️.ts";

const source = process.argv[2];
if (!source) throw Error("A compiled plugin component is required");
const directory = join(import.meta.dir, "🗑️generated", "native-transpile-probe");
mkdirSync(directory, { recursive: true });
const slim = join(directory, "input.wasm");
const bytes = stripWasmDebugSectionsV1(readFileSync(source));
writeFileSync(slim, bytes);
console.log("[DEBUG] Stripped component bytes", bytes.length);
const output = Bun.spawn([
  nativeTranspileBinaryV1(process.cwd(), "prototype"),
  ...nativeTranspileArgsV1(slim, join(directory, "out"), "component", PLUGIN_COMPONENT_IMPORT_MAPS_V1),
], { stdout: "inherit", stderr: "inherit" });
process.exitCode = await output.exited;
console.log("[DEBUG] Native transpiler exit", process.exitCode, "signal", output.signalCode);
