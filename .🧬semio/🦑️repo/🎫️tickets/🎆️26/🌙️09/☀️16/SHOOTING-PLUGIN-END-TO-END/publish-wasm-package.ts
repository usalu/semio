import { readdirSync, statSync } from "node:fs";
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts";
import { withRepositoryCacheStoreV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts";
import { repositoryWasmPackageRequestV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🕸️wasm/📋️contract/🟦️.ts";
import { stageRepositoryArtifacts, withRepositoryArtifactDestinationV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/📦️artifacts/🟦️.ts";
import { rendererWasmPublicationV1 } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🏗️compiler/🌐️wasm/📦️publication/🟦️.ts";

class PublishWasmScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [sourceOwner, outDir] = args;
    if (!sourceOwner || !outDir || args.length !== 2) throw new Error("publish-wasm <sourceOwner> <outDir>");
    const files = new Map(readdirSync(outDir).filter(name => !name.startsWith(".") && statSync(resolve(outDir, name)).isFile()).map(name => [name, resolve(outDir, name)]));
    const control = scriptCompilerAdmissionControlV1(this.invocation, "wasm:publish", 0);
    await withRepositoryCacheStoreV1(this.repoRoot, "products", this.invocation, control, async (products, phase) => {
      const request = repositoryWasmPackageRequestV1(products.root, { version: 1, mode: "prototype", sourceOwner });
      await withRepositoryArtifactDestinationV1({ workspace: this.repoRoot, invocation: this.invocation, storage: products.storage, control: products.control, phaseControl: phase, directory: request.path }, destination => stageRepositoryArtifacts(destination, request.owner, files));
      console.log("published wasm", request.path);
      console.log("owner", request.owner);
      console.log("files", [...files.keys()].join(","));
    });
  }
}


class PublishRendererScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    const [owner, outDir] = args;
    if (!owner || !outDir || args.length !== 2) throw new Error("publish-renderer <owner> <outDir>");
    const files = new Map(readdirSync(outDir).filter(name => !name.startsWith(".") && statSync(resolve(outDir, name)).isFile()).map(name => [name, resolve(outDir, name)]));
    const control = scriptCompilerAdmissionControlV1(this.invocation, "renderer:publish", 0);
    await withRepositoryCacheStoreV1(this.repoRoot, "products", this.invocation, control, async (products, phase) => {
      const publication = rendererWasmPublicationV1(products.root, { version: 1, mode: "prototype", owner });
      await withRepositoryArtifactDestinationV1({ workspace: this.repoRoot, invocation: this.invocation, storage: products.storage, control: products.control, phaseControl: phase, directory: publication.directory }, destination => stageRepositoryArtifacts(destination, publication.owner, files));
      console.log("published renderer", publication.directory);
      console.log("owner", publication.owner);
      console.log("files", [...files.keys()].join(","));
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("publish-wasm", PublishWasmScript).register("publish-renderer", PublishRendererScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
