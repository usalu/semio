// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/puzzle-5d-play` (`play/index.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@semio-tech/framework-playground-core", replacement: resolve(root, "../../../framework/product/playground/core/index.ts") },
			{ find: "@semio-tech/framework-platform-core", replacement: resolve(root, "../../../framework/product/platform/core/index.ts") },
			{ find: "@semio-tech/infinite-cavas-react-renderer", replacement: resolve(root, "../../../infinite/cavas/react-renderer/index.tsx") },
			{ find: "@semio-tech/infinite-world-r3f", replacement: resolve(root, "../../../infinite/world/r3f/index.tsx") },
			{ find: "@semio-tech/puzzle-2d-play", replacement: resolve(root, "../../2d/play/index.ts") },
			{ find: "@semio-tech/puzzle-3d-play", replacement: resolve(root, "../../3d/play/index.ts") },
			{ find: "@semio-tech/puzzle-2d-react", replacement: resolve(root, "../../2d/react/index.tsx") },
			{
				find: "@semio-tech/puzzle-3d-react",
				replacement: resolve(root, "../../3d/react/index.tsx"),
			},
			{ find: "@semio-tech/puzzle-5d-react", replacement: resolve(root, "../react/index.tsx") },
			{ find: "@semio-tech/ui-react", replacement: resolve(root, "../../../ui/react/index.tsx") },
		],
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
