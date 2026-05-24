import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));
const frameworkCore = resolve(root, "../../framework/core/index.ts");
const frameworkReact = resolve(root, "../../framework/renderer/index.tsx");
const frameworkReactWorkbench = frameworkReact;
const uiCore = resolve(root, "../core/index.tsx");

/** @emoji 🧪 Vitest entry for `@elements/board` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/framework": frameworkCore,
			"@elements/framework-react": frameworkReact,
			"@elements/framework-react/workbench": frameworkReactWorkbench,
			"@elements/playground": resolve(root, "../../playground/index.ts"),
			"@elements/ui": uiCore,
		},
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
