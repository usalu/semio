#!/usr/bin/env bun
/** 🧭 `@semio-tech/sequence-react` task router: `bun ./script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runBun, runVitest } from "../../repo/lib/js/index.ts";

const wasmScript = join(import.meta.dir, "../core/script.ts");

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root);
		runVitest(this.root, segments, "vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
