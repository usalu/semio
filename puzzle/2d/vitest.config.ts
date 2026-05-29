import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const playgroundCore = resolve(root, "../../framework/playground/core/index.ts");
const playgroundReact = resolve(root, "../../framework/playground/renderer/react/index.tsx");
const uiCore = resolve(root, "../../ui/react/index.tsx");

/** @emoji 🧪 Vitest entry for `@puzzle/board` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@framework/playground-react", replacement: playgroundReact },
			{ find: "@framework/playground", replacement: playgroundCore },
			{ find: "@ui/react", replacement: uiCore },
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
