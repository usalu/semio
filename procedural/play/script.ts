#!/usr/bin/env bun
/** 🧭 `@procedural/play` task router. */
import { join } from "node:path";
import {
	BundleScript,
	ScriptRouter,
	playPollingEnv,
	runBun,
	runBundleScriptMain,
	runViteBunxDev,
	runVitest,
} from "../../repo/lib/js/src/index.ts";
import { playgroundDevPortString, playgroundPortEnv } from "../../ui/styling/playground-dev-ports.ts";

const wasmScript = join(import.meta.dir, "../../flow/core/script.ts");
const moduleWasmScripts = ["core", "math", "text", "logic", "dictionary", "list", "brep", "bim"].map((name) =>
	join(import.meta.dir, `../../flow/module/${name}/script.ts`),
);

function runFlowModuleWasmBuilds(root: string): void {
	for (const script of moduleWasmScripts) {
		runBun([script, "wasm"], root, playPollingEnv());
	}
}

class DevScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root, playPollingEnv());
		runFlowModuleWasmBuilds(this.root);
		runViteBunxDev(this.root, segments, {
			portEnv: playgroundPortEnv("procedural"),
			defaultPort: playgroundDevPortString("procedural"),
			fixedPort: true,
		});
	}
}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root, playPollingEnv());
		runFlowModuleWasmBuilds(this.root);
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runBun([wasmScript, "wasm"], this.root, playPollingEnv());
		runFlowModuleWasmBuilds(this.root);
		runVitest(this.root, segments);
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("dev", DevScript)
	.register("build", BuildScript)
	.register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
