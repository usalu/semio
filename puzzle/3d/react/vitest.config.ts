// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const playgroundRendererRoot = resolve(root, "../../../framework/product/playground/renderer/react");

/** @emoji 🧪 Vitest for `@semio-tech/puzzle-3d-react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@semio-tech/infinite-world-r3f": resolve(root, "../../../infinite/world/r3f/index.tsx"),
			"@semio-tech/puzzle-3d-rs": resolve(root, "../rs/pkg/puzzle_3d.js"),
			"@semio-tech/ui-react": resolve(root, "../../../ui/react/index.tsx"),
			"@semio-tech/puzzle-3d-core": resolve(root, "../core/index.ts"),
			"@semio-tech/framework-playground-core": resolve(root, "../../../framework/product/playground/core/index.ts"),
			"@semio-tech/framework-playground-renderer-react": resolve(playgroundRendererRoot, "index.tsx"),
			"@semio-tech/framework-platform-core": resolve(root, "../../../framework/product/platform/core/index.ts"),
			"@semio-tech/framework-platform-renderer-react": resolve(root, "../../../framework/product/platform/renderer/react/index.tsx"),
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
