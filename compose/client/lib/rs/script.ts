#!/usr/bin/env bun
/** 🦀 `@semio-tech/compose-rs-wasm` router: `bun ./script.ts <wasm|build|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: this.root,
      skipEnvVar: "COMPOSE_SKIP_WASM_BUILD",
      logPrefix: "compose/rs",
      wasmBaseName: "compose",
      pkg: {
        name: "@semio-tech/compose-rs-wasm",
        files: ["compose_bg.wasm", "compose.js", "compose.d.ts", "compose_bg.wasm.d.ts"],
        main: "compose.js",
        module: "compose.js",
        types: "compose.d.ts",
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
    if (process.env.COMPOSE_RS_SKIP_WASM !== "1") {
      new WasmScript(this.root, this.repoRoot).run();
    }
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

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
