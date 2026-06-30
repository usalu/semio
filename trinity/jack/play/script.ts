#!/usr/bin/env bun
/** 🧭 `@semio-tech/trinity-jack-play` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, playgroundDevPortString, playgroundPortEnv, runBun, runBundleScriptMain, runCargo, runViteBunxDev, runVitest } from "../../../repo/lib/js/src/index.ts";

const wasmScript = join(import.meta.dir, "../../rewrite/engine/script.ts");
const jackLspWasmScript = join(import.meta.dir, "../lsp/script.ts");

function buildJackPlayWasm(): void {
  runBun([wasmScript, "wasm"], join(import.meta.dir, "../../rewrite/engine"), playPollingEnv());
  runBun([jackLspWasmScript, "wasm"], join(import.meta.dir, "../lsp"), playPollingEnv());
}

class DevScript extends BundleScript {
  run(segments: string[]): void {
    buildJackPlayWasm();
    runViteBunxDev(this.root, segments, {
      portEnv: playgroundPortEnv("trinity-jack"),
      defaultPort: playgroundDevPortString("trinity-jack"),
      fixedPort: true,
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    buildJackPlayWasm();
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "trinity_rewrite", "-p", "trinity_jack"], this.repoRoot, playPollingEnv());
    buildJackPlayWasm();
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
