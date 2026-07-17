#!/usr/bin/env bun
/** @emoji 🪞 `@semio-tech/vcs-core` task router: `bun ./script.ts <test|typecheck> [args…]`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runBunx, runVitest } from "../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runVitest(this.root, segments, "js/vitest.config.ts");
  }
}

class TypecheckScript extends BundleScript {
  run(segments: string[]): void {
    runBunx(["tsc", "--noEmit", "-p", "js/tsconfig.json", ...segments], this.root);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("typecheck", TypecheckScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
