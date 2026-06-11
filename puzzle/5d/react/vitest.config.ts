// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/5d/react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@infinite/world/r3f", replacement: resolve(root, "../../../infinite/world/r3f/index.tsx") },
			{ find: "@ui/react", replacement: resolve(root, "../../../ui/react/index.tsx") },
			{ find: "@puzzle/2d/react", replacement: resolve(root, "../../2d/react/index.tsx") },
			{ find: "@puzzle/3d/react", replacement: resolve(root, "../../3d/react/index.tsx") },
			{ find: "@puzzle/3d/rs", replacement: resolve(root, "../../3d/rs/pkg/puzzle_3d.js") },
		],
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
