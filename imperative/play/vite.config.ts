import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "imperative",
	extraAliases: [
		{ find: "@semio-tech/imperative-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/imperative-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@semio-tech/imperative-core/pkg/imperative_core.js", replacement: path.resolve(playDir, "../core/pkg/imperative_core.js") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/imperative-react"],
	optimizeDeps: {
		include: ["react", "react-dom"],
		exclude: ["@semio-tech/framework-playground-renderer-react/imperative", "@semio-tech/imperative-react"],
		esbuildOptions: { target: "esnext" },
	},
	watchIgnored: ["../core/lib.rs", "../engine/**", "../module/**", "../core/target/**", "../core/pkg/**"],
});
