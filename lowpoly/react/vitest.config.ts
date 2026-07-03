import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	resolve: {
		alias: [
			{ find: "@semio-tech/lowpoly-core", replacement: resolve(root, "../core/index.ts") },
			{ find: "@semio-tech/lowpoly-core/rs/pkg/lowpoly_core.js", replacement: resolve(root, "../core/rs/pkg/lowpoly_core.js") },
		],
	},
	test: {
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		environment: "node",
		passWithNoTests: false,
	},
});
