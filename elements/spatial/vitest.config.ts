import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Spatial Vitest config covers the imperative brep model and the React play shell. */
export default defineConfig({
	root,
	test: {
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["js/index.ts", "react/index.tsx", "play/index.ts", "geometry-spatial-play-host.tsx"],
		passWithNoTests: false,
	},
});
