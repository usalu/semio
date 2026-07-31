#!/usr/bin/env bun
/** 🔍️ Wasm command round-trip verification for puzzle 5d board/world parity. */
import { join } from "node:path";

const repoRoot = "/Users/ueli/Documents/semio";
const moduleUrl = `file://${join(repoRoot, "framework/product/os/dev/plugin-modules/puzzle/puzzle_plugin.js")}`;
const { loadPluginModule } = await import(join(repoRoot, "framework/core/js/index.ts"));
const handle = await loadPluginModule("puzzle", moduleUrl);
const instanceId = await handle.createApp("puzzle5d-play");
const viewState = {};
const cmd = (command: string, args: Record<string, unknown>) => handle.handleCommand(instanceId, JSON.stringify({ controllerId: "puzzle5d-play", command, args }), viewState);

const board = (await handle.render(instanceId, "puzzle.5d.play.2d", viewState)) as { puzzle2dBoard?: Record<string, unknown> };
if (!board.puzzle2dBoard) throw new Error("2d body did not render a puzzle2dBoard scene");
const fixture = JSON.parse(String(board.puzzle2dBoard.fixtureJson)) as { nodes: unknown[]; edges: unknown[] };
console.log("[DEBUG] board scene nodes:", fixture.nodes.length, "edges:", fixture.edges.length, "activeTool:", board.puzzle2dBoard.activeTool);
if (fixture.nodes.length < 1) throw new Error("board fixture empty");

await cmd("setActiveExample", { exampleId: "nakagin-capsule-tower" });
const nakaginBoard = (await handle.render(instanceId, "puzzle.5d.play.2d", viewState)) as { puzzle2dBoard?: Record<string, unknown> };
const nakaginFixture = JSON.parse(String(nakaginBoard.puzzle2dBoard?.fixtureJson)) as { nodes: unknown[]; edges: unknown[] };
console.log("[DEBUG] nakagin board nodes:", nakaginFixture.nodes.length, "edges:", nakaginFixture.edges.length);
if (nakaginFixture.nodes.length !== 180 || nakaginFixture.edges.length !== 179) throw new Error("nakagin board fixture wrong size");

await cmd("setActiveExample", { exampleId: "concrete-forest" });
await cmd("applyBoardEvents", { eventsJson: JSON.stringify([{ name: "select", payload: { ids: ["seed-left-001"] } }]) });
const world = (await handle.render(instanceId, "puzzle.5d.play.3d", viewState)) as { world3d?: Record<string, string> };
const selection = JSON.parse(world.world3d?.selectionJson ?? "{}") as Record<string, unknown>;
console.log("[DEBUG] world selection gumballActive:", selection.gumballActive, "transformTool:", selection.transformTool, "activeObjectId:", selection.activeObjectId);
if (selection.gumballActive !== true || selection.transformTool !== "move") throw new Error("gumball fields missing after board select");
if (!world.world3d?.contextMenuJson?.includes("duplicateSelection")) throw new Error("context menu missing for selection");

const boardAfterSelect = (await handle.render(instanceId, "puzzle.5d.play.2d", viewState)) as { puzzle2dBoard?: Record<string, unknown> };
const boardSelection = JSON.parse(String(boardAfterSelect.puzzle2dBoard?.selectionJson)) as string[];
if (!boardSelection.includes("seed-left-001")) throw new Error("board selection not paired");
console.log("[DEBUG] paired selection on board:", boardSelection);

await cmd("setHover", { objectId: "seed-left-001" });
const hoveredBoard = (await handle.render(instanceId, "puzzle.5d.play.2d", viewState)) as { puzzle2dBoard?: Record<string, unknown> };
if (hoveredBoard.puzzle2dBoard?.hoveredId !== "seed-left-001") throw new Error("board hoveredId not paired with 3d hover");
console.log("[DEBUG] paired hover on board:", hoveredBoard.puzzle2dBoard?.hoveredId);

await cmd("engagementPossibleSelect", { window: "puzzle5d-3d", possibleId: "puzzle5d.tool.fill" });
const engagements = await handle.windowEngagements(instanceId, viewState);
const fill = engagements["puzzle5d-3d"];
if (fill?.control?.kind !== "slider") throw new Error("fill slider missing");
await cmd("setFillCount", { value: 3 });
const parts = JSON.parse(((await handle.render(instanceId, "puzzle.5d.play.2d", viewState)) as { puzzle2dBoard?: Record<string, string> }).puzzle2dBoard?.fixtureJson ?? "{}") as { nodes: { x: number; y: number }[] };
console.log("[DEBUG] parts after fill 3:", parts.nodes.length);
const flatPlaced = parts.nodes.every((node) => Number.isFinite(node.x) && Number.isFinite(node.y));
if (!flatPlaced) throw new Error("fill produced parts without flat centers");

await cmd("engagementPossibleSelect", { window: "puzzle5d-3d", possibleId: "puzzle5d.tool.brush" });
await cmd("worldVortexHover", { fullId: "seed-left-001:v0" });
const brushControl = (await handle.windowEngagements(instanceId, viewState))["puzzle5d-3d"]?.control;
console.log("[DEBUG] brush control:", brushControl?.kind, "options:", brushControl?.options?.length ?? 0);

const measures = await handle.windowMeasures?.(instanceId, viewState);
if (measures) {
  console.log("[DEBUG] 2d measures:", (measures["puzzle5d-2d"] ?? []).length, "3d measures:", (measures["puzzle5d-3d"] ?? []).length);
}

const documentTree = JSON.stringify(await handle.render(instanceId, "puzzle.5d.play.document", viewState));
if (!documentTree.includes("puzzle5d-play-document.fasteners")) throw new Error("document tree missing fasteners section");
const kindsTree = JSON.stringify(await handle.render(instanceId, "puzzle.5d.play.kinds", viewState));
if (!kindsTree.includes("Hexagonal Cut Concrete Forest Right") || !kindsTree.includes("puzzle5d-play-kinds.ropes")) throw new Error("catalogue not derived from kind catalogs");

console.log("[DEBUG] wasm-verify passed");
