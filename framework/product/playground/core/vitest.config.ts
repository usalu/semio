// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@semio-tech/framework-playground-core`. */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@semio-tech/framework-core": resolve(root, "../../../core/index.ts"),
			"@semio-tech/framework-platform-core": resolve(root, "../../platform/core/index.ts"),
			"@semio-tech/ui-react": resolve(root, "../../../../ui/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
