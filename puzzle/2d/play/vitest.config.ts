// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for puzzle 2d play playground wiring (`play/index.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/playground/core": resolve(root, "../../../framework/product/playground/core/index.ts"),
			"@framework/platform/core": resolve(root, "../../../framework/product/platform/core/index.ts"),
			"@infinite/cavas/react-renderer": resolve(root, "../../../infinite/cavas/react-renderer/index.tsx"),
			"@puzzle/2d/react": resolve(root, "../react/index.tsx"),
			"@ui/react": resolve(root, "../../../ui/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
