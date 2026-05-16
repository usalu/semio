import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest entry for `@elements/board` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	test: {
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts", "index.tsx"],
		includeSource: ["index.ts", "index.tsx"],
		passWithNoTests: true,
	},
});
