import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "lowpoly",
	extraAliases: [
		{ find: "@semio-tech/lowpoly-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/lowpoly-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@semio-tech/lowpoly-core/pkg/lowpoly_core.js", replacement: path.resolve(playDir, "../core/pkg/lowpoly_core.js") },
		{ find: /^three$/, replacement: threeModule },
	],
	resolveDedupe: ["react", "react-dom", "three", "scheduler", "@semio-tech/lowpoly-react"],
	optimizeDeps: {
		include: ["react", "react-dom", "three", "@react-three/fiber", "@react-three/drei"],
		exclude: ["@semio-tech/framework-playground-renderer-react/lowpoly", "@semio-tech/lowpoly-react"],
		esbuildOptions: { target: "esnext" },
	},
	watchIgnored: ["../core/lib.rs", "../core/target/**", "../core/pkg/**"],
});
