import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/framework-react` declarative renderer. */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@elements/framework", replacement: resolve(root, "../../core/index.ts") },
			{ find: "@elements/ui/chrome", replacement: resolve(root, "../../../react/core/chrome.ts") },
			{ find: "@elements/ui", replacement: resolve(root, "../../../react/core/index.tsx") },
		],
	},
	test: {
		environment: "jsdom",
		include: ["ui-declarative-renderer.tsx", "workbench-view.tsx", "workbench-history.tsx", "level-context.tsx"],
		passWithNoTests: false,
	},
});
