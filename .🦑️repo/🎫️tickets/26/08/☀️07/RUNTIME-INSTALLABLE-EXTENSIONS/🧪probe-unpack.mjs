import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { zipSync } from "fflate";

const ticketDir = dirname(fileURLToPath(import.meta.url));
const repoRoot = join(ticketDir, "../../../../../../");

const { encodePackValue } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🟦️glue.ts"));
const { unpackExtensionPackage } = await import(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript/🏪️store/📜️store.ts"));

const manifest = encodePackValue({ extensionId: "ticket.probe", label: "Probe", version: "0.0.1", extends: "flow" });
const wasm = new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]);
const zip = zipSync({ "manifest.semio": manifest, "component.wasm": wasm });
const outPath = join(ticketDir, "probe-package.zip");
writeFileSync(outPath, zip);
const unpacked = unpackExtensionPackage(zip);
mkdirSync(join(ticketDir, "probe-out"), { recursive: true });
writeFileSync(join(ticketDir, "probe-out", "result.json"), JSON.stringify(unpacked.manifest, null, 2));
console.log("[DEBUG] probe-unpack", unpacked.manifest.extensionId, "bytes", unpacked.wasmBytes.length);
