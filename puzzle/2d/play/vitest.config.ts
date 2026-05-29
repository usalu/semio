import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for board play playground wiring (`play/index.ts`). */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/playground-renderer-react": resolve(root, "../../../framework/playground/renderer/react/index.tsx"),
			"@framework/playground": resolve(root, "../../../framework/playground/core/index.ts"),
			"@ui/react": resolve(root, "../../../ui/react/index.tsx"),
		},
	},
	test: {
		environment: "node",
		include: ["index.ts"],
		passWithNoTests: false,
	},
});
