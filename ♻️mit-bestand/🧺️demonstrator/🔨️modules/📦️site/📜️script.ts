
import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runBuildScript } from "./🟦️.ts";

/** 🎪️ Publishes the Demonstrator using prerequisites selected by the outer build graph. */
class BuildScript extends BundleScript {
  async run(args: string[]): Promise<void> {
    await runBuildScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
