import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const dir = dirname(fileURLToPath(import.meta.url));
const coreEntry = resolve(dir, "../../core/index.ts");
const kernelEntry = resolve(dir, "../../kernel-brepjs/index.ts");
const machineStatelyEntry = resolve(dir, "../../machine-stately/index.ts");

export default defineConfig({
	root: dir,
	publicDir: false,
	plugins: [react()],
	resolve: {
		alias: {
			"@spatial/js-core": coreEntry,
			"@spatial/js-kernel-brepjs": kernelEntry,
			"@spatial/js-machine-stately": machineStatelyEntry,
		},
	},
});
