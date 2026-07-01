#!/usr/bin/env bun
/** 🧭 `@semio-tech/forms-core` task router. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runVitest } from "../../repo/lib/js/index.ts";

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments);
	}
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
