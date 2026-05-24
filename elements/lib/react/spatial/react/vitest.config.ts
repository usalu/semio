import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/geometry-spatial-react` (R3F spatial surfaces and panels). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/framework-react": resolve(root, "../../../framework/renderer/react/index.tsx"),
			"@elements/framework-react/workbench": resolve(root, "../../../framework/renderer/react/workbench-bridge.tsx"),
			"@elements/ui": resolve(root, "../../core/index.tsx"),
			"@elements/geometry-spatial-js": resolve(root, "../js/index.ts"),
		},
	},
	test: {
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
