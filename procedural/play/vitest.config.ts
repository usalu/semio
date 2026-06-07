// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/playground/core": resolve(root, "../../framework/product/playground/core/index.ts"),
			"@framework/platform/core": resolve(root, "../../framework/product/platform/core/index.ts"),
			"@procedural/react": resolve(root, "../react/index.tsx"),
			"@flow/react": resolve(root, "../../flow/react/index.tsx"),
			"@flow/core": resolve(root, "../../flow/core/pkg/flow_core.js"),
			"@geometry/brep/js": resolve(root, "../../geometry/brep/js/index.ts"),
			"@ui/react": resolve(root, "../../ui/react/index.tsx"),
			"@infinite/world/r3f": resolve(root, "../../infinite/world/r3f/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.ts"],
		includeSource: ["index.ts"],
		passWithNoTests: false,
	},
});
