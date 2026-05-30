// #region 🔌Adapters
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

export default defineConfig({
	test: {
		include: ["index.ts"],
	},
});
