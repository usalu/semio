import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@elements/framework-react` declarative renderer. */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/framework": resolve(root, "../../core/index.ts"),
			"@elements/ui": resolve(root, "../../../react/core/index.tsx"),
		},
	},
	test: {
		environment: "jsdom",
		include: ["ui-declarative-renderer.tsx"],
		passWithNoTests: false,
	},
});
