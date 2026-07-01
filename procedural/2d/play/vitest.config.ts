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
			"@semio-tech/framework-playground-core": resolve(root, "../../../framework/product/playground/core/index.ts"),
			"@semio-tech/framework-platform-core": resolve(root, "../../../framework/product/platform/core/index.ts"),
			"@semio-tech/procedural-2d-react": resolve(root, "../react/index.tsx"),
			"@semio-tech/flow-react": resolve(root, "../../../flow/react/index.tsx"),
			"@semio-tech/flow-core": resolve(root, "../../../flow/core/pkg/flow_core.js"),
			"@semio-tech/kernel-2d-js": resolve(root, "../../../kernel/2d/js/index.ts"),
			"@semio-tech/flow-module-draw": resolve(root, "../../../flow/module/draw/pkg/flow_module_draw.js"),
			"@semio-tech/flow-module-core": resolve(root, "../../../flow/module/core/pkg/flow_module_core.js"),
			"@semio-tech/flow-module-math": resolve(root, "../../../flow/module/math/pkg/flow_module_math.js"),
			"@semio-tech/ui-react": resolve(root, "../../../ui/react/index.tsx"),
			"@semio-tech/puzzle-2d-react": resolve(root, "../../../puzzle/2d/react/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.ts", "fixture-slugs.ts"],
		passWithNoTests: false,
	},
});
