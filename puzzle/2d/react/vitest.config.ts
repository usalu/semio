import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/2d-react` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: [{ find: "@ui/react", replacement: resolve(root, "../../ui/react/index.tsx") }],
	},
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		passWithNoTests: true,
	},
});
