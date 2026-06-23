// #region 🔌Adapters
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/framework-presentation-renderer-react`. */
export default defineConfig({
	root,
	plugins: [react()],
	resolve: {
		alias: [
			{ find: "@semio-tech/framework-presentation-core", replacement: resolve(root, "../../core/index.ts") },
			{ find: "@semio-tech/framework-core", replacement: resolve(root, "../../../core/index.ts") },
			{ find: "@semio-tech/ui-react", replacement: resolve(root, "../../../../../ui/react/index.tsx") },
			{
				find: "@semio-tech/mit-bestand-praesentation-projektetage-spec",
				replacement: resolve(root, "../../../../../mit-bestand/präsentation/33.projektetage/spec.ts"),
			},
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx", "markdown.ts", "json.tsx"],
		passWithNoTests: false,
		setupFiles: ["./vitest.setup.ts"],
	},
});
