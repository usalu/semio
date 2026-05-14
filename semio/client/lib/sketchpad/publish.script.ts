#!/usr/bin/env bun
/** 📦 Publishes sketchpad via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/sketchpad:publish"], { cwd: root, stdio: "inherit" });
