#!/usr/bin/env bun
/** 🧪 Typecheck-focused builds (@semio/js, @semio/react, GraphQL codegen) then every Nx `test` except `workspace`. */
import { execFileSync } from "node:child_process";

const root = import.meta.dir;
execFileSync("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/js", "@semio/react"], {
  cwd: root,
  stdio: "inherit",
});
execFileSync("bun", ["nx", "run", "semio/graphql:build"], { cwd: root, stdio: "inherit" });
execFileSync("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], {
  cwd: root,
  stdio: "inherit",
});
