import { readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

const ROOT = resolve(import.meta.dir, "../../../../../../");

function read(path: string): string {
  return readFileSync(resolve(ROOT, path), "utf8");
}

function write(path: string, content: string): void {
  writeFileSync(resolve(ROOT, path), content);
}

function patchOptional(path: string, from: string, to: string): void {
  const content = read(path);
  if (content.includes(from)) write(path, content.replace(from, to));
}

const handlers: Record<string, string> = {
  createSequenceAppVcsHandler: `/** @emoji 📜️ S app VCS handler for sequence documents. */
export function createSequenceAppVcsHandler() {
	type Doc = { readonly schema: string; readonly steps: readonly unknown[]; readonly edges: readonly unknown[] };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"sequence.fixture",
		"sequence.fixture",
		() => ({ schema: "sequence.fixture", steps: [], edges: [] }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}`,
  createLayoutAppVcsHandler: `/** @emoji 📄️ S app VCS handler for layout documents. */
export function createLayoutAppVcsHandler() {
	type Doc = { readonly schema: string; readonly pages: readonly unknown[] };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"layout.fixture",
		"layout.fixture",
		() => ({ schema: "layout.fixture", pages: [] }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}`,
  createImperativeAppVcsHandler: `/** @emoji ⚙️ S app VCS handler for imperative documents. */
export function createImperativeAppVcsHandler() {
	type Doc = { readonly schema: string; readonly path: { readonly steps: readonly unknown[] } };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"imperative.document",
		"imperative.document",
		() => ({ schema: "imperative.document", path: { steps: [] } }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}`,
  createLowpolyAppVcsHandler: `/** @emoji 🔷️ S app VCS handler for lowpoly fixtures. */
export function createLowpolyAppVcsHandler() {
	type Doc = { readonly schema: string; readonly objects: readonly unknown[] };
	type Op = { readonly op: "setDocument"; readonly document: Doc };
	return createTypedAppVcsHandler<Doc, Op>(
		"lowpoly.fixture",
		"lowpoly.fixture",
		() => ({ schema: "lowpoly.fixture", objects: [] }),
		(doc, op) => (op.op === "setDocument" ? op.document : doc),
	);
}`,
  createVcsDemoAppVcsHandler: `/** @emoji 🗄️ S app VCS handler for vcs demo documents. */
export function createVcsDemoAppVcsHandler() {
	type Doc = { readonly schema: string; readonly title: string; readonly counter: number };
	type Op = { readonly op: "setDocument"; readonly document: Doc } | { readonly op: "setCounter"; readonly counter: number };
	return createTypedAppVcsHandler<Doc, Op>(
		"vcs.demo",
		"vcs.demo",
		() => ({ schema: "vcs.demo", title: "VCS Demo", counter: 0 }),
		(doc, op) => {
			if (op.op === "setDocument") return op.document;
			return { ...doc, counter: op.counter };
		},
	);
}`,
};

function appendHandlerRegion(path: string, handlerName: string): void {
  const content = read(path);
  if (content.includes(`export function ${handlerName}(`)) return;
  const region = `\n//#region 🔖️DocumentVcs\nimport { createTypedAppVcsHandler } from "@semio-tech/framework-os-core";\n\n${handlers[handlerName]}\n//#endregion 🔖️DocumentVcs\n`;
  write(path, content.trimEnd() + region);
}

for (const [path, handler] of [
  ["layout/core/js/internal.ts", "createLayoutAppVcsHandler"],
  ["sequence/core/js/internal.ts", "createSequenceAppVcsHandler"],
  ["imperative/core/js/internal.ts", "createImperativeAppVcsHandler"],
  ["lowpoly/core/js/internal.ts", "createLowpolyAppVcsHandler"],
  ["vcs/core/js/internal.ts", "createVcsDemoAppVcsHandler"],
] as const) {
  appendHandlerRegion(path, handler);
  const indexPath = path.replace("/internal.ts", "/index.ts");
  patchOptional(indexPath, `import { ${handler} } from "@semio-tech/framework-os-core";\n`, `import { ${handler} } from "./internal.ts";\n`);
}

{
  const path = "shooting/core/js/index.ts";
  let content = read(path);
  if (!content.includes("export function createShootingAppVcsHandler(")) {
    const shootingHandler = `\n//#region 🔖️DocumentVcs\nimport { createTypedAppVcsHandler } from "@semio-tech/framework-os-core";\n\ntype ShootingVcsAsset = { readonly id: string; readonly name: string; readonly url: string; readonly format: "glb" };\ntype ShootingVcsFixture = {\n\treadonly schema: "shooting.fixture";\n\treadonly assets: readonly ShootingVcsAsset[];\n\treadonly camera: { readonly position: readonly [number, number, number]; readonly target: readonly [number, number, number]; readonly zoom: number };\n\treadonly savedCameras: readonly unknown[];\n\treadonly scene: Record<string, unknown>;\n\treadonly shots: readonly unknown[];\n\treadonly activeShotId?: string;\n\treadonly activeAssetId?: string;\n};\ntype ShootingVcsOp =\n\t| { readonly op: "addAsset"; readonly asset: ShootingVcsAsset }\n\t| { readonly op: "removeAsset"; readonly assetId: string }\n\t| { readonly op: "setActiveAsset"; readonly assetId: string };\n\nfunction defaultShootingVcsFixture(): ShootingVcsFixture {\n\treturn {\n\t\tschema: "shooting.fixture",\n\t\tassets: [{ id: "base", name: "Base", url: "/mesh/base.glb", format: "glb" }],\n\t\tcamera: { position: [420, -420, 320], target: [0, 0, 40], zoom: 1 },\n\t\tsavedCameras: [],\n\t\tscene: {},\n\t\tshots: [{ id: "overview-svg", label: "Overview", width: 256, height: 256, format: "svg" }],\n\t\tactiveShotId: "overview-svg",\n\t\tactiveAssetId: "base",\n\t};\n}\n\nfunction applyShootingVcsOp(fixture: ShootingVcsFixture, op: ShootingVcsOp): ShootingVcsFixture {\n\tswitch (op.op) {\n\t\tcase "addAsset":\n\t\t\treturn { ...fixture, assets: [...fixture.assets, op.asset] };\n\t\tcase "removeAsset":\n\t\t\treturn { ...fixture, assets: fixture.assets.filter((asset) => asset.id !== op.assetId) };\n\t\tcase "setActiveAsset":\n\t\t\treturn { ...fixture, activeAssetId: op.assetId };\n\t}\n}\n\n/** @emoji 📸️ S app VCS handler for shooting scene documents. */\nexport function createShootingAppVcsHandler() {\n\treturn createTypedAppVcsHandler<ShootingVcsFixture, ShootingVcsOp>(\n\t\t"shooting.scene",\n\t\t"shooting.fixture",\n\t\tdefaultShootingVcsFixture,\n\t\tapplyShootingVcsOp,\n\t\tundefined,\n\t\t{\n\t\t\tapplyInputBindings: (fixture, inputBindings) => {\n\t\t\t\tconst mesh = inputBindings.mesh as { readonly url?: string } | undefined;\n\t\t\t\tif (!mesh?.url) return fixture;\n\t\t\t\tconst activeId = fixture.activeAssetId ?? fixture.assets[0]?.id;\n\t\t\t\tif (!activeId) return fixture;\n\t\t\t\treturn {\n\t\t\t\t\t...fixture,\n\t\t\t\t\tassets: fixture.assets.map((asset) => (asset.id === activeId ? { ...asset, url: mesh.url! } : asset)),\n\t\t\t\t};\n\t\t\t},\n\t\t},\n\t);\n}\n//#endregion 🔖️DocumentVcs\n`;
    content = content.replace("//#region 🔖️MediaExport", `${shootingHandler}\n//#region 🔖️MediaExport`);
    write(path, content);
  }
  patchOptional(path, 'import { createShootingAppVcsHandler } from "@semio-tech/framework-os-core";\n', "");
}

patchOptional("trinity/jack/host-core/js/index.ts", 'import { createTrinityGraphAppVcsHandler } from "@semio-tech/framework-os-core";\n', 'import { createTrinityGraphAppVcsHandler } from "@semio-tech/trinity-rewrite-core";\n');
patchOptional("reasoning/mindmap/wires/core/js/index.ts", 'import { createPuzzle2dAppVcsHandler } from "@semio-tech/framework-os-core";\n', 'import { createPuzzle2dAppVcsHandler } from "@semio-tech/puzzle-2d-core";\n');
patchOptional("framework/product/presentation/core/js/index.ts", 'import { createPresentationDeckAppVcsHandler } from "@semio-tech/framework-os-core";\n', "");
patchOptional(
  "framework/product/presentation/core/js/index.ts",
  `\t\tregisterAppVcsHandler(createPresentationAppVcsHandler());\n\t\tmergeOsProgramDefinition("presentation.deck", buildPresentationDeckProgramDefinition(), { "presentation.deck": osBaselineResource("presentation.deck", "presentation.deck", "panel", [{ id: "edit", label: "Edit" }]) });\n\t\tregisterAppVcsHandler(createPresentationDeckAppVcsHandler());`,
  `\t\tregisterAppVcsHandler(createPresentationAppVcsHandler());\n\t\tmergeOsProgramDefinition("presentation.deck", buildPresentationDeckProgramDefinition(), { "presentation.deck": osBaselineResource("presentation.deck", "presentation.deck", "panel", [{ id: "edit", label: "Edit" }]) });`,
);

patchOptional(
  "cad/renderer/core/js/index.ts",
  `\t\tmergeOsProgramDefinition("cad", buildCadProgramDefinition(), cadProgramContributionResources);\n\t\t\n\t},`,
  `\t\tmergeOsProgramDefinition("cad", buildCadProgramDefinition(), cadProgramContributionResources);\n\t\tregisterAppVcsHandler(createCadSceneAppVcsHandler());\n\t\tregisterCadMediaExportHandlers();\n\t},`,
);

const mediaExportByProgram: Array<[string, string]> = [
  ["draw/core/js/index.ts", "registerDrawMediaExportHandlers();"],
  ["raster/core/js/index.ts", "registerRasterMediaExportHandlers();"],
  ["note/core/js/index.ts", "registerNoteMediaExportHandlers();"],
  ["gis/2d/core/js/index.ts", "registerGisMediaExportHandlers();"],
  ["procedural/2d/core/js/index.ts", "registerProcedural2dMediaExportHandlers();"],
  ["shooting/core/js/index.ts", "registerShootingMediaExportHandlers();"],
  ["layout/core/js/index.ts", "registerLayoutMediaExportHandlers();"],
  ["framework/product/presentation/core/js/index.ts", "registerPresentationMediaExportHandlers();"],
  ["lowpoly/core/js/index.ts", "registerLowpolyMediaExportHandlers();"],
  ["procedural/3d/core/js/index.ts", "registerProcedural3dMediaExportHandlers();"],
  ["puzzle/2d/core/js/index.ts", "registerPuzzle2dMediaExportHandlers();"],
  ["puzzle/3d/core/js/index.ts", "registerPuzzle3dMediaExportHandlers();"],
  ["puzzle/5d/core/js/index.ts", "registerPuzzle5dMediaExportHandlers();"],
];

for (const [path, call] of mediaExportByProgram) {
  const content = read(path);
  if (content.includes(call)) continue;
  patchOptional(path, "\t\tregisterAppVcsHandler(", `\t\t${call}\n\t\tregisterAppVcsHandler(`);
}

console.log("completion done");
