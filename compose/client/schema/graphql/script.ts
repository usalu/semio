#!/usr/bin/env bun
/** 🧭 GraphQL schema bundle router: `bun ./script.ts build`. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { BundleScript, ScriptRouter, runBundleScriptMain } from "../../../../repo/lib/js/index.ts";

class BuildScript extends BundleScript {
  run(): void {
    const out = join(this.root, "schema.graphql");
    const cargoTargetDir = join(this.root, "target");
    execFileSync(
      "cargo",
      [
        "test",
        "--manifest-path",
        join(this.root, "..", "..", "lib", "rs", "Cargo.toml"),
        "export_compose_graphql_schema_file",
        "--",
        "--ignored",
        "--nocapture",
      ],
      {
        cwd: this.root,
        stdio: "inherit",
        env: {
          ...process.env,
          CARGO_BUILD_JOBS: "1",
          CARGO_TARGET_DIR: cargoTargetDir,
          COMPOSE_GRAPHQL_SCHEMA_OUT: out,
        },
      },
    );
  }
}

const router = new ScriptRouter(import.meta.dir).register("build", BuildScript);

await runBundleScriptMain(router, import.meta.url, { defaultCommand: "build" });
