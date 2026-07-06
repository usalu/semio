#!/usr/bin/env bun
/** One-shot: legacy board/scene/topology play ids → puzzle 2d/3d/5d ids (no backwards compatibility). */
import { readdirSync, readFileSync, statSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../..");
const roots = [
	join(repoRoot, "puzzle"),
	join(repoRoot, "framework/playground"),
	join(repoRoot, ".storybook"),
];

const replacements: ReadonlyArray<[string, string]> = [
	["elements.topology.fixture/v1", "puzzle.5d.fixture/v1"],
	["elements.scene.fixture/v1", "puzzle.3d.fixture/v1"],
	["elements.board.fixture/v1", "puzzle.2d.fixture/v1"],
	["elements.topology.play.", "puzzle.5d.play."],
	["elements.scene.play.", "puzzle.3d.play."],
	["elements.board.play.", "puzzle.2d.play."],
	["elements.topology-play.", "puzzle.5d-play."],
	["elements.scene-play.", "puzzle.3d-play."],
	["elements.board-play.", "puzzle.2d-play."],
	["elements-topology-play", "puzzle-5d-play"],
	["elements-board-play", "puzzle-2d-play"],
	["elements.scene.placeholder", "puzzle.3d.placeholder"],
	["elements.topology/", "puzzle.5d/"],
	["elements.scene/", "puzzle.3d/"],
	["elements.board/", "puzzle.2d/"],
	["elements.board/v1", "puzzle.2d.surface/v1"],
	["TOPOLOGY_PLAY_", "PUZZLE_5D_PLAY_"],
	["SCENE_PLAY_", "PUZZLE_3D_PLAY_"],
	["BOARD_PLAY_", "PUZZLE_2D_PLAY_"],
	["topology-play-document", "puzzle-5d-play-document"],
	["topology-play", "puzzle-5d-play"],
	["scene-play-document", "puzzle-3d-play-document"],
	["scene-play-inspector", "puzzle-3d-play-inspector"],
	["scene-play-settings", "puzzle-3d-play-settings"],
	["scene-play-kinds", "puzzle-3d-play-kinds"],
	["scene-play", "puzzle-3d-play"],
	["board-play-document", "puzzle-2d-play-document"],
	["board-play-library", "puzzle-2d-play-library"],
	["board-play-inspector", "puzzle-2d-play-inspector"],
	["board-play-settings", "puzzle-2d-play-settings"],
	["board-play", "puzzle-2d-play"],
	["topology-scene", "puzzle-5d-3d"],
	["topology-board", "puzzle-5d-2d"],
	["scene-main", "puzzle-3d-main"],
	["board-selection", "2d-selection"],
	["board-detail", "2d-detail"],
	["board-overview", "2d-overview"],
	["BoardPlayPaneId", "Puzzle2dPlayPaneId"],
	["BoardPlayExtensionManifest", "Puzzle2dPlayExtensionManifest"],
	["BoardPlayground", "Playground2d"],
	["ScenePlayground", "Playground3d"],
	["TopologyPlayground", "Playground5d"],
	["ScenePlaySelection", "Puzzle3dPlaySelection"],
	["ScenePlay", "Puzzle3dPlay"],
	['= "Scene"', '= "Puzzle 3d"'],
	['= "Sketch board"', '= "Puzzle 2d"'],
	['= "Spatial scene"', '= "Puzzle 3d"'],
	["NAKAGIN_BOARD_PLAY_", "NAKAGIN_PUZZLE_2D_PLAY_"],
	["BOARD_PLAYWRIGHT_CHANNEL", "PUZZLE_2D_PLAYWRIGHT_CHANNEL"],
];

function walk(dir: string, out: string[]): void {
	for (const name of readdirSync(dir)) {
		if (name === "node_modules" || name === "dist" || name === "pkg") continue;
		const p = join(dir, name);
		const st = statSync(p);
		if (st.isDirectory()) walk(p, out);
		else if (/\.(ts|tsx|json)$/.test(name)) out.push(p);
	}
}

let files = 0;
let touched = 0;
for (const root of roots) {
	const paths: string[] = [];
	walk(root, paths);
	for (const file of paths) {
		files++;
		let text = readFileSync(file, "utf8");
		const before = text;
		for (const [from, to] of replacements) text = text.split(from).join(to);
		if (text !== before) {
			writeFileSync(file, text);
			touched++;
			console.log(file.replace(repoRoot + "\\", "").replace(repoRoot + "/", ""));
		}
	}
}
console.log(`[rename-play-ids] ${touched}/${files} files updated`);
