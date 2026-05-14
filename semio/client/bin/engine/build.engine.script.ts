#!/usr/bin/env bun
/** 🏗️ Invokes Nx build for the Semio engine bundle. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/engine:build"], { cwd: root, stdio: "inherit" });
