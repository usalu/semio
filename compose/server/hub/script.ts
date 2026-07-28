#!/usr/bin/env bun
/** 🧭 `compose-hub` router: `bun ./script.ts <setup|build|test [level]>`. `compose-hub/rs` is a standalone Cargo workspace, kept out of the repo-root workspace. */
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain, runCargo, runCargoTestBudgeted, resolveTestLevel } from "../../../repo/lib/js/index.ts";

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

/** ⏱️Level-budgeted; the full db-backed integration suite lives in `mod exhaustive` (see bin.rs) — no external services required (`db::Database` is a zero-touch `FsStorage` in a tempdir) — only run at the `exhaustive` level. */
class TestScript extends BundleScript {
  run(segments: string[]): void {
    const { rest } = resolveTestLevel(segments);
    runCargoTestBudgeted(["compose-hub"], join(this.root, "rs"), rest);
  }
}

const router = new ScriptRouter(import.meta.dir).register("setup", SetupScript).register("build", BuildScript).register("test", TestScript);

await runBundleScriptMain(router, import.meta.url);
