
import {receiveScriptProcessInvocation} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runMaterialScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 🧱️ Checks both actual dimension-specific Material owners before native table admission and closure. */
class MaterialScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runMaterialScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
await receiveScriptProcessInvocation(process.env,invocation=>runScriptMain(new ScriptRouter(import.meta.dir).register("test",MaterialScript),{invocation}));