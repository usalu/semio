// #region 🧲Header
/** @emoji 🧪 Vitest for `@semio-tech/framework-presentation-play`. */
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
			"@semio-tech/framework-core": path.resolve(playDir, "../../../core/index.ts"),
			"@semio-tech/framework-presentation-core": path.resolve(playDir, "../core/index.ts"),
			"@semio-tech/framework-playground-core": path.resolve(playDir, "../../playground/core/index.ts"),
			"@semio-tech/framework-platform-core": path.resolve(playDir, "../../platform/core/index.ts"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
