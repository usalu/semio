#!/usr/bin/env bun
import { receiveScriptProcessInvocation } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { ScriptRouter } from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { NativeDependenciesScript } from "./🟦️.ts";


if (import.meta.main) await receiveScriptProcessInvocation(process.env, original => (new ScriptRouter(import.meta.dir).register("sync", NativeDependenciesScript)).run(process.argv.slice(2), original));