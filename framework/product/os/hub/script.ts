#!/usr/bin/env bun
/** @emoji 🧭 `os-hub` router: `bun ./script.ts <setup|build|test|dev>`. */
import { BundleScript, ScriptRouter, OS_HUB_PORT, OS_HUB_PORT_ENV, runBundleScriptMain, runCargo, runCmd } from "../../../../repo/lib/js/index.ts";
import { join } from "node:path";

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

class DevScript extends BundleScript {
	run(): void {
		runCargo(["run", "--manifest-path", "Cargo.toml"], this.root, {
			[OS_HUB_PORT_ENV]: String(OS_HUB_PORT),
		});
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("setup", SetupScript)
	.register("build", BuildScript)
	.register("test", TestScript)
	.register("dev", DevScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
