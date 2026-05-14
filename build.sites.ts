#!/usr/bin/env bun
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run-many", "-t", "build", "-p", "@semio/play", "@semio/docs"], {
  cwd: import.meta.dir,
  stdio: "inherit",
});
