import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	resolve: {
		alias: {
			"@semio-tech/framework-playground-core": resolve(root, "../../framework/product/playground/core/index.ts"),
			"@semio-tech/framework-platform-core": resolve(root, "../../framework/product/platform/core/index.ts"),
			"@semio-tech/shooting-react": resolve(root, "../react/index.tsx"),
			"@semio-tech/ui-react": resolve(root, "../../ui/react/index.tsx"),
			"@semio-tech/ui-styling": resolve(root, "../../ui/styling/js/index.ts"),
			"@semio-tech/infinite-world-r3f": resolve(root, "../../infinite/world/r3f/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.ts", "fixture-slugs.ts"],
		passWithNoTests: false,
	},
});
