// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@ui/react` (inline tests in index.tsx). */
export default defineConfig({
	root,
	resolve: {
		alias: [{ find: "@ui/react", replacement: resolve(root, "index.tsx") }],
	},
	test: {
		environment: "jsdom",
		includeSource: ["index.tsx"],
		passWithNoTests: true,
		setupFiles: [resolve(root, "vitest.setup.ts")],
	},
});
