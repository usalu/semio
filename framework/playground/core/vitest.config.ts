import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/playground`. */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@ui/react": resolve(root, "../../../ui/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts", "core.ts"],
		passWithNoTests: false,
	},
});
