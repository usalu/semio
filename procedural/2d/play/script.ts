#!/usr/bin/env bun
/** 🧭 `@semio-tech/procedural-2d-play` task router: `bun ./script.ts <dev|build|test> [fixture <id>] [args…]`. */
import { join } from "node:path";
import {
    BundleScript,
    ScriptRouter,
    consumePlaygroundFixtureArgv,
    playPollingEnv,
    playgroundDevPortString,
    playgroundPortEnv,
    runBun,
    runBundleScriptMain,
    runViteBunxDev,
    runVitest,
} from "../../../repo/lib/js/index.ts";
import { resolveProcedural2dPlayFixtureSlug } from "./fixture-slugs.js";

const wasmScript = join(import.meta.dir, "../../../flow/core/script.ts");
const moduleWasmScripts = ["core", "math", "text", "logic", "dictionary", "list", "brep", "bim", "draw"].map((name) =>
	join(import.meta.dir, `../../../flow/module/${name}/script.ts`),
);

function runFlowModuleWasmBuilds(root: string, env: NodeJS.ProcessEnv = playPollingEnv()): void {
	for (const script of moduleWasmScripts) {
		runBun([script, "wasm"], root, env);
	}
}

class DevScript extends BundleScript {
	run(segments: string[]): void {
		const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolveProcedural2dPlayFixtureSlug);
		const env = playPollingEnv(fixtureEnv);
		runBun([wasmScript, "wasm"], this.root, env);
		runFlowModuleWasmBuilds(this.root, env);
		Object.assign(process.env, fixtureEnv);
		runViteBunxDev(this.root, viteSegments, {
			portEnv: playgroundPortEnv("procedural-2d"),
			defaultPort: playgroundDevPortString("procedural-2d"),
			fixedPort: true,
		});
	}
}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolveProcedural2dPlayFixtureSlug);
		const env = playPollingEnv(fixtureEnv);
		runBun([wasmScript, "wasm"], this.root, env);
		runFlowModuleWasmBuilds(this.root, env);
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...viteSegments], this.root, env);
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
