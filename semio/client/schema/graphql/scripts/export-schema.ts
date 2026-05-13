#!/usr/bin/env bun
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const cwd = join(import.meta.dir, "..");
const out = join(cwd, "schema.graphql");
execFileSync(
  "cargo",
  [
    "test",
    "--manifest-path",
    join(cwd, "..", "rs", "Cargo.toml"),
    "export_semio_graphql_schema_file",
    "--",
    "--ignored",
    "--nocapture",
  ],
  {
    cwd,
    stdio: "inherit",
    env: { ...process.env, SEMIO_GRAPHQL_SCHEMA_OUT: out },
  },
);
