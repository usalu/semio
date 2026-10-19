#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolveTestLevel } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** fem TypeScript package */
import { runVitest } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧪️testing/🧪️vitest/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runVitest(this.root, rest, "../../🧪️tests/🎚️config/🟦️.ts");
  }
}
/** ⚖️ Checks portable combined-stress vectors with an independent principal-stress oracle. */
class CombinationStressScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("verify-combination-stress accepts no arguments");
    const source = resolve(this.repoRoot, "✏️s/🔨️modules/🏗️fem/⚙️engine/🧮️analyses/🧪️tests/⚖️combination-stress/🟦️.ts");
    const { verifyCombinationStressOracle } = await import(pathToFileURL(source).href);
    verifyCombinationStressOracle();
  }
}
const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("verify-combination-stress", CombinationStressScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, ...({ defaultCommand: "test" }) }));
