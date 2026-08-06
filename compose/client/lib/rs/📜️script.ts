#!/usr/bin/env bun
/** 🦀️ `@semio-tech/compose-rs-wasm` router: `bun ./📜️script.ts <wasm|build|test>`. */
import { BundleScript, ScriptRouter, buildBudgetMs, runBundleScriptMain, runWasmPackWebBuild, runCargoTestBudgeted, resolveTestLevel, runCmd } from "../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️lib/📦️packages/🟦️typescript/📦️index.ts";

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
    runCmd("rustup", ["target", "add", "wasm32-unknown-unknown"]);
    runCmd("cargo", ["fetch", "--manifest-path", "Cargo.toml"], { cwd: this.root });
  }
}

class BuildScript extends BundleScript {
  run(): void {
    if (process.env.COMPOSE_RS_SKIP_WASM !== "1") {
      new WasmScript(this.root, this.repoRoot).run();
    }
    runCmd("cargo", ["build", "--release"], { cwd: this.root, budgetMs: buildBudgetMs() });
  }
}

/** ⏱️Level-budgeted; heavy fixture/sqlite-replay tests live in `mod quick`, and tests that route through `dispatch_wip_wait`'s 30s bus-poll deadline live in `mod long` (both nested in `mod tests`, see `lib.rs`). */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["compose"], this.root, rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("setup", SetupScript).register("wasm", WasmScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
