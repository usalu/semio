#!/usr/bin/env bun
/** 🦀 `@semio/rs-wasm` router: `bun ./script.ts <wasm|build|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../repo/lib/js/src/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "SEMIO_SKIP_WASM_BUILD",
      logPrefix: "semio/rs",
      wasmBaseName: "semio",
      pkg: {
        name: "@semio/rs-wasm",
        files: ["semio_bg.wasm", "semio.js", "semio.d.ts", "semio_bg.wasm.d.ts"],
        main: "semio.js",
        module: "semio.js",
        types: "semio.d.ts",
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

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
