import { copyFile, mkdir, readFile, writeFile } from "node:fs/promises";
import { dirname, join, resolve } from "node:path";

const workspace = resolve(import.meta.dirname, "../../../../../../..");
const framework = join(workspace, "🧰️framework");
const processRoot = join(framework, "🔨️modules/🏃️process/📦️packages/🟦️typescript");
const os = join(framework, "🛍️products/💻️os/🔨️modules");
const plugin = join(os, "🔌️plugin");
const repo = join(framework, "🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript");
const { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } = await import(join(processRoot, "🧭️routing/🟦️.ts"));
const { scriptCompilerAdmissionControlV1 } = await import(join(processRoot, "🏗️build/🎟️capacity/🟦️.ts"));
const { withArtifactScratchV1, withArtifactDestinationTaskV1, withRepositoryArtifactV1, withRepositoryArtifactTaskV1, withRepositoryArtifactDestinationV1, stageRepositoryArtifacts, artifactsStorageSchema, readNativeCallerAuthorityV1 } = await import(join(repo, "📦️artifacts/🟦️.ts"));
const { withRepositoryCacheStoreV1 } = await import(join(repo, "💾️storage/🟦️.ts"));
const { componentModuleOwnerV1, MODULE_BRIDGE_FILE } = await import(join(plugin, "📇️registry/📦️deployment/🟦️.ts"));
const { artifactFiles, fileDigest } = await import(join(plugin, "🌐️browser-bundle/📦️distribution/📋️inventory/🟦️.ts"));
const { pluginComponentBridgeSource } = await import(join(plugin, "🌐️browser-bundle/🏗️materialization/🌉️component-bridge-source/🟦️.ts"));
const { componentStageTargetV1 } = await import(join(plugin, "🖨️describe/🧩️component-stage/🟦️.ts"));
const { COMPONENT_RECEIPT_FILE, readComponentReceiptV1 } = await import(join(plugin, "🖨️describe/🏷️component-receipt/🟦️.ts"));
const { builtPluginModulesRootIn } = await import(join(os, "🧑‍💻dev/📦️packages/🟦️typescript/♻️activation/🟦️.ts"));

const composition = join(workspace, "🌎️hub/🧩️compositions/📕️norm");
const components = new Map([
  ["norm", ""],
  ["norm-energy", "🧩️extensions/📒️energy"],
  ["norm-eurocode", "🧩️extensions/📗️eurocode"],
  ["norm-eurocode-materials", "🧩️extensions/📙️eurocode-materials"],
  ["norm-eurocode-structural", "🧩️extensions/📓️eurocode-structural"],
]);

/** 🌉️ Regenerates bridge text for an exact completed browser module through owned artifact publication. */
class BridgeRefresh extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [action, ...selections] = args;
    if ((action !== "prove" && action !== "publish") || !selections.length) throw Error("Expected prove|publish <component=wasm-sha256>...");
    const selected = new Map(selections.map(value => {
      const [id, digest, extra] = value.split("=");
      if (!id || !components.has(id) || !digest || !/^[0-9a-f]{64}$/u.test(digest) || extra !== undefined) throw Error("Invalid component identity or WASM digest");
      return [id, digest];
    }));
    if (selected.size !== selections.length) throw Error("Duplicate component identity");
    const authority = readNativeCallerAuthorityV1(this.invocation.capabilities);
    const mode = authority.original.buildMode;
    if (workspace !== authority.repositoryRoot || mode !== "prototype") throw Error("Prototype repository authority required");
    const limits = artifactsStorageSchema.definitions.Protocol.const;
    const control = scriptCompilerAdmissionControlV1(this.invocation, "browser:bridge-refresh", 0);
    await withRepositoryCacheStoreV1(workspace, "products", this.invocation, control, async (store, phaseControl) => {
      for (const [id, digest] of selected) {
        const target = componentStageTargetV1(workspace, join(composition, components.get(id)!, "🧩️component/📦️packages/🦀️rust/Cargo.toml"));
        if (target.pluginId !== id || target.directoryName === undefined) throw Error("Browser component identity mismatch");
        const module = join(builtPluginModulesRootIn(workspace, mode), target.directoryName);
        const owner = componentModuleOwnerV1(target.cratePath);
        const funding = { workspace, invocation: this.invocation, storage: store.storage, control: store.control };
        await withArtifactScratchV1({ ...funding, phaseControl, request: { version: 1, mode, owner: "browser/bridge-proof" } }, scratch =>
          withArtifactDestinationTaskV1(scratch, this.invocation, async sink => {
            const inputs = join(sink.directory, "source");
            const receiptPath = join(module, COMPONENT_RECEIPT_FILE);
            const request = { version: 1, path: module, owner, maximumBytes: limits.maximumOutputBytes, maximumFiles: limits.maximumFiles };
            const priorReceipt = await withRepositoryArtifactV1({ ...funding, phaseControl: sink.control, request }, reader =>
              withRepositoryArtifactTaskV1(reader, this.invocation, async artifact => {
                const receipt = readComponentReceiptV1(receiptPath);
                if (receipt.pluginId !== id || receipt.profile !== mode || receipt.wasmSha256 !== digest) throw Error(`${id}: staged receipt differs from the requested guest`);
                const prior = await readFile(receiptPath);
                for (const name of artifact.files) {
                  const path = join(inputs, name);
                  await mkdir(dirname(path), { recursive: true });
                  await copyFile(join(artifact.path, name), path);
                }
                await artifact.recheck();
                return prior;
              }));
            const bridge = pluginComponentBridgeSource(target.packageName.replaceAll("-", "_"), target.wasmOut);
            const before = await readFile(join(inputs, MODULE_BRIDGE_FILE), "utf8");
            await writeFile(join(inputs, MODULE_BRIDGE_FILE), bridge);
            const destination = action === "publish" ? module : join(sink.directory, "verified");
            if (action === "publish" && !(await readFile(receiptPath)).equals(priorReceipt)) throw Error(`${id}: staged receipt moved during refresh`);
            if (action === "prove" || before !== bridge) await withRepositoryArtifactDestinationV1({ ...funding, phaseControl: sink.control, directory: destination }, token =>
              stageRepositoryArtifacts(token, owner, artifactFiles(inputs)));
            const files = artifactFiles(inputs);
            const published = artifactFiles(destination);
            if (!published.delete(".semio-artifact.json")) throw Error("Browser artifact owner marker is missing");
            if (files.size !== published.size) throw Error("Browser artifact inventory changed during publication");
            for (const [name, path] of files) {
              const output = published.get(name);
              if (output === undefined || await fileDigest(path, sink.control.signal) !== await fileDigest(output, sink.control.signal)) throw Error(`Browser artifact bytes differ: ${name}`);
            }
            console.log(`${action} ${id} ${digest}: ${files.size} verified files; bridge ${before === bridge ? "current" : "regenerated"}`);
          }));
      }
    });
  }
}

const router = new ScriptRouter(import.meta.dirname).register("refresh", BridgeRefresh);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
