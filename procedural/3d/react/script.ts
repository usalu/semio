#!/usr/bin/env bun
/** 🧭 `@semio-tech/procedural-3d-react` task router. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runVitest } from "../../../repo/lib/js/index.ts";

const flowWasmScript = join(import.meta.dir, "../../../flow/core/script.ts");
const moduleWasmScripts = ["math", "text", "logic", "dictionary", "list", "brep", "bim", "draw"].map((name) =>
	join(import.meta.dir, `../../../flow/module/${name}/script.ts`),
);

function runFlowModuleWasmBuilds(): void {
	for (const script of moduleWasmScripts) {
		runBun([script, "wasm"], import.meta.dir, playPollingEnv());
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runBun([flowWasmScript, "wasm"], this.root, playPollingEnv());
		runFlowModuleWasmBuilds();
		runVitest(this.root, segments, "js/vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
