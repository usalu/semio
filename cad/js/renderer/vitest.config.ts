// #region 🔌Adapters
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vitest/config";
// #endregion 🔌Adapters

const root = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(root, "../../..");
const reactRoot = resolve(repoRoot, "node_modules/react");
const reactDomRoot = resolve(repoRoot, "node_modules/react-dom");
const threeModule = resolve(repoRoot, "node_modules/three/build/three.module.js");
const threePackageRoot = resolve(repoRoot, "node_modules/three");

export default defineConfig({
	root,
	assetsInclude: ["**/*.wasm"],
	server: {
		fs: {
			allow: [repoRoot],
		},
	},
	resolve: {
		alias: [
			{ find: "@framework/playground/renderer/react", replacement: resolve(root, "../../../framework/playground/renderer/react/index.tsx") },
			{ find: "@framework/playground", replacement: resolve(root, "../../../framework/playground/core/core.ts") },
			{ find: /^@framework\/playground\/(.*)$/, replacement: `${resolve(root, "../../../framework/playground/core")}/$1` },
			{ find: "@ui/react", replacement: resolve(root, "../../../ui/react/index.tsx") },
			{ find: "@cad/js/core", replacement: resolve(root, "../core/index.ts") },
			{ find: "@cad/js/kernel/brepjs", replacement: resolve(root, "../kernel/brepjs/index.ts") },
			{ find: "@cad/js/machine/stately", replacement: resolve(root, "../machine/stately/index.ts") },
			{ find: "@cad/js/query", replacement: resolve(root, "../query/index.ts") },
			{ find: /^react$/, replacement: resolve(reactRoot, "index.js") },
			{ find: /^react\/jsx-runtime$/, replacement: resolve(reactRoot, "jsx-runtime.js") },
			{ find: /^react\/jsx-dev-runtime$/, replacement: resolve(reactRoot, "jsx-dev-runtime.js") },
			{ find: /^react-dom$/, replacement: resolve(reactDomRoot, "index.js") },
			{ find: /^react-dom\/client$/, replacement: resolve(reactDomRoot, "client.js") },
			{ find: /^three$/, replacement: threeModule },
			{ find: /^three\/addons\/(.*)$/, replacement: `${threePackageRoot}/examples/jsm/$1` },
		],
	},
	test: {
		mode: "test",
		environment: "jsdom",
		testTimeout: 120_000,
		fileParallelism: false,
		maxConcurrency: 1,
		include: ["index.tsx", "play/main.tsx"],
	},
});
