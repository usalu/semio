import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "../../../../../");

export default defineConfig({
	root: __dirname,
	plugins: [tailwindcss(), react()],
	build: {
		target: "esnext",
	},
	resolve: {
		alias: {
			"@elements/ui": path.resolve(__dirname, "../../react/index.tsx"),
		},
	},
	server: {
		fs: {
			allow: [repoRoot],
		},
	},
});