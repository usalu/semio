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
			"@framework/playground/core": resolve(root, "../../framework/product/playground/core/index.ts"),
			"@procedural/react": resolve(root, "../react/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.ts"],
		includeSource: ["index.ts"],
		passWithNoTests: false,
	},
});
