// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@framework/platform/renderer/react` monolith. */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@framework/platform/core", replacement: resolve(root, "../../core/index.ts") },
			{ find: "@framework/platform/renderer/react", replacement: resolve(root, "index.tsx") },
			{ find: "@framework/core", replacement: resolve(root, "../../../../core/index.ts") },
			{ find: "@ui/react", replacement: resolve(root, "../../../../../ui/react/index.tsx") },
			{ find: "@puzzle/2d/react", replacement: resolve(root, "../../../../../puzzle/2d/react/index.tsx") },
			{ find: "@puzzle/3d/react", replacement: resolve(root, "../../../../../puzzle/3d/react/index.tsx") },
			{ find: "@puzzle/5d/react", replacement: resolve(root, "../../../../../puzzle/5d/react/index.tsx") },
			{ find: "@cad/js/renderer", replacement: resolve(root, "../../../../../cad/js/renderer/index.tsx") },
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
