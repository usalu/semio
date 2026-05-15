#!/usr/bin/env bun
/**
 * 💻 Root `dev` entry: default `@semio/desktop` via Nx; `mcp`, `mcp repo`, `mcp engine` delegate to bundle-local MCP scripts.
 */
import { execFileSync } from "node:child_process";
import { join } from "node:path";

const root = import.meta.dir;
const argv = process.argv.slice(2);

if (argv[0] === "mcp") {
  const mode = argv[1];
  if (mode === "engine") {
    execFileSync("bun", [join(root, "semio", "client", "bin", "engine", "dev.mcp.engine.script.ts")], {
      cwd: root,
      stdio: "inherit",
    });
  } else if (mode === "repo") {
    execFileSync("bun", [join(root, "repo", "client", "dev.mcp.script.ts"), "repo"], { cwd: root, stdio: "inherit" });
  } else {
    execFileSync("bun", [join(root, "repo", "client", "dev.mcp.script.ts")], { cwd: root, stdio: "inherit" });
  }
  process.exit(0);
}

execFileSync("bun", ["nx", "run", "@semio/desktop:dev"], { cwd: root, stdio: "inherit" });
