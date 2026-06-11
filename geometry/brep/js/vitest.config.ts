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
			"@flow/module-brep": resolve(root, "../../flow/module/brep/pkg/flow_module_brep.js"),
		},
	},
	assetsInclude: ["**/*.wasm"],
	test: {
		mode: "test",
		environment: "node",
		testTimeout: 120_000,
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts"],
	},
});
