#!/usr/bin/env bun
/** 🏗️ Builds every Nx project except the synthetic `workspace` root. */
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run-many", "-t", "build", "--all", "--exclude", "workspace"], {
  cwd: import.meta.dir,
  stdio: "inherit",
});
