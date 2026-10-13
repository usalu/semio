#!/usr/bin/env bun
import { join } from "node:path";
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts";
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { runRepositoryCargoTests, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/🔨️modules/🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

class TestScript extends BundleScript {
  async run(_segments: string[]): Promise<void> {
    await runRepositoryCargoTests(["semio-s-plugin-flow-bim"], this.repoRoot, this.invocation.control);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the Nx-owned deliverable and accepts no output override");
    await runExtensionComponentPackage({ componentDir: join(import.meta.dir, "../../🧩️component/📦️packages/🦀️rust"), repoRoot: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("package", PackageScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
