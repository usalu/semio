#!/usr/bin/env bun
import { execFileSync } from "node:child_process";

execFileSync("bun", ["nx", "run", "@elements/ui:dev"], { cwd: import.meta.dir, stdio: "inherit" });
