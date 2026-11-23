import { chmodSync } from "node:fs";
import { relative, resolve } from "node:path";
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { stageArtifacts } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/📤️publication/🟦️.ts";
import { publishedCargoArtifactDirectoryV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📤️published/🟦️.ts";
import { withRepositoryCompilerStoreV1 } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🏗️compiler/🟦️.ts";
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🏗️build/🎟️capacity/🎫️lease/🟦️.ts";

const manifest = '/home/deck/semio/🧰️framework/🔨️modules/🖼️canvas/🔤️fonts/📦️packages/🦀️rust/Cargo.toml';
const binary = "/dev/shm/semio-lane-d-font-target/debug/pack-typst-font-assets";

class PublishFontToolScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    if (args.length) throw new Error("publish-font-tool takes no arguments");
    chmodSync(binary, 0o755);
    const control = scriptCompilerAdmissionControlV1(this.invocation, "cargo:publish-font-tool", 0);
    await withRepositoryCompilerStoreV1(this.repoRoot, "prototype", control, process.env, async (store, held) => {
      const directory = publishedCargoArtifactDirectoryV1(store.root, manifest, "prototype");
      const owner = relative(this.repoRoot, manifest).replaceAll("\\", "/");
      await stageArtifacts(directory, owner, new Map([["pack-typst-font-assets", binary]]), { control: held, leaseDirectory: resolve(this.repoRoot, ".cache/b/l/m/data") });
      console.log("published font tool", directory);
    });
  }
}

const router = new ScriptRouter(import.meta.dir).register("publish", PublishFontToolScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original }));
