/** @emoji 🔧 Repairs merge-script corruption in playground core index files. */
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const repoRoot = join(import.meta.dir, "../../../../../../");

const appPaths = [
  "draw",
  "note",
  "writer",
  "forms",
  "raster",
  "shooting",
  "procedural/2d",
  "procedural/3d",
  "gis/2d",
  "puzzle/2d",
  "puzzle/3d",
  "puzzle/5d",
  "layout",
  "sequence",
  "imperative",
  "lowpoly",
  "flow",
  "mathematical/graph/port/directed/dag",
  "reasoning/mindmap/wires",
  "trinity/jack/host-core",
  "trinity/rewrite",
  "s",
  "vcs",
  "framework/product/presentation",
  "cad/js/renderer",
];

function fixImports(source: string): string {
  let next = source;
  next = next.replace(/\} createPlaygroundApp,\s*\n\s*createProductPlaygroundPlatform,\s*\n\}/g, "}");
  next = next.replace(/,\s*\n\s*,\s*\n\s*createPlaygroundApp,\s*\n\s*createProductPlaygroundPlatform,\s*\n\}/g, "\n}");
  next = next.replace(/import type \{ WindowMeasure ,\s*\n\s*createPlaygroundApp,\s*\n\s*createProductPlaygroundPlatform,\s*\n\} from "@semio-tech\/framework-playground-core";\n/g, "");
  return next;
}

function removeSelfImports(source: string): string {
  return source.replace(/\nimport \{[^}]*\} from "\.\/index\.ts";\n/g, "\n");
}

function fixBrokenFunctions(source: string): string {
  let next = source;
  const replacements: [string, string][] = [
    [
      "/** @emoji 🔒 Resolves a playground fixture slug (e.g. `concrete`) to a puzzle 2d fixture id. */\nslug: string): string | undefined {",
      "/** @emoji 🔒 Resolves a playground fixture slug (e.g. `concrete`) to a puzzle 2d fixture id. */\nexport function resolvePuzzle2dPlayExampleSlug(slug: string): string | undefined {",
    ],
    [
      "/** @emoji 🧪 Resolves imported puzzle 2d fixture JSON by catalog id. */\nexampleId: string = PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID): unknown {",
      "/** @emoji 🧪 Resolves imported puzzle 2d fixture JSON by catalog id. */\nexport function puzzle2dPlayFixtureJson(fixtureId: string = PUZZLE_2D_PLAY_EXAMPLE_CONCRETE_FOREST_ID): unknown {",
    ],
    [
      "/** @emoji 📋 Parses a puzzle 2d play fixture by catalog id. */\nfixtureId: string): Puzzle2dFixture {",
      "/** @emoji 📋 Parses a puzzle 2d play fixture by catalog id. */\nexport function puzzle2dPlayFixtureForId(fixtureId: string): Puzzle2dFixture {",
    ],
    [
      "/** @emoji 📄 Serializes a puzzle 2d fixture for Jack and VCS bridges. */\nfixture: Puzzle2dFixture): string {",
      "/** @emoji 📄 Serializes a puzzle 2d fixture for Jack and VCS bridges. */\nexport function puzzle2dFixtureToJson(fixture: Puzzle2dFixture): string {",
    ],
    [
      "/** @emoji 🃏 Normalizes a puzzle 2d fixture into board-shaped JSON for Jack queries. */\nfixtureOrJson: Puzzle2dFixture | string): string {",
      "/** @emoji 🃏 Normalizes a puzzle 2d fixture into board-shaped JSON for Jack queries. */\nexport function puzzle2dFixtureToJackBoardJson(fixtureOrJson: Puzzle2dFixture | string): string {",
    ],
    [
      "/** @emoji 🔌 Renders a puzzle fixture as wire-literal compiled DAG text. */\nfixtureOrJson: Puzzle2dFixture | string): string {",
      "/** @emoji 🔌 Renders a puzzle fixture as wire-literal compiled DAG text. */\nexport function puzzle2dFixtureToCompiledDagWireLiteral(fixtureOrJson: Puzzle2dFixture | string): string {",
    ],
    [
      "/** @emoji 📷 Viewport camera centered on fixture node bounds with zoom fitted for growth. */\nfixture: Puzzle2dFixture, rawFixture?: unknown): CameraState {",
      "/** @emoji 📷 Viewport camera centered on fixture node bounds with zoom fitted for growth. */\nexport function puzzle2dPlayViewportCameraFromFixture(fixture: Puzzle2dFixture, rawFixture?: unknown): CameraState {",
    ],
    [
      "/** @emoji 📷 Viewport camera for a play fixture catalog id (uses raw JSON bounds before circle normalization). */\nfixtureId: string): CameraState {",
      "/** @emoji 📷 Viewport camera for a play fixture catalog id (uses raw JSON bounds before circle normalization). */\nexport function puzzle2dPlayViewportCameraForFixtureId(fixtureId: string): CameraState {",
    ],
    [
      "/** @emoji 📷 Default cameras for all puzzle 2d play panes (wide overview, tight detail, regional selection). */\nfixture: Puzzle2dFixture, rawFixture?: unknown): Record<Puzzle2dPlayPaneId, CameraState> {",
      "/** @emoji 📷 Default cameras for all puzzle 2d play panes (wide overview, tight detail, regional selection). */\nexport function puzzle2dPlayTriptychCamerasFromFixture(fixture: Puzzle2dFixture, rawFixture?: unknown): Record<Puzzle2dPlayPaneId, CameraState> {",
    ],
    ["\nfixture: FlowFixtureV1): string {", "\nexport function proceduralFixtureToJson(fixture: FlowFixtureV1): string {"],
    [
      "/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */\nexampleId: string = PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID): string {",
      "/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */\nexport function procedural2dPlayFixtureJson(fixtureId: string = PROCEDURAL_2D_PLAY_EXAMPLE_DEFAULT_ID): string {",
    ],
    [
      "/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */\nexampleId: string = PROCEDURAL_PLAY_EXAMPLE_DEFAULT_ID): string {",
      "/** @emoji 🧪 Resolves procedural play fixture JSON by catalog id. */\nexport function proceduralPlayFixtureJson(fixtureId: string = PROCEDURAL_PLAY_EXAMPLE_DEFAULT_ID): string {",
    ],
  ];
  for (const [from, to] of replacements) next = next.replace(from, to);
  return next;
}

function fixCreateRuntimeThisId(source: string): string {
  return source.replace(/createRuntime: \(\) => \{\s*\n\s*const runtime = new Platform\(\{ id: this\.id \}\);/g, "createRuntime: () => {\n\t\tconst runtime = new Platform({ id: PUZZLE_2D_PLAY_APP_ID });");
}

function movePlayRegionBeforeController(source: string, controllerMarker: string): string {
  const playStart = source.indexOf("//#region 🔖Play");
  if (playStart < 0) return source;
  const playEnd = source.indexOf("//#endregion 🔖Play", playStart);
  if (playEnd < 0) return source;
  const playBlock = source.slice(playStart, playEnd + "//#endregion 🔖Play".length);
  const controllerIdx = source.indexOf(controllerMarker);
  if (controllerIdx < 0 || controllerIdx < playStart) return source;
  const appDefMatch = playBlock.match(/export const \w+PlayAppDefinition = createPlaygroundApp\([\s\S]*?\}\);\n/);
  if (!appDefMatch) return source;
  const examplesBlock = playBlock
    .replace(appDefMatch[0], "")
    .replace(/^\/\/#region 🔖Play\n/, "//#region 🔖PlayExamples\n")
    .replace(/\/\/#endregion 🔖Play$/, "//#endregion 🔖PlayExamples");
  const appDefOnly = `//#region 🔖Play\n${appDefMatch[0]}//#endregion 🔖Play\n`;
  const withoutPlay = source.slice(0, playStart) + source.slice(playEnd + "//#endregion 🔖Play".length);
  const insertAt = withoutPlay.indexOf(controllerMarker);
  if (insertAt < 0) return source;
  const before = withoutPlay.slice(0, insertAt).trimEnd();
  const after = withoutPlay.slice(insertAt);
  return `${before}\n\n${examplesBlock}\n\n${after}\n\n${appDefOnly}`;
}

const moveTargets: Record<string, string> = {
  "puzzle/2d/core/index.ts": "export class Puzzle2dPlayShellController",
  "puzzle/5d/core/index.ts": "export class Puzzle5dPlayShellController",
  "procedural/2d/core/index.ts": "export class Procedural2dPlayController",
  "procedural/3d/core/index.ts": "export class ProceduralPlayController",
};

for (const appPath of appPaths) {
  const filePath = join(repoRoot, appPath, "core/index.ts");
  let source = readFileSync(filePath, "utf8");
  const rel = `${appPath}/core/index.ts`;
  source = fixImports(source);
  source = removeSelfImports(source);
  source = fixBrokenFunctions(source);
  if (rel === "puzzle/2d/core/index.ts") source = fixCreateRuntimeThisId(source);
  if (moveTargets[rel]) source = movePlayRegionBeforeController(source, moveTargets[rel]!);
  writeFileSync(filePath, source);
  console.log(`fixed ${rel}`);
}
