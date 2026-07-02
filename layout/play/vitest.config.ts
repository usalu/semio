import path from "node:path";
import { defineConfig } from "vitest/config";

export default defineConfig({
	resolve: {
		alias: {
			"@semio-tech/framework-playground-core": path.resolve(__dirname, "../../framework/product/playground/core/index.ts"),
			"@semio-tech/framework-platform-core": path.resolve(__dirname, "../../framework/product/platform/core/index.ts"),
			"@semio-tech/layout-core": path.resolve(__dirname, "../core/index.ts"),
			"@semio-tech/layout-react": path.resolve(__dirname, "../react/index.tsx"),
			"@semio-tech/layout-rs": path.resolve(__dirname, "../rs/pkg/layout_rs.js"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		includeSource: ["index.ts"],
		passWithNoTests: false,
	},
});
