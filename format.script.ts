#!/usr/bin/env bun
/** ✨ Formats the repository with Prettier. */
import { execFileSync } from "node:child_process";

execFileSync("bunx", ["prettier", "-w", "."], { cwd: import.meta.dir, stdio: "inherit", shell: true });
