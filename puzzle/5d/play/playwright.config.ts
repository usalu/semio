// #region 🔌Adapters
import { defineConfig, devices } from "@playwright/test";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { playgroundTestPortString } from "../../../repo/lib/js/index.ts";
// #endregion 🔌Adapters

const playDir = dirname(fileURLToPath(import.meta.url));
const puzzle5dRoot = dirname(playDir);
const port = process.env.PUZZLE_5D_PLAY_PORT ?? playgroundTestPortString("puzzle-5d") ?? "6035";

export default defineConfig({
	testDir: join(playDir, "e2e"),
	timeout: 120_000,
	use: {
		...devices["Desktop Chrome"],
		baseURL: `http://127.0.0.1:${port}`,
	},
	webServer: {
		command: `bunx vite --config play/vite.config.ts --host 127.0.0.1 --port ${port}`,
		cwd: puzzle5dRoot,
		url: `http://127.0.0.1:${port}`,
		reuseExistingServer: !process.env.CI,
		timeout: 180_000,
	},
});
