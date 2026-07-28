#!/usr/bin/env bun
/** 📊 `@semio-tech/compose-gql` router: `bun script.ts <build|dev|test>`. `compose-gql/rs` is a
 * regular root-workspace member (not a standalone Cargo workspace — that was a stale claim). */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargoTestBudgeted, resolveTestLevel } from "../../../../repo/lib/js/index.ts";
import { join } from "node:path";

const crate = "compose-gql";

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

/** ⏱️Warm-cache unit tests under the active level's wall-clock budget (see `resolveTestLevel`); build is un-timed. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted([crate], join(this.root, "rs"), rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript).register("dev", DevScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
