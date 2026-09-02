// 🛂️ Regenerates `s`'s plugin descriptor from the materialized component, without the dev pipeline.
//
// `materializePlugin` produces the descriptor by EXECUTING the component's `describe` export and then
// binding the result to the artifact's hashes. A descriptor left over from a different build is
// hash-mismatched, and the host then reports "No plugins loaded" with no error at all. The pipeline
// path is unusable here because its cargo step sits behind a permanently contended `target/` lock.
//
// Mirrors `describeBuiltPlugin` + `finalizePluginDescriptor` + `stagePluginDescriptor`.
import { createHash } from "node:crypto";
import { execFileSync } from "node:child_process";
import { copyFileSync, existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { decodePackValue, encodePackValue } from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🟦️.ts";

const repoRoot = "/Users/ueli/Documents/semio";
const outDir = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/🔌️plugin-modules/s");
const ownerRoot = join(repoRoot, "✏️s/🔌️plugins/🪐️space");
const componentModule = join(outDir, "semio_s_plugin_space_component.js");
const coreWasm = join(outDir, "semio_s_plugin_space_component.core.wasm");
const artifact = process.argv[2];

const probe = `
import { pathToFileURL } from "node:url";
const component = await import(pathToFileURL(process.argv[1]).href);
const bytes = await component.describe.describe();
if (!(bytes instanceof Uint8Array) || bytes.length === 0) throw new Error("Invalid descriptor byte extent");
process.stdout.write(Buffer.from(bytes).toString("base64"));
`;
const base64 = execFileSync("node", ["--experimental-wasm-jspi", "--input-type=module", "--eval", probe, componentModule],
  { cwd: repoRoot, encoding: "utf8", maxBuffer: 64 * 1024 * 1024 });

const digest = (path: string) => createHash("sha256").update(readFileSync(path)).digest("hex");
const descriptor = decodePackValue(Buffer.from(base64, "base64")) as { manifest?: { pluginId?: string }; hashes?: Record<string, string> };
if (descriptor?.manifest?.pluginId !== "s" || !descriptor.hashes) throw new Error("Plugin descriptor identity mismatch");
descriptor.hashes.wasmSha256 = digest(artifact);
descriptor.hashes.coreWasmSha256 = digest(coreWasm);
descriptor.hashes.descriptorSha256 = "";
descriptor.hashes.descriptorSha256 = createHash("sha256").update(encodePackValue(descriptor)).digest("hex");

writeFileSync(join(ownerRoot, "🛂️.descriptor.semio"), encodePackValue(descriptor));
writeFileSync(join(ownerRoot, "🔣️.json"), JSON.stringify(descriptor, null, 2) + "\n");
for (const name of ["🛂️.descriptor.semio", "🔣️.json"]) {
  const source = join(ownerRoot, name);
  if (existsSync(source)) copyFileSync(source, join(outDir, name));
}
console.log("descriptor regenerated and staged for", descriptor.manifest?.pluginId);
