import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/spatial-js` (imperative brep kernel and topologic fixture). */
export default defineConfig({
	root,
	test: {
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
