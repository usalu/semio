#!/usr/bin/env repo
import { receiveScriptProcessInvocation } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
/** 📋️ Forms TypeScript contract and authoring verification. */
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runArtifactTypeScriptPackageMain } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🏗️build/🟦️typescript/🟦️.ts";
import { runOwnedVerifyScript } from "../../🧪️tests/🔬️typescript-package-commands/🟦️.ts";
const publicOptions={suites:["🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts","🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧪️tests/🪶️consumers/🟦️.ts","🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📝️text/📸️snapshot/🔣️json/🧪️tests/🔣️transport/🟦️.ts"]};
class PublicScript extends BundleScript{async run():Promise<void>{await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/forms-js",publicOptions);}}
class TestScript extends BundleScript {
  async run(): Promise<void> {
    await runArtifactTypeScriptPackageMain(import.meta.dir,"@semio-tech/forms-js",publicOptions);

  }
}
/** 🧪️ Runs the artifact-owned original physical transport corpus with cancellation and progress. */
class OwnedVerifyScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runOwnedVerifyScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("verify", OwnedVerifyScript).register("build",PublicScript).register("check",PublicScript).register("test", TestScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));




