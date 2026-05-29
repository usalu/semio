#!/usr/bin/env bun
/** 🏛️ `@semio/architect` — `bun script.ts <build|test|wasm>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../repo/lib/js/src/bundle-script.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "SEMIO_SKIP_WASM_BUILD",
      logPrefix: "architect",
      wasmBaseName: "architect",
      pkg: {
        name: "@semio/architect-wasm",
        files: ["architect_bg.wasm", "architect.js", "architect.d.ts", "architect_bg.wasm.d.ts"],
        main: "architect.js",
        module: "architect.js",
        types: "architect.d.ts",
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
