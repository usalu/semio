#!/usr/bin/env bun
/** 🏗️ Invokes Nx build for Coda desktop. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@coda/desktop:build"], { cwd: root, stdio: "inherit" });
