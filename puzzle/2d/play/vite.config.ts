// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@puzzle/2d/play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = path.resolve(repoRoot, "node_modules/three");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "2d",
	extraAliases: [
		{ find: "@puzzle/2d/react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: /^three$/, replacement: threeModule },
		{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/example/jsm/$1` },
	],
	resolveDedupe: ["react", "react-dom", "three", "@puzzle/2d/react"],
	optimizeDeps: {
		include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "three", "@react-three/fiber", "@react-three/drei", "lucide-react", "@infinite/cavas/react-renderer", "@puzzle/2d/react"],
		esbuildOptions: { target: "esnext" },
	},
	// Rebuild wasm writes to `../rs/pkg` — do not ignore pkg or play keeps stale edge rendering after `bun ./script.ts wasm`.
	watchIgnored: ["../rs/lib.rs", "../rs/target/**", "../rs/Cargo.toml", "../rs/Cargo.lock", "../rs/script.ts"],
});
