import { resolve } from "node:path";
import { defineConfig } from "vitest/config";
const sp = "/home/user/semio/semio/client/lib/sketchpad/js";
export default defineConfig({
	define: { __SEMIO_SKETCHPAD_RUN_EMBEDDED_TESTS__: "false" },
	root: sp,
	resolve: {
		alias: {
			"@framework/core": resolve(sp, "../../../../../framework/core/index.ts"),
			"@framework/platform/core": resolve(sp, "../../../../../framework/product/platform/core/index.ts"),
			"@semio/js": resolve(sp, "../../js/index.ts"),
			"@semio/react": resolve(sp, "../../react/index.ts"),
			"@reasoning/mindmap/wires/react": resolve(sp, "../../../../../reasoning/mindmap/wires/react/index.ts"),
			"@reasoning/mindmap/react": resolve(sp, "../../../../../reasoning/mindmap/react/index.tsx"),
			"@infinite/cavas/react-renderer": resolve(sp, "../../../../../infinite/cavas/react-renderer/index.tsx"),
			"@infinite/world/r3f": resolve(sp, "../../../../../infinite/world/r3f/index.tsx"),
			"@puzzle/2d/react": resolve(sp, "../../../../../puzzle/2d/react/index.tsx"),
		},
	},
	test: { environment: "node", include: [process.env.F2_DEBUG_TEST!], root: process.env.F2_DEBUG_ROOT ?? "/" },
});
