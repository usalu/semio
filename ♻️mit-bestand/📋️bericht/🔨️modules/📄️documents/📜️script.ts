import { ScriptRouter, receiveScriptProcessInvocation } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BuildScript, CompleteScript } from "./🟦️.ts";

const router=new ScriptRouter(import.meta.dir).register("build",BuildScript).register("complete",CompleteScript);
if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));