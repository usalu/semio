import { createHash } from "node:crypto";
import { copyFileSync, mkdirSync, readFileSync, readdirSync, writeFileSync } from "node:fs";
import { basename, dirname, join, resolve } from "node:path";
import { spawnSync } from "node:child_process";
import { authoredReleaseTargetsV1, authoredTargetDigestV1, workspace } from "./authored-release-descriptors.ts";
import { demonstratorRuntimeAssetSources } from "../../../../../../../♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/📦️assets/🟦️.ts";
import { copyBrowserArtifacts } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/📦️distribution/🟦️.ts";
import { finalizePluginDescriptor, PLUGIN_DESCRIPTOR_PROBE_SOURCE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🛂️descriptor/🟦️.ts";
import { MODULE_BRIDGE_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🟦️.ts";
import { rewritePreview2ShimImports, transpilePluginComponentModulesAsync } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { hostShimSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪢️host-shim-source/🟦️.ts";
import { monolithicComponentTable, pluginComponentBridgeSource } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts";
import { PLUGIN_HOST_SHIM_FILE } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🧾️contract/🟦️.ts";

const [overlay, plugin, raw, expectedRaw, prepared, auditOnly] = process.argv.slice(2);
if (!overlay || !plugin || !raw || !expectedRaw || !/^[0-9a-f]{64}$/.test(expectedRaw)) throw Error("Exact diagnostic component input required");
const digest = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
if (digest(readFileSync(raw)) !== expectedRaw) throw Error("Raw WASI input differs");
if (auditOnly !== undefined && auditOnly !== "--audit-only") throw Error("Unknown raw probe argument");
const owner = authoredReleaseTargetsV1().filter(target => target.pluginId === plugin);
if (owner.length !== 1 || !owner[0]!.directoryName) throw Error("Diagnostic needs one authored deployment owner");
const target = owner[0]!, base = target.packageName.replaceAll("-", "_");
const expectedSemantic = authoredTargetDigestV1(target);
const all = demonstratorRuntimeAssetSources(workspace, "release");
const support = all.filter(source => ["browser-support:preview2", "browser-support:shard"].includes(source.owner));
const vendor = support.find(source => source.owner === "browser-support:preview2");
const matched = all.filter(source => basename(source.root) === target.directoryName);
if (support.length !== 2 || !vendor || matched.length !== 1) throw Error("Diagnostic delivery inventory differs");
const source = matched[0]!;
const destination = resolve(overlay), staged = join(dirname(destination), "r"), module = join(staged, source.destination);
mkdirSync(module, {recursive: true});
await copyBrowserArtifacts(staged, support, {gzipComponentCores: false});
const stagedFiles = (path: string): string[] => readdirSync(path, {withFileTypes: true}).flatMap(entry => {
  if (entry.isDirectory()) return stagedFiles(join(path,entry.name)).map(file => entry.name + "/" + file);
  if (!entry.isFile()) throw Error("Unexpected staged raw factory entry: " + entry.name);
  return [entry.name];
});
if (prepared) {
  for (const file of stagedFiles(prepared)) {
    const destination = join(module, file);
    if (destination.length > 256) throw Error("Diagnostic file path exceeds 256 characters");
    mkdirSync(dirname(destination), {recursive: true});
    copyFileSync(join(prepared, file), destination);
  }
} else await transpilePluginComponentModulesAsync(resolve(raw), module, base, {repoRoot: workspace, mode: "release", cores: 1, preview2VendorDir: ""});
const verifiedCores = spawnSync("repo", ["tools", "exec", "--light", "python", "--", "-I",
  join(import.meta.dirname,"verify-transpiled-component.py"), raw, module],
  {cwd:workspace, encoding:"utf8", maxBuffer:65536, timeout:30000});
if (verifiedCores.status !== 0 || verifiedCores.error) throw Error("Unbound transpiled core inventory: " + (verifiedCores.error ?? verifiedCores.stderr));
writeFileSync(join(dirname(destination), "raw-cores.json"), verifiedCores.stdout);
const factory = join(module, base + ".js");
writeFileSync(join(module, PLUGIN_HOST_SHIM_FILE), hostShimSource());
rewritePreview2ShimImports(factory, join(staged, vendor.destination));
const probe = join(dirname(destination), "describe.mjs");
writeFileSync(probe, PLUGIN_DESCRIPTOR_PROBE_SOURCE.replace("process.argv[1]", "process.argv[2]"));
const semanticCall = spawnSync("repo", ["tools", "exec", "--light", "node", "--", "--experimental-wasm-jspi", probe, factory],
  {cwd: workspace, encoding: "utf8", maxBuffer: 12 * 1024 * 1024, timeout: 120_000, env: {...process.env, SEMIO_COMPILER_PARALLELISM: "1"}});
if (semanticCall.status !== 0 || semanticCall.error) throw Error("Guest describe failed: " + (semanticCall.error ?? semanticCall.stderr));
const answer = semanticCall.stdout.trim();
if (!/^[A-Za-z0-9+/=]+$/.test(answer)) throw Error("Guest descriptor was missing");
const actual = Buffer.from(answer, "base64");
const semantic = digest(actual);
if (digest(readFileSync(raw)) !== expectedRaw) throw Error("Raw WASI input changed during transpilation");
const descriptor = finalizePluginDescriptor(actual, plugin);
writeFileSync(join(dirname(destination), "guest.json"), descriptor.json);
writeFileSync(join(dirname(destination), "guest.pack"), descriptor.pack);
if (semantic !== expectedSemantic) throw Error(`Guest descriptor differs from checked source pair: ${semantic} != ${expectedSemantic}`);
if (auditOnly) {
  console.log(JSON.stringify({plugin, raw: expectedRaw, semantic, matched: true}));
  process.exit(0);
}
writeFileSync(join(module, "🛂️.descriptor.semio"), descriptor.pack);
writeFileSync(join(module, "🔣️.json"), descriptor.json);
writeFileSync(join(module, MODULE_BRIDGE_FILE), pluginComponentBridgeSource(base, monolithicComponentTable(JSON.parse(descriptor.json), base)));
writeFileSync(join(module, ".semio-artifact.json"), JSON.stringify({version: 1, owner: source.owner, files: stagedFiles(module).sort()}));
await copyBrowserArtifacts(destination, [{...source, root: module}, ...support], {gzipComponentCores: true});
console.log(JSON.stringify({modules: [source.destination], mounts: [source.destination, ...support.map(asset => asset.destination)], raw: expectedRaw, semantic}));
