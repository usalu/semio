import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/playground`. */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/ui": resolve(root, "../react/core/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts", "react/index.tsx"],
		passWithNoTests: false,
	},
});
