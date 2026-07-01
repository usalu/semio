#!/usr/bin/env bun
/** 🧭 `@semio-tech/lowpoly-play` task router. */
import { join } from "node:path";
import {
	BundleScript,
	ScriptRouter,
	playPollingEnv,
	playgroundDevPortString,
	playgroundPortEnv,
	runBun,
	runBundleScriptMain,
	runVitest,
	runViteBunxDev,
} from "../../repo/lib/js/index.ts";

const wasmScript = join(import.meta.dir, "../core/script.ts");

class DevScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root, playPollingEnv());
		runViteBunxDev(this.root, segments, {
			portEnv: playgroundPortEnv("lowpoly"),
			defaultPort: playgroundDevPortString("lowpoly"),
			fixedPort: true,
		});
	}
}

class ValidateScript extends DevScript {}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root, playPollingEnv());
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root, playPollingEnv());
		runVitest(this.root, segments);
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("dev", DevScript)
	.register("validate", ValidateScript)
	.register("build", BuildScript)
	.register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
