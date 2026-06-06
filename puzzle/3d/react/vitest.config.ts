// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const playgroundRendererRoot = resolve(root, "../../../framework/product/playground/renderer/react");

/** @emoji 🧪 Vitest for `@puzzle/3d/react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@infinite/world/r3f": resolve(root, "../../../infinite/world/r3f/index.tsx"),
			"@puzzle/3d/rs": resolve(root, "../rs/pkg/puzzle_3d.js"),
			"@ui/react": resolve(root, "../../../ui/react/index.tsx"),
			"@puzzle/3d/play": resolve(root, "../play/index.ts"),
			"@framework/playground/core": resolve(root, "../../../framework/product/playground/core/index.ts"),
			"@framework/playground/renderer/react/puzzle/3d": resolve(playgroundRendererRoot, "index.tsx"),
			"@framework/platform/core": resolve(root, "../../../framework/product/platform/core/index.ts"),
			"@framework/platform/renderer/react": resolve(root, "../../../framework/product/platform/renderer/react/index.tsx"),
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
