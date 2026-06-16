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
			{
				find: "@mit-bestand/praesentation/projektetage-spec",
				replacement: resolve(root, "../../../../../mit-bestand/präsentation/33.projektetage/spec.ts"),
			},
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx", "markdown.ts"],
		passWithNoTests: false,
		setupFiles: ["./vitest.setup.ts"],
	},
});
