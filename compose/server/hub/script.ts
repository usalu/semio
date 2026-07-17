#!/usr/bin/env bun
/** 🧭 `compose-hub` router: `bun ./script.ts <setup|build|test|test-e2e>`. `compose-hub/rs` is a standalone Cargo workspace, kept out of the repo-root workspace. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, runCargoTestBudgeted } from "../../../repo/lib/js/index.ts";

class SetupScript extends BundleScript {
  run(): void {
    runCargo(["fetch"], join(this.root, "rs"));
  }
}

class BuildScript extends BundleScript {
  run(): void {
    runCargo(["build", "--release"], join(this.root, "rs"));
  }
}

/** ⏱️Warm-cache unit tests only — Postgres testcontainer e2e tests are `#[ignore]`d, see `test-e2e`. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    runCargoTestBudgeted(["compose-hub"], join(this.root, "rs"), segments);
  }
}

/** 🐘Postgres testcontainer suite; needs Docker, excluded from the default ≤30s `test` budget. */
class TestE2eScript extends BundleScript {
  run(segments: string[]): void {
    runCargo(["test", "-p", "compose-hub", "--", "--ignored", ...segments], join(this.root, "rs"));
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("setup", SetupScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("test-e2e", TestE2eScript);

await runBundleScriptMain(router, import.meta.url);
