// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@framework/platform/renderer/react` monolith. */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@framework/platform/core", replacement: resolve(root, "../../core/index.ts") },
			{ find: "@framework/platform/renderer/react", replacement: resolve(root, "index.tsx") },
			{ find: "@framework/core", replacement: resolve(root, "../../../../core/index.ts") },
			{ find: "@ui/react", replacement: resolve(root, "../../../../../ui/react/index.tsx") },
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
