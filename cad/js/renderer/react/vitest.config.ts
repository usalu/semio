// #region 🔌Adapters
import { dirname } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	test: {
		mode: "test",
		environment: "node",
		include: ["index.tsx"],
		passWithNoTests: true,
	},
});
