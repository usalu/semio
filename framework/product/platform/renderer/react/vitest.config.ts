// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/framework-platform-renderer-react` monolith. */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@semio-tech/framework-platform-core", replacement: resolve(root, "../../core/index.ts") },
			{ find: "@semio-tech/framework-platform-renderer-react", replacement: resolve(root, "index.tsx") },
			{ find: "@semio-tech/framework-core", replacement: resolve(root, "../../../../core/index.ts") },
			{ find: "@semio-tech/ui-react", replacement: resolve(root, "../../../../../ui/react/index.tsx") },
			{ find: "@semio-tech/puzzle-2d-react", replacement: resolve(root, "../../../../../puzzle/2d/react/index.tsx") },
			{ find: "@semio-tech/puzzle-3d-react", replacement: resolve(root, "../../../../../puzzle/3d/react/index.tsx") },
			{ find: "@semio-tech/puzzle-5d-react", replacement: resolve(root, "../../../../../puzzle/5d/react/index.tsx") },
			{ find: "@semio-tech/cad-js-renderer", replacement: resolve(root, "../../../../../cad/renderer/index.tsx") },
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
