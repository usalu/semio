// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/framework-presentation-play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "presentation",
	extraAliases: [
		{ find: "@semio-tech/framework-presentation-core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@semio-tech/framework-presentation-renderer-react", replacement: path.resolve(playDir, "../renderer/react/index.tsx") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/framework-presentation-core"],
	optimizeDeps: {
		include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@semio-tech/framework-presentation-core"],
		esbuildOptions: { target: "esnext" },
	},
});
