// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const jsRoot = resolve(root, "..");

/** @emoji 🧪 Vitest for `@cad/js/core` (inline `import.meta.vitest`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@cad/js/core": resolve(root, "index.ts"),
			"@cad/js/kernel/brepjs": resolve(jsRoot, "kernel-brepjs/index.ts"),
			"@cad/js/machine/stately": resolve(jsRoot, "machine-stately/index.ts"),
			"@cad/js/query": resolve(jsRoot, "query/index.ts"),
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
