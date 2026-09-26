#!/usr/bin/env bun
/** 🌿️ VCS plugin package command router. */
import { registerPlaygroundSiteBuildCommands, BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { NativeCodecCheckScript } from "../../🧪️tests/📇️native-codecs/🟦️.ts";
import { NativeOpenableIdentityCheckScript } from "../../🧪️tests/🪪️native-openable-identity/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-vcs"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("test", TestScript)
  .register("native-openable-identity-check", NativeOpenableIdentityCheckScript)
  .register("native-codec-check", NativeCodecCheckScript);
registerPlaygroundSiteBuildCommands(router);
if (import.meta.main) await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
