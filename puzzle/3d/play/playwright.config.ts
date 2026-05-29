// #region 🔌Adapters
import { defineConfig, devices } from "@playwright/test";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌Adapters

const playDir = dirname(fileURLToPath(import.meta.url));
const sceneRoot = dirname(playDir);
const port = process.env.PUZZLE_3D_PLAY_PORT ?? "6028";

export default defineConfig({
	testDir: join(playDir, "e2e"),
	timeout: 120_000,
	use: {
		...devices["Desktop Chrome"],
		baseURL: `http://127.0.0.1:${port}`,
	},
	webServer: {
		command: `bunx vite --config play/vite.config.ts --host 127.0.0.1 --port ${port}`,
		cwd: sceneRoot,
		url: `http://127.0.0.1:${port}`,
		reuseExistingServer: !process.env.CI,
		timeout: 180_000,
	},
});
