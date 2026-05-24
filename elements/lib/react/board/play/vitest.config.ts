import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for board play framework wiring (`play/index.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/framework": resolve(root, "../../../framework/core/index.ts"),
			"@elements/playground": resolve(root, "../../../playground/index.ts"),
			"@elements/ui": resolve(root, "../../core/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
