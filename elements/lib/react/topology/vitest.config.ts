import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji ┬¡ãÆ┬║┬¼ Vitest for `@elements/topology` (react + play sources with `import.meta.vitest`). */
export default defineConfig({
	root,
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["react/index.tsx", "play/index.ts", "topology-play-host.tsx"],
		passWithNoTests: true,
	},
});
