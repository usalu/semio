#!/usr/bin/env bun
/** 🧵 `@semio-tech/framework-sync-worker` router: `bun ./script.ts <wasm|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runWasmPackWebBuild } from "../../../repo/lib/js/index.ts";

class WasmScript extends BundleScript {
  run(): void {
    runWasmPackWebBuild({
      rsDir: `${this.root}/rs`,
      skipEnvVar: "FRAMEWORK_SYNC_WORKER_SKIP_WASM_BUILD",
      logPrefix: "framework/sync/worker",
      wasmBaseName: "semio_framework_sync_worker",
      pkg: {
        name: "@semio-tech/framework-sync-worker",
        files: ["semio_framework_sync_worker_bg.wasm", "semio_framework_sync_worker.js", "semio_framework_sync_worker.d.ts"],
        main: "semio_framework_sync_worker.js",
        module: "semio_framework_sync_worker.js",
        types: "semio_framework_sync_worker.d.ts",
      },
    });
  }
}

class TestScript extends BundleScript {
  run(): void {
    console.log("[DEBUG] framework-sync-worker has no native tests; wasm build is the verification target");
  }
}

const router = new ScriptRouter(import.meta.dir).register("wasm", WasmScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "wasm" });
