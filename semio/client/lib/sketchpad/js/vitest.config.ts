// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/core": resolve(root, "../../../../../framework/core/index.ts"),
			"@framework/platform/core": resolve(root, "../../../../../framework/product/platform/core/index.ts"),
			"@puzzle/2d/react": resolve(root, "../../../../../puzzle/2d/react/index.tsx"),
			"@puzzle/3d/react": resolve(root, "../../../../../puzzle/3d/react/index.tsx"),
			"@puzzle/5d/react": resolve(root, "../../../../../puzzle/5d/react/index.tsx"),
			"@semio/js": resolve(root, "../../js/index.ts"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
	},
});
