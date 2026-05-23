import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/geometry` (fixture parser, transform updates, and play-shell selection wiring). */
export default defineConfig({
	root,
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["react/index.tsx", "play/index.tsx"],
		passWithNoTests: false,
	},
});