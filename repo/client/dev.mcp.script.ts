#!/usr/bin/env bun
/**
 * 🔌 Dev-time MCP inspector: default generic inspector; `repo` uses `.cursor/mcp.json` server `repo`.
 */
import { spawn } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..");
const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const mode = process.argv[2] ?? "default";

const child =
  mode === "repo"
    ? spawn(
        "npx",
        ["--yes", "@modelcontextprotocol/inspector", "--config", ".cursor/mcp.json", "--server", "repo"],
        {
          stdio: "inherit",
          shell: true,
          cwd: root,
          env: { ...process.env, HOST: host },
        },
      )
    : spawn("npx", ["--yes", "@modelcontextprotocol/inspector"], {
        stdio: "inherit",
        shell: true,
        cwd: root,
      });
child.on("exit", (c) => process.exit(c ?? 0));
