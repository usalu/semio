// #region 🧲Header
// 💻 elements/client/lib/board/play/playwright.config.ts — E2E against the Vite board play harness (WebGPU raster).
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const boardRoot = path.resolve(__dirname, "..");
const playPort = process.env.BOARD_PLAY_PORT ?? "6027";
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${playPort}`;
const rawChannel = process.env.BOARD_PLAYWRIGHT_CHANNEL;
const chromeChannel = rawChannel === "chrome" || rawChannel === "msedge" ? rawChannel : undefined;

export default defineConfig({
	testDir: path.join(__dirname, "e2e"),
	fullyParallel: false,
	forbidOnly: Boolean(process.env.CI),
	retries: process.env.CI ? 2 : 0,
	timeout: 300_000,
	workers: 1,
	reporter: [["list"]],
	use: {
		baseURL,
		trace: "on-first-retry",
	},
	projects: [
		{
			name: "chromium",
			use: {
				...devices["Desktop Chrome"],
				...(chromeChannel ? { channel: chromeChannel } : {}),
				launchOptions: {
					args: [
						"--enable-unsafe-webgpu",
						"--disable-background-timer-throttling",
						"--ignore-gpu-blocklist",
						"--enable-features=UseSkiaRenderer",
					],
				},
			},
		},
	],
	webServer: {
		command: `bun ./rs/scripts/build-wasm.script.ts && bunx vite --config play/vite.config.ts --host 127.0.0.1 --port ${playPort}`,
		cwd: boardRoot,
		env: { ...process.env, BOARD_PLAY_PORT: playPort },
		url: `${baseURL}/`,
		reuseExistingServer: !process.env.CI,
		timeout: 180_000,
		stdout: "pipe",
		stderr: "pipe",
	},
});
