#!/usr/bin/env bun
/** 🔌 MCP inspector (generic); cwd is monorepo root. */
import { spawn } from "node:child_process";
import { join } from "node:path";

const root = join(import.meta.dir, "..", "..");
const child = spawn("npx", ["--yes", "@modelcontextprotocol/inspector"], {
  stdio: "inherit",
  shell: true,
  cwd: root,
});
child.on("exit", (c) => process.exit(c ?? 0));
