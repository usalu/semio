#!/usr/bin/env bun
/** 🏗️ Builds Semio play + docs sites via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..");
execFileSync("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/play", "@semio/docs"], {
  cwd: root,
  stdio: "inherit",
});
