#!/usr/bin/env bun
/** 🧪 Starts `@semio/algorithms` Storybook via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/algorithms:dev"], { cwd: root, stdio: "inherit" });
