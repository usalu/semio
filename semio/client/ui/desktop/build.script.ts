#!/usr/bin/env bun
/** 🏗️ Invokes Nx build for Semio desktop. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/desktop:build"], { cwd: root, stdio: "inherit" });
