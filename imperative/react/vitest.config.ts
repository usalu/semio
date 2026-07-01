import path from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
	resolve: {
		alias: {
			"@semio-tech/imperative-core": path.resolve(__dirname, "../core/index.ts"),
		},
	},
	test: {
		environment: "node",
	},
});
