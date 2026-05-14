#!/usr/bin/env bun
/**
 * 💻 Vite dev for the docs site; forwards argv to Vite (e.g. `--port 4321`).
 */
import { spawn } from "node:child_process";

//#region 🔖ViteDev
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
});
child.on("exit", (c) => process.exit(c ?? 0));
//#endregion
