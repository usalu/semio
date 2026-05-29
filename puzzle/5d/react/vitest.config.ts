import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/5d-react` (react + play sources with `import.meta.vitest`). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@framework/playground-renderer-react", replacement: resolve(root, "../../framework/playground/renderer/react/index.tsx") },
			{ find: "@framework/playground", replacement: resolve(root, "../../framework/playground/core/index.ts") },
			{ find: "@ui/react", replacement: resolve(root, "../../ui/react/index.tsx") },
			{ find: "@puzzle/2d-react", replacement: resolve(root, "../2d/index.tsx") },
			{ find: "@puzzle/3d-react", replacement: resolve(root, "../3d/index.tsx") },
		],
	},
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx", "play/index.ts"],
		includeSource: ["index.tsx", "play/index.ts"],
		passWithNoTests: true,
	},
});
