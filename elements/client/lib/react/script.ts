#!/usr/bin/env bun
/** 🧭 Elements react UI router: `bun ./script.ts dev [storybook args…]`. */
import { spawn } from "node:child_process";
import { join } from "node:path";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);

if (segs[0] !== "dev") {
  console.error("usage: bun ./script.ts dev [storybook args…]");
  process.exit(1);
}

const repoRoot = join(cwd, "..", "..", "..", "..");
const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const port = process.env.STORYBOOK_PORT ?? "6006";
const extra = segs.slice(1);

const env = {
  ...process.env,
  WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
  CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
};

const child = spawn(
  "bunx",
  ["storybook", "dev", "-c", ".storybook", "-p", port, "--exact-port", "--host", host, "--no-open", "--debug", ...extra],
  { stdio: "inherit", shell: true, env, cwd: repoRoot },
);
child.on("exit", (c) => process.exit(c ?? 0));
