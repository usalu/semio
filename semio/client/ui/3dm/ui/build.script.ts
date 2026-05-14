#!/usr/bin/env bun
/** 🏗️ Invokes Nx build for this 3dm UI bundle. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/3dm-ui:build"], { cwd: root, stdio: "inherit" });
