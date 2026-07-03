import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		environment: "node",
		root: path.dirname(fileURLToPath(import.meta.url)),
		include: ["index.tsx"],
		includeSource: ["index.tsx"],
		passWithNoTests: false,
	},
});
