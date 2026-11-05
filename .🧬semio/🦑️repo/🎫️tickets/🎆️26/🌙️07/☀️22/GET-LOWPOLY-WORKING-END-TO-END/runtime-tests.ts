#!/usr/bin/env repo
/** 🧪️ Runs the owned Lowpoly editor, engine and IO regressions through repository Cargo admission. */
import { BundleScript, ScriptRouter, receiveScriptProcessInvocation, runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runRepositoryCargoTests } from "../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🦀️cargo/🎟️admission/🟦️.ts";

class TestScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runRepositoryCargoTests(["semio-s-artifact-lowpoly-lowpoly-editor", "semio-s-artifact-lowpoly-lowpoly-engine", "semio-s-artifact-lowpoly-lowpoly-io"], this.repoRoot, this.invocation, args);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, invocation => runScriptMain(router, { invocation }));
