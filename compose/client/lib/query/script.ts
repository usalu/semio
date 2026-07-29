#!/usr/bin/env bun
/** 🏛️ `@semio-tech/compose-query` — `bun script.ts <build|test|wasm>`. */
import { BundleScript, ScriptRouter, buildBudgetMs, resolveTestLevel, runBundleScriptMain, runCargoTestBudgeted, runCmd, runWasmPackWebBuild } from "../../../../repo/lib/js/index.ts";
import { join } from "node:path";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: join(this.root, "rs"),
      skipEnvVar: "COMPOSE_SKIP_WASM_BUILD",
      logPrefix: "compose/query",
      wasmBaseName: "compose_query",
      pkg: {
        name: "@semio-tech/compose-query/pkg",
        files: ["compose_query_bg.wasm", "compose_query.js", "compose_query.d.ts", "compose_query_bg.wasm.d.ts"],
        main: "compose_query.js",
        module: "compose_query.js",
        types: "compose_query.d.ts",
      },
    });
  }
}

class SetupScript extends BundleScript {
  run(): void {
    runCmd("rustup", ["target", "add", "wasm32-unknown-unknown"]);
    runCmd("cargo", ["fetch", "--manifest-path", "Cargo.toml"], { cwd: join(this.root, "rs") });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    new WasmScript(this.root, this.repoRoot).run();
    runCmd("cargo", ["build", "--release"], { cwd: join(this.root, "rs"), budgetMs: buildBudgetMs() });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["compose_query"], this.repoRoot, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("setup", SetupScript).register("wasm", WasmScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
