import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const coreEntry = resolve(root, "../core/index.ts");

export default defineConfig({
	root,
	assetsInclude: ["**/*.wasm"],
	resolve: {
		alias: {
			"@spatial/js-core": coreEntry,
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
