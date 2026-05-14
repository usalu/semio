#!/usr/bin/env bun
/** 🏗️ Builds every Nx project except the synthetic `workspace` root. */
import { execFileSync } from "node:child_process";

const root = import.meta.dir;
execFileSync("bun", ["./generate.script.ts"], { cwd: root, stdio: "inherit" });

execFileSync("bun", ["nx", "run-many", "-t", "build", "--all", "--exclude", "workspace"], {
  cwd: root,
  stdio: "inherit",
});
