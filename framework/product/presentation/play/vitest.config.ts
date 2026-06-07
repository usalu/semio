// #region 🧲Header
/** @emoji 🧪 Vitest for `@framework/presentation/play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root: playDir,
	resolve: {
		alias: {
			"@framework/core": path.resolve(playDir, "../../../core/index.ts"),
			"@framework/presentation/core": path.resolve(playDir, "../core/index.ts"),
			"@framework/playground/core": path.resolve(playDir, "../../playground/core/index.ts"),
			"@framework/platform/core": path.resolve(playDir, "../../platform/core/index.ts"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
