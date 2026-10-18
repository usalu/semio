#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 🀄️ `@semio-tech/wfc-js` router: `bun ./📜️script.ts test`. */
import { resolve } from "node:path";
import { runCmd } from "../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";

const suites = ["🗿️artifacts/**/📚️examples/**/🧪️tests/🧩️example/🟦️.ts", "🗿️artifacts/**/🧬️schema/🧪️tests/🧩️suite/🟦️.ts"];

class TestScript extends BundleScript {
  async run(): Promise<void> {
    const plugin = resolve(this.root, "../..");
    const files = suites.flatMap(pattern => [...new Bun.Glob(pattern).scanSync({ cwd: plugin })].map(path => resolve(plugin, path))).sort();
    runCmd(process.execPath, ["test", ...files], { cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
