// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	resolve: {
		alias: [
			{ find: "@semio-tech/geometry-brep-js", replacement: resolve(root, "../../../geometry/brep/js/index.ts") },
			{ find: "@semio-tech/flow-core", replacement: resolve(root, "../../../flow/core/pkg/flow_core.js") },
			{ find: "@semio-tech/flow-react", replacement: resolve(root, "../../../flow/react/index.tsx") },
			{ find: "@semio-tech/flow-module-brep", replacement: resolve(root, "../../../flow/module/brep/pkg/flow_module_brep.js") },
			{ find: "@semio-tech/flow-module-bim", replacement: resolve(root, "../../../flow/module/bim/pkg/flow_module_bim.js") },
			{ find: "@semio-tech/flow-module-core", replacement: resolve(root, "../../../flow/module/core/pkg/flow_module_core.js") },
			{ find: "@semio-tech/flow-module-math", replacement: resolve(root, "../../../flow/module/math/pkg/flow_module_math.js") },
			{ find: "@semio-tech/flow-module-text", replacement: resolve(root, "../../../flow/module/text/pkg/flow_module_text.js") },
			{ find: "@semio-tech/flow-module-logic", replacement: resolve(root, "../../../flow/module/logic/pkg/flow_module_logic.js") },
			{ find: "@semio-tech/flow-module-dictionary", replacement: resolve(root, "../../../flow/module/dictionary/pkg/flow_module_dictionary.js") },
			{ find: "@semio-tech/flow-module-list", replacement: resolve(root, "../../../flow/module/list/pkg/flow_module_list.js") },
			{ find: "@semio-tech/flow-module-draw", replacement: resolve(root, "../../../flow/module/draw/pkg/flow_module_draw.js") },
			{ find: "@semio-tech/infinite-world-r3f", replacement: resolve(root, "../../../infinite/world/r3f/index.tsx") },
			{ find: "@semio-tech/puzzle-3d-react", replacement: resolve(root, "../../../puzzle/3d/react/index.tsx") },
			{ find: "@semio-tech/ui-react", replacement: resolve(root, "../../../ui/react/index.tsx") },
			{ find: "@semio-tech/ui-styling", replacement: resolve(root, "../../../ui/styling/js/index.ts") },
		],
	},
	test: {
		mode: "test",
		environment: "jsdom",
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx"],
		passWithNoTests: true,
	},
});
