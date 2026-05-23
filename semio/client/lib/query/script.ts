#!/usr/bin/env bun
/** 🏛️ `@semio/query` — `bun script.ts <build|test|wasm>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "SEMIO_SKIP_WASM_BUILD",
      logPrefix: "semio/query",
      wasmBaseName: "semio_query",
      pkg: {
        name: "@semio/query/pkg",
        files: ["semio_query_bg.wasm", "semio_query.js", "semio_query.d.ts", "semio_query_bg.wasm.d.ts"],
        main: "semio_query.js",
        module: "semio_query.js",
        types: "semio_query.d.ts",
      },
    });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    new WasmScript(this.root, this.repoRoot).run();
    execFileSync("cargo", ["build", "--release"], { stdio: "inherit", cwd: this.root });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["test", ...segments], { stdio: "inherit", cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("wasm", WasmScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
