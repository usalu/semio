#!/usr/bin/env bun
/** 📦 Publishes Coda desktop via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@coda/desktop:publish"], { cwd: root, stdio: "inherit" });
