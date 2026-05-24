import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/topology` (react + play sources with `import.meta.vitest`). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@elements/framework-react", replacement: resolve(root, "../../framework/renderer/react/index.tsx") },
			{ find: "@elements/framework", replacement: resolve(root, "../../framework/core/index.ts") },
			{ find: "@elements/ui", replacement: resolve(root, "../core/index.tsx") },
			{ find: "@elements/board", replacement: resolve(root, "../../board/index.tsx") },
			{ find: "@elements/scene", replacement: resolve(root, "../scene/index.tsx") },
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
