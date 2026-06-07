// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../../../../");
const reactRoot = resolve(repoRoot, "node_modules/react");
const reactDomRoot = resolve(repoRoot, "node_modules/react-dom");
const threeModule = resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = resolve(repoRoot, "node_modules/three");
const coreEntry = resolve(dir, "../../core/index.ts");
const kernelEntry = resolve(dir, "../../kernel/brepjs/index.ts");
const machineStatelyEntry = resolve(dir, "../../machine/stately/index.ts");
const queryEntry = resolve(dir, "../../query/index.ts");
const runtimeEntry = resolve(dir, "../../runtime/index.ts");
const spatialShapeModuleEntry = resolve(dir, "../../module/spatial-shape/index.ts");
const aecBuildingModuleEntry = resolve(dir, "../../module/aec-building/index.ts");
const aecBuildingEnergyModuleEntry = resolve(dir, "../../module/aec-building-energy/index.ts");
const aecBuildingStructureModuleEntry = resolve(dir, "../../module/aec-building-structure/index.ts");

export default createPlaygroundPlayViteConfig({
	playDir: dir,
	repoRoot,
	extraAliases: [
		{ find: "@cad/js/core", replacement: coreEntry },
		{ find: "@cad/js/kernel/brepjs", replacement: kernelEntry },
		{ find: "@cad/js/machine/stately", replacement: machineStatelyEntry },
		{ find: "@cad/js/query", replacement: queryEntry },
		{ find: "@cad/js/runtime", replacement: runtimeEntry },
		{ find: "@cad/js/module/spatial-shape", replacement: spatialShapeModuleEntry },
		{ find: "@cad/js/module/aec-building", replacement: aecBuildingModuleEntry },
		{ find: "@cad/js/module/aec-building-energy", replacement: aecBuildingEnergyModuleEntry },
		{ find: "@cad/js/module/aec-building-structure", replacement: aecBuildingStructureModuleEntry },
		{ find: /^react$/, replacement: resolve(reactRoot, "index.js") },
		{ find: /^react\/jsx-runtime$/, replacement: resolve(reactRoot, "jsx-runtime.js") },
		{ find: /^react\/jsx-dev-runtime$/, replacement: resolve(reactRoot, "jsx-dev-runtime.js") },
		{ find: /^react-dom$/, replacement: resolve(reactDomRoot, "index.js") },
		{ find: /^react-dom\/client$/, replacement: resolve(reactDomRoot, "client.js") },
		{ find: /^three$/, replacement: threeModule },
		{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
	],
	resolveDedupe: ["react", "react-dom", "three", "scheduler"],
	optimizeDeps: {
		include: [
			"react",
			"react-dom",
			"react/jsx-runtime",
			"react/jsx-dev-runtime",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
			"@infinite/world/r3f",
			"brepjs",
			"brepjs-opencascade",
			"golden-layout",
			"lucide-react",
			"chevrotain",
		],
		esbuildOptions: {
			target: "esnext",
		},
	},
});
