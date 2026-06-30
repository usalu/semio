// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/forms-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "forms",
	extraAliases: [
		{ find: "@semio-tech/forms-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/forms-core", replacement: path.resolve(playDir, "../core/index.ts") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/forms-react"],
	optimizeDeps: {
		include: ["react", "react-dom", "@semio-tech/forms-react"],
		esbuildOptions: { target: "esnext" },
	},
});
