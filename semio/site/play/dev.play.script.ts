#!/usr/bin/env bun
/** 🌐 Starts the play site dev server via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/play:dev"], { cwd: root, stdio: "inherit" });
