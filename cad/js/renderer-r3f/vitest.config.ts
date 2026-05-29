import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	assetsInclude: ["**/*.wasm"],
	resolve: {
		alias: [
			{ find: "@framework/playground-react", replacement: resolve(root, "../../../framework/playground/renderer/react/index.tsx") },
			{ find: "@framework/playground", replacement: resolve(root, "../../../framework/playground/core/index.ts") },
			{ find: "@ui/react", replacement: resolve(root, "../../../ui/react/index.tsx") },
			{ find: "@cad/js-core", replacement: resolve(root, "../core/index.ts") },
			{ find: "@cad/js-kernel-brepjs", replacement: resolve(root, "../kernel-brepjs/index.ts") },
			{ find: "@cad/js-machine-stately", replacement: resolve(root, "../machine-stately/index.ts") },
			{ find: "@cad/js-query", replacement: resolve(root, "../query/index.ts") },
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
