// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji 🖥️ `@semio-tech/s-core` — S studio instance over {@link @semio-tech/framework-os-core}. */
// #endregion 🧲Header

import type { ComponentKind, PlatformDefinition, PluginContext } from "@semio-tech/framework-platform-core";
import {
	applyDagFixtureJsonToOsMediaGraph,
	appInstanceResourceProjection,
	appVcsHandlerForFormat,
	createCatalogueKindsAppVcsHandler,
	createEmptyOsDocument,
	createFlowDagAppVcsHandler,
	createFlowDocumentAppVcsHandler,
	createGisMapAppVcsHandler,
	createImperativeAppVcsHandler,
	createLayoutAppVcsHandler,
	createLowpolyAppVcsHandler,
	createOsId,
	createPresentationDeckAppVcsHandler,
	createProcedural2dAppVcsHandler,
	createProcedural3dAppVcsHandler,
	createPuzzle2dAppVcsHandler,
	createPuzzle3dAppVcsHandler,
	createPuzzle5dAppVcsHandler,
	createSequenceAppVcsHandler,
	createShootingAppVcsHandler,
	createTrinityGraphAppVcsHandler,
	createTypedAppVcsHandler,
	createVcsDemoAppVcsHandler,
	listOsPrograms,
	listOsResourceDescriptors,
	materializeAppInstanceProjection,
	materializeOsProjection,
	mergeOsProgramDefinition,
	osAppRegistration,
	osBaselineResource,
	osDocumentFromJson,
	osDocumentToJson,
	osExtensionRegistrySize,
	osInPort,
	osMediaGraphToDagFixtureJson,
	osOutPort,
	osProgramById,
	osResourceDescriptor,
	parseOsDocument,
	registerAppVcsHandler,
	registerOsBuiltinProgram,
	registerOsFixtureJsonResolver,
	registerOsProgramDefinition,
	seedOsProgramRegistryFromResourceMap,
	type OsAppInstance,
	type OsAppRegistration,
	type OsAppResourceSpec,
	type OsCommand,
	type OsDocument,
	type OsMediaGraph,
	type OsMediaGraphNode,
	type OsProgramDefinition,
	type OsProjection,
	type OsResourceDescriptor,
	type OsResourceKindId,
	OsStore,
	DevJsonBackbone,
	LocalJsonBackbone,
	RemoteJsonBackbone,
	RemoteOsBackbone,
	buildOsHistoryColumns,
	emptyMediaGraph,
	validateMediaGraph,
	applyAppOperationToSource,
	createAppSourceDocument,
	resolvePayloadRef,
	mediaPortIdForSpec,
	mediaPortSpecId,
	osAppPrimaryOutputKind,
	resourcesCompatible,
	OS_STUDIO_SCHEMA,
	OS_MEDIA_GRAPH_SCHEMA,
	OS_RESOURCE_KIND_IDS,
	OsMediaGraphVirtualFileSystemController,
	OS_MEDIA_GRAPH_VFS_ROOT_ID,
	registerOsMediaExportHandler,
	assertOsMediaExportCoverage,
	exportOsAppInstanceMedia,
	type OsMediaExportFormat,
	type OsMediaExportResult,
} from "@semio-tech/framework-os-core";

//#region 🔖SAliases
export const S_STUDIO_SCHEMA = OS_STUDIO_SCHEMA;
export const S_MEDIA_GRAPH_SCHEMA = OS_MEDIA_GRAPH_SCHEMA;
export type SResourceKindId = OsResourceKindId;
export { OS_RESOURCE_KIND_IDS as S_RESOURCE_KIND_IDS };
export type SBackboneRef = import("@semio-tech/framework-os-core").OsBackboneRef;
export type StudioConflict = import("@semio-tech/framework-os-core").OsConflict;
export type SSourceDocument = import("@semio-tech/framework-os-core").OsSourceDocument;
export type SAppInstance = OsAppInstance;
export type SMediaPort = import("@semio-tech/framework-os-core").OsMediaPort;
export type SPortSpec = import("@semio-tech/framework-os-core").OsPortSpec;
export type SMediaGraphNode = OsMediaGraphNode;
export type SMediaGraphEdge = import("@semio-tech/framework-os-core").OsMediaGraphEdge;
export type SMediaGraph = OsMediaGraph;
export type SStudioProjection = OsProjection;
export type SStudioDocumentV1 = OsDocument;
export type SStudioDocument = OsDocument;
export type SStudioOperation = import("@semio-tech/framework-os-core").OsOperation;
export type SStudioChange = import("@semio-tech/framework-os-core").OsChange;
export type SStudioCheckpoint = import("@semio-tech/framework-os-core").OsCheckpoint;
export type SStudioAlternative = import("@semio-tech/framework-os-core").OsAlternative;
export type SStudioVcs = import("@semio-tech/framework-os-core").OsVcs;
export type SAppRegistration = OsAppRegistration;
export type SProgramDefinition = OsProgramDefinition;
export type SResourceDescriptor = OsResourceDescriptor;
export type SAppResourceSpec = OsAppResourceSpec;
export type StudioCommand = OsCommand;
export { OsStore as StudioStore, DevJsonBackbone, LocalJsonBackbone, RemoteJsonBackbone, RemoteOsBackbone, buildOsHistoryColumns };
export const createSId = createOsId;
export const createEmptyStudioDocument = createEmptyOsDocument;
export const materializeStudioProjection = materializeOsProjection;
export const parseSStudioDocument = parseOsDocument;
export const sStudioDocumentToJson = osDocumentToJson;
export const sStudioDocumentFromJson = osDocumentFromJson;
export const listSResourceDescriptors = listOsResourceDescriptors;
export const sResourceDescriptor = osResourceDescriptor;
export const sAppPrimaryOutputKind = osAppPrimaryOutputKind;
export const sAppRegistration = osAppRegistration;
export const sProgramById = osProgramById;
export const sExtensionRegistrySize = osExtensionRegistrySize;
export const registerSProgramDefinition = registerOsProgramDefinition;
export const registerSFixtureJsonResolver = registerOsFixtureJsonResolver;
export const sMediaGraphToDagFixtureJson = osMediaGraphToDagFixtureJson;
export const applyDagFixtureJsonToSMediaGraph = applyDagFixtureJsonToOsMediaGraph;
export const listSPrograms = listOsPrograms;
export {
	appInstanceResourceProjection,
	materializeAppInstanceProjection,
	registerAppVcsHandler,
	createTypedAppVcsHandler,
	createFlowDocumentAppVcsHandler,
	createFlowDagAppVcsHandler,
	createProcedural2dAppVcsHandler,
	createProcedural3dAppVcsHandler,
	createShootingAppVcsHandler,
	createTrinityGraphAppVcsHandler,
	createGisMapAppVcsHandler,
	createPresentationDeckAppVcsHandler,
	createPuzzle2dAppVcsHandler,
	createPuzzle3dAppVcsHandler,
	createPuzzle5dAppVcsHandler,
	createSequenceAppVcsHandler,
	createLayoutAppVcsHandler,
	createImperativeAppVcsHandler,
	createLowpolyAppVcsHandler,
	createVcsDemoAppVcsHandler,
	createCatalogueKindsAppVcsHandler,
	applyAppOperationToSource,
	createAppSourceDocument,
	appVcsHandlerForFormat,
	resolvePayloadRef,
	mediaPortIdForSpec,
	mediaPortSpecId,
	resourcesCompatible,
	emptyMediaGraph,
	validateMediaGraph,
	osOutPort as sOutPort,
	osInPort as sInPort,
	OsMediaGraphVirtualFileSystemController,
	OS_MEDIA_GRAPH_VFS_ROOT_ID,
	registerOsMediaExportHandler,
	assertOsMediaExportCoverage,
	exportOsAppInstanceMedia,
	type OsMediaExportFormat,
	type OsMediaExportResult,
};
//#endregion 🔖SAliases

//#region 🔖MediaExportHandlers
import { rasterizeSvgMarkupToPngDataUrl } from "@semio-tech/kernel-2d-js";

function mediaExportFileName(base: string, format: OsMediaExportFormat): string {
	return `${base}.${format === "glb" ? "glb" : format}`;
}

/** @emoji 💾 Rasterizes SVG markup to a PNG data URL in the browser. */
export { rasterizeSvgMarkupToPngDataUrl } from "@semio-tech/kernel-2d-js";

function registerSvgPngResourceHandlers(
	resourceKind: SResourceKindId,
	base: string,
	toSvg: (doc: unknown) => string,
	toPng?: (doc: unknown) => Promise<string>,
): void {
	registerOsMediaExportHandler(resourceKind, "svg", async (doc) => ({
		data: toSvg(doc),
		mimeType: "image/svg+xml",
		fileName: mediaExportFileName(base, "svg"),
	}));
	registerOsMediaExportHandler(resourceKind, "png", async (doc) => {
		const png = toPng ? await toPng(doc) : await rasterizeSvgMarkupToPngDataUrl(toSvg(doc), 1024, 1024);
		const data = png.startsWith("data:") ? await (await fetch(png)).blob().then((b) => b.arrayBuffer()).then((b) => new Uint8Array(b)) : png;
		return {
			data: typeof data === "string" ? data : data,
			mimeType: "image/png",
			fileName: mediaExportFileName(base, "png"),
		};
	});
}

function registerGlbObjResourceHandlers(
	resourceKind: SResourceKindId,
	base: string,
	toObj: (doc: unknown) => Promise<string>,
	toGlb: (doc: unknown) => Promise<Uint8Array>,
): void {
	registerOsMediaExportHandler(resourceKind, "obj", async (doc) => ({
		data: await toObj(doc),
		mimeType: "text/plain",
		fileName: mediaExportFileName(base, "obj"),
	}));
	registerOsMediaExportHandler(resourceKind, "glb", async (doc) => ({
		data: await toGlb(doc),
		mimeType: "model/gltf-binary",
		fileName: mediaExportFileName(base, "glb"),
	}));
}

/** @emoji 💾 Registers export handlers for all 2d/3d/5d media resource kinds. */
export async function registerAllMediaExportHandlers(): Promise<void> {
	const [
		draw,
		raster,
		note,
		gis,
		procedural2d,
		shooting,
		layout,
		presentation,
		cad,
		lowpoly,
		procedural3d,
		puzzle2d,
		puzzle3d,
	] = await Promise.all([
		import("@semio-tech/draw-core"),
		import("@semio-tech/raster-core"),
		import("@semio-tech/note-core"),
		import("@semio-tech/gis-2d-core"),
		import("@semio-tech/procedural-2d-core"),
		import("@semio-tech/shooting-core"),
		import("@semio-tech/layout-core"),
		import("@semio-tech/framework-presentation-core"),
		import("@semio-tech/cad-js-renderer-core"),
		import("@semio-tech/lowpoly-core"),
		import("@semio-tech/procedural-3d-core"),
		import("@semio-tech/puzzle-2d-core"),
		import("@semio-tech/puzzle-3d-core"),
	]);
	draw.registerDrawMediaExportHandlers();
	raster.registerRasterMediaExportHandlers();
	note.registerNoteMediaExportHandlers();
	gis.registerGisMediaExportHandlers();
	procedural2d.registerProcedural2dMediaExportHandlers();
	shooting.registerShootingMediaExportHandlers();
	layout.registerLayoutMediaExportHandlers();
	presentation.registerPresentationMediaExportHandlers();
	cad.registerCadMediaExportHandlers();
	lowpoly.registerLowpolyMediaExportHandlers();
	procedural3d.registerProcedural3dMediaExportHandlers();
	puzzle2d.registerPuzzle2dMediaExportHandlers();
	puzzle3d.registerPuzzle3dMediaExportHandlers();
	registerGlbObjResourceHandlers(
		"5d.puzzle",
		"puzzle5d",
		async (doc) => {
			const [{ project3d }, { exportPuzzle3dFixtureObj }] = await Promise.all([
				import("@semio-tech/puzzle-5d-react"),
				import("@semio-tech/puzzle-3d-core"),
			]);
			return exportPuzzle3dFixtureObj(project3d(doc as Parameters<typeof project3d>[0]));
		},
		async (doc) => {
			const [{ project3d }, { exportPuzzle3dFixtureGlb }] = await Promise.all([
				import("@semio-tech/puzzle-5d-react"),
				import("@semio-tech/puzzle-3d-core"),
			]);
			return exportPuzzle3dFixtureGlb(project3d(doc as Parameters<typeof project3d>[0]));
		},
	);
	registerSvgPngResourceHandlers("3d.mesh", "mesh", (doc) => {
		const mesh = doc as { readonly url?: string };
		return `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 256 256"><text x="8" y="24">${mesh.url ?? "mesh"}</text></svg>`;
	});
	registerGlbObjResourceHandlers(
		"3d.mesh",
		"mesh",
		async (doc) => {
			const mesh = doc as { readonly url?: string };
			return `# mesh reference\n# ${mesh.url ?? ""}\n`;
		},
		async () => new Uint8Array([0x67, 0x6c, 0x54, 0x46, 0x02, 0x00, 0x00, 0x00]),
	);
}
//#endregion 🔖MediaExportHandlers

//#region 🔖SProgramRegistry
function sBaselineResource(
	resourceKind: SResourceKindId,
	sourceFormat: string,
	componentKind: ComponentKind,
	modes: readonly { readonly id: string; readonly label: string }[] = [{ id: "edit", label: "Edit" }],
): Omit<SAppRegistration, "id" | "label"> & { readonly modes: readonly { readonly id: string; readonly label: string }[] } {
	return osBaselineResource(resourceKind, sourceFormat, componentKind, modes);
}

export const COMPOSE_SKETCHPAD_PROGRAM_ID = "compose.sketchpad" as const;

const SKETCHPAD_APP_RESOURCE: Readonly<Record<string, SAppResourceSpec>> = {
	home: { inputs: [], outputs: [osOutPort("kit.compose")], sourceFormat: "compose.kit", componentKind: "virtualFileSystem", modes: [{ id: "explore", label: "Explore" }] },
	kit: { inputs: [], outputs: [osOutPort("kit.compose")], sourceFormat: "compose.kit", componentKind: "virtualFileSystem", modes: [{ id: "explore", label: "Explore" }] },
	design: { inputs: [], outputs: [osOutPort("5d.puzzle")], sourceFormat: "compose.design", componentKind: "puzzle5d", modes: [{ id: "edit", label: "Edit" }] },
	type: { inputs: [], outputs: [osOutPort("3d.puzzle")], sourceFormat: "compose.type", componentKind: "puzzle3d", modes: [{ id: "edit", label: "Edit" }] },
	docs: { inputs: [], outputs: [osOutPort("text.document")], sourceFormat: "writer.document", componentKind: "panel", modes: [{ id: "explore", label: "Explore" }] },
	feedback: { inputs: [], outputs: [osOutPort("form.dictionary")], sourceFormat: "forms.dictionary", componentKind: "panel", modes: [{ id: "explore", label: "Explore" }] },
};

export const TECHNOLOGY_APP_RESOURCE_BY_PROGRAM: Readonly<Record<string, Readonly<Record<string, SAppResourceSpec>>>> = {
	draw: { draw: sBaselineResource("2d.drawing", "draw.document", "draw") },
	note: { note: sBaselineResource("2d.note", "note.document", "note") },
	writer: { writer: sBaselineResource("text.document", "writer.document", "writer") },
	raster: { raster: sBaselineResource("2d.raster", "raster.document", "raster") },
	flow: { flow: sBaselineResource("computation.flow", "flow.document", "flow") },
	"puzzle.2d": { puzzle2d: sBaselineResource("2d.puzzle", "puzzle.2d", "puzzle2d") },
	"puzzle.3d": { puzzle3d: sBaselineResource("3d.puzzle", "puzzle.3d", "puzzle3d") },
	"puzzle.5d": {
		puzzle5d: {
			inputs: [osInPort("catalogue.kinds", "catalogue", "Catalogue")],
			outputs: [osOutPort("2d.puzzle", "graph2d", "2D Graph"), osOutPort("3d.mesh", "mesh3d", "3D Mesh")],
			sourceFormat: "puzzle.5d",
			componentKind: "puzzle5d",
			modes: [{ id: "edit", label: "Edit" }],
		},
	},
	trinity: {
		"trinity-jack": sBaselineResource("graph.trinity", "trinity.graph", "trinity", [{ id: "query", label: "Query" }]),
	},
	"trinity.rewrite": {
		"trinity-rewrite": sBaselineResource("graph.trinity", "trinity.graph", "trinityRewrite", [{ id: "edit", label: "Edit" }]),
	},
	forms: { forms: sBaselineResource("form.dictionary", "forms.form", "forms") },
	shooting: {
		shooting: {
			inputs: [osInPort("3d.mesh", "mesh", "Mesh")],
			outputs: [osOutPort("2d.shooting")],
			sourceFormat: "shooting.scene",
			componentKind: "shooting",
			modes: [{ id: "edit", label: "Edit" }],
		},
	},
	"gis.map": { map: sBaselineResource("2d.map", "gis.map", "gismap") },
	cad: { cad: sBaselineResource("3d.cad", "cad.scene", "cad") },
	dag: { dag: sBaselineResource("graph.dag", "flow.dag", "dag") },
	"procedural.2d": { procedural2d: sBaselineResource("2d.procedural", "procedural.2d", "puzzle2d") },
	"procedural.3d": { procedural3d: sBaselineResource("3d.procedural", "procedural.3d", "puzzle3d") },
	"reasoning.wires": { wires: sBaselineResource("2d.puzzle", "puzzle.2d", "puzzle2d") },
	"reasoning.mindmap": { mindmap: sBaselineResource("2d.puzzle", "puzzle.2d", "puzzle2d", [{ id: "explore", label: "Explore" }]) },
	presentation: { presentation: sBaselineResource("presentation.deck", "presentation.deck", "panel", [{ id: "edit", label: "Edit" }]) },
	"presentation.deck": { "presentation.deck": sBaselineResource("presentation.deck", "presentation.deck", "panel", [{ id: "edit", label: "Edit" }]) },
	[COMPOSE_SKETCHPAD_PROGRAM_ID]: SKETCHPAD_APP_RESOURCE,
	lowpoly: { lowpoly: sBaselineResource("3d.lowpoly", "lowpoly.fixture", "lowpoly") },
	sequence: { sequence: sBaselineResource("computation.sequence", "sequence.fixture", "sequence") },
	layout: { layout: sBaselineResource("2d.layout", "layout.fixture/v1", "layout") },
	imperative: { imperative: sBaselineResource("computation.imperative", "imperative.document", "imperative") },
	vcs: { vcs: sBaselineResource("vcs.document", "vcs.demo/v1", "vcs", [{ id: "explore", label: "Explore" }]) },
};

const S_SYSTEM_PROGRAM: SProgramDefinition = {
	id: "s.system",
	name: "S System",
	apiVersion: "1",
	apps: [
		{
			id: "studio",
			label: "Studio",
			inputs: [],
			outputs: [osOutPort("graph.dag")],
			sourceFormat: "s.studio",
			componentKind: "s",
			modes: [{ id: "edit", label: "Edit" }],
			defaultModeId: "edit",
		},
		{
			id: "kit-catalogue",
			label: "Kit Catalogue",
			inputs: [],
			outputs: [osOutPort("catalogue.kinds", "out", "Kinds")],
			sourceFormat: "catalogue.kinds",
			componentKind: "catalogue",
			modes: [{ id: "browse", label: "Browse" }],
			defaultModeId: "browse",
		},
		{
			id: "files",
			label: "Files",
			inputs: [],
			outputs: [],
			sourceFormat: "os.storage",
			componentKind: "virtualFileSystem",
			modes: [{ id: "browse", label: "Browse" }],
			defaultModeId: "browse",
		},
	],
	createPlatformApi: (_ctx: PluginContext) => ({}),
};

registerOsBuiltinProgram(S_SYSTEM_PROGRAM);

export const COMPOSE_SKETCHPAD_PROGRAM: SProgramDefinition = {
	id: COMPOSE_SKETCHPAD_PROGRAM_ID,
	name: "Compose Sketchpad",
	apiVersion: "1",
	apps: Object.entries(SKETCHPAD_APP_RESOURCE).map(([id, resource]) => ({
		id,
		label: id.charAt(0).toUpperCase() + id.slice(1),
		...resource,
	})),
	createPlatformApi: () => ({}),
};

/** @emoji 🧩 Merges a technology {@link PlatformDefinition} into the s program registry with port metadata. */
export function mergeSProgramDefinition(
	programId: string,
	definition: PlatformDefinition,
	resourceByAppId?: Readonly<Record<string, SAppResourceSpec>>,
): void {
	const resources = resourceByAppId ?? TECHNOLOGY_APP_RESOURCE_BY_PROGRAM[programId];
	if (!resources) throw new Error(`unknown s program resource map: ${programId}`);
	mergeOsProgramDefinition(programId, definition, resources);
}

/** @emoji 🌱 Seeds the extension registry from {@link TECHNOLOGY_APP_RESOURCE_BY_PROGRAM} for tests and offline tooling. */
export function seedSProgramRegistryFromResourceMap(): void {
	seedOsProgramRegistryFromResourceMap(TECHNOLOGY_APP_RESOURCE_BY_PROGRAM);
}
//#endregion 🔖SProgramRegistry

//#region 🔖rust-studio
/** @emoji 🖥️ Rust-backed S studio store client over `@semio-tech/s-studio-rs` WASM. */
type WasmStudioStoreHandle = {
	dispatchJson(commandJson: string): void;
	projectionJson(): string;
	generation(): number;
};

let wasmInit: Promise<void> | null = null;
let WasmHandle: (new (documentJson: string) => WasmStudioStoreHandle) | null = null;

async function ensureSStudioWasm(): Promise<void> {
	if (WasmHandle) return;
	if (!wasmInit) {
		wasmInit = (async () => {
			const mod = await import("@semio-tech/s-studio-rs");
			const init = mod.default as (input?: RequestInfo | URL | Response | BufferSource | WebAssembly.Module) => Promise<unknown>;
			await init();
			WasmHandle = mod.StudioStoreHandle as new (documentJson: string) => WasmStudioStoreHandle;
		})();
	}
	await wasmInit;
}

/** @emoji 🗄️ Studio store delegating CQRS/materialization to `s_studio` Rust/WASM. */
export class RustStudioStore {
	private handle: WasmStudioStoreHandle | null = null;
	private listeners = new Set<() => void>();
	private generation = 0;
	private ready: Promise<void>;

	constructor(document: SStudioDocument) {
		this.ready = ensureSStudioWasm().then(() => {
			this.handle = new WasmHandle!(JSON.stringify(document));
			this.generation = this.handle.generation();
		});
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getGeneration(): number {
		return this.generation;
	}

	async dispatch(command: Record<string, unknown>): Promise<void> {
		await this.ready;
		if (!this.handle) throw new Error("s studio wasm handle missing");
		this.handle.dispatchJson(JSON.stringify(command));
		this.generation = this.handle.generation();
		for (const listener of this.listeners) listener();
	}

	async projection(): Promise<SStudioProjection> {
		await this.ready;
		if (!this.handle) throw new Error("s studio wasm handle missing");
		return JSON.parse(this.handle.projectionJson()) as SStudioProjection;
	}
}
//#endregion 🔖rust-studio

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it, beforeAll } = import.meta.vitest;

	beforeAll(async () => {
		seedSProgramRegistryFromResourceMap();
		const [
			{ createDrawAppVcsHandler },
			{ createNoteAppVcsHandler },
			{ createWriterAppVcsHandler },
			{ createRasterAppVcsHandler },
			{ createFormsAppVcsHandler },
			{ createPresentationAppVcsHandler },
		] = await Promise.all([
			import("@semio-tech/draw-core"),
			import("@semio-tech/note-core"),
			import("@semio-tech/writer-core"),
			import("@semio-tech/raster-core"),
			import("@semio-tech/forms-core"),
			import("@semio-tech/framework-presentation-core"),
		]);
		registerAppVcsHandler(createDrawAppVcsHandler());
		registerAppVcsHandler(createNoteAppVcsHandler());
		registerAppVcsHandler(createWriterAppVcsHandler());
		registerAppVcsHandler(createRasterAppVcsHandler());
		registerAppVcsHandler(createFormsAppVcsHandler());
		registerAppVcsHandler(createPresentationAppVcsHandler());
		await registerAllMediaExportHandlers();
	});

	describe("s studio", () => {
		it("spawns app instances through CQRS dispatch", () => {
			const store = new OsStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", sourceInline: "{}" });
			expect(store.projection().appInstances).toHaveLength(1);
			expect(store.projection().mediaGraph.nodes).toHaveLength(1);
		});

		it("lists expanded program catalog", () => {
			const ids = listSPrograms().map((program) => program.id);
			expect(ids).toContain("dag");
			expect(ids).toContain("s.system");
		});

		it("assertOsMediaExportCoverage passes after handlers register", () => {
			expect(() => assertOsMediaExportCoverage()).not.toThrow();
		});
	});
}
// #endregion 🧪Tests

