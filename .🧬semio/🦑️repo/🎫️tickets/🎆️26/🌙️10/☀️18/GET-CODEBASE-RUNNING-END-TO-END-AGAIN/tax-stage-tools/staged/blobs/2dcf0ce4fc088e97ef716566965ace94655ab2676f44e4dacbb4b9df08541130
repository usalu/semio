#!/usr/bin/env bun
import { ScriptRouter, receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { RustUnitScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";

const router=new ScriptRouter(import.meta.dir).register("unit",RustUnitScript);if(import.meta.main)await receiveScriptProcessInvocation(process.env,original=>router.run(process.argv.slice(2),original));