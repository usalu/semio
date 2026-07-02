import path from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
	resolve: {
		alias: {
			"@semio-tech/imperative-core": path.resolve(__dirname, "./index.ts"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		includeSource: ["index.ts"],
		passWithNoTests: false,
	},
});
