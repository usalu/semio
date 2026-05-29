import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	assetsInclude: ["**/*.wasm"],
	resolve: {
		alias: [
			{ find: "@elements/playground/react", replacement: resolve(root, "../../../elements/lib/playground/react/index.tsx") },
			{ find: "@elements/playground", replacement: resolve(root, "../../../elements/lib/playground/index.ts") },
			{ find: "@elements/ui", replacement: resolve(root, "../../../elements/lib/react/core/index.tsx") },
			{ find: "@spatial/js-core", replacement: resolve(root, "../core/index.ts") },
			{ find: "@spatial/js-kernel-brepjs", replacement: resolve(root, "../kernel-brepjs/index.ts") },
			{ find: "@spatial/js-machine-stately", replacement: resolve(root, "../machine-stately/index.ts") },
			{ find: "@spatial/js-query", replacement: resolve(root, "../query/index.ts") },
		],
	},
		test: {
		mode: "test",
		environment: "jsdom",
		testTimeout: 120_000,
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx", "play/index.ts", "play/main.tsx"],
	},
});
