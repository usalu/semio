import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "../../../../");

export default defineConfig({
	root: __dirname,
	plugins: [tailwindcss(), react()],
	build: {
		target: "esnext",
	},
	resolve: {
		alias: {
			"@elements/framework": path.resolve(__dirname, "../../../framework/core/index.ts"),
			"@elements/framework-react": path.resolve(__dirname, "../../../framework/renderer/react/index.tsx"),
			"@elements/framework-react/workbench": path.resolve(__dirname, "../../../framework/renderer/react/index.tsx"),
			"@elements/playground": path.resolve(__dirname, "../../../playground/index.ts"),
			"@elements/spatial-js": path.resolve(__dirname, "../js/index.ts"),
			"@elements/spatial-react": path.resolve(__dirname, "../react/index.tsx"),
			"@elements/ui": path.resolve(__dirname, "../../core/index.tsx"),
			"@elements/styling/elements.css": path.resolve(__dirname, "../../../styling/js/elements.css"),
		},
	},
	server: {
		fs: {
			allow: [repoRoot],
		},
	},
});
