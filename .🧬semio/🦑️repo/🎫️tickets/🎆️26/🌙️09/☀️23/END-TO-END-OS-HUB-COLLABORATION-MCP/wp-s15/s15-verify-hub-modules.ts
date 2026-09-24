#!/usr/bin/env bun
/** 🔐️ S15: verifies every plugin module a hub serves exactly as the shell's hub source does — index schema, each manifest by
 * its content address (canonical, bound to its package), every file by length, SHA-256 and BLAKE3.
 * usage: bun s15-verify-hub-modules.ts <hubOrigin> */
import {
  decodeTrustedPluginModuleBundleV1,
  trustedPluginModuleBundleSha256V1,
  trustedPluginModuleSourceOfEntryV1,
  validateTrustedPluginModuleIndexV1,
  verifyTrustedPluginModuleFileV1,
} from "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/🧬️schema/🟦️.ts";
const origin = process.argv[2] ?? "http://127.0.0.1:7800";
const route = `${origin}/trusted-catalog/plugin-modules`;
const index = validateTrustedPluginModuleIndexV1(await (await fetch(route)).json());
let failed = 0;
for (const entry of index.modules) {
  const manifestBytes = new Uint8Array(await (await fetch(`${route}/${entry.bundleSha256}`)).arrayBuffer());
  if ((await trustedPluginModuleBundleSha256V1(manifestBytes)) !== entry.bundleSha256) throw new Error(`${entry.pluginId}: manifest content address`);
  const bundle = decodeTrustedPluginModuleBundleV1(manifestBytes, trustedPluginModuleSourceOfEntryV1(entry));
  let bytes = 0, bad = 0;
  for (const file of bundle.files) {
    const body = new Uint8Array(await (await fetch(`${route}/${entry.bundleSha256}/${file.path.split("/").map(encodeURIComponent).join("/")}`)).arrayBuffer());
    bytes += body.byteLength;
    if (!(await verifyTrustedPluginModuleFileV1(file, body))) bad += 1;
  }
  failed += bad;
  console.log(`${entry.pluginId.padEnd(8)} ${entry.version} files=${bundle.files.length} bytes=${bytes} bad=${bad} entry=${bundle.entry} deps=[${entry.dependencies.join(",")}]`);
}
console.log(`generation=${index.generationId} modules=${index.modules.length} failed-files=${failed}`);
process.exit(failed === 0 ? 0 : 1);
