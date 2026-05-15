#!/usr/bin/env bun
/**
 * 🧭 GraphQL schema bundle router: `bun ./script.ts build` exports `schema.graphql` from Rust tests.
 */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);

if (segs[0] === "build") {
  const out = join(cwd, "schema.graphql");
  const cargoTargetDir = join(cwd, "target");
  execFileSync(
    "cargo",
    [
      "test",
      "--manifest-path",
      join(cwd, "..", "..", "client", "lib", "rs", "Cargo.toml"),
      "export_semio_graphql_schema_file",
      "--",
      "--ignored",
      "--nocapture",
    ],
    {
      cwd,
      stdio: "inherit",
      env: {
        ...process.env,
        CARGO_BUILD_JOBS: "1",
        CARGO_TARGET_DIR: cargoTargetDir,
        SEMIO_GRAPHQL_SCHEMA_OUT: out,
      },
    },
  );
} else {
  console.error("usage: bun ./script.ts build");
  process.exit(1);
}
