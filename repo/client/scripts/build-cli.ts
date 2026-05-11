#!/usr/bin/env bun
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..");
const ext = process.platform === "win32" ? ".exe" : "";
execFileSync("go", ["build", "-o", `client${ext}`, "../mcp"], {
  cwd: root,
  stdio: "inherit",
});
