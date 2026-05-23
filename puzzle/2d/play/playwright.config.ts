// #region 🧲Header
// 💻 elements/client/lib/board/play/playwright.config.ts — E2E against the Vite board play harness (WebGPU raster).
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";

import { defineConfig, devices } from "@playwright/test";
// #endregion 🔌Adapters

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const boardRoot = path.resolve(__dirname, "..");
const playPort = process.env.PUZZLE_2D_PLAY_PORT ?? "6027";
const baseURL = process.env.PLAYWRIGHT_BASE_URL ?? `http://127.0.0.1:${playPort}`;
/** Use real Chrome/Edge for WebGPU: `PUZZLE_2D_PLAYWRIGHT_CHANNEL=chrome bunx playwright test …` (bundled Chromium may lack an adapter on some Windows setups). */
const rawChannel = process.env.PUZZLE_2D_PLAYWRIGHT_CHANNEL;
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
	// Playwright web server: Vite only; `script.ts test` already builds wasm—rebuilding wasm here rewrote `rs/pkg` and triggered Vite HMR mid-test.
	webServer: {
		command: `bunx vite --config play/vite.config.ts --host 127.0.0.1 --port ${playPort}`,
		cwd: boardRoot,
		env: { ...process.env, PUZZLE_2D_PLAY_PORT: playPort },
		url: `${baseURL}/`,
		/** Avoid picking up an unrelated process already bound to the play port (stale local dev servers). */
		reuseExistingServer: false,
		timeout: 180_000,
		stdout: "pipe",
		stderr: "pipe",
	},
});
