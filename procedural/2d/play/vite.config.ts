// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/procedural-2d-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig, FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE } from "../../../ui/styling/vite-elements-assets.js";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "procedural-2d",
	extraAliases: [
		{ find: "@semio-tech/procedural-2d-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/geometry-drawing-js", replacement: path.resolve(repoRoot, "geometry/drawing/js/index.ts") },
		{ find: "@semio-tech/flow-react", replacement: path.resolve(repoRoot, "flow/react/index.tsx") },
		{ find: "@semio-tech/flow-module-draw", replacement: path.resolve(repoRoot, "flow/module/draw/pkg/flow_module_draw.js") },
	],
	resolveDedupe: ["react", "react-dom", "scheduler", "@semio-tech/flow-react", "@semio-tech/procedural-2d-react"],
	optimizeDeps: {
		include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@semio-tech/infinite-cavas-react-renderer"],
		exclude: [...FLOW_WASM_MODULE_OPTIMIZE_DEPS_EXCLUDE],
		esbuildOptions: { target: "esnext" },
	},
	watchIgnored: [
		"../../../flow/core/lib.rs",
		"../../../flow/core/target/**",
		"../../../flow/module/**/lib.rs",
		"../../../flow/module/**/target/**",
	],
});
