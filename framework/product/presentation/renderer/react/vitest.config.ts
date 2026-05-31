// #region 🔌Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@framework/presentation/renderer/react`. */
export default defineConfig({
	root,
	plugins: [react()],
	resolve: {
		alias: [
			{ find: "@framework/presentation/core", replacement: resolve(root, "../../core/index.ts") },
			{ find: "@framework/core", replacement: resolve(root, "../../../core/index.ts") },
			{ find: "@ui/react", replacement: resolve(root, "../../../../../ui/react/index.tsx") },
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
