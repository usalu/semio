import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../../ui/styling/vite-elements-assets.ts";

const configDir = path.dirname(fileURLToPath(import.meta.url));
const playDir = path.resolve(configDir, "..");
const repoRoot = path.resolve(playDir, "../../../..");
const playEntryKind = process.env.PLAYGROUND_APP_KIND ?? process.env.PLAYGROUND_APP ?? "draw";
const packageRoot = process.env.PLAYGROUND_PACKAGE_ROOT;
const extraAliases = [
	{ find: /^three$/, replacement: path.resolve(repoRoot, "node_modules/three/build/three.module.js") },
	{ find: /^three\/addons\/(.*)$/, replacement: `${path.resolve(repoRoot, "node_modules/three/examples/jsm")}/$1` },
	{ find: /^three\/examples\/jsm\/(.*)$/, replacement: `${path.resolve(repoRoot, "node_modules/three/examples/jsm")}/$1` },
	...(packageRoot
		? [
				{ find: `@semio-tech/${packageRoot}-react`, replacement: path.resolve(repoRoot, packageRoot, "react/index.tsx") },
				{ find: `@semio-tech/${packageRoot}-core`, replacement: path.resolve(repoRoot, packageRoot, "core/js/index.ts") },
			]
		: []),
	{
		find: "@semio-tech/framework-playground-core/app-registry",
		replacement: path.resolve(repoRoot, "framework/product/playground/core/js/app-registry.ts"),
	},
];

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind,
	extraAliases,
	resolveDedupe: ["react", "react-dom"],
});
