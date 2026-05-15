#!/usr/bin/env bun
/** 🔌 Build engine MCP bundle then launch MCPJam inspector (`dev` + `mcp` + `engine`). */
import { execFileSync } from "node:child_process";
import { spawn } from "node:child_process";
import { join } from "node:path";

const engineDir = import.meta.dir;
const root = join(engineDir, "..", "..", "..", "..");

const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";

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
