#!/usr/bin/env bun
/** 🌿️ VCS plugin package command router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";
import { NativeCodecCheckScript } from "../../🧪️tests/📇️native-codecs/🟦️.ts";
import { NativeOpenableIdentityCheckScript } from "../../🧪️tests/🪪️native-openable-identity/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-vcs"], this.repoRoot, rest);
  }
}

class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-vcs", join(this.root, "..", "..")));
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("describe", DescribeScript)
  .register("native-openable-identity-check", NativeOpenableIdentityCheckScript)
  .register("native-codec-check", NativeCodecCheckScript);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
