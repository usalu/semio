#!/usr/bin/env bun
/** 🧹 Nx `lint` for every `@repo/*` project only (invoked from `lint.script.ts` or directly). */
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run-many", "-t", "lint", "-p", "@repo/*"], {
  cwd: import.meta.dir,
  stdio: "inherit",
});
