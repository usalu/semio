
import { BundleScript, ScriptRouter } from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {receiveScriptProcessInvocation} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import {runScriptMain} from "../../../../../../../🧰️framework/📦️packages/🟦️typescript/🏃️process/🧭️routing/🟦️.ts";
import { runNodeScript } from "../../🧪️tests/🔬️rust-package-commands/🟦️.ts";
/** 📍️ Verifies both actual dimensional node owner implementations and independent schema projections. */
class NodeScript extends BundleScript {
  async run(args:string[]): Promise<void> {
    await runNodeScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, args);
  }
}
await receiveScriptProcessInvocation(process.env,invocation=>runScriptMain(new ScriptRouter(import.meta.dir).register("test",NodeScript),{invocation}));
