#!/usr/bin/env bun
/** 📄 `@semio-tech/layout-react` router: `bun ./script.ts test`. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, playPollingEnv, runBun, runBundleScriptMain, runVitest } from "../../repo/lib/js/index.ts";

const wasmScript = join(import.meta.dir, "../rs/script.ts");

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root, playPollingEnv());
		runVitest(this.root, segments, "js/vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "test" });
