// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/5d/play` (`play/play.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@framework/playground/core", replacement: resolve(root, "../../../framework/product/playground/core/core.ts") },
			{ find: "@framework/platform/core", replacement: resolve(root, "../../../framework/product/platform/core/index.ts") },
			{ find: "@puzzle/2d/play", replacement: resolve(root, "../../2d/play/play.ts") },
			{ find: "@puzzle/3d/play", replacement: resolve(root, "../../3d/play/play.ts") },
			{ find: "@puzzle/2d/react", replacement: resolve(root, "../../2d/react/index.tsx") },
			{
				find: "@puzzle/3d/react",
				replacement: resolve(root, "../../3d/react/index.tsx"),
			},
			{ find: "@puzzle/5d/react", replacement: resolve(root, "../react/index.tsx") },
		],
	},
	test: {
		environment: "node",
		include: ["play.ts"],
		passWithNoTests: false,
	},
});
