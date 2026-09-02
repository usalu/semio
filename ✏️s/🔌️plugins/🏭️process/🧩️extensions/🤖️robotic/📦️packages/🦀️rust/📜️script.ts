#!/usr/bin/env bun
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-process-robotic"], this.repoRoot);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const outPath = segments[0];
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot, outPath });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("package", PackageScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
