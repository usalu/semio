#!/usr/bin/env bun
/** 📚 Starts Elements UI Storybook via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..");
execFileSync("bun", ["nx", "run", "@elements/ui:dev"], { cwd: root, stdio: "inherit" });
