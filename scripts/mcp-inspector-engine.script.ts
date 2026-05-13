#!/usr/bin/env bun
import { spawn } from "node:child_process";
import { resolve } from "node:path";

const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const engineDir = resolve(import.meta.dir, "..", "semio", "engine");

const child = spawn(
  "npx",
  [
    "--yes",
    "@mcpjam/inspector@latest",
    "uv",
    "--directory",
    engineDir,
    "run",
    "main.py",
    "--mcp-stdio",
  ],
  {
    stdio: "inherit",
    shell: true,
    env: { ...process.env, HOST: host },
  },
);
child.on("exit", (c) => process.exit(c ?? 0));
