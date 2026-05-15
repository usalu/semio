#!/usr/bin/env bun
/** 💻 Vite dev for the elements board play app (multi-pane Nakagin harness). */
import { spawn } from "node:child_process";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

//#region 🔖ViteDev
const host = process.env.DEVCONTAINER === "true" ? "0.0.0.0" : "127.0.0.1";
const port = process.env.BOARD_PLAY_PORT ?? "6012";
const args = process.argv.slice(2);
const env = {
	...process.env,
	...(process.env.WATCHPACK_POLLING !== undefined
		? {}
		: { WATCHPACK_POLLING: "true", CHOKIDAR_USEPOLLING: "true" }),
};

const playDir = join(dirname(fileURLToPath(import.meta.url)), "play");
const child = spawn("bunx", ["vite", "--host", host, "--port", port, ...args], {
	cwd: playDir,
	env,
	shell: true,
	stdio: "inherit",
});
child.on("exit", (c) => process.exit(c ?? 0));
//#endregion
