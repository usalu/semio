#!/usr/bin/env bun
/** 🧪 Runs tests for every Nx project except `workspace`. */
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run-many", "-t", "test", "--all", "--exclude", "workspace"], {
  cwd: import.meta.dir,
  stdio: "inherit",
});
