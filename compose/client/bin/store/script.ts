#!/usr/bin/env bun
/** 🏪 `@semio-tech/compose-store` router: `bun script.ts <build|dev|test>`. `compose-store/rs` is a standalone Cargo workspace, kept out of the repo-root workspace. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted } from "../../../../repo/lib/js/index.ts";
import { join } from "node:path";

const crate = "compose-store";

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["build", "--release", "-p", crate, ...segments], { stdio: "inherit", cwd: join(this.root, "rs") });
  }
}

class DevScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["run", "-p", crate, ...segments], { stdio: "inherit", cwd: join(this.root, "rs") });
  }
}

/** ⏱️Warm-cache unit tests under the 30s wall-clock budget; build is un-timed. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted([crate], join(this.root, "rs"), segments);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("dev", DevScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
