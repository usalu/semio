import path from "node:path";
import { fileURLToPath } from "node:url";
import { createPlaygroundPlayViteConfig } from "../../../../ui/styling/vite-elements-assets.ts";
import { playgroundAppByEntryKind } from "@semio-tech/framework-playground-core/app-registry";

const playDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(playDir, "../../../..");
const playEntryKind = process.env.PLAYGROUND_APP ?? process.env.PUZZLE_PLAY_ENTRY ?? "draw";
const app = playgroundAppByEntryKind(playEntryKind);
if (!app?.devHost) throw new Error(`[playground-dev] unknown app: ${playEntryKind}`);

const devHost = app.devHost;
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
	playEntryKind: devHost.playEntryKind,
	extraAliases,
	resolveDedupe: devHost.resolveDedupe ? [...devHost.resolveDedupe] : ["react", "react-dom"],
	optimizeDeps: devHost.optimizeDeps,
	watchIgnored: devHost.watchIgnored ? [...devHost.watchIgnored] : [],
});
