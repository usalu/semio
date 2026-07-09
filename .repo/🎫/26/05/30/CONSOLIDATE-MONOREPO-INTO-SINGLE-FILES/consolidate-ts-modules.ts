import { readFileSync, writeFileSync, unlinkSync, existsSync } from "node:fs";
import { dirname, join, basename } from "node:path";

const root = join(import.meta.dirname, "../../../../../../");

function stripHeader(src: string): string {
  const lines = src.split(/\r?\n/);
  let i = 0;
  if (lines[i]?.includes("#region") && (lines[i]?.includes("Header") || lines[i]?.includes("🧲Header"))) {
    i++;
    while (i < lines.length && !lines[i]?.includes("#endregion")) i++;
    i++;
  }
  while (i < lines.length && lines[i]?.trim() === "") i++;
  return lines.slice(i).join("\n");
}

function stripLocalImports(src: string, localNames: string[]): string {
  return src
    .split(/\r?\n/)
    .filter((line) => {
      const trimmed = line.trim();
      for (const name of localNames) {
        const escaped = name.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
        if (new RegExp(`^import\\s+.*from\\s+["']\\.\\/?${escaped}(\\.tsx?)?["'];?\\s*$`).test(trimmed)) return false;
      }
      return true;
    })
    .join("\n");
}

function rewriteImports(from: string, to: string, files: string[]): void {
  for (const file of files) {
    if (!existsSync(file)) continue;
    const src = readFileSync(file, "utf8");
    const next = src.split(from).join(to);
    if (next !== src) {
      writeFileSync(file, next);
      console.log(`rewrote imports in ${file}`);
    }
  }
}

function mergeSatellites(targetPath: string, satellitePaths: string[], regionPrefix: string, stripPatterns: RegExp[] = []): void {
  const satelliteNames = satellitePaths.map((p) => basename(p).replace(/\.(tsx?)$/, ""));
  let target = readFileSync(targetPath, "utf8");
  for (const pattern of stripPatterns) target = target.replace(pattern, "\n");
  const blocks: string[] = [];
  for (const satellitePath of satellitePaths) {
    const name = basename(satellitePath).replace(/\.(tsx?)$/, "");
    let body = stripHeader(readFileSync(satellitePath, "utf8"));
    body = stripLocalImports(
      body,
      satelliteNames.filter((n) => n !== name),
    );
    blocks.push(`//#region ${regionPrefix}${name}\n${body.trimEnd()}\n//#endregion ${regionPrefix}${name}`);
    unlinkSync(satellitePath);
  }
  writeFileSync(targetPath, target.trimEnd() + "\n\n" + blocks.join("\n\n") + "\n");
  console.log(`merged ${satellitePaths.length} satellites into ${targetPath}`);
}

// 1. Graph canvas triad
mergeSatellites(
  join(root, "framework/renderer/react/components/node-graph-host.tsx"),
  [join(root, "framework/renderer/react/components/graph-canvas-overlays.tsx"), join(root, "framework/renderer/react/components/flow-graph-canvas-host.tsx")],
  "🔖",
  [
    /\nimport \{ FlowGraphCanvasHost \} from "\.\/flow-graph-canvas-host\.tsx";\n/,
    /\nimport \{\n\tcomputeDagMarqueeOverlay,\n\tGraphParamOverlays,\n\tGraphStepperOverlays,\n\tpaintDagLabelOverlays,\n\tparseDagNodeIdArray,\n\tparseDagSelectionUnionBoundsScreen,\n\tSelectionAlignChrome,\n\talignModeToDag,\n\tsceneToSyncJson,\n\} from "\.\/graph-canvas-overlays\.tsx";\n/,
  ],
);

// 2. OS shell satellites
const rendererDir = join(root, "framework/renderer/react");
const shellSatellites = ["types.ts", "plugin-runtime.ts", "wasm-session-loader.ts", "ui-search-find.tsx", "tool-tree.tsx", "os-chrome-panels.tsx"].map((f) => join(rendererDir, f));
const shellSatelliteNames = shellSatellites.map((p) => basename(p).replace(/\.(tsx?)$/, ""));
let shell = stripLocalImports(readFileSync(join(rendererDir, "os-shell.tsx"), "utf8"), shellSatelliteNames);
const shellBlocks: string[] = [];
for (const satellitePath of shellSatellites) {
  const name = basename(satellitePath).replace(/\.(tsx?)$/, "");
  let body = stripHeader(readFileSync(satellitePath, "utf8"));
  body = stripLocalImports(
    body,
    shellSatelliteNames.filter((n) => n !== name),
  );
  shellBlocks.push(`//#region 🔖${name}\n${body.trimEnd()}\n//#endregion 🔖${name}`);
  unlinkSync(satellitePath);
}
writeFileSync(join(rendererDir, "os-shell.tsx"), shell.trimEnd() + "\n\n" + shellBlocks.join("\n\n") + "\n");
console.log("merged os-shell satellites");

const rendererFiles = [
  join(rendererDir, "ui-interpreter.tsx"),
  join(rendererDir, "index.tsx"),
  join(rendererDir, "index.test.ts"),
  join(rendererDir, "components/node-graph-host.tsx"),
  join(rendererDir, "components/world-3d-host.tsx"),
  join(rendererDir, "components/canvas-2d-host.tsx"),
  join(rendererDir, "components/text-editor-host.tsx"),
  join(rendererDir, "components/raster-host.tsx"),
  join(rendererDir, "components/table-host.tsx"),
];
for (const file of rendererFiles) {
  if (!existsSync(file)) continue;
  let src = readFileSync(file, "utf8");
  src = src.replaceAll("../types.ts", "./os-shell.tsx");
  src = src.replaceAll("../wasm-session-loader.ts", "./os-shell.tsx");
  src = src.replaceAll("../ui-search-find.tsx", "./os-shell.tsx");
  src = src.replaceAll("./types.ts", "./os-shell.tsx");
  src = src.replaceAll("./plugin-runtime.ts", "./os-shell.tsx");
  writeFileSync(file, src);
}
console.log("rewrote renderer imports to os-shell.tsx");

// 3. Storybook kit-store
const kitStoreDir = join(root, ".storybook/compose/algorithm/kit-store");
const kitStoreFiles = ["composeWasm.ts", "commandSchema.ts", "useKitStore.ts", "HistoryControls.tsx", "CommandForm.tsx", "EntityPicker.tsx", "EventsFeed.tsx", "SnapshotViewer.tsx", "DiffViewer.tsx"];
const kitStoreNames = kitStoreFiles.map((f) => f.replace(/\.(tsx?)$/, ""));
const kitStoreBlocks: string[] = [];
for (const file of kitStoreFiles) {
  const satellitePath = join(kitStoreDir, file);
  const name = file.replace(/\.(tsx?)$/, "");
  let body = stripHeader(readFileSync(satellitePath, "utf8"));
  body = stripLocalImports(
    body,
    kitStoreNames.filter((n) => n !== name),
  );
  kitStoreBlocks.push(`//#region 🔖${name}\n${body.trimEnd()}\n//#endregion 🔖${name}`);
  unlinkSync(satellitePath);
}
writeFileSync(join(kitStoreDir, "index.tsx"), kitStoreBlocks.join("\n\n") + "\n");
const kitStoreStory = join(root, ".storybook/stories/compose/algorithm/KitStore.stories.tsx");
let kitStoreStorySrc = readFileSync(kitStoreStory, "utf8");
kitStoreStorySrc = kitStoreStorySrc.replace(
  /import \{ CommandForm \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/CommandForm";\nimport \{ ALL_CHANGE_KIT_ROOT_KEYS, CHANGE_TYPE_COMMAND_KEYS, KIT_STORE_COVERAGE_ROWS \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/commandSchema";\nimport \{ DiffViewer \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/DiffViewer";\nimport \{ applyEntityPlaceholders, EntityPicker \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/EntityPicker";\nimport \{ EventsFeed \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/EventsFeed";\nimport \{ HistoryControls, KitTreeGraph \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/HistoryControls";\nimport \{ SnapshotViewer \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/SnapshotViewer";\nimport \{ useKitStore \} from "\.\.\/\.\.\/\.\.\/compose\/algorithm\/kit-store\/useKitStore";/,
  `import {
	ALL_CHANGE_KIT_ROOT_KEYS,
	applyEntityPlaceholders,
	CHANGE_TYPE_COMMAND_KEYS,
	CommandForm,
	DiffViewer,
	EntityPicker,
	EventsFeed,
	HistoryControls,
	KIT_STORE_COVERAGE_ROWS,
	KitTreeGraph,
	SnapshotViewer,
	useKitStore,
} from "../../../compose/algorithm/kit-store/index.tsx";`,
);
writeFileSync(kitStoreStory, kitStoreStorySrc);
console.log("merged kit-store");

// 4. plugin-registry
const osDevDir = join(root, "framework/product/os/dev/js");
if (existsSync(join(osDevDir, "plugin-registry.ts"))) {
  let index = readFileSync(join(osDevDir, "index.ts"), "utf8");
  const registry = stripHeader(readFileSync(join(osDevDir, "plugin-registry.ts"), "utf8"));
  index = index
    .replaceAll('const { PLUGIN_BUILD_TARGETS, pluginModuleUrl } = await import("./plugin-registry.ts");\n', "")
    .replaceAll('const { PLUGIN_BUILD_TARGETS, pluginModuleUrl } = await import("./index.ts");\n', "")
    .replace(/import "\.\.\/globals\.css";\n/, `import "../globals.css";\n\n//#region 🔖plugin-registry\n${registry.trimEnd()}\n//#endregion 🔖plugin-registry\n`);
  if (!index.includes("const plugins = PLUGIN_BUILD_TARGETS.map")) {
    index = index.replace(/void bootFrameworkOsWgpu\(\{\s*plugin: pluginFilter,\s*plugins: PLUGIN_BUILD_TARGETS\.map\(\(target\) => \(\{[\s\S]*?\}\)\),\s*\}\)/, "void bootFrameworkOsWgpu({ plugin: pluginFilter, plugins })");
    index = index.replace(/void bootFrameworkOs\(\{\s*plugin: pluginFilter,\s*plugins: PLUGIN_BUILD_TARGETS\.map\(\(target\) => \(\{[\s\S]*?\}\)\),\s*\}\)/, "void bootFrameworkOs({ plugin: pluginFilter, plugins })");
    index = index.replace(
      /if \(typeof document !== "undefined" && document\.getElementById\("root"\) != null && !import\.meta\.vitest\) \{/,
      `if (typeof document !== "undefined" && document.getElementById("root") != null && !import.meta.vitest) {\n\tconst plugins = PLUGIN_BUILD_TARGETS.map((target) => ({\n\t\tpluginId: target.pluginId,\n\t\tmoduleUrl: pluginModuleUrl(target.pluginId, target.wasmOut),\n\t}));`,
    );
  }
  writeFileSync(join(osDevDir, "index.ts"), index);
  unlinkSync(join(osDevDir, "plugin-registry.ts"));
  const scriptPath = join(osDevDir, "../script.ts");
  if (existsSync(scriptPath)) {
    let script = readFileSync(scriptPath, "utf8");
    script = script.replaceAll("./js/plugin-registry.ts", "./js/index.ts");
    writeFileSync(scriptPath, script);
  }
  console.log("merged plugin-registry");
}

// 5. storybook decorators
const storybookDir = join(root, ".storybook");
let preview = readFileSync(join(storybookDir, "preview.ts"), "utf8");
preview = preview.replace('import { withLevel } from "./withLevel";\n', "");
preview = preview.replace('import { withTheme } from "./withTheme";\n', "");
for (const deco of ["withLevel.tsx", "withTheme.tsx"]) {
  const body = stripHeader(readFileSync(join(storybookDir, deco), "utf8"));
  const name = deco.replace(".tsx", "");
  preview = preview.trimEnd() + `\n\n//#region 🔖${name}\n${body.trimEnd()}\n//#endregion 🔖${name}\n`;
  unlinkSync(join(storybookDir, deco));
}
writeFileSync(join(storybookDir, "preview.ts"), preview);
console.log("merged storybook decorators");

// 6. jack lsp protocol
const jackLspDir = join(root, "trinity/jack/lsp/js");
let worker = readFileSync(join(jackLspDir, "worker.ts"), "utf8");
const protocol = stripHeader(readFileSync(join(jackLspDir, "protocol.ts"), "utf8"));
worker = worker.replace('import type { JsonRpcMessage } from "./protocol.ts";\n', "");
worker = worker.replace('import { parseJsonRpcMessage, serializeJsonRpcMessage } from "./protocol.ts";\n', "");
writeFileSync(join(jackLspDir, "worker.ts"), worker.trimEnd() + `\n\n//#region 🔖protocol\n${protocol.trimEnd()}\n//#endregion 🔖protocol\n`);
unlinkSync(join(jackLspDir, "protocol.ts"));
console.log("merged jack lsp protocol");

// 7. sketchpad docs-mdx
const sketchpadDir = join(root, "compose/client/lib/sketchpad/js");
let sketchpad = readFileSync(join(sketchpadDir, "index.ts"), "utf8");
let docsMdx = stripHeader(readFileSync(join(sketchpadDir, "docs-mdx.ts"), "utf8"));
docsMdx = docsMdx.replace("export function sketchpadBuildDocsRegistry()", "function sketchpadBuildDocsRegistryFromGlob()");
sketchpad = sketchpad.replace('const { sketchpadBuildDocsRegistry: build } = await import("./docs-mdx.ts");\n\tsketchpadDocsRegistryCache = build();', "sketchpadDocsRegistryCache = sketchpadBuildDocsRegistryFromGlob();");
sketchpad = sketchpad.replace('return (await import("./docs-mdx.ts")).sketchpadResolveMdxModuleKey(docsPath);', "return sketchpadResolveMdxModuleKey(docsPath);");
sketchpad = sketchpad.replace('return (await import("./docs-mdx.ts")).sketchpadLoadMdxModule(docsPath);', "return sketchpadLoadMdxModule(docsPath);");
sketchpad = sketchpad.replace('return (await import("./docs-mdx.ts")).sketchpadMdxTitle(module, docsPath);', "return sketchpadMdxTitle(module, docsPath);");
writeFileSync(join(sketchpadDir, "index.ts"), sketchpad.trimEnd() + `\n\n//#region 🔖docs-mdx\n${docsMdx.trimEnd()}\n//#endregion 🔖docs-mdx\n`);
unlinkSync(join(sketchpadDir, "docs-mdx.ts"));
console.log("merged sketchpad docs-mdx");

// 8. generate.neo4j.gen
const scriptPath = join(root, "script.ts");
if (existsSync(join(root, "generate.neo4j.gen.ts"))) {
  let script = readFileSync(scriptPath, "utf8");
  const neo4j = stripHeader(readFileSync(join(root, "generate.neo4j.gen.ts"), "utf8"));
  script = script.replace('import { Neo4jCypherExport, getAllNeo4jGraphExportSpecs, joinNeo4jGraphDatabaseName, partitionNeo4jGraphCliArgv } from "./generate.neo4j.gen.ts";\n', "");
  writeFileSync(scriptPath, script.trimEnd() + `\n\n//#region 🔖generate-neo4j-gen\n${neo4j.trimEnd()}\n//#endregion 🔖generate-neo4j-gen\n`);
  unlinkSync(join(root, "generate.neo4j.gen.ts"));
  console.log("merged generate.neo4j.gen");
}
