import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		include: ["index.tsx", "demo.ts"],
		environment: "jsdom",
	},
});
