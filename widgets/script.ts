#!/usr/bin/env bun
/** 🧭 `@widgets/react` task router: `bun ./script.ts typecheck`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx } from "../repo/lib/js/src/index.ts";

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "tsconfig.json", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "typecheck" });