#!/usr/bin/env bun
/**
 * 🖥️ Shared Vite dev server launcher (host + optional polling).
 * Usage: bun run.vite.ts --port 4321 [--strictPort] [-- extra vite args...]
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
