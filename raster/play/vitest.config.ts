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
			"@semio-tech/framework-playground-core": resolve(root, "../../framework/product/playground/core/index.ts"),
			"@semio-tech/framework-platform-core": resolve(root, "../../framework/product/platform/core/index.ts"),
			"@semio-tech/raster-core": resolve(root, "../core/index.ts"),
			"@semio-tech/raster-react": resolve(root, "../react/index.tsx"),
			"@semio-tech/ui-react": resolve(root, "../../ui/react/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.ts", "fixture-slugs.ts"],
		passWithNoTests: false,
	},
});
