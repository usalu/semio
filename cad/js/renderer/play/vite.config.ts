import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";
import { elementsAssetsVitePlugin } from "../../../../ui/styling/vite-elements-assets.ts";

const dir = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(dir, "../../../../");
const elementsAssetsRoot = resolve(repoRoot, "ui/assets");
const jsRoot = resolve(dir, "../..");
const reactRoot = resolve(jsRoot, "node_modules/react");
const reactDomRoot = resolve(jsRoot, "node_modules/react-dom");
const threeModule = resolve(repoRoot, "ui/react/node_modules/three/build/three.module.js");
const threePackageRoot = resolve(repoRoot, "ui/react/node_modules/three");
const coreEntry = resolve(dir, "../../core/index.ts");
const kernelEntry = resolve(dir, "../../kernel-brepjs/index.ts");
const machineStatelyEntry = resolve(dir, "../../machine-stately/index.ts");
const queryEntry = resolve(dir, "../../query/index.ts");

export default defineConfig({
	root: dir,
	publicDir: false,
	assetsInclude: ["**/*.wasm"],
	worker: { format: "es" },
	plugins: [elementsAssetsVitePlugin(elementsAssetsRoot), tailwindcss(), react()],
	server: {
		fs: {
			allow: [repoRoot],
		},
	},
	build: {
		target: "esnext",
	},
	resolve: {
		alias: [
			{ find: "@framework/playground-renderer-react", replacement: resolve(dir, "../../../../framework/playground/renderer/react/index.tsx") },
			{ find: "@framework/playground", replacement: resolve(dir, "../../../../framework/playground/core/index.ts") },
			{ find: "@ui/react", replacement: resolve(dir, "../../../../ui/react/index.tsx") },
			{ find: "@cad/js-core", replacement: coreEntry },
			{ find: "@cad/js-kernel-brepjs", replacement: kernelEntry },
			{ find: "@cad/js-machine-stately", replacement: machineStatelyEntry },
			{ find: "@cad/js-query", replacement: queryEntry },
			{ find: /^react$/, replacement: resolve(reactRoot, "index.js") },
			{ find: /^react\/jsx-runtime$/, replacement: resolve(reactRoot, "jsx-runtime.js") },
			{ find: /^react\/jsx-dev-runtime$/, replacement: resolve(reactRoot, "jsx-dev-runtime.js") },
			{ find: /^react-dom$/, replacement: resolve(reactDomRoot, "index.js") },
			{ find: /^react-dom\/client$/, replacement: resolve(reactDomRoot, "client.js") },
			{ find: /^three$/, replacement: threeModule },
			{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
		],
		dedupe: ["react", "react-dom", "three", "scheduler"],
	},
	optimizeDeps: {
		include: [
			"react",
			"react-dom",
			"react/jsx-runtime",
			"react/jsx-dev-runtime",
			"three",
			"@react-three/fiber",
			"@react-three/drei",
			"golden-layout",
			"lucide-react",
		],
		esbuildOptions: {
			target: "esnext",
		},
	},
});
