#!/usr/bin/env bun
/** 💻 Starts `@semio/ui` Storybook via Nx. */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..", "..", "..");
execFileSync("bun", ["nx", "run", "@semio/ui:dev"], { cwd: root, stdio: "inherit" });
