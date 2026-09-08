#!/usr/bin/env bun
/** 🧪️ Sequence artifact browser, protocol and example integration tests. */
import { resolve } from "node:path";
import { BundleScript, ScriptRouter, runCmd, runBundleScriptMain } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

class TestScript extends BundleScript {
  run(): void {
    const plugin = resolve(this.repoRoot, "✏️s/🔌️plugins/🎬️sequence");
    const subset = resolve(plugin, "🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any");
    for (const entry of [resolve(subset, "✏️editor/🌉️wasm/🧪️tests/🧬️schema/🟨️.js"), resolve(subset, "✏️editor/🌉️wasm/🧪️tests/🖥️host/🟨️.js"), resolve(subset, "✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js"), resolve(plugin, "🧪️tests/🌐️browser-consumer/🟨️.js"), resolve(plugin, "🧪️tests/🔮️protocol-oracle/🟨️.js")]) {
      runCmd(process.execPath, [entry], { cwd: this.repoRoot });
    }
    runCmd(process.execPath, ["test", resolve(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts"), resolve(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts")], { cwd: this.repoRoot });
    console.log("[DEBUG] Sequence artifact browser, protocol oracle and examples passed");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
