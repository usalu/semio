#!/usr/bin/env bun
/** 💻 Storybook dev: delegates to the monorepo root `.storybook` config. */
import { spawn } from "node:child_process";
import { join } from "node:path";

//#region 🔖StorybookDev
const repoRoot = join(import.meta.dir, "..", "..", "..", "..");
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
	["storybook", "dev", "-c", ".storybook", "-p", port, "--exact-port", "--host", host, "--no-open", "--debug", ...extra],
	{ stdio: "inherit", shell: true, env, cwd: repoRoot },
);
child.on("exit", (c) => process.exit(c ?? 0));
//#endregion
