import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/playground`. */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/framework": resolve(root, "../framework/core/index.ts"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
