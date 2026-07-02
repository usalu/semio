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
			"@semio-tech/forms-core": resolve(root, "../core/index.ts"),
			"@semio-tech/ui-react": resolve(root, "../../ui/react/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
