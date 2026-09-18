#!/usr/bin/env bun
/** 🀄️ `@semio-tech/wfc-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { registerPlaygroundSiteBuildCommands, BundleScript, ScriptRouter, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/🏭️fresh-component/🟦️.ts";

process.env.RUST_MIN_STACK ??= String(32 * 1024 * 1024);

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-wfc"], this.repoRoot, rest);
  }
}

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-wfc", join(this.root, "..", "..")));
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("describe", DescribeScript);
registerPlaygroundSiteBuildCommands(router);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
