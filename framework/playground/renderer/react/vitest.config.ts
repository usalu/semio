import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

/** @emoji 🧪 Vitest for `@framework/playground-react`. */
export default defineConfig({
	root,
	resolve: {
		alias: {
			"@framework/playground": resolve(root, "../../core/index.ts"),
			"@framework/playground-react": resolve(root, "index.tsx"),
			"@ui/react": resolve(root, "../../../../ui/react/index.tsx"),
		},
	},
	test: {
		environment: "jsdom",
		include: ["index.tsx"],
		passWithNoTests: true,
	},
});
