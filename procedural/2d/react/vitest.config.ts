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
			"@semio-tech/flow-react": resolve(root, "../../../flow/react/index.tsx"),
			"@semio-tech/flow-core": resolve(root, "../../../flow/core/pkg/flow_core.js"),
			"@semio-tech/kernel-2d-js": resolve(root, "../../../kernel/2d/js/index.ts"),
			"@semio-tech/infinite-cavas-react-renderer": resolve(root, "../../../infinite/cavas/react-renderer/index.tsx"),
			"@semio-tech/ui-react": resolve(root, "../../../ui/react/index.tsx"),
			"@semio-tech/ui-styling": resolve(root, "../../../ui/styling/js/index.ts"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
