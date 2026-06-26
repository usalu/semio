// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/procedural-3d-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig, FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE } from "../../../ui/styling/vite-elements-assets.js";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "procedural-3d",
	extraAliases: [
		{ find: "@semio-tech/procedural-3d-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/geometry-brep-js", replacement: path.resolve(repoRoot, "geometry/brep/js/index.ts") },
		{ find: "@semio-tech/flow-react", replacement: path.resolve(repoRoot, "flow/react/index.tsx") },
		{ find: "@semio-tech/flow-module-brep", replacement: path.resolve(repoRoot, "flow/module/brep/pkg/flow_module_brep.js") },
		{ find: "@semio-tech/flow-module-draw", replacement: path.resolve(repoRoot, "flow/module/draw/pkg/flow_module_draw.js") },
		{ find: "@semio-tech/flow-module-bim", replacement: path.resolve(repoRoot, "flow/module/bim/pkg/flow_module_bim.js") },
		{ find: /^three$/, replacement: threeModule },
	],
	resolveDedupe: ["react", "react-dom", "three", "scheduler", "@semio-tech/flow-react", "@semio-tech/procedural-3d-react"],
	optimizeDeps: {
		include: [
			"react",
			"react-dom",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
			"@semio-tech/infinite-world-r3f",
			"@semio-tech/flow-react",
			"@semio-tech/procedural-3d-react",
		],
		exclude: [...FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE],
		esbuildOptions: { target: "esnext" },
	},
	watchIgnored: [
		"../../flow/core/lib.rs",
		"../../flow/core/target/**",
		"../../flow/module/**/lib.rs",
		"../../flow/module/**/target/**",
	],
});
