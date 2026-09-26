#!/usr/bin/env bun
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-process-metal"], this.repoRoot);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw new Error("package writes the Nx-owned deliverable and accepts no output override");
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("package", PackageScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
