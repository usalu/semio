import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const playgroundCore = resolve(root, "../playground/index.ts");
const playgroundReact = resolve(root, "../playground/react/index.tsx");
const uiCore = resolve(root, "../react/core/index.tsx");

/** @emoji 🧪 Vitest entry for `@elements/board` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@elements/playground/react", replacement: playgroundReact },
			{ find: "@elements/playground", replacement: playgroundCore },
			{ find: "@elements/ui", replacement: uiCore },
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
