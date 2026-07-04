import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "..");
const repoRoot = path.resolve(playDir, "../../../..");
const renderer = process.env.SEMIO_RENDERER ?? "react";

export default defineConfig({
	root: playDir,
	publicDir: path.join(playDir, "public"),
	resolve: {
		alias: {
			"@semio-tech/framework-renderer-react": path.resolve(repoRoot, "framework/renderer/react/index.tsx"),
			"@semio-tech/framework-renderer-wgpu": path.resolve(repoRoot, "framework/renderer/wgpu/index.ts"),
			"@semio-tech/framework-core": path.resolve(repoRoot, "framework/core/js/index.ts"),
		},
	},
	server: {
		port: Number(process.env.S_OS_PORT ?? 6066),
		strictPort: true,
		fs: { allow: [repoRoot] },
	},
	plugins: renderer === "wgpu" ? [tailwindcss()] : [react(), tailwindcss()],
	optimizeDeps: {
		exclude: renderer === "wgpu" ? ["@semio-tech/framework-renderer-react"] : [],
	},
	define: {
		"import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? "s"),
		"import.meta.env.VITE_SEMIO_RENDERER": JSON.stringify(renderer),
	},
});
