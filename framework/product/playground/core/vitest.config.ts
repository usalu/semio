// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@framework/playground/core`. */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@ui/react": resolve(root, "../../../ui/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["core.ts"],
		passWithNoTests: false,
	},
});
