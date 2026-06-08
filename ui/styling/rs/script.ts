#!/usr/bin/env bun
/** @emoji ⚙️ Runs `cargo test` for the `ui_styling` crate. */
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/src/index.ts";

class TestScript extends BundleScript {
	run(): void {
		Bun.spawnSync(["cargo", "test", "-p", "ui_styling"], { cwd: import.meta.dir, stdio: "inherit" });
	}
}

if (import.meta.main) {
	const router = new ScriptRouter(import.meta.dir).register("test", TestScript);
	await runBundleScriptMain(router, import.meta.url);
}
