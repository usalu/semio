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
			"@framework/playground": resolve(root, "../../../framework/playground/core/core.ts"),
			"@puzzle/3d/react": resolve(root, "../react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
