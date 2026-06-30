#!/usr/bin/env bun
/** 🧭 `@semio-tech/draw-play` task router: `bun ./script.ts <dev|build|test> [fixture <id>] [args…]`. */
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
} from "../../repo/lib/js/src/index.ts";
import { resolveDrawPlayFixtureSlug } from "./fixture-slugs.ts";

class DevScript extends BundleScript {
	run(segments: string[]): void {
		const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolveDrawPlayFixtureSlug);
		const env = playPollingEnv(fixtureEnv);
		Object.assign(process.env, fixtureEnv);
		runViteBunxDev(this.root, viteSegments, {
			portEnv: playgroundPortEnv("draw"),
			defaultPort: playgroundDevPortString("draw"),
			fixedPort: true,
		});
	}
}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		const { segments: viteSegments, fixtureEnv } = consumePlaygroundFixtureArgv(segments, resolveDrawPlayFixtureSlug);
		const env = playPollingEnv(fixtureEnv);
		Object.assign(process.env, fixtureEnv);
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...viteSegments], this.root, env);
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments);
	}
}

const router = new ScriptRouter(import.meta.dir)
	.register("dev", DevScript)
	.register("build", BuildScript)
	.register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
