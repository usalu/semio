#!/usr/bin/env bun
/** 🏪 `@semio-tech/compose-store` router: `bun script.ts <build|dev|test>`. */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/index.ts";
import { join } from "node:path";

const crate = "compose-store";

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["build", "--release", "-p", crate, ...segments], { stdio: "inherit", cwd: this.repoRoot });
  }
}

class DevScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["run", "-p", crate, ...segments], { stdio: "inherit", cwd: this.repoRoot });
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    execFileSync("cargo", ["test", "-p", crate, ...segments], { stdio: "inherit", cwd: this.repoRoot });
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("build", BuildScript)
  .register("dev", DevScript)
  .register("test", TestScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
