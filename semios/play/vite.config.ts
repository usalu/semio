// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/semios-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "semios",
	extraAliases: [
		{ find: "@semio-tech/semios-play", replacement: path.resolve(playDir, "./index.ts") },
		{ find: "@semio-tech/semios-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/semios-core", replacement: path.resolve(playDir, "../core/index.ts") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/semios-react"],
	optimizeDeps: {
		include: ["react", "react-dom", "@semio-tech/semios-react"],
		esbuildOptions: { target: "esnext" },
	},
});
