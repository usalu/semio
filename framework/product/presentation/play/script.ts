#!/usr/bin/env bun
/** 🧭 `@framework/presentation/play` task router: `bun ./script.ts <dev|build|test>`. */
import {
	BundleScript,
	ScriptRouter,
	playPollingEnv,
	runBun,
	runBundleScriptMain,
	runVitest,
	runViteBunxDev,
} from "../../../../repo/lib/js/src/index.ts";

class DevScript extends BundleScript {
	run(segments: string[]): void {
		runViteBunxDev(this.root, segments, { portEnv: "PRESENTATION_PLAY_PORT", defaultPort: "6051" });
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
