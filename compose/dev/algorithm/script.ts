#!/usr/bin/env bun
/** 🧭 Algorithms bundle router: `bun ./script.ts dev [args…]` / `bun ./script.ts test [args…]`. */
import { BundleScript, ScriptRouter, devToolingEnv, resolveTestLevel, runBundleScriptMain, runCmd, runVitest } from "../../../repo/lib/js/index.ts";

/** This bundle has no local `.storybook` config — its stories live in the root Storybook's `compose/algorithm` scope, so `dev` delegates there instead of running a broken standalone instance. */
class DevScript extends BundleScript {
  run(segments: string[]): void {
    const env = devToolingEnv({
      WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
      CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
    });
    runCmd("bun", ["./script.ts", "dev", "storybook", "compose/algorithm", ...segments], { cwd: this.repoRoot, env });
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("bun", ["./script.ts", "build", "storybook", ...segments], { cwd: this.repoRoot, env: devToolingEnv({ STORYBOOK_SCOPE: "compose/algorithm" }) });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runVitest(this.root, rest, "js/🧪vitest.config.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("dev", DevScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
