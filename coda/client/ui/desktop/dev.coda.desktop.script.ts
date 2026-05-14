#!/usr/bin/env bun
/** 💻 Starts Coda desktop via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@coda/desktop:dev"], { cwd: root, stdio: "inherit" });
