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
			"@flow/module-brep": resolve(root, "../../flow/modules/brep/pkg/flow_module_brep.js"),
			"@flow/module-bim": resolve(root, "../../flow/modules/bim/pkg/flow_module_bim.js"),
			"@flow/module-core": resolve(root, "../../flow/modules/core/pkg/flow_module_core.js"),
			"@flow/module-math": resolve(root, "../../flow/modules/math/pkg/flow_module_math.js"),
			"@ui/react": resolve(root, "../../ui/react/index.tsx"),
			"@infinite/world/r3f": resolve(root, "../../infinite/world/r3f/index.tsx"),
			"@puzzle/3d/react": resolve(root, "../../puzzle/3d/react/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
