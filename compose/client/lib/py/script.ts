#!/usr/bin/env bun
/** 🧭 `@semio-tech/compose-py` router: `bun ./script.ts <build|test>`. */
import { BundleScript, ScriptRouter, runBundleScriptMain, runCmd } from "../../../../repo/lib/js/index.ts";

class BuildScript extends BundleScript {
  run(): void {
    runCmd("uv", ["build"], { cwd: this.root });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCmd("uv", ["run", "pytest", ...segments], { cwd: this.root });
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
