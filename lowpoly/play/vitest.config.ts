import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	resolve: {
		alias: [
			{ find: "@semio-tech/lowpoly-core", replacement: resolve(root, "../core/index.ts") },
			{ find: "@semio-tech/lowpoly-react", replacement: resolve(root, "../react/index.tsx") },
			{ find: "@semio-tech/framework-playground-core", replacement: resolve(root, "../../framework/product/playground/core/index.ts") },
		],
	},
	test: {
		include: ["index.ts"],
		includeSource: ["index.ts"],
		environment: "node",
		passWithNoTests: false,
	},
});
