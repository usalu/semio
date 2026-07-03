// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji 🖥️ `@semio-tech/s-core` — S studio instance over {@link @semio-tech/framework-os-core}. */
// #endregion 🧲Header

import type { ComponentKind, PlatformDefinition, PluginContext } from "@semio-tech/framework-platform-core";
import {
	applyDagFixtureJsonToOsMediaGraph,
	applyFlowFixtureJsonToOsMediaGraph,
	buildOsMediaFlowOperatorInfos,
	appInstanceResourceProjection,
	appVcsHandlerForFormat,
	createCatalogueKindsAppVcsHandler,
	createEmptyOsDocument,
	createOsId,
	createTypedAppVcsHandler,
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
	osMediaGraphToFlowFixtureJson,
	OS_MEDIA_FLOW_MODULE_ID,
	osOutPort,
	osParameterTypesCompatible,
	osParameterValue,
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
	type OsParameterFieldSpec,
	type OsParameterType,
	type OsProgramDefinition,
	type OsProjection,
	type OsResourceDescriptor,
	type OsResourceKindId,
	OsStore,
	DevJsonBackbone,
	LocalJsonBackbone,
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
export type SParameterType = OsParameterType;
export type SParameterFieldSpec = OsParameterFieldSpec;
export type SParameter = import("@semio-tech/framework-os-core").OsParameter;
export type SParameterFieldBinding = import("@semio-tech/framework-os-core").OsParameterFieldBinding;
export { OsStore as StudioStore, DevJsonBackbone, LocalJsonBackbone, RemoteOsBackbone, buildOsHistoryColumns };
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
export const sMediaGraphToFlowFixtureJson = osMediaGraphToFlowFixtureJson;
export const applyDagFixtureJsonToSMediaGraph = applyDagFixtureJsonToOsMediaGraph;
export const applyFlowFixtureJsonToSMediaGraph = applyFlowFixtureJsonToOsMediaGraph;
export const buildSMediaFlowOperatorInfos = buildOsMediaFlowOperatorInfos;
export { OS_MEDIA_FLOW_MODULE_ID };
export const listSPrograms = listOsPrograms;
export {
	appInstanceResourceProjection,
	materializeAppInstanceProjection,
	registerAppVcsHandler,
	createTypedAppVcsHandler,
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
	osParameterTypesCompatible,
	osParameterValue,
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

/** @emoji 💾 Registers generic 3d.mesh media export handlers. */
export function registerGenericMeshMediaExportHandlers(): void {
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
function sParameterField(fieldPath: string, label: string, type: OsParameterType): OsParameterFieldSpec {
	return { fieldPath, label, type };
}

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
	kit: { inputs: [], outputs: [osOutPort("kit.compose")], sourceFormat: "compose.kit", componentKind: "virtualFileSystem", modes: [{ id: "explore", label: "Explore" }] },
	design: { inputs: [], outputs: [osOutPort("5d.puzzle")], sourceFormat: "compose.design", componentKind: "puzzle5d", modes: [{ id: "edit", label: "Edit" }] },
	type: { inputs: [], outputs: [osOutPort("3d.puzzle")], sourceFormat: "compose.type", componentKind: "puzzle3d", modes: [{ id: "edit", label: "Edit" }] },
	docs: { inputs: [], outputs: [osOutPort("text.document")], sourceFormat: "writer.document", componentKind: "panel", modes: [{ id: "explore", label: "Explore" }] },
	feedback: { inputs: [], outputs: [osOutPort("form.dictionary")], sourceFormat: "forms.dictionary", componentKind: "panel", modes: [{ id: "explore", label: "Explore" }] },
};

export const TECHNOLOGY_APP_RESOURCE_BY_PROGRAM: Readonly<Record<string, Readonly<Record<string, SAppResourceSpec>>>> = {
	[COMPOSE_SKETCHPAD_PROGRAM_ID]: SKETCHPAD_APP_RESOURCE,
};

const S_SYSTEM_PROGRAM: SProgramDefinition = {
	id: "s.system",
	name: "S System",
	apiVersion: "1",
	apps: [
		{
			id: "home",
			label: "Home",
			inputs: [],
			outputs: [],
			sourceFormat: "os.storage",
			componentKind: "virtualFileSystem",
			modes: [{ id: "explore", label: "Explore" }],
			defaultModeId: "explore",
		},
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
		const { loadAllOsProgramContributions } = await import("@semio-tech/framework-playground-core/app-registry");
		const contributions = await loadAllOsProgramContributions();
		for (const contribution of contributions) {
			await contribution.register();
		}
		registerGenericMeshMediaExportHandlers();
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

