#!/usr/bin/env bun
/** @emoji ⚙️ Delegates styling generation and Python import smoke test. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../repo/lib/js/index.ts";
import { generateStylingArtifacts } from "../script.ts";

class GenerateScript extends BundleScript {
	run(): void {
		generateStylingArtifacts();
	}
}

class TestScript extends BundleScript {
	run(): void {
		runCmd(
			"uv",
			["run", "python", "-c", "from styling.generated import BOARD_LIGHT, STYLING_TOKENS; assert STYLING_TOKENS['primary']"],
			{ cwd: import.meta.dir },
		);
	}
}

if (import.meta.main) {
	const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("test", TestScript);
	await runBundleScriptMain(router, import.meta.url);
}
