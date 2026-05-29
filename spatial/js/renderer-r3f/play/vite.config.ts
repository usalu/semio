import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const dir = dirname(fileURLToPath(import.meta.url));
const coreEntry = resolve(dir, "../../core/index.ts");
const kernelEntry = resolve(dir, "../../kernel-brepjs/index.ts");
const machineStatelyEntry = resolve(dir, "../../machine-stately/index.ts");
const queryEntry = resolve(dir, "../../query/index.ts");

export default defineConfig({
	root: dir,
	publicDir: false,
	assetsInclude: ["**/*.wasm"],
	worker: { format: "es" },
	plugins: [react()],
	resolve: {
		alias: [
			{ find: "@elements/playground/react", replacement: resolve(dir, "../../../elements/lib/playground/react/index.tsx") },
			{ find: "@elements/playground", replacement: resolve(dir, "../../../elements/lib/playground/index.ts") },
			{ find: "@elements/ui", replacement: resolve(dir, "../../../elements/lib/react/core/index.tsx") },
			{ find: "@spatial/js-core", replacement: coreEntry },
			{ find: "@spatial/js-kernel-brepjs", replacement: kernelEntry },
			{ find: "@spatial/js-machine-stately", replacement: machineStatelyEntry },
			{ find: "@spatial/js-query", replacement: queryEntry },
		],
	},
});
