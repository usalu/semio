// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@framework/presentation/play`. */
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
		{ find: "@framework/presentation/core", replacement: path.resolve(playDir, "../core/index.ts") },
		{ find: "@framework/presentation/renderer/react", replacement: path.resolve(playDir, "../renderer/react/index.tsx") },
	],
	resolveDedupe: ["react", "react-dom", "@framework/presentation/core"],
	optimizeDeps: {
		include: ["react", "react-dom", "react/jsx-runtime", "react/jsx-dev-runtime", "@framework/presentation/core"],
		esbuildOptions: { target: "esnext" },
	},
});
