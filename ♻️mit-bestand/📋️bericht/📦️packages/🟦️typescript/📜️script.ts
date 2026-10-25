#!/usr/bin/env repo
import { receiveScriptProcessInvocation, runScriptMain, ScriptRouter } from "../../../../🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { PreviewScript, TestScript } from "../../🔨️modules/✅️report-verification/🟦️.ts";

/** 🧭️ `@semio-tech/mit-bestand-bericht` router: `repo ./📜️script.ts <test|preview-generated>`; the routes live in `🔨️modules/✅️report-verification`. */
const router = new ScriptRouter(import.meta.dirname).register("test", TestScript).register("preview-generated", PreviewScript);
await receiveScriptProcessInvocation(process.env, original => runScriptMain(router, { invocation: original, defaultCommand: "test" }));
