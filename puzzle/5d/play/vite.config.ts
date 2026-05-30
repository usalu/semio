// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@puzzle/5d/play`. */
// #endregion 🧲Header

// #region 🔌Adapters
import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../ui/styling/vite-elements-assets.ts";
// #endregion 🔌Adapters

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "5d",
	extraAliases: [
		{ find: "@puzzle/2d/react", replacement: path.resolve(repoRoot, "puzzle/2d/react/index.tsx") },
		{ find: "@puzzle/3d/react", replacement: path.resolve(repoRoot, "puzzle/3d/react/index.tsx") },
		{ find: "@puzzle/5d/react", replacement: path.resolve(playDir, "../react/index.tsx") },
	],
});
