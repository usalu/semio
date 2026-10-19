import { receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BundleScript, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runActorNetworkScript } from "./🟦️.ts";
//#endregion Akteursnetz

export class ActorNetworkScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runActorNetworkScript({ root: this.root, repoRoot: this.repoRoot, invocation: this.invocation }, segments);
  }
}


const router = new ScriptRouter(import.meta.dir).register("akteursnetz", ActorNetworkScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (router).run(process.argv.slice(2), original));
