// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/ui-react` (inline tests in index.tsx). */
export default defineConfig({
	root,
	resolve: {
		alias: [{ find: "@semio-tech/ui-react", replacement: resolve(root, "index.tsx") }],
	},
	test: {
		environment: "jsdom",
		includeSource: ["index.tsx"],
		passWithNoTests: true,
		setupFiles: [resolve(root, "vitest.setup.ts")],
	},
});
