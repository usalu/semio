import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/geometry-spatial-play` (framework-free shell wiring). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/framework": resolve(root, "../../../framework/core/index.ts"),
			"@elements/playground": resolve(root, "../../../playground/index.ts"),
			"@elements/geometry-spatial-js": resolve(root, "../js/index.ts"),
		},
	},
	test: {
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
