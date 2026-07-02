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
			"@semio-tech/flow-core": resolve(root, "../../../flow/core/rs/pkg/flow_core.js"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
