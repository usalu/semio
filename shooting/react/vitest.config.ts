import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";

const root = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root,
	resolve: {
		alias: {
			"@semio-tech/ui-react": resolve(root, "../../ui/react/index.tsx"),
			"@semio-tech/ui-styling": resolve(root, "../../ui/styling/js/index.ts"),
			"@semio-tech/ui-styling": resolve(root, "../../ui/styling/js/index.ts"),
			"@semio-tech/infinite-world-r3f": resolve(root, "../../infinite/world/r3f/index.tsx"),
		},
	},
	test: {
		mode: "test",
		environment: "node",
		include: ["index.tsx"],
		passWithNoTests: false,
	},
});
