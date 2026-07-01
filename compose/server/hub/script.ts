#!/usr/bin/env bun
/** 🧭 `compose-hub` router: `bun ./script.ts <setup|build|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, runCmd } from "../../../repo/lib/js/index.ts";

class SetupScript extends BundleScript {
	run(): void {
		runCargo(["fetch", "--manifest-path", "Cargo.toml"], this.root);
	}
}

class BuildScript extends BundleScript {
	run(): void {
		runCargo(["build", "--release", "--manifest-path", "Cargo.toml"], this.root);
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runCargo(["test", "--manifest-path", "Cargo.toml", ...segments], this.root);
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("setup", SetupScript)
	.register("build", BuildScript)
	.register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
