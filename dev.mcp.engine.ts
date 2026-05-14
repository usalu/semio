#!/usr/bin/env bun
/** 🔌 Build engine MCP bundle then launch MCPJam inspector against `semio/client/bin/engine`. */
import { execFileSync } from "node:child_process";
import { spawn } from "node:child_process";
import { resolve } from "node:path";

const root = import.meta.dir;
const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const engineDir = resolve(root, "semio", "client", "bin", "engine");

execFileSync(
  "bunx",
  ["vite", "build", "--config", "semio/client/bin/engine/vite.mcp-app.config.ts"],
  { cwd: root, stdio: "inherit", shell: true },
);

const child = spawn(
  "npx",
  ["--yes", "@mcpjam/inspector@latest", "uv", "--directory", engineDir, "run", "main.py", "--mcp-stdio"],
  {
    stdio: "inherit",
    shell: true,
    env: { ...process.env, HOST: host },
    cwd: root,
  },
);
child.on("exit", (c) => process.exit(c ?? 0));
