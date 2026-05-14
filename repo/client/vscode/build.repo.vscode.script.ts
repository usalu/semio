#!/usr/bin/env bun
/** 🏗️ Builds repo VSIX via Nx `repo:build-vsix`. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
execFileSync("bun", ["nx", "run", "repo:build-vsix"], { cwd: root, stdio: "inherit" });
