#!/usr/bin/env bun
/** @emoji 🧭 `@semio-tech/framework-os-dev` task router. */
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
} from "../../../../repo/lib/js/index.ts";

class DevScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		runViteBunxDev(this.root, segments, {
			portEnv: playgroundPortEnv("s"),
			defaultPort: playgroundDevPortString("s"),
			fixedPort: true,
			env: {
				...playPollingEnv(),
				PUZZLE_PLAY_ENTRY: "s",
			},
			expectedPlayEntry: "s",
		});
	}
}

class BuildScript extends BundleScript {
	async run(segments: string[]): Promise<void> {
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, {
			...playPollingEnv(),
			PUZZLE_PLAY_ENTRY: "s",
		});
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments, "js/vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
