#!/usr/bin/env bun
/** 🧭 `@semio-tech/flow-play` task router. */
import { join } from "node:path";
import {
  BundleScript,
  ScriptRouter,
  playPollingEnv,
  runBun,
  runBundleScriptMain,
  runCargo,
  runViteBunxDev,
  runVitest,
} from "../../repo/lib/js/src/index.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../ui/styling/playground-dev-ports.ts";

const wasmScript = join(import.meta.dir, "../core/script.ts");
const moduleWasmScripts = ["core", "math", "text", "logic", "dictionary", "list", "brep", "bim"].map((name) => join(import.meta.dir, `../module/${name}/script.ts`));

function runFlowModuleWasmBuilds(root: string): void {
  for (const script of moduleWasmScripts) {
    runBun([script, "wasm"], root, playPollingEnv());
  }
}
const validateRuntimeScript = join(import.meta.dir, "../../.repo/🎫/26/06/07/FLOW-RUNTIME-LOADABLE-MODULES/validate-flow-runtime.mjs");

class DevScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runFlowModuleWasmBuilds(this.root);
    runViteBunxDev(this.root, segments, {
      portEnv: playgroundPortEnv("flow"),
      defaultPort: playgroundDevPortString("flow"),
      fixedPort: true,
    });
  }
}

class ValidateScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runFlowModuleWasmBuilds(this.root);
    runBun([validateRuntimeScript, ...segments], this.root, {
      ...playPollingEnv(),
      FLOW_PLAY_PORT: process.env.FLOW_PLAY_PORT ?? playgroundDevPortString("flow"),
    });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runFlowModuleWasmBuilds(this.root);
    runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(
      ["test", "-p", "flow_module_wasm", "-p", "flow_module_core", "-p", "flow_module_math", "-p", "flow_module_text", "-p", "flow_module_logic", "-p", "flow_module_dictionary", "-p", "flow_module_list", "-p", "flow_module_bim", "-p", "flow_core", "-p", "neural_engine", "--", "--test-threads=1"],
      this.repoRoot,
      playPollingEnv(),
    );
    runBun([wasmScript, "wasm"], this.root, playPollingEnv());
    runFlowModuleWasmBuilds(this.root);
    runVitest(this.root, segments);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("validate", ValidateScript)
  .register("build", BuildScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
