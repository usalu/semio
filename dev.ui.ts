#!/usr/bin/env bun
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run", "@semio/ui:dev"], { cwd: import.meta.dir, stdio: "inherit" });
