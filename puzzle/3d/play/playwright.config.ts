// #region 🔌Adapters
import { defineConfig, devices } from "@playwright/test";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
// #endregion 🔌Adapters

const playDir = dirname(fileURLToPath(import.meta.url));
const puzzle3dPlayRoot = dirname(playDir);
const port = process.env.PUZZLE_3D_PLAY_PORT ?? "6013";

export default defineConfig({
	testDir: join(playDir, "e2e"),
	timeout: 180_000,
	use: {
		...devices["Desktop Chrome"],
		baseURL: `http://127.0.0.1:${port}`,
	},
	webServer: {
		command: `bunx vite --config play/vite.config.ts --host 127.0.0.1 --port ${port}`,
		cwd: puzzle3dPlayRoot,
		url: `http://127.0.0.1:${port}`,
		reuseExistingServer: !process.env.CI,
		timeout: 180_000,
	},
});
