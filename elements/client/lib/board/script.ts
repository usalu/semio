#!/usr/bin/env bun
/** 🧭 Board package task router: `bun ./script.ts <dev|build|test> [args…]`. */
import { spawn } from "node:child_process";
import { join } from "node:path";

const cwd = import.meta.dir;
const segs = process.argv.slice(2);

const command = segs[0];
const extra = segs.slice(1);

const env = {
  ...process.env,
  ...(process.env.WATCHPACK_POLLING !== undefined
    ? {}
    : { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};

function run(args: string[], options: { cwd?: string } = {}): void {
  const child = spawn("bunx", args, {
    cwd: options.cwd ?? cwd,
    env,
    shell: true,
    stdio: "inherit",
  });
  child.on("exit", (code) => process.exit(code ?? 0));
  child.on("error", (error) => {
    console.error(error);
    process.exit(1);
  });
}

if (command === "dev") {
  const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
  const port = process.env.BOARD_PLAY_PORT ?? "6012";
  run(["vite", "--host", host, "--port", port, ...extra], { cwd: join(cwd, "play") });
} else if (command === "build") {
  run(["vite", "build", ...extra], { cwd: join(cwd, "play") });
} else if (command === "test") {
  run(["vitest", "run", "--passWithNoTests", "--config", "vitest.config.ts", ...extra]);
} else {
  console.error("usage: bun ./script.ts <dev|build|test> [args…]");
  process.exit(1);
}
