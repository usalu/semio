#!/usr/bin/env bun
/** 🏗️ Builds the repo coordinator server bundle. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
execFileSync("bun", ["nx", "run", "@repo/coordinator:build"], { cwd: root, stdio: "inherit" });
