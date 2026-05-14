#!/usr/bin/env bun
/** 💻 Starts Semio desktop via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/desktop:dev"], { cwd: root, stdio: "inherit" });
