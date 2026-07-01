#!/usr/bin/env bun
/** 🧭 `@semio-tech/vcs-play` task router: `bun ./script.ts <dev|build|test>`. */
import {
	BundleScript,
	ScriptRouter,
	playgroundDevPortString,
	playgroundPortEnv,
	runBundleScriptMain,
	runViteBunxDev,
	runViteBuild,
	runVitest,
} from "../../repo/lib/js/index.ts";

class DevScript extends BundleScript {
	run(segments: string[]): void {
		runViteBunxDev(this.root, segments, {
			portEnv: playgroundPortEnv("vcs"),
			defaultPort: playgroundDevPortString("vcs"),
			fixedPort: true,
		});
	}
}

class BuildScript extends BundleScript {
	run(segments: string[]): void {
		runViteBuild(this.root, segments, "vite.config.ts");
	}
}

class TestScript extends BundleScript {
	run(segments: string[]): void {
		runVitest(this.root, segments, "vitest.config.ts");
	}
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
