#!/usr/bin/env bun
/** 🔌 MCP inspector (generic). */
import { spawn } from "node:child_process";

const child = spawn("npx", ["--yes", "@modelcontextprotocol/inspector"], {
  stdio: "inherit",
  shell: true,
  cwd: import.meta.dir,
});
child.on("exit", (c) => process.exit(c ?? 0));
