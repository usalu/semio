import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		include: ["index.ts"],
		includeSource: ["index.ts"],
		passWithNoTests: false,
	},
});
