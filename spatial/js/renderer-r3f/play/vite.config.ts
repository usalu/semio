import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const dir = dirname(fileURLToPath(import.meta.url));

export default defineConfig({
	root: dir,
	publicDir: false,
	plugins: [react()],
	resolve: {
		alias: {
			"@spatial/js-core": resolve(dir, "../core/index.ts"),
			"@spatial/js-kernel-brepjs": resolve(dir, "../kernel-brepjs/index.ts"),
		},
	},
});
