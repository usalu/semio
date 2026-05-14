#!/usr/bin/env bun
/** 🧹 Runs `@repo/lib` lint via Nx from the monorepo root. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
execFileSync("bun", ["nx", "run", "@repo/lib:lint"], { cwd: root, stdio: "inherit" });
