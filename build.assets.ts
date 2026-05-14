#!/usr/bin/env bun
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run", "@semio/assets:build"], { cwd: import.meta.dir, stdio: "inherit" });
