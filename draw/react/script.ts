#!/usr/bin/env bun
/** 🧭 `@semio-tech/draw-react` task router: `bun ./script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBun, runVitest } from "../../repo/lib/js/index.ts";

const flowCoreWasmScript = join(import.meta.dir, "../../flow/core/script.ts");

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runBun([flowCoreWasmScript, "wasm"], this.root);
		runVitest(this.root, segments, "js/vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
