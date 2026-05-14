#!/usr/bin/env bun
/**
 * 💻 Vite dev server; **cwd** must be the bundle root. Forwards argv after the script name to Vite
 * (e.g. `--strictPort --port 4000`).
 */
import { spawn } from "node:child_process";

const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const args = process.argv.slice(2);

const env = {
  ...process.env,
  ...(process.env.WATCHPACK_POLLING !== undefined
    ? {}
    : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};

const child = spawn("bunx", ["vite", "--host", host, ...args], {
  stdio: "inherit",
  shell: true,
  env,
  cwd: process.cwd(),
});
child.on("exit", (c) => process.exit(c ?? 0));
