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
			"@semio/js": resolve(root, "../../js/index.ts"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
	},
});
