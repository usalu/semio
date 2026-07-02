import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../ui/styling/vite-elements-assets.ts";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../..");

export default createPlaygroundPlayViteConfig({
	playDir,
	repoRoot,
	playEntryKind: "s",
	extraAliases: [
		{ find: "@semio-tech/s-react", replacement: path.resolve(repoRoot, "s/react/index.tsx") },
		{ find: "@semio-tech/s-core", replacement: path.resolve(repoRoot, "s/core/index.ts") },
	],
	resolveDedupe: ["react", "react-dom", "@semio-tech/s-react"],
});
