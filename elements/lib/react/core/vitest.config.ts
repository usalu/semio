import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/ui` (inline tests in index.tsx). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@elements/framework-react/ui-declarative-renderer", replacement: resolve(root, "../../framework/renderer/react/ui-declarative-renderer.tsx") },
			{ find: "@elements/framework-react/shell-bridge", replacement: resolve(root, "../../framework/renderer/react/shell-bridge.tsx") },
			{ find: "@elements/framework-react/workbench-app-context", replacement: resolve(root, "../../framework/renderer/react/workbench-app-context.tsx") },
			{ find: "@elements/framework-react", replacement: resolve(root, "../../framework/renderer/react/index.tsx") },
			{ find: "@elements/framework", replacement: resolve(root, "../../framework/core/index.ts") },
			{ find: "@elements/ui", replacement: resolve(root, "index.tsx") },
		],
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		passWithNoTests: true,
	},
});
