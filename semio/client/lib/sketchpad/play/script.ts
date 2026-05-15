#!/usr/bin/env bun
/**
 * 🧭 Sketchpad play router: `bun ./script.ts dev [vite args…]`.
 */
import { spawn } from "node:child_process";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);

if (segs[0] !== "dev") {
  console.error("usage: bun ./script.ts dev [vite args…]");
  process.exit(1);
}

const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const extra = segs.slice(1);

const env = {
  ...process.env,
  ...(process.env.WATCHPACK_POLLING !== undefined
    ? {}
    : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};

const child = spawn("bunx", ["vite", "--host", host, ...extra], {
  stdio: "inherit",
  shell: true,
  env,
  cwd,
});
child.on("exit", (c) => process.exit(c ?? 0));
