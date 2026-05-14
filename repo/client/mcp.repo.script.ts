#!/usr/bin/env bun
/** 🔌 MCP inspector wired to the `repo` server from `.cursor/mcp.json`. */
import { spawn } from "node:child_process";
import { join } from "node:path";

const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const root = join(import.meta.dir, "..", "..");

const child = spawn(
  "npx",
  ["--yes", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"],
  {
    stdio: "inherit",
    shell: true,
    cwd: root,
    env: { ...process.env, HOST: host },
  },
);
child.on("exit", (c) => process.exit(c ?? 0));
