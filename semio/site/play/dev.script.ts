#!/usr/bin/env bun
/**
 * 💻 Vite dev server for the play site bundle.
 * Usage: bun dev.script.ts --strictPort --port 4000 [-- extra vite args...]
 */
import { spawn } from "node:child_process";

const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const args = process.argv.slice(2);
const viteArgs = ["vite", "--host", host, ...args];

const env = {
  ...process.env,
  ...(process.env.WATCHPACK_POLLING !== undefined
    ? {}
    : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};

const child = spawn("bunx", viteArgs, {
  stdio: "inherit",
  shell: true,
  env,
});
child.on("exit", (c) => process.exit(c ?? 0));
