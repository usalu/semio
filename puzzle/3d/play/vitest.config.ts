// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/3d/play` (`play/index.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/playground/core": resolve(root, "../../../framework/product/playground/core/index.ts"),
			"@framework/platform/core": resolve(root, "../../../framework/product/platform/core/index.ts"),
			"@infinite/world/r3f": resolve(root, "../../../infinite/world/r3f/index.tsx"),
			"@puzzle/3d/react": resolve(root, "../react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
