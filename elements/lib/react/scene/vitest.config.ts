import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji ­ƒº¬ Vitest for `@elements/scene` (react + play sources with `import.meta.vitest`). */
export default defineConfig({
	root,
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx", "play/index.ts", "scene-play-host.tsx"],
		passWithNoTests: true,
	},
});
