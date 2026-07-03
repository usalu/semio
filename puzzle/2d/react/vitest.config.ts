// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/puzzle-2d-react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@semio-tech/infinite-cavas-react-renderer", replacement: resolve(root, "../../../infinite/cavas/react-renderer/index.tsx") },
			{ find: "@semio-tech/ui-react", replacement: resolve(root, "../../../ui/react/index.tsx") },
			{ find: "@semio-tech/puzzle-2d-rs", replacement: resolve(root, "../rs/pkg/puzzle_2d.js") },
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
