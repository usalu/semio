import path from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
	resolve: {
		alias: {
			"@semio-tech/sequence-core": path.resolve(__dirname, "../core/index.ts"),
			"@semio-tech/imperative-core": path.resolve(__dirname, "../../imperative/core/index.ts"),
			"@semio-tech/imperative-react": path.resolve(__dirname, "../../imperative/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		passWithNoTests: false,
	},
});
