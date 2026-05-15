#!/usr/bin/env bun
/** ✨ Prettier write for the repository root. */
import { execFileSync } from "node:child_process";

execFileSync("bunx", ["prettier", "-w", "."], { cwd: import.meta.dir, stdio: "inherit", shell: true });
