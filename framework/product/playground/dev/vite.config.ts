import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../..");
const playEntryKind = process.env.PUZZLE_PLAY_ENTRY ?? process.env.PLAYGROUND_APP ?? "draw";
const packageRoot = process.env.PLAYGROUND_PACKAGE_ROOT;
const extraAliases = packageRoot
	? [
			{ find: `@semio-tech/${packageRoot}-react`, replacement: path.resolve(repoRoot, packageRoot, "react/index.tsx") },
			{ find: `@semio-tech/${packageRoot}-core`, replacement: path.resolve(repoRoot, packageRoot, "core/index.ts") },
		]
	: [];

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind,
	extraAliases,
	resolveDedupe: ["react", "react-dom"],
});
