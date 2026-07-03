import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		include: ["js/index.ts"],
		includeSource: ["js/index.ts"],
		passWithNoTests: false,
	},
});
