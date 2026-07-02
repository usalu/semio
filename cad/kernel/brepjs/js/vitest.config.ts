// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const coreEntry = resolve(root, "../../core/index.ts");

export default defineConfig({
	root,
	assetsInclude: ["**/*.wasm"],
	resolve: {
		alias: {
			"@semio-tech/kernel-3d-js": resolve(root, "../../../../kernel/3d/brep/js/index.ts"),
			"@semio-tech/cad-js-core": coreEntry,
			"@semio-tech/cad-js-runtime": resolve(root, "../../runtime/index.ts"),
			"@semio-tech/cad-js-module-aec-building": resolve(root, "../../module/aec-building/index.ts"),
			"@semio-tech/cad-js-module-aec-building-energy": resolve(root, "../../module/aec-building-energy/index.ts"),
			"@semio-tech/cad-js-module-aec-building-structure": resolve(root, "../../module/aec-building-structure/index.ts"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		testTimeout: 120_000,
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts"],
	},
});
