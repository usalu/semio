// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/3d/react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@ui/react": resolve(root, "../../../ui/react/index.tsx"),
			"@puzzle/3d/play": resolve(root, "../play/index.ts"),
			"@framework/playground/core": resolve(root, "../../../framework/product/playground/core/core.ts"),
			"@framework/platform/core": resolve(root, "../../../framework/product/platform/core/index.ts"),
		},
	},
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		passWithNoTests: true,
	},
});
