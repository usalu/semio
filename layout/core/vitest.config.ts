import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "node",
		include: ["index.ts"],
		includeSource: ["index.ts"],
		passWithNoTests: false,
	},
});
