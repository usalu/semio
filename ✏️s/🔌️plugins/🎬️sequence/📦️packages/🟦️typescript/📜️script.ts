#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 🧪️ Sequence artifact browser, protocol and example integration tests. */
import { resolve } from "node:path";
import { runCmd } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🎛️execution/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";

class TestScript extends BundleScript {
  run(): void {
    const plugin = resolve(this.repoRoot, "✏️s/🔌️plugins/🎬️sequence");
    const subset = resolve(plugin, "🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any");
    for (const entry of [resolve(subset, "✏️editor/🌉️wasm/🧪️tests/🛂️interface/🟨️.js"), resolve(subset, "✏️editor/🌉️wasm/🧪️tests/🖥️host/🟨️.js"), resolve(subset, "✏️editor/🌉️wasm/🧪️tests/📌️retained-actions/🟨️.js"), resolve(plugin, "🗿️artifacts/🎬️sequence/🧪️tests/🌐️browser-consumer/🟨️.js"), resolve(plugin, "🗿️artifacts/🎬️sequence/🧪️tests/🔮️protocol-oracle/🟨️.js")]) {
      runCmd(process.execPath, [entry], { cwd: this.repoRoot });
    }
    runCmd(process.execPath, ["test", resolve(subset, "✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts"), resolve(subset, "📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts")], { cwd: this.repoRoot });
    console.log("Sequence artifact browser, protocol oracle and examples passed");
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
