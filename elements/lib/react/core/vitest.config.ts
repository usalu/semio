import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/ui` (inline tests in index.tsx). */
export default defineConfig({
	root,
	resolve: {
		alias: [{ find: "@elements/ui", replacement: resolve(root, "index.tsx") }],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		passWithNoTests: true,
	},
});
