#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { BoundedPreparationScript, BoundedSynchronizeScript } from "./🧪️tests/🔬️script-commands/🟦️.ts";


const router = new ScriptRouter(import.meta.dir).register("prepare", BoundedPreparationScript).register("synchronize", BoundedSynchronizeScript);
if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => router.run(process.argv.slice(2), original));