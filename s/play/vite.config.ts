// #region 🧲Header
/** @emoji 🛝 Vite dev/build for `@semio-tech/s-play`. */
// #endregion 🧲Header

import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "s",
	extraAliases: [
		{ find: "@semio-tech/s-play", replacement: path.resolve(playDir, "./index.ts") },
		{ find: "@semio-tech/s-react", replacement: path.resolve(playDir, "../react/index.tsx") },
		{ find: "@semio-tech/s-core", replacement: path.resolve(playDir, "../core/index.ts") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/s-react"],
	optimizeDeps: {
		include: ["react", "react-dom"],
		exclude: [
			"@semio-tech/s-react",
			"@semio-tech/compose-js",
			"@semio-tech/compose-sketchpad",
			"@semio-tech/compose-rs-wasm",
			"playwright",
			"playwright-core",
			"@playwright/test",
			"chromium-bidi",
			"vitest",
			"@vitest/expect",
			"@testing-library/react",
		],
		holdUntilCrawlEnd: true,
		esbuildOptions: { target: "esnext" },
	},
});
