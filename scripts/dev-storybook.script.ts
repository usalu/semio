#!/usr/bin/env bun
/**
 * 💻 Storybook dev server; **cwd** must be the bundle root (Nx `cwd` / run from that directory).
 * Forwards extra argv to Storybook after the fixed flags.
 */
import { spawn } from "node:child_process";

const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const port = process.env.STORYBOOK_PORT ?? "6006";
const extra = process.argv.slice(2);

const env = {
  ...process.env,
  WATCHPACK_POLLING: process.env.WATCHPACK_POLLING ?? "true",
  CHOKIDAR_USEPOLLING: process.env.CHOKIDAR_USEPOLLING ?? "true",
};

const child = spawn(
  "bunx",
  ["storybook", "dev", "-p", port, "--exact-port", "--host", host, "--no-open", "--debug", ...extra],
  { stdio: "inherit", shell: true, env, cwd: process.cwd() },
);
child.on("exit", (c) => process.exit(c ?? 0));
