#!/usr/bin/env bun
import { existsSync, mkdirSync, readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";

const root = "c:/git/semio";
const rendererPuzzle = join(root, "framework/playground/renderer/react/puzzle");
mkdirSync(rendererPuzzle, { recursive: true });

const moves: { from: string; to: string; playImport: string; reactImport: string; fixturePrefix: string }[] = [
	{
		from: "puzzle/2d/play/host.tsx",
		to: "framework/playground/renderer/react/puzzle/board-play-host.tsx",
		playImport: "../../../../../puzzle/2d/play/index.ts",
		reactImport: "@puzzle/2d-react",
		fixturePrefix: "../../../../../puzzle/2d/play/fixtures/",
	},
	{
		from: "puzzle/3d/play/host.tsx",
		to: "framework/playground/renderer/react/puzzle/scene-play-host.tsx",
		playImport: "../../../../../puzzle/3d/play/index.ts",
		reactImport: "@puzzle/3d-react",
		fixturePrefix: "../../../../../puzzle/3d/play/fixtures/",
	},
	{
		from: "puzzle/5d/play/host.tsx",
		to: "framework/playground/renderer/react/puzzle/topology-play-host.tsx",
		playImport: "../../../../../puzzle/5d/play/index.ts",
		reactImport: "@puzzle/5d-react",
		fixturePrefix: "../../../../../puzzle/5d/play/fixtures/",
	},
];

for (const { from, to, playImport, reactImport, fixturePrefix } of moves) {
	const src = join(root, from);
	if (!existsSync(src)) {
		console.warn("[skip] missing", from);
		continue;
	}
	let c = readFileSync(src, "utf8");
	c = c.replace(/from "\.\/index\.ts"/g, `from "${playImport}"`);
	c = c.replace(/from "\.\.\/react\/index\.tsx"/g, `from "${reactImport}"`);
	c = c.replace(/import \* as Board from "@puzzle\/2d-react";\n\n/, "");
	c = c.replace(/import \* as Board from "\.\.\/react\/index\.tsx";\n\n/, "");
	c = c.replace(/from "\.\/fixtures\//g, `from "${fixturePrefix}`);
	c = c.replace(/import "\.\/globals\.css";\n?/g, "");
	c = c.replace(/\nsetBoardPlaySurfaceHostRegistrar\([^)]+\);\n/g, "\n");
	c = c.replace(/\nsetScenePlaySurfaceHostRegistrar\([^)]+\);\n/g, "\n");
	c = c.replace(/,\n\tsetBoardPlaySurfaceHostRegistrar,/g, ",");
	c = c.replace(/,\n\tsetScenePlaySurfaceHostRegistrar,/g, ",");
	c = c.replace(/\n\/\*\* @emoji 🚀 Vite host entry[\s\S]*?^export function mount\w+Play[\s\S]*?\n\}/gm, "");
	c = c.replace(/\nexport function create\w+PlayElement[\s\S]*?\n\}/gm, "");
	c = c.replace(
		/from "@framework\/playground-renderer-react"/g,
		'from "../index.tsx"',
	);
	c = c.replace(
		/^\/\/ #region 🧲Header[\s\S]*?\/\/ #endregion 🧲Header\n\n/,
		`// #region 🧲Header\n/** @emoji 🛝 Puzzle play React chrome in \`@framework/playground/core-renderer-react\` (not in play packages). */\n// #endregion 🧲Header\n\n`,
	);
	writeFileSync(join(root, to), c);
	unlinkSync(src);
	console.log("[move]", from, "->", to);
}
