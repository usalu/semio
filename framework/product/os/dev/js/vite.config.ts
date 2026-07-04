import path from "node:path";
import { fileURLToPath } from "node:url";
import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { defineConfig } from "vite";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "..");
const repoRoot = path.resolve(playDir, "../../../..");

export default defineConfig({
	root: playDir,
	publicDir: path.join(playDir, "public"),
	resolve: {
		alias: {
			"@semio-tech/framework-renderer-react": path.resolve(repoRoot, "framework/renderer/react/index.tsx"),
		},
	},
	server: {
		port: Number(process.env.S_OS_PORT ?? 6066),
		strictPort: true,
		fs: { allow: [repoRoot] },
	},
	plugins: [react(), tailwindcss()],
	define: {
		"import.meta.env.VITE_SEMIO_PLUGIN": JSON.stringify(process.env.SEMIO_PLUGIN ?? "s"),
	},
});
