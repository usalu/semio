#!/usr/bin/env bun
/** 📦 Publishes the play site via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/play:publish"], { cwd: root, stdio: "inherit" });
