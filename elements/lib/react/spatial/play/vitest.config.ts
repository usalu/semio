import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const uiShellRoot = resolve(root, "../../core/index.ts");

/** @emoji 🧪 Vitest for `@elements/geometry-spatial-play` (framework-free shell wiring). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/ui-shell": uiShellRoot,
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
