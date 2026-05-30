// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/3d/play` (`play/play.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/playground/core": resolve(root, "../../../framework/product/playground/core/core.ts"),
			"@framework/platform/core": resolve(root, "../../../framework/product/platform/core/index.ts"),
			"@puzzle/3d/react": resolve(root, "../react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["play.ts"],
		passWithNoTests: false,
	},
});
