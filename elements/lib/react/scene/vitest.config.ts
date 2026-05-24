import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/scene` (react + play sources with `import.meta.vitest`). */
export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@elements/framework-react/ui-declarative-renderer", replacement: resolve(root, "../../framework/renderer/react/ui-declarative-renderer.tsx") },
			{ find: "@elements/framework-react/workbench-view", replacement: resolve(root, "../../framework/renderer/react/workbench-view.tsx") },
			{ find: "@elements/framework-react/workbench-mount", replacement: resolve(root, "../../framework/renderer/react/workbench-mount.tsx") },
			{ find: "@elements/framework-react/workbench-app-context", replacement: resolve(root, "../../framework/renderer/react/workbench-app-context.tsx") },
			{ find: "@elements/framework-react/shell-bridge", replacement: resolve(root, "../../framework/renderer/react/shell-bridge.tsx") },
			{ find: "@elements/framework-react", replacement: resolve(root, "../../framework/renderer/react/index.tsx") },
			{ find: "@elements/framework", replacement: resolve(root, "../../framework/core/index.ts") },
			{ find: "@elements/ui", replacement: resolve(root, "../core/index.tsx") },
		],
	},
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx", "play/index.ts", "scene-play-host.tsx"],
		passWithNoTests: true,
	},
});
