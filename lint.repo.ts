#!/usr/bin/env bun
/** 🧹 Lints all `@repo/*` projects. */
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run-many", "-t", "lint", "-p", "@repo/*"], {
  cwd: import.meta.dir,
  stdio: "inherit",
});
