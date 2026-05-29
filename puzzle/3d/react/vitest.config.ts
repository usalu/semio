import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@puzzle/scene` (react + play sources with `import.meta.vitest`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/playground-react": resolve(root, "../../framework/playground/renderer/react/index.tsx"),
			"@framework/playground": resolve(root, "../../framework/playground/core/index.ts"),
			"@ui/react": resolve(root, "../../ui/react/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts", "index.tsx", "play/index.ts"],
		includeSource: ["index.ts", "index.tsx", "play/index.ts"],
		passWithNoTests: true,
	},
});
