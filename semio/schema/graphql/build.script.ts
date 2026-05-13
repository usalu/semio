#!/usr/bin/env bun
import { execFileSync } from "node:child_process";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const cwd = dirname(fileURLToPath(import.meta.url));
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
