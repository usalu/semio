import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "layout",
	extraAliases: [
		{ find: "@semio-tech/layout-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/layout-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@semio-tech/layout-rs", replacement: path.resolve(playDir, "../rs/pkg/layout_rs.js") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/layout-react"],
	optimizeDeps: {
		include: ["react", "react-dom"],
		exclude: ["@semio-tech/framework-playground-renderer-react/layout", "@semio-tech/layout-react"],
		esbuildOptions: { target: "esnext" },
	},
	watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/pkg/**"],
});
