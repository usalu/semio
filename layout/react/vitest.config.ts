import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

export default defineConfig({
	resolve: {
		alias: {
			"@semio-tech/layout-rs": path.resolve(__dirname, "../rs/pkg/layout_rs.js"),
			"@semio-tech/infinite-cavas-react-renderer": path.resolve(__dirname, "../../infinite/cavas/react-renderer/index.tsx"),
		},
	},
	test: {
		environment: "node",
		root: path.dirname(fileURLToPath(import.meta.url)),
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		passWithNoTests: false,
	},
});
