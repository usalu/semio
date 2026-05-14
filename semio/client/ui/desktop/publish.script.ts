#!/usr/bin/env bun
/** 📦 Publishes Semio desktop via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/desktop:publish"], { cwd: root, stdio: "inherit" });
