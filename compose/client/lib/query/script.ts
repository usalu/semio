#!/usr/bin/env bun
/** 🏛️ `@semio-tech/compose-query` — `bun script.ts <build|test|wasm>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
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
    execFileSync("rustup", ["target", "add", "wasm32-unknown-unknown"], { stdio: "inherit" });
    execFileSync("cargo", ["fetch", "--manifest-path", "Cargo.toml"], { stdio: "inherit", cwd: this.root });
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
  .register("setup", SetupScript)
  .register("wasm", WasmScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
