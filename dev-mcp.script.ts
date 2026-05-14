#!/usr/bin/env bun
/**
 * 🔌 Root dispatcher for MCP inspector flows: default repo inspector, `repo` (`.cursor/mcp.json`), or `engine` (Semio engine MCPJam).
 */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const mode = process.argv[2] ?? "default";

if (mode === "engine") {
  execFileSync("bun", [join(root, "semio", "client", "bin", "engine", "dev.script.ts")], {
    cwd: root,
    stdio: "inherit",
  });
  process.exit(0);
}

const repoDev = join(root, "repo", "client", "dev.script.ts");
if (mode === "repo") {
  execFileSync("bun", [repoDev, "repo"], { cwd: root, stdio: "inherit" });
} else {
  execFileSync("bun", [repoDev], { cwd: root, stdio: "inherit" });
}
