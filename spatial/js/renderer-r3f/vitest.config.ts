import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	resolve: {
		alias: [{ find: "@spatial/js-core", replacement: resolve(root, "../core/index.ts") }],
	},
	test: {
		mode: "test",
		environment: "jsdom",
		testTimeout: 120_000,
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx", "play/index.ts"],
		includeSource: ["index.tsx", "play/index.ts"],
	},
});
