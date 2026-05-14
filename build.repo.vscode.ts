#!/usr/bin/env bun
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run", "repo:build-vsix"], { cwd: import.meta.dir, stdio: "inherit" });
