// #region 🧲Header
/** @emoji 🗄️ Vite dev/build for `@semio-tech/vcs-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "vcs",
	extraAliases: [
		{ find: "@semio-tech/vcs-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/vcs-core", replacement: path.resolve(playDir, "../core/index.ts") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/ui-react", "@semio-tech/vcs-react"],
	optimizeDeps: {
		include: ["react", "react-dom"],
		esbuildOptions: { target: "esnext" },
	},
});
