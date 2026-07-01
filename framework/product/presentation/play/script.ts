#!/usr/bin/env bun
/** 🧭 `@semio-tech/framework-presentation-play` task router: `bun ./script.ts <dev|build|test>`. */
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
	run(segments: string[]): void {
		runViteBunxDev(this.root, segments, {
			portEnv: playgroundPortEnv("presentation"),
			defaultPort: playgroundDevPortString("presentation"),
			fixedPort: true,
		});
	}
}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		runBun(["run", "vite", "build", "--config", "vite.config.ts", ...segments], this.root, playPollingEnv());
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
