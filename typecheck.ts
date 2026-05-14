#!/usr/bin/env bun
/** 🔎 Typecheck-focused build for core JS bundles plus GraphQL codegen. */
import { execFileSync } from "node:child_process";

const root = import.meta.dir;
execFileSync("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/js", "@semio/react"], {
  cwd: root,
  stdio: "inherit",
});
execFileSync("bun", ["nx", "run", "semio/graphql:build"], { cwd: root, stdio: "inherit" });
