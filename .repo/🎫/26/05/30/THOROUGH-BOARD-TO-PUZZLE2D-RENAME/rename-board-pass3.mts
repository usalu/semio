#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const repoRoot = resolve(import.meta.dir, "../../../../../..");

const replacements: [string, string][] = [
  ["BoardHostWire", "Puzzle2dHostWire"],
  ["BoardHostEdge", "Puzzle2dHostEdge"],
  ["BoardHostHandle", "Puzzle2dHostHandle"],
  ["BoardHostNode", "Puzzle2dHostTreeNode"],
  ["BoardHostInstance", "Puzzle2dHostInstance"],
  ["BoardHostMount", "Puzzle2dHostMount"],
  ["BoardHostSubtree", "Puzzle2dHostSubtree"],
  ["BoardFixtureSceneMarkersOptions", "Puzzle2dFixtureSceneMarkersOptions"],
  ["BoardNodeRectangleProps", "Puzzle2dNodeRectangleProps"],
  ["BoardNodeCircleProps", "Puzzle2dNodeCircleProps"],
  ["BoardNodeProps", "Puzzle2dNodeProps"],
  ["createBoardHostMount", "createPuzzle2dHostMount"],
  ["updateBoardHostMount", "updatePuzzle2dHostMount"],
  ["unmountBoardHostMount", "unmountPuzzle2dHostMount"],
  ["BOARD_HOST_MOUNT_DEFAULTS", "PUZZLE_2D_HOST_MOUNT_DEFAULTS"],
  ["reportBoardHostUncaughtError", "reportPuzzle2dHostUncaughtError"],
  ["reportBoardHostCaughtError", "reportPuzzle2dHostCaughtError"],
  ["reportBoardHostRecoverableError", "reportPuzzle2dHostRecoverableError"],
  ["newBoardNodeFromProps", "newPuzzle2dNodeFromProps"],
  ["appendToBoardParent", "appendToPuzzle2dHostParent"],
  ["boardSceneHost", "puzzle2dSceneHost"],
  ["enqueueBoardGraphObservationFlush", "enqueuePuzzle2dGraphObservationFlush"],
  ["flushBoardGraphObservation", "flushPuzzle2dGraphObservation"],
  ["syncBoardAppearanceFromDocument", "syncPuzzle2dAppearanceFromDocument"],
  ["shouldBoardHandleDeleteShortcut", "shouldPuzzle2dHandleDeleteShortcut"],
  ["stripLegacyImageDataPrefixForBoardIcon", "stripLegacyImageDataPrefixForPuzzle2dIcon"],
  ["isRasterDataUrlPayloadForBoardIcon", "isRasterDataUrlPayloadForPuzzle2dIcon"],
  ["looksLikeAsciiCatalogishVectorStemForBoardIcon", "looksLikeAsciiCatalogishVectorStemForPuzzle2dIcon"],
  ["initBoardWasm", "initPuzzle2dWasm"],
  ["prevBoardRedrawPlayingRef", "prevPuzzle2dRedrawPlayingRef"],
  ["TestBoardHost", "TestPuzzle2dHost"],
  ["dataset.boardSurfaceState", "dataset.puzzle2dSurfaceState"],
  ["dataset.boardSurfaceFailure", "dataset.puzzle2dSurfaceFailure"],
  ["dataset.boardRaster", "dataset.puzzle2dRaster"],
  ["dataset.boardWorldTiling", "dataset.puzzle2dWorldTiling"],
  ["dataset.boardLod", "dataset.puzzle2dLod"],
  ["dataset.boardSceneNodeCount", "dataset.puzzle2dSceneNodeCount"],
  ["dataset.boardSelection", "dataset.puzzle2dSelection"],
  ["dataset.boardHover", "dataset.puzzle2dHover"],
  ["data-board-", "data-puzzle2d-"],
  ["board-canvas", "puzzle2d-canvas"],
  ["__boardRenderer", "__puzzle2dRenderer"],
  ['test("board ', 'test("puzzle2d '],
  ["controlled board interaction", "controlled puzzle 2d interaction"],
  ["board handles nested", "puzzle 2d handles nested"],
  ["elements/client/lib/board/rs", "puzzle/2d/rs"],
];

const files = [
  "puzzle/2d/react/index.tsx",
  "framework/product/playground/renderer/react/index.tsx",
  ".storybook/puzzle-2d.spec.ts",
  ".storybook/playwright.config.ts",
].map((p) => resolve(repoRoot, p));

for (const file of files) {
  let text: string;
  try {
    text = readFileSync(file, "utf8");
  } catch {
    console.warn("skip", file);
    continue;
  }
  const original = text;
  for (const [from, to] of replacements) {
    text = text.split(from).join(to);
  }
  if (text !== original) {
    writeFileSync(file, text);
    console.log("updated", file.replace(repoRoot + "/", ""));
  }
}
