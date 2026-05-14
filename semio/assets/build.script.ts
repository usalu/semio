#!/usr/bin/env bun
/** 🏗️ Invokes Nx build for `@semio/assets`. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..");
execFileSync("bun", ["nx", "run", "@semio/assets:build"], { cwd: root, stdio: "inherit" });
