#!/usr/bin/env bun
/** 🧹 Lints all `@repo/*` projects from the repo technology root. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..");
execFileSync("bun", ["nx", "run-many", "-t", "lint", "-p", "@repo/*"], {
  cwd: root,
  stdio: "inherit",
});
