import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for board play playground wiring (`play/index.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@elements/playground/react": resolve(root, "../../playground/react/index.tsx"),
			"@elements/playground": resolve(root, "../../playground/index.ts"),
			"@elements/ui": resolve(root, "../../react/core/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
