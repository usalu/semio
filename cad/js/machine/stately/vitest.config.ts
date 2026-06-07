// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const coreEntry = resolve(root, "../../core/index.ts");

export default defineConfig({
	root,
	resolve: {
		alias: {
			"@cad/js/core": coreEntry,
			"@cad/js/runtime": resolve(root, "../../runtime/index.ts"),
			"@cad/js/module/spatial-shape": resolve(root, "../../module/spatial-shape/index.ts"),
			"@cad/js/module/aec-building": resolve(root, "../../module/aec-building/index.ts"),
			"@cad/js/module/aec-building-energy": resolve(root, "../../module/aec-building-energy/index.ts"),
			"@cad/js/module/aec-building-structure": resolve(root, "../../module/aec-building-structure/index.ts"),
			"@cad/js/kernel/brepjs": resolve(root, "../../kernel/brepjs/index.ts"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		testTimeout: 120_000,
		fileParallelism: false,
		maxConcurrency: 1,
		includeSource: ["index.ts"],
	},
});
