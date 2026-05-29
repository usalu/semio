// #region 🔌Adapters
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@cad/js-query` (inline `import.meta.vitest`). */
export default defineConfig({
	root,
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
