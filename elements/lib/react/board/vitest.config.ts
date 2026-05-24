import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest entry for `@elements/board` inlined source tests (`import.meta.vitest` blocks). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/framework": `${root}/../../../framework/core/index.ts`,
			"@elements/framework-react": `${root}/../../../framework/renderer/react/index.tsx`,
			"@elements/framework-react/workbench": `${root}/../../../framework/renderer/react/workbench-bridge.tsx`,
			"@elements/ui": `${root}/../core/index.tsx`,
		},
	},
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.ts", "index.tsx", "play/index.ts", "board-play-host.tsx"],
		includeSource: ["index.ts", "index.tsx", "play/index.ts", "board-play-host.tsx"],
		passWithNoTests: true,
	},
});
