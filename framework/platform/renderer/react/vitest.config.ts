import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/framework-react` monolith. */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@framework/platform", replacement: resolve(root, "../../core/index.ts") },
			{ find: "@framework/platform-react", replacement: resolve(root, "index.tsx") },
			{ find: "@ui/react", replacement: resolve(root, "../../../../ui/react/index.tsx") },
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
