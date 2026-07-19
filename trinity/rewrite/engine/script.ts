#!/usr/bin/env bun
/** 🦀 `@semio-tech/trinity-core` router: `bun ./script.ts wasm`. */
import { BundleScript, ScriptRouter, playPollingEnv, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "TRINITY_CORE_SKIP_WASM_BUILD",
      logPrefix: "trinity/rewrite/engine",
      wasmBaseName: "trinity_rewrite",
      pkg: {
        name: "@semio-tech/trinity-core",
        files: ["trinity_rewrite_bg.wasm", "trinity_rewrite.js", "trinity_rewrite.d.ts", "trinity_rewrite_bg.wasm.d.ts"],
        main: "trinity_rewrite.js",
        module: "trinity_rewrite.js",
        types: "trinity_rewrite.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["trinity_rewrite"], this.repoRoot, rest, playPollingEnv());
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
