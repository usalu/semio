// #region 🔌Adapters
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	resolve: {
		alias: {
			"@framework/core": resolve(root, "../../../core/index.ts"),
		},
	},
	test: {
		include: ["index.ts"],
	},
});
