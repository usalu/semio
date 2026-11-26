import { createHash } from "node:crypto";
import { mkdtempSync, readFileSync, readdirSync, rmSync, statSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { componentStageTargetsV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🧩️component-stage/🟦️.ts";
import { readComponentReceiptV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏷️component-receipt/🟦️.ts";
import { componentModelLayoutV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️model/📋️contract/🟦️.ts";
import { transpilePluginComponentModulesAsync } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts";
import { optimizePluginCoreModuleAsync, pluginCoreModuleFilesV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle/🏗️materialization/🪶️wasm-opt/🟦️.ts";
import { stageKeyDigestV1, STAGE_CACHE_POLICY_V1, type StageEntryV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import stageSchema from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";

const root = resolve(import.meta.dirname, "../../../../../../..");
const products = join(root, ".cache/b/p/m");
const receiptPath = resolve(process.argv[2]!);
const moduleId = "sourcing-module-windows";
const staged = dirname(receiptPath);
const receipt = readComponentReceiptV1(receiptPath);
const owners = componentStageTargetsV1(root).filter(target => target.pluginId === moduleId && target.packageId === receipt.packageId);
if (receipt.pluginId !== moduleId || owners.length !== 1 || receipt.profile !== "release") throw new Error("Exact Sourcing Windows release required");
const owner = owners[0]!;
const raw = componentModelLayoutV1(products, { version: 1, mode: "release", pluginId: moduleId, cratePath: owner.cratePath }).path;
const component = join(raw, "📦️component.wasm");
const hash = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const digest = (file: string) => hash(readFileSync(file));
if (!readFileSync(join(raw, "🧾️receipt.json")).equals(readFileSync(receiptPath))) throw new Error("Retained linked receipt changed");
if (statSync(component).size !== receipt.wasmBytes || digest(component) !== receipt.wasmSha256) throw new Error("Retained raw component changed");
if (digest(join(raw, "🛂️.descriptor.semio")) !== receipt.semanticDescriptorSha256) throw new Error("Retained descriptor changed");

const base = owner.packageName.replaceAll("-", "_");
const scratch = mkdtempSync(join(import.meta.dirname, "🗑️generated/rw-"));
const output = join(scratch, "out");
const context = { repoRoot: root, mode: "release" as const, preview2VendorDir: "", optimize: true, cores: 1 };
try {
  if (join(output, "interfaces/semio-framework-instance-lifetime.d.ts").length > 256) throw new Error("Replay path exceeds the workspace bound");
  await transpilePluginComponentModulesAsync(component, output, base, context);
  const cores = pluginCoreModuleFilesV1(output, base);
  const external = pluginCoreModuleFilesV1(staged, base);
  const binding = readFileSync(join(staged, base + ".js"), "utf8");
  const inline = new Map(Array.from(binding.matchAll(/const module([1-9]\d*) = base64Compile\('([A-Za-z0-9+/=]+)'\);/g),
    ([, index, bytes]) => [`${base}.core${Number(index) + 1}.wasm`, Buffer.from(bytes!, "base64")]));
  const stagedCores = [...external, ...inline.keys()].sort();
  if (!external.length || JSON.stringify(cores) !== JSON.stringify(stagedCores)) throw new Error(`Replay core inventory ${JSON.stringify(cores)} differs from staged ${JSON.stringify(stagedCores)}`);
  for (const [path, bytes] of inline) {
    const source = readFileSync(join(output, path));
    if (!source.equals(bytes)) throw new Error(`Replay adapter ${path} differs from inline: ${hash(source)} vs ${hash(bytes)}`);
  }
  const files = [];
  for (const path of external) {
    const replay = join(scratch, path);
    const current = join(staged, path);
    await optimizePluginCoreModuleAsync(join(output, path), replay, context);
    const bytes = statSync(replay).size;
    const sha256 = digest(replay);
    if (statSync(current).size !== bytes || digest(current) !== sha256) throw new Error(`Production replay changed ${path}: ${sha256} vs ${digest(current)}`);
    files.push({ path, bytes, sha256 });
  }

  const probes = resolve(process.argv[3]!);
  const expectedInputs = Object.fromEntries([...external, base + ".js", "🟨️.js"].map(path => [path, digest(join(staged, path))]));
  const proven = readdirSync(probes, { withFileTypes: true }).filter(directory => directory.isDirectory() && /^[a-f0-9]{64}$/.test(directory.name)).flatMap(directory => {
    const path = join(probes, directory.name);
    const entry = JSON.parse(readFileSync(join(path, STAGE_CACHE_POLICY_V1.entryFile), "utf8")) as StageEntryV1;
    const [file] = entry.files;
    if (entry.key.schemaVersion !== 2 || file?.sha256 !== receipt.semanticDescriptorSha256) return [];
    if (!Object.entries(expectedInputs).every(([name, sha256]) => entry.key.inputs[name] === sha256)) return [];
    if (entry.digest !== directory.name || entry.key.stage !== "probe" || stageKeyDigestV1(entry.key) !== entry.digest ||
        validateJsonSchemaSubset(stageSchema.definitions.StageEntryV1, entry, stageSchema).length ||
        entry.files.length !== 1 || file.path !== "semantic.bin" ||
        statSync(join(path, file.path)).size !== file.bytes || digest(join(path, file.path)) !== file.sha256) {
      throw new Error("Invalid retained semantic probe");
    }
    return [entry.digest];
  });
  if (proven.length !== 1) throw new Error("Expected one exact retained semantic probe");
  console.log(JSON.stringify({ version: 1, files }));
} finally {
  rmSync(scratch, { recursive: true, force: true });
}
