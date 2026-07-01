import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "sequence",
	extraAliases: [
		{ find: "@semio-tech/sequence-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/sequence-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@semio-tech/imperative-react", replacement: path.resolve(playDir, "../../imperative/react/index.tsx") },
		{ find: "@semio-tech/imperative-core", replacement: path.resolve(playDir, "../../imperative/core/index.ts") },
		{ find: "@semio-tech/sequence-core/pkg/sequence_core.js", replacement: path.resolve(playDir, "../core/pkg/sequence_core.js") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/sequence-react"],
	optimizeDeps: {
		include: ["react", "react-dom"],
		exclude: ["@semio-tech/framework-playground-renderer-react/sequence", "@semio-tech/sequence-react"],
		esbuildOptions: { target: "esnext" },
	},
	watchIgnored: ["../core/lib.rs", "../../imperative/**", "../core/target/**", "../core/pkg/**"],
});
