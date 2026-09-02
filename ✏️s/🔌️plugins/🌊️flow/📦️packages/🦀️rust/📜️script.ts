#!/usr/bin/env bun
/** 🌊️ `@semio-tech/flow-plugin` router: `bun ./📜️script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, resolveTestLevel, runCargo, runCargoTestBudgeted } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { describePluginComponent } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️describe/📦️packages/🦀️rust/📜️script.ts";

//#region 🧪️Validation
class CheckScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runCargo(["check", "-p", "semio-s-plugin-flow", ...(segments.length ? segments : ["--lib"])], this.repoRoot);
  }
}

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestBudgeted(["semio-s-plugin-flow"], this.repoRoot, rest);
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️fixtures/📜️script.ts");
  }
}
//#endregion 🧪️Validation

/** @emoji 🛂️ Builds this crate's `wasm32-wasip2` component and re-emits `🛂️.descriptor.semio` +
 * `🔣️.json` at this plugin's own owner root (D0-descriptor-plumbing) — the command
 * `📇️registry:check`'s own descriptor-gate warning tells a developer to run. */
class DescribeScript extends BundleScript {
  run(): void {
    process.exit(describePluginComponent(this.repoRoot, "semio-s-plugin-flow", join(this.root, "..", "..")));
  }
}

const router = new ScriptRouter(import.meta.dir).register("check", CheckScript).register("test", TestScript).register("test-source", SourceTestScript).register("describe", DescribeScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
