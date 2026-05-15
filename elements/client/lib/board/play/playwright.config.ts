// #region 🧲Header
// 💻 elements/client/lib/board/play/playwright.config.ts — E2E against the Vite board play harness (Vello + WebGPU).
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const boardRoot = path.resolve(__dirname, "..");
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? "http://127.0.0.1:6012";
const rawChannel = process.env.BOARD_PLAYWRIGHT_CHANNEL;
const chromeChannel = rawChannel === "chrome" || rawChannel === "msedge" ? rawChannel : undefined;

export default defineConfig({
	testDir: path.join(__dirname, "e2e"),
	fullyParallel: false,
	forbidOnly: Boolean(process.env.CI),
	retries: process.env.CI ? 2 : 0,
	timeout: 120_000,
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
		command: "bun ./script.ts dev",
		cwd: boardRoot,
		url: `${baseURL}/`,
		reuseExistingServer: false,
		timeout: 180_000,
		stdout: "pipe",
		stderr: "pipe",
	},
});
