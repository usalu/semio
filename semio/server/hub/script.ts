#!/usr/bin/env bun
/** 🌐 `semio-hub` router: `bun ./script.ts <dev|build|test|setup> [args…]` (Hub Protocol v1 server; `DATABASE_URL`, `LISTEN_ADDR` and `RUST_LOG` default in `bin.rs` region `🚀Main`). */
import { execFileSync } from "node:child_process";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../repo/lib/js/src/index.ts";

/** 🦀 Runs cargo from the bundle root with the caller's environment. */
function cargo(root: string, args: string[]): void {
  execFileSync("cargo", args, { stdio: "inherit", cwd: root });
}

class DevScript extends BundleScript {
  run(segments: string[]): void {
    cargo(this.root, ["run", "-p", "semio-hub", ...segments]);
  }
}

class BuildScript extends BundleScript {
  run(segments: string[]): void {
    cargo(this.root, ["build", "--release", "-p", "semio-hub", ...segments]);
  }
}

class TestScript extends BundleScript {
  run(segments: string[]): void {
    cargo(this.root, ["test", "-p", "semio-hub", ...segments]);
  }
}

class SetupScript extends BundleScript {
  run(): void {
    cargo(this.root, ["fetch", "--manifest-path", "Cargo.toml"]);
  }
}

const router = new ScriptRouter(import.meta.dir)
  .register("dev", DevScript)
  .register("build", BuildScript)
  .register("test", TestScript)
  .register("setup", SetupScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "dev" });
