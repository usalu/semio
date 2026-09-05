#!/usr/bin/env bun
/** 📦️ Extension package router: `bun ./📜️script.ts <test|package|describe>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, runExtensionComponentPackage } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describeExtensionComponent } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts";

class TestScript extends BundleScript {
  run(_segments: string[]): void {
    runCargoTestBudgeted(["semio-s-plugin-process-metal"], this.repoRoot);
  }
}

class PackageScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const outPath = segments[0];
    await runExtensionComponentPackage({ rsDir: import.meta.dir, repoRoot: this.repoRoot, outPath });
  }
}

class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describeExtensionComponent(this.repoRoot, import.meta.dir));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("package", PackageScript).register("describe", DescribeScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
