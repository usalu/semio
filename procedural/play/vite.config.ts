// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@procedural/play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig, FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");
const threeModule = path.resolve(repoRoot, "node_modules/three/build/three.module.js");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "procedural",
	extraAliases: [
		{ find: "@procedural/react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@geometry/brep/js", replacement: path.resolve(repoRoot, "geometry/brep/js/index.ts") },
		{ find: "@flow/react", replacement: path.resolve(repoRoot, "flow/react/index.tsx") },
		{ find: "@flow/module-brep", replacement: path.resolve(repoRoot, "flow/modules/brep/pkg/flow_module_brep.js") },
		{ find: /^three$/, replacement: threeModule },
	],
	resolveDedupe: ["react", "react-dom", "three", "scheduler", "@flow/react", "@procedural/react"],
	optimizeDeps: {
		include: [
			"react",
			"react-dom",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
			"@infinite/world/r3f",
			"@flow/react",
			"@procedural/react",
		],
		exclude: [...FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE],
		esbuildOptions: { target: "esnext" },
	},
	watchIgnored: [
		"../../flow/core/lib.rs",
		"../../flow/core/target/**",
		"../../flow/modules/**/lib.rs",
		"../../flow/modules/**/target/**",
	],
});
