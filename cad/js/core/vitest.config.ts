// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const jsRoot = resolve(root, "..");

/** @emoji 🧪 Vitest for `@semio-tech/cad-js-core` (inline `import.meta.vitest`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@semio-tech/cad-js-core": resolve(root, "index.ts"),
			"@semio-tech/cad-js-runtime": resolve(jsRoot, "runtime/index.ts"),
			"@semio-tech/cad-js-module-spatial-shape": resolve(jsRoot, "module/spatial-shape/index.ts"),
			"@semio-tech/cad-js-module-aec-building": resolve(jsRoot, "module/aec-building/index.ts"),
			"@semio-tech/cad-js-module-aec-building-energy": resolve(jsRoot, "module/aec-building-energy/index.ts"),
			"@semio-tech/cad-js-module-aec-building-structure": resolve(jsRoot, "module/aec-building-structure/index.ts"),
			"@semio-tech/cad-js-kernel-brepjs": resolve(jsRoot, "kernel/brepjs/index.ts"),
			"@semio-tech/cad-js-machine-stately": resolve(jsRoot, "machine/stately/index.ts"),
			"@semio-tech/cad-js-query": resolve(jsRoot, "query/index.ts"),
			"@semio-tech/geometry-brep-js": resolve(root, "../../../geometry/brep/js/index.ts"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts"],
		includeSource: ["index.ts"],
		passWithNoTests: false,
	},
});
