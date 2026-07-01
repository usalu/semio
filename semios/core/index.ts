// #region 🧲Header
/// <reference types="vitest/importMeta" />
/** @emoji 🖥️ `@semio-tech/semios-core` — studio CQRS store, programs, resources, media graph, dev JSON backbone. */
// #endregion 🧲Header

import {
	createDocumentVcsEnvelope,
	createDocumentVcsId,
	type DocumentVcsEnvelope,
	DocumentVcsStore,
	materializeDocumentProjection,
} from "@semio-tech/vcs-core";
import {
	SEMIOSRESOURCES_DESCRIPTOR_IDS,
	SEMIOSRESOURCES_MANIFEST_DOCUMENT,
	type SemiosResourcesDescriptorKindId,
} from "@semio-tech/graph-manifest";
import type { ComponentKind, PlatformDefinition, PluginContext } from "@semio-tech/framework-platform-core";

//#region 🔖Schemas
export const SEMIOS_STUDIO_SCHEMA = "semios.studio/v1" as const;
export const SEMIOS_MEDIA_GRAPH_SCHEMA = "semios.media-graph/v1" as const;

export type SemiosResourceKindId = SemiosResourcesDescriptorKindId;
export { SEMIOSRESOURCES_DESCRIPTOR_IDS as SEMIOS_RESOURCE_KIND_IDS };

export interface SemiosBackboneRef {
	readonly kind: "dev" | "local" | "remote";
	readonly uri: string;
}

export interface StudioConflict {
	readonly kind: "studio-conflict";
	readonly uri: string;
	readonly message: string;
	readonly localRevision?: string;
	readonly remoteRevision?: string;
}

export interface SemiosSourceDocument {
	readonly format: string;
	readonly vcsJson?: string;
	readonly inline?: string;
	readonly payloadRef?: string;
}

export interface SemiosAppInstance {
	readonly id: string;
	readonly programId: string;
	readonly appId: string;
	readonly label: string;
	readonly yields: SemiosResourceKindId;
	readonly sourceDocument: SemiosSourceDocument;
}

export interface SemiosMediaPort {
	readonly id: string;
	readonly resourceKind: SemiosResourceKindId;
	readonly direction: "in" | "out";
}

export interface SemiosMediaGraphNode {
	readonly id: string;
	readonly instanceId: string;
	readonly x: number;
	readonly y: number;
	readonly width: number;
	readonly height: number;
	readonly inputs: readonly SemiosMediaPort[];
	readonly outputs: readonly SemiosMediaPort[];
}

export interface SemiosMediaGraphEdge {
	readonly id: string;
	readonly sourceNodeId: string;
	readonly sourcePortId: string;
	readonly targetNodeId: string;
	readonly targetPortId: string;
}

export interface SemiosMediaGraphV1 {
	readonly schema: typeof SEMIOS_MEDIA_GRAPH_SCHEMA;
	readonly nodes: readonly SemiosMediaGraphNode[];
	readonly edges: readonly SemiosMediaGraphEdge[];
}

export interface SemiosStudioProjection {
	readonly programs: readonly string[];
	readonly activeProgramId: string | null;
	readonly activeAlternativeId: string | null;
	readonly appInstances: readonly SemiosAppInstance[];
	readonly mediaGraph: SemiosMediaGraphV1;
}

export interface SemiosStudioOperation {
	readonly op:
		| "spawnAppInstance"
		| "removeAppInstance"
		| "connectMediaPorts"
		| "disconnectMediaEdge"
		| "moveMediaNode"
		| "patchAppInstance"
		| "patchAppSource"
		| "applyAppOperation"
		| "setStudioName"
		| "setActiveProgram"
		| "createAlternative"
		| "switchAlternative";
	readonly payload: Record<string, unknown>;
}

export interface SemiosStudioChange {
	readonly id: string;
	readonly forwards: readonly SemiosStudioOperation[];
	readonly backwards: readonly SemiosStudioOperation[];
	readonly description?: string;
	readonly savedAt?: string;
}

export interface SemiosStudioCheckpoint {
	readonly id: string;
	readonly changeIds: readonly string[];
	readonly message?: string;
	readonly savedAt: string;
}

export interface SemiosStudioAlternative {
	readonly id: string;
	readonly name: string;
	readonly checkpointIds: readonly string[];
}

export interface SemiosStudioVcs {
	readonly initialProjection: SemiosStudioProjection;
	readonly operations: readonly SemiosStudioChange[];
	readonly checkpoints: readonly SemiosStudioCheckpoint[];
	readonly alternatives: readonly SemiosStudioAlternative[];
}

export interface SemiosStudioDocumentV1 {
	readonly schema: typeof SEMIOS_STUDIO_SCHEMA;
	readonly id: string;
	readonly name: string;
	readonly vcs: SemiosStudioVcs;
	readonly backbone?: SemiosBackboneRef;
}
//#endregion 🔖Schemas

//#region 🔖ProgramRegistry
export interface SemiosAppRegistration {
	readonly id: string;
	readonly label: string;
	readonly yields: SemiosResourceKindId;
	readonly sourceFormat: string;
	readonly componentKind: ComponentKind;
	readonly defaultModeId?: string;
}

export interface SemiosProgramDefinition extends PlatformDefinition {
	readonly apps: readonly (SemiosAppRegistration & { readonly modes: readonly { readonly id: string; readonly label: string }[] })[];
}

export interface SemiosResourceDescriptor {
	readonly kind: SemiosResourceKindId;
	readonly name: string;
	readonly sourceFormat: string;
	readonly componentKind: ComponentKind;
	readonly dimension: string;
}

function descriptorPresentation(kind: SemiosResourceKindId): SemiosResourceDescriptor {
	const row = SEMIOSRESOURCES_MANIFEST_DOCUMENT.descriptorKinds?.find((entry) => entry.id === kind);
	const presentation = (row?.presentation ?? {}) as Record<string, string>;
	return {
		kind,
		name: row?.name ?? kind,
		sourceFormat: presentation.sourceFormat ?? kind,
		componentKind: (presentation.componentKind ?? "panel") as ComponentKind,
		dimension: presentation.dimension ?? "unknown",
	};
}

export function listSemiosResourceDescriptors(): readonly SemiosResourceDescriptor[] {
	return SEMIOSRESOURCES_DESCRIPTOR_IDS.map(descriptorPresentation);
}

export function semiosResourceDescriptor(kind: SemiosResourceKindId): SemiosResourceDescriptor {
	return descriptorPresentation(kind);
}

export function resourcesCompatible(left: SemiosResourceKindId, right: SemiosResourceKindId): boolean {
	return left === right;
}

const SEMIOS_SYSTEM_PROGRAM: SemiosProgramDefinition = {
	id: "semios.system",
	name: "Semios System",
	apiVersion: "1",
	apps: [
		{
			id: "studio",
			label: "Studio",
			yields: "graph.dag",
			sourceFormat: "semios.studio/v1",
			componentKind: "semios",
			modes: [{ id: "edit", label: "Edit" }],
			defaultModeId: "edit",
		},
	],
	createPlatformApi: (_ctx: PluginContext) => ({}),
};

const TECHNOLOGY_PLAY_PROGRAMS: readonly SemiosProgramDefinition[] = [
	{
		id: "draw",
		name: "Draw",
		apiVersion: "1",
		apps: [{ id: "draw", label: "Draw", yields: "2d.drawing", sourceFormat: "draw.document/v1", componentKind: "draw", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "writer",
		name: "Writer",
		apiVersion: "1",
		apps: [{ id: "writer", label: "Writer", yields: "text.document", sourceFormat: "writer.document/v1", componentKind: "writer", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "raster",
		name: "Raster",
		apiVersion: "1",
		apps: [{ id: "raster", label: "Raster", yields: "2d.raster", sourceFormat: "raster.document/v1", componentKind: "raster", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "flow",
		name: "Flow",
		apiVersion: "1",
		apps: [{ id: "flow", label: "Flow", yields: "computation.flow", sourceFormat: "flow.document/v1", componentKind: "flow", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "puzzle.2d",
		name: "Puzzle 2D",
		apiVersion: "1",
		apps: [{ id: "puzzle2d", label: "Puzzle 2D", yields: "2d.puzzle", sourceFormat: "puzzle.2d/v1", componentKind: "puzzle2d", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "puzzle.3d",
		name: "Puzzle 3D",
		apiVersion: "1",
		apps: [{ id: "puzzle3d", label: "Puzzle 3D", yields: "3d.puzzle", sourceFormat: "puzzle.3d/v1", componentKind: "puzzle3d", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "puzzle.5d",
		name: "Puzzle 5D",
		apiVersion: "1",
		apps: [{ id: "puzzle5d", label: "Puzzle 5D", yields: "5d.puzzle", sourceFormat: "puzzle.5d/v1", componentKind: "puzzle5d", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "trinity",
		name: "Trinity",
		apiVersion: "1",
		apps: [{ id: "trinity", label: "Trinity", yields: "graph.trinity", sourceFormat: "trinity.graph/v1", componentKind: "trinity", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "forms",
		name: "Forms",
		apiVersion: "1",
		apps: [{ id: "forms", label: "Forms", yields: "form.dictionary", sourceFormat: "forms.form/v1", componentKind: "forms", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "shooting",
		name: "Shooting",
		apiVersion: "1",
		apps: [{ id: "shooting", label: "Shooting", yields: "2d.shooting", sourceFormat: "shooting.scene/v1", componentKind: "shooting", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "gis.map",
		name: "GIS Map",
		apiVersion: "1",
		apps: [{ id: "map", label: "Map", yields: "2d.map", sourceFormat: "gis.map/v1", componentKind: "gismap", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "cad",
		name: "CAD",
		apiVersion: "1",
		apps: [{ id: "cad", label: "CAD", yields: "3d.cad", sourceFormat: "cad.scene/v1", componentKind: "cad", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "dag",
		name: "DAG",
		apiVersion: "1",
		apps: [{ id: "dag", label: "DAG", yields: "graph.dag", sourceFormat: "flow.dag/v1", componentKind: "dag", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "procedural.2d",
		name: "Procedural 2D",
		apiVersion: "1",
		apps: [{ id: "procedural2d", label: "Procedural 2D", yields: "2d.procedural", sourceFormat: "procedural.2d/v1", componentKind: "puzzle2d", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "procedural.3d",
		name: "Procedural 3D",
		apiVersion: "1",
		apps: [{ id: "procedural3d", label: "Procedural 3D", yields: "3d.procedural", sourceFormat: "procedural.3d/v1", componentKind: "puzzle3d", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "reasoning.wires",
		name: "Reasoning Wires",
		apiVersion: "1",
		apps: [{ id: "wires", label: "Wires", yields: "2d.puzzle", sourceFormat: "puzzle.2d/v1", componentKind: "puzzle2d", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
	{
		id: "presentation",
		name: "Presentation",
		apiVersion: "1",
		apps: [{ id: "presentation", label: "Presentation", yields: "presentation.deck", sourceFormat: "presentation.deck/v1", componentKind: "panel", modes: [{ id: "edit", label: "Edit" }] }],
		createPlatformApi: () => ({}),
	},
];

const SKETCHPAD_APP_RESOURCE: Readonly<Record<string, Omit<SemiosAppRegistration, "id" | "label"> & { readonly modes: readonly { readonly id: string; readonly label: string }[] }>> = {
	home: { yields: "kit.compose", sourceFormat: "compose.kit/v1", componentKind: "virtualFileSystem", modes: [{ id: "explore", label: "Explore" }] },
	kit: { yields: "kit.compose", sourceFormat: "compose.kit/v1", componentKind: "virtualFileSystem", modes: [{ id: "explore", label: "Explore" }] },
	design: { yields: "5d.puzzle", sourceFormat: "compose.design/v1", componentKind: "puzzle5d", modes: [{ id: "edit", label: "Edit" }] },
	type: { yields: "3d.puzzle", sourceFormat: "compose.type/v1", componentKind: "puzzle3d", modes: [{ id: "edit", label: "Edit" }] },
	docs: { yields: "text.document", sourceFormat: "writer.document/v1", componentKind: "panel", modes: [{ id: "explore", label: "Explore" }] },
	feedback: { yields: "form.dictionary", sourceFormat: "forms.dictionary/v1", componentKind: "panel", modes: [{ id: "explore", label: "Explore" }] },
};

let composeSketchpadProgramOverride: SemiosProgramDefinition | null = null;

export const COMPOSE_SKETCHPAD_PROGRAM_ID = "compose.sketchpad" as const;

export const COMPOSE_SKETCHPAD_PROGRAM: SemiosProgramDefinition = {
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

/** @emoji 🧩 Merges compose sketchpad {@link PlatformDefinition} apps into the semios program registry. */
export function mergeComposeSketchpadProgramDefinition(definition: PlatformDefinition): void {
	composeSketchpadProgramOverride = {
		id: COMPOSE_SKETCHPAD_PROGRAM_ID,
		name: definition.name ?? COMPOSE_SKETCHPAD_PROGRAM.name,
		apiVersion: definition.apiVersion ?? "1",
		apps: definition.apps.map((app) => {
			const resource = SKETCHPAD_APP_RESOURCE[app.id] ?? SKETCHPAD_APP_RESOURCE.home!;
			return {
				id: app.id,
				label: app.label,
				yields: resource.yields,
				sourceFormat: resource.sourceFormat,
				componentKind: resource.componentKind,
				modes: app.modes.length > 0 ? app.modes : resource.modes,
			};
		}),
		createPlatformApi: () => ({}),
	};
}

function composeSketchpadProgram(): SemiosProgramDefinition {
	return composeSketchpadProgramOverride ?? COMPOSE_SKETCHPAD_PROGRAM;
}

const PROGRAMS: readonly SemiosProgramDefinition[] = [SEMIOS_SYSTEM_PROGRAM, ...TECHNOLOGY_PLAY_PROGRAMS];

export function listSemiosPrograms(): readonly SemiosProgramDefinition[] {
	return [SEMIOS_SYSTEM_PROGRAM, composeSketchpadProgram(), ...TECHNOLOGY_PLAY_PROGRAMS];
}

export function semiosProgramById(programId: string): SemiosProgramDefinition | undefined {
	return listSemiosPrograms().find((program) => program.id === programId);
}

export function semiosAppRegistration(programId: string, appId: string): SemiosAppRegistration | undefined {
	const program = semiosProgramById(programId);
	return program?.apps.find((app) => app.id === appId);
}
//#endregion 🔖ProgramRegistry

//#region 🔖AppVcsRegistry
export interface AppVcsHandler<TProjection = unknown, TOp = unknown> {
	readonly format: string;
	readonly createEnvelope: (id: string) => DocumentVcsEnvelope<TProjection, TOp>;
	readonly applyOp: (projection: TProjection, operation: TOp) => TProjection;
	readonly backwardsOp?: (projection: TProjection, operation: TOp) => readonly TOp[];
	readonly serializeEnvelope: (envelope: DocumentVcsEnvelope<TProjection, TOp>) => string;
	readonly deserializeEnvelope: (json: string) => DocumentVcsEnvelope<TProjection, TOp>;
	readonly materializeProjection: (source: SemiosSourceDocument) => TProjection;
}

const appVcsHandlers = new Map<string, AppVcsHandler>();

/** @emoji 📚 Registers a technology document VCS handler for semios applyAppOperation dispatch. */
export function registerAppVcsHandler<TProjection, TOp>(handler: AppVcsHandler<TProjection, TOp>): void {
	appVcsHandlers.set(handler.format, handler as AppVcsHandler);
}

export function appVcsHandlerForFormat(format: string): AppVcsHandler | undefined {
	return appVcsHandlers.get(format);
}

/** @emoji 📦 Typed VCS handler factory for technologies without a dedicated core export yet. */
export function createTypedAppVcsHandler<TProjection, TOp>(
	format: string,
	schema: string,
	empty: () => TProjection,
	applyOp: (projection: TProjection, operation: TOp) => TProjection,
	backwardsOp?: (projection: TProjection, operation: TOp) => readonly TOp[],
): AppVcsHandler<TProjection, TOp> {
	return {
		format,
		createEnvelope: (id) => createDocumentVcsEnvelope<TProjection, TOp>(schema, id, empty()),
		applyOp,
		serializeEnvelope: (envelope) => JSON.stringify(envelope),
		deserializeEnvelope: (json) => JSON.parse(json) as DocumentVcsEnvelope<TProjection, TOp>,
		materializeProjection: (source) => {
			if (source.vcsJson) {
				const envelope = JSON.parse(source.vcsJson) as DocumentVcsEnvelope<TProjection, TOp>;
				const appliedIds = envelope.vcs.edits.map((edit) => edit.id);
				return materializeDocumentProjection(envelope, appliedIds, applyOp);
			}
			if (source.inline) return JSON.parse(source.inline) as TProjection;
			return empty();
		},
		...(backwardsOp ? { backwardsOp } : {}),
	};
}

type ShootingScene = { readonly schema: string; readonly id: string; readonly entities: readonly { readonly id: string; readonly label: string }[] };
type ShootingOp =
	| { readonly op: "addEntity"; readonly entity: { readonly id: string; readonly label: string } }
	| { readonly op: "removeEntity"; readonly entityId: string };

function applyShootingOp(scene: ShootingScene, op: ShootingOp): ShootingScene {
	switch (op.op) {
		case "addEntity":
			return { ...scene, entities: [...scene.entities, op.entity] };
		case "removeEntity":
			return { ...scene, entities: scene.entities.filter((entity) => entity.id !== op.entityId) };
	}
}

/** @emoji 🌊 Semios app VCS handler for flow documents. */
export function createFlowDocumentAppVcsHandler() {
	return createTypedAppVcsHandler("flow.document/v1", "flow.document/v1", () => ({ flow: {}, tree: {} }), (doc, op) => {
		if (op.op === "setFlow") return { ...doc, flow: op.flow };
		return { ...doc, tree: op.tree };
	});
}

/** @emoji 🌳 Semios app VCS handler for DAG documents. */
export function createFlowDagAppVcsHandler() {
	type DagDoc = { readonly nodes: readonly unknown[]; readonly edges: readonly unknown[] };
	type DagOp = { readonly op: "setNodes"; readonly nodes: readonly unknown[] } | { readonly op: "setEdges"; readonly edges: readonly unknown[] };
	return createTypedAppVcsHandler<DagDoc, DagOp>("flow.dag/v1", "flow.dag/v1", () => ({ nodes: [], edges: [] }), (doc, op) => {
		if (op.op === "setNodes") return { ...doc, nodes: op.nodes };
		return { ...doc, edges: op.edges };
	});
}

/** @emoji 📏 Semios app VCS handler for procedural 2d documents. */
export function createProcedural2dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("procedural.2d/v1", "procedural.2d/v1", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

/** @emoji 📐 Semios app VCS handler for procedural 3d documents. */
export function createProcedural3dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("procedural.3d/v1", "procedural.3d/v1", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

/** @emoji 📸 Semios app VCS handler for shooting scene documents. */
export function createShootingAppVcsHandler() {
	return createTypedAppVcsHandler<ShootingScene, ShootingOp>(
		"shooting.scene/v1",
		"shooting.scene/v1",
		() => ({ schema: "shooting.scene/v1", id: "shooting", entities: [] }),
		applyShootingOp,
	);
}

/** @emoji 🔺 Semios app VCS handler for trinity graph documents. */
export function createTrinityGraphAppVcsHandler() {
	type Doc = { readonly nodes: readonly unknown[] };
	type Op = { readonly op: "setNodes"; readonly nodes: readonly unknown[] };
	return createTypedAppVcsHandler<Doc, Op>("trinity.graph/v1", "trinity.graph/v1", () => ({ nodes: [] }), (doc, op) => ({ nodes: op.nodes }));
}

/** @emoji 🗺️ Semios app VCS handler for GIS map documents. */
export function createGisMapAppVcsHandler() {
	type Doc = { readonly layers: readonly unknown[] };
	type Op = { readonly op: "setLayers"; readonly layers: readonly unknown[] };
	return createTypedAppVcsHandler<Doc, Op>("gis.map/v1", "gis.map/v1", () => ({ layers: [] }), (doc, op) => ({ layers: op.layers }));
}

/** @emoji 📽 Semios app VCS handler for presentation deck documents. */
export function createPresentationDeckAppVcsHandler() {
	type Tile = { readonly id: string; readonly name: string };
	type Doc = { readonly schema: string; readonly tiles: readonly Tile[] };
	type Op = { readonly op: "addTile"; readonly tile: Tile } | { readonly op: "removeTile"; readonly tileId: string };
	return createTypedAppVcsHandler<Doc, Op>(
		"presentation.deck/v1",
		"presentation.deck/v1",
		() => ({ schema: "presentation.deck/v1", tiles: [] }),
		(doc, op) => {
			if (op.op === "addTile") return { ...doc, tiles: [...doc.tiles, op.tile] };
			return { ...doc, tiles: doc.tiles.filter((tile) => tile.id !== op.tileId) };
		},
	);
}

/** @emoji 🩻 Semios app VCS handler for puzzle 2d documents. */
export function createPuzzle2dAppVcsHandler() {
	type Doc = { readonly nodes: readonly string[] };
	type Op = { readonly op: "addNode"; readonly nodeId: string } | { readonly op: "removeNode"; readonly nodeId: string };
	return createTypedAppVcsHandler<Doc, Op>("puzzle.2d/v1", "puzzle.2d/v1", () => ({ nodes: [] }), (doc, op) => {
		if (op.op === "addNode") return { ...doc, nodes: [...doc.nodes, op.nodeId] };
		return { ...doc, nodes: doc.nodes.filter((id) => id !== op.nodeId) };
	});
}

/** @emoji 🏙️ Semios app VCS handler for puzzle 3d documents. */
export function createPuzzle3dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("puzzle.3d/v1", "puzzle.3d/v1", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

/** @emoji 👯 Semios app VCS handler for puzzle 5d documents. */
export function createPuzzle5dAppVcsHandler() {
	type Doc = { readonly revision: number };
	type Op = { readonly op: "setRevision"; readonly revision: number };
	return createTypedAppVcsHandler<Doc, Op>("puzzle.5d/v1", "puzzle.5d/v1", () => ({ revision: 0 }), (doc, op) => ({ revision: op.revision }));
}

registerAppVcsHandler(createFlowDocumentAppVcsHandler());
registerAppVcsHandler(createFlowDagAppVcsHandler());
registerAppVcsHandler(createProcedural2dAppVcsHandler());
registerAppVcsHandler(createProcedural3dAppVcsHandler());
registerAppVcsHandler(createShootingAppVcsHandler());
registerAppVcsHandler(createTrinityGraphAppVcsHandler());
registerAppVcsHandler(createGisMapAppVcsHandler());
registerAppVcsHandler(createPresentationDeckAppVcsHandler());
registerAppVcsHandler(createPuzzle2dAppVcsHandler());
registerAppVcsHandler(createPuzzle3dAppVcsHandler());
registerAppVcsHandler(createPuzzle5dAppVcsHandler());
registerAppVcsHandler(
	createTypedAppVcsHandler<
		{ readonly schema: string; readonly id: string; readonly nodes: readonly { readonly id: string; readonly label: string }[] },
		| { readonly op: "addNode"; readonly node: { readonly id: string; readonly label: string } }
		| { readonly op: "removeNode"; readonly nodeId: string }
	>("cad.scene/v1", "cad.scene/v1", () => ({ schema: "cad.scene/v1", id: "cad", nodes: [] }), (doc, op) => {
		if (op.op === "addNode") return { ...doc, nodes: [...doc.nodes, op.node] };
		return { ...doc, nodes: doc.nodes.filter((node) => node.id !== op.nodeId) };
	}),
);
registerAppVcsHandler(
	createTypedAppVcsHandler<{ readonly id: string }, { readonly op: "setId"; readonly id: string }>(
		"compose.design/v1",
		"compose.design/v1",
		() => ({ id: "design" }),
		(doc, op) => ({ id: op.id }),
	),
);
registerAppVcsHandler(
	createTypedAppVcsHandler<{ readonly id: string }, { readonly op: "setId"; readonly id: string }>(
		"compose.type/v1",
		"compose.type/v1",
		() => ({ id: "type" }),
		(doc, op) => ({ id: op.id }),
	),
);
registerAppVcsHandler(
	createTypedAppVcsHandler<{ readonly id: string }, { readonly op: "setId"; readonly id: string }>(
		"compose.kit/v1",
		"compose.kit/v1",
		() => ({ id: "kit" }),
		(doc, op) => ({ id: op.id }),
	),
);

export function resolvePayloadRef(payloadRef: string): string | null {
	if (payloadRef.startsWith("fixture:")) return payloadRef.slice("fixture:".length);
	if (payloadRef.startsWith("upstream:")) return payloadRef.slice("upstream:".length);
	return null;
}

export interface AppMaterializeContext {
	readonly resolveFixtureJson?: (slug: string) => string | null;
	readonly graph?: SemiosMediaGraphV1;
	readonly instances?: readonly SemiosAppInstance[];
}

let semiosFixtureJsonResolver: ((slug: string) => string | null) | null = null;

/** @emoji 📎 Registers bundled fixture JSON lookup for payloadRef materialization. */
export function registerSemiosFixtureJsonResolver(resolver: (slug: string) => string | null): void {
	semiosFixtureJsonResolver = resolver;
}

function resolveSourceDocument(source: SemiosSourceDocument, context?: AppMaterializeContext): SemiosSourceDocument {
	const resolveFixture = context?.resolveFixtureJson ?? semiosFixtureJsonResolver ?? undefined;
	if (source.payloadRef?.startsWith("fixture:") && resolveFixture) {
		const slug = resolvePayloadRef(source.payloadRef);
		const json = slug ? resolveFixture(slug) : null;
		if (json) return { ...source, inline: json };
	}
	if (source.payloadRef?.startsWith("upstream:") && context?.instances) {
		const upstreamId = source.payloadRef.slice("upstream:".length);
		const upstream = context.instances.find((entry) => entry.id === upstreamId);
		if (upstream) return upstream.sourceDocument;
	}
	return source;
}

export function createAppSourceDocument(
	format: string,
	instanceId: string,
	options?: { readonly payloadRef?: string; readonly inline?: string },
): SemiosSourceDocument {
	const handler = appVcsHandlerForFormat(format);
	const envelope = handler?.createEnvelope(instanceId);
	return {
		format,
		vcsJson: envelope ? handler!.serializeEnvelope(envelope) : undefined,
		inline: options?.inline,
		payloadRef: options?.payloadRef,
	};
}

export function materializeAppInstanceProjection(instance: SemiosAppInstance, context?: AppMaterializeContext): unknown {
	const source = resolveSourceDocument(instance.sourceDocument, context);
	const handler = appVcsHandlerForFormat(source.format);
	if (!handler) {
		if (source.inline) return JSON.parse(source.inline);
		return null;
	}
	return handler.materializeProjection(source);
}

export function applyAppOperationToSource(
	source: SemiosSourceDocument,
	forwards: readonly unknown[],
	backwards: readonly unknown[],
): SemiosSourceDocument {
	const handler = appVcsHandlerForFormat(source.format);
	if (!handler) throw new Error(`no app VCS handler for ${source.format}`);
	const envelope = source.vcsJson ? handler.deserializeEnvelope(source.vcsJson) : handler.createEnvelope(createDocumentVcsId("app"));
	const store = new DocumentVcsStore({
		envelope,
		applyOp: handler.applyOp as (projection: unknown, operation: unknown) => unknown,
		backwardsOp: (handler.backwardsOp ?? ((projection, _operation) => [{ op: "setDocument", document: projection }])) as (
			projection: unknown,
			operation: unknown,
		) => readonly unknown[],
		diffOp: (_projection, operation) => operation,
	});
	store.dispatch({ kind: "apply", operations: forwards as never[] });
	return { ...source, vcsJson: handler.serializeEnvelope(store.getEnvelope()) };
}
//#endregion 🔖AppVcsRegistry

//#region 🔖MediaGraphEngine
export interface SemiosMediaGraphValidation {
	readonly ok: boolean;
	readonly errors: readonly string[];
}

export function emptyMediaGraph(): SemiosMediaGraphV1 {
	return { schema: SEMIOS_MEDIA_GRAPH_SCHEMA, nodes: [], edges: [] };
}

export function validateMediaGraph(graph: SemiosMediaGraphV1): SemiosMediaGraphValidation {
	const errors: string[] = [];
	const nodeById = new Map(graph.nodes.map((node) => [node.id, node]));
	const adjacency = new Map<string, string[]>();
	for (const edge of graph.edges) {
		const source = nodeById.get(edge.sourceNodeId);
		const target = nodeById.get(edge.targetNodeId);
		if (!source) errors.push(`missing source node ${edge.sourceNodeId}`);
		if (!target) errors.push(`missing target node ${edge.targetNodeId}`);
		const outPort = source?.outputs.find((port) => port.id === edge.sourcePortId);
		const inPort = target?.inputs.find((port) => port.id === edge.targetPortId);
		if (outPort && inPort && !resourcesCompatible(outPort.resourceKind, inPort.resourceKind)) {
			errors.push(`incompatible resources ${outPort.resourceKind} → ${inPort.resourceKind}`);
		}
		const next = adjacency.get(edge.sourceNodeId) ?? [];
		next.push(edge.targetNodeId);
		adjacency.set(edge.sourceNodeId, next);
	}
	const visiting = new Set<string>();
	const visited = new Set<string>();
	function dfs(nodeId: string): boolean {
		if (visiting.has(nodeId)) return false;
		if (visited.has(nodeId)) return true;
		visiting.add(nodeId);
		for (const child of adjacency.get(nodeId) ?? []) {
			if (!dfs(child)) return false;
		}
		visiting.delete(nodeId);
		visited.add(nodeId);
		return true;
	}
	for (const node of graph.nodes) {
		if (!dfs(node.id)) errors.push(`cycle detected at ${node.id}`);
	}
	return { ok: errors.length === 0, errors };
}

export function mediaGraphNodeForInstance(instance: SemiosAppInstance, position: { readonly x: number; readonly y: number }): SemiosMediaGraphNode {
	const registration = semiosAppRegistration(instance.programId, instance.appId);
	const resource = registration?.yields ?? "graph.dag";
	return {
		id: `node-${instance.id}`,
		instanceId: instance.id,
		x: position.x,
		y: position.y,
		width: 160,
		height: 72,
		inputs: [{ id: `${instance.id}:in`, resourceKind: resource, direction: "in" }],
		outputs: [{ id: `${instance.id}:out`, resourceKind: resource, direction: "out" }],
	};
}

export function resolveUpstreamResourceHandle(
	graph: SemiosMediaGraphV1,
	instances: readonly SemiosAppInstance[],
	targetInstanceId: string,
): string | null {
	const node = graph.nodes.find((entry) => entry.instanceId === targetInstanceId);
	if (!node) return null;
	const edge = graph.edges.find((entry) => entry.targetNodeId === node.id);
	if (!edge) return null;
	const sourceNode = graph.nodes.find((entry) => entry.id === edge.sourceNodeId);
	if (!sourceNode) return null;
	const source = instances.find((entry) => entry.id === sourceNode.instanceId);
	return source?.id ?? null;
}

export function appInstanceResourceProjection(
	graph: SemiosMediaGraphV1,
	instances: readonly SemiosAppInstance[],
	instanceId: string,
	context?: Omit<AppMaterializeContext, "graph" | "instances">,
): {
	readonly kind: SemiosResourceKindId;
	readonly projection: unknown;
	readonly upstreamInstanceId: string | null;
	readonly upstreamProjection: unknown | null;
} | null {
	const instance = instances.find((entry) => entry.id === instanceId);
	if (!instance) return null;
	const materializeContext: AppMaterializeContext = { ...context, graph, instances };
	const upstreamInstanceId = resolveUpstreamResourceHandle(graph, instances, instanceId);
	let projection = materializeAppInstanceProjection(instance, materializeContext);
	let upstreamProjection: unknown | null = null;
	if (upstreamInstanceId) {
		const upstream = instances.find((entry) => entry.id === upstreamInstanceId);
		if (upstream) {
			upstreamProjection = materializeAppInstanceProjection(upstream, materializeContext);
			if (!instance.sourceDocument.inline && !instance.sourceDocument.vcsJson && projection == null) {
				projection = upstreamProjection;
			}
		}
	}
	return {
		kind: instance.yields,
		projection,
		upstreamInstanceId,
		upstreamProjection,
	};
}
//#endregion 🔖MediaGraphEngine

//#region 🔖Projection
let semiosIdCounter = 0;

export function createSemiosId(prefix = "semios"): string {
	semiosIdCounter += 1;
	return `${prefix}-${semiosIdCounter}`;
}

export function defaultStudioProjection(): SemiosStudioProjection {
	return { programs: [], activeProgramId: null, activeAlternativeId: null, appInstances: [], mediaGraph: emptyMediaGraph() };
}

export function createEmptyStudioDocument(id = "studio", name = "Studio"): SemiosStudioDocumentV1 {
	return {
		schema: SEMIOS_STUDIO_SCHEMA,
		id,
		name,
		vcs: {
			initialProjection: defaultStudioProjection(),
			operations: [],
			checkpoints: [],
			alternatives: [],
		},
		backbone: { kind: "dev", uri: "dev://studio.json" },
	};
}

function cloneProjection(projection: SemiosStudioProjection): SemiosStudioProjection {
	return {
		programs: [...projection.programs],
		activeProgramId: projection.activeProgramId,
		activeAlternativeId: projection.activeAlternativeId,
		appInstances: projection.appInstances.map((instance) => ({
			...instance,
			sourceDocument: { ...instance.sourceDocument },
		})),
		mediaGraph: {
			schema: SEMIOS_MEDIA_GRAPH_SCHEMA,
			nodes: projection.mediaGraph.nodes.map((node) => ({
				...node,
				inputs: node.inputs.map((port) => ({ ...port })),
				outputs: node.outputs.map((port) => ({ ...port })),
			})),
			edges: projection.mediaGraph.edges.map((edge) => ({ ...edge })),
		},
	};
}

function applyStudioOperation(projection: SemiosStudioProjection, operation: SemiosStudioOperation): SemiosStudioProjection {
	const next = cloneProjection(projection);
	switch (operation.op) {
		case "setStudioName":
			return next;
		case "setActiveProgram":
			return { ...next, activeProgramId: (operation.payload.programId as string | null) ?? null };
		case "setActiveAlternative":
			return { ...next, activeAlternativeId: (operation.payload.alternativeId as string | null) ?? null };
		case "createAlternative":
			return next;
		case "applyAppOperation": {
			const instanceId = String(operation.payload.instanceId);
			const nextSource = operation.payload.nextSource as SemiosSourceDocument;
			next.appInstances = next.appInstances.map((instance) =>
				instance.id === instanceId ? { ...instance, sourceDocument: { ...nextSource } } : instance,
			);
			return next;
		}
		case "spawnAppInstance": {
			const instance = operation.payload.instance as SemiosAppInstance;
			if (!next.programs.includes(instance.programId)) next.programs.push(instance.programId);
			next.appInstances.push(instance);
			const position = (operation.payload.position as { x: number; y: number }) ?? { x: 0, y: 0 };
			next.mediaGraph.nodes.push(mediaGraphNodeForInstance(instance, position));
			return next;
		}
		case "removeAppInstance": {
			const instanceId = String(operation.payload.instanceId);
			next.appInstances = next.appInstances.filter((instance) => instance.id !== instanceId);
			const nodeId = next.mediaGraph.nodes.find((node) => node.instanceId === instanceId)?.id;
			next.mediaGraph.nodes = next.mediaGraph.nodes.filter((node) => node.instanceId !== instanceId);
			if (nodeId) next.mediaGraph.edges = next.mediaGraph.edges.filter((edge) => edge.sourceNodeId !== nodeId && edge.targetNodeId !== nodeId);
			return next;
		}
		case "connectMediaPorts": {
			const edge = operation.payload.edge as SemiosMediaGraphEdge;
			next.mediaGraph.edges.push(edge);
			return next;
		}
		case "disconnectMediaEdge": {
			const edgeId = String(operation.payload.edgeId);
			next.mediaGraph.edges = next.mediaGraph.edges.filter((edge) => edge.id !== edgeId);
			return next;
		}
		case "moveMediaNode": {
			const nodeId = String(operation.payload.nodeId);
			const x = Number(operation.payload.x);
			const y = Number(operation.payload.y);
			next.mediaGraph.nodes = next.mediaGraph.nodes.map((node) => (node.id === nodeId ? { ...node, x, y } : node));
			return next;
		}
		case "patchAppSource": {
			const instanceId = String(operation.payload.instanceId);
			const inline = String(operation.payload.inline ?? "");
			next.appInstances = next.appInstances.map((instance) =>
				instance.id === instanceId ? { ...instance, sourceDocument: { ...instance.sourceDocument, inline } } : instance,
			);
			return next;
		}
		case "patchAppInstance": {
			const instanceId = String(operation.payload.instanceId);
			const label = typeof operation.payload.label === "string" ? operation.payload.label : undefined;
			next.appInstances = next.appInstances.map((instance) =>
				instance.id === instanceId && label !== undefined ? { ...instance, label } : instance,
			);
			return next;
		}
		default:
			return next;
	}
}

export function materializeStudioProjection(document: SemiosStudioDocumentV1, appliedChangeIds: readonly string[] = []): SemiosStudioProjection {
	let projection = cloneProjection(document.vcs.initialProjection);
	for (const changeId of appliedChangeIds) {
		const change = document.vcs.operations.find((entry) => entry.id === changeId);
		if (!change) continue;
		for (const operation of change.forwards) projection = applyStudioOperation(projection, operation);
	}
	return { ...projection, activeAlternativeId: projection.activeAlternativeId ?? null };
}

export function parseSemiosStudioDocument(raw: unknown): SemiosStudioDocumentV1 {
	const value = raw as Partial<SemiosStudioDocumentV1>;
	if (value.schema !== SEMIOS_STUDIO_SCHEMA) throw new Error(`expected schema ${SEMIOS_STUDIO_SCHEMA}`);
	if (!value.id || !value.name || !value.vcs?.initialProjection) throw new Error("studio document requires id, name, and vcs.initialProjection");
	return value as SemiosStudioDocumentV1;
}

export function semiosStudioDocumentToJson(document: SemiosStudioDocumentV1): string {
	return JSON.stringify(document, null, 2);
}

export function semiosStudioDocumentFromJson(json: string): SemiosStudioDocumentV1 {
	return parseSemiosStudioDocument(JSON.parse(json));
}
//#endregion 🔖Projection

//#region 🔖StudioStore
export type StudioCommand =
	| { readonly kind: "spawnAppInstance"; readonly programId: string; readonly appId: string; readonly label?: string; readonly sourceInline?: string; readonly payloadRef?: string; readonly position?: { readonly x: number; readonly y: number } }
	| { readonly kind: "removeAppInstance"; readonly instanceId: string }
	| { readonly kind: "connectMediaPorts"; readonly sourceNodeId: string; readonly sourcePortId: string; readonly targetNodeId: string; readonly targetPortId: string }
	| { readonly kind: "disconnectMediaEdge"; readonly edgeId: string }
	| { readonly kind: "moveMediaNode"; readonly nodeId: string; readonly x: number; readonly y: number }
	| { readonly kind: "patchAppInstances"; readonly instanceIds: readonly string[]; readonly field: "label"; readonly value?: string }
	| { readonly kind: "patchAppSource"; readonly instanceId: string; readonly inline: string }
	| { readonly kind: "applyAppOperation"; readonly instanceId: string; readonly forwards: readonly unknown[]; readonly backwards: readonly unknown[] }
	| { readonly kind: "openProgram"; readonly programId: string }
	| { readonly kind: "setStudioName"; readonly name: string }
	| { readonly kind: "commitCheckpoint"; readonly message?: string }
	| { readonly kind: "createAlternative"; readonly name: string }
	| { readonly kind: "switchAlternative"; readonly alternativeId: string }
	| { readonly kind: "undo" }
	| { readonly kind: "redo" };

export class StudioStore {
	private document: SemiosStudioDocumentV1;
	private appliedChangeIds: string[] = [];
	private redoChangeIds: string[] = [];
	private listeners = new Set<() => void>();
	private generation = 0;
	private onAfterMutation?: () => void;

	constructor(document: SemiosStudioDocumentV1, options?: { readonly onAfterMutation?: () => void }) {
		this.document = document;
		this.onAfterMutation = options?.onAfterMutation;
	}

	setOnAfterMutation(listener: (() => void) | undefined): void {
		this.onAfterMutation = listener;
	}

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getGeneration(): number {
		return this.generation;
	}

	getDocument(): SemiosStudioDocumentV1 {
		return this.document;
	}

	projection(): SemiosStudioProjection {
		return materializeStudioProjection(this.document, this.appliedChangeIds);
	}

	mediaGraphSnapshot(): SemiosMediaGraphV1 {
		return this.projection().mediaGraph;
	}

	appInstanceResource(instanceId: string): SemiosResourceKindId | null {
		const instance = this.projection().appInstances.find((entry) => entry.id === instanceId);
		if (!instance) return null;
		return semiosAppRegistration(instance.programId, instance.appId)?.yields ?? null;
	}

	dispatch(command: StudioCommand): void {
		if (command.kind === "undo") {
			const last = this.appliedChangeIds.pop();
			if (!last) return;
			this.redoChangeIds.push(last);
			this.generation += 1;
			for (const listener of this.listeners) listener();
			return;
		}
		if (command.kind === "redo") {
			const next = this.redoChangeIds.pop();
			if (!next) return;
			this.appliedChangeIds.push(next);
			this.generation += 1;
			for (const listener of this.listeners) listener();
			return;
		}
		if (command.kind === "commitCheckpoint") {
			this.document = {
				...this.document,
				vcs: {
					...this.document.vcs,
					checkpoints: [
						...this.document.vcs.checkpoints,
						{
							id: createSemiosId("checkpoint"),
							changeIds: [...this.appliedChangeIds],
							message: command.message,
							savedAt: new Date().toISOString(),
						},
					],
				},
			};
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		if (command.kind === "createAlternative") {
			let checkpoint = this.document.vcs.checkpoints.at(-1);
			if (!checkpoint) {
				checkpoint = {
					id: createSemiosId("checkpoint"),
					changeIds: [...this.appliedChangeIds],
					message: "auto",
					savedAt: new Date().toISOString(),
				};
				this.document = {
					...this.document,
					vcs: { ...this.document.vcs, checkpoints: [...this.document.vcs.checkpoints, checkpoint] },
				};
			}
			const alternative: SemiosStudioAlternative = {
				id: createSemiosId("alt"),
				name: command.name,
				checkpointIds: [checkpoint.id],
			};
			this.document = {
				...this.document,
				vcs: { ...this.document.vcs, alternatives: [...this.document.vcs.alternatives, alternative] },
			};
			this.appliedChangeIds = [...checkpoint.changeIds];
			this.redoChangeIds = [];
			this.document = {
				...this.document,
				vcs: {
					...this.document.vcs,
					initialProjection: {
						...this.document.vcs.initialProjection,
						activeAlternativeId: alternative.id,
					},
				},
			};
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		if (command.kind === "switchAlternative") {
			const alternative = this.document.vcs.alternatives.find((entry) => entry.id === command.alternativeId);
			if (!alternative) return;
			const checkpointId = alternative.checkpointIds.at(-1);
			const checkpoint = this.document.vcs.checkpoints.find((entry) => entry.id === checkpointId);
			if (!checkpoint) return;
			this.appliedChangeIds = [...checkpoint.changeIds];
			this.redoChangeIds = [];
			this.document = {
				...this.document,
				vcs: {
					...this.document.vcs,
					initialProjection: {
						...this.document.vcs.initialProjection,
						activeAlternativeId: alternative.id,
					},
				},
			};
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		if (command.kind === "setStudioName") {
			this.document = { ...this.document, name: command.name };
			this.generation += 1;
			for (const listener of this.listeners) listener();
			this.onAfterMutation?.();
			return;
		}
		const change = this.commandToChange(command);
		if (!change) return;
		this.document = {
			...this.document,
			vcs: { ...this.document.vcs, operations: [...this.document.vcs.operations, change] },
		};
		this.appliedChangeIds.push(change.id);
		this.redoChangeIds = [];
		this.generation += 1;
		for (const listener of this.listeners) listener();
		this.onAfterMutation?.();
	}

	private commandToChange(command: StudioCommand): SemiosStudioChange | null {
		switch (command.kind) {
			case "undo":
			case "redo":
			case "commitCheckpoint":
			case "createAlternative":
			case "switchAlternative":
			case "setStudioName":
				return null;
			case "openProgram":
				return {
					id: createSemiosId("change"),
					forwards: [{ op: "setActiveProgram", payload: { programId: command.programId } }],
					backwards: [{ op: "setActiveProgram", payload: { programId: this.projection().activeProgramId } }],
				};
			case "spawnAppInstance": {
				const registration = semiosAppRegistration(command.programId, command.appId);
				if (!registration) throw new Error(`unknown app ${command.programId}/${command.appId}`);
				const instance: SemiosAppInstance = {
					id: createSemiosId("app"),
					programId: command.programId,
					appId: command.appId,
					label: command.label ?? registration.label,
					yields: registration.yields,
					sourceDocument: createAppSourceDocument(registration.sourceFormat, createSemiosId("app-doc"), {
						inline: command.sourceInline,
						payloadRef: command.payloadRef,
					}),
				};
				const spawn: SemiosStudioOperation = { op: "spawnAppInstance", payload: { instance, position: command.position ?? { x: 40, y: 40 } } };
				const remove: SemiosStudioOperation = { op: "removeAppInstance", payload: { instanceId: instance.id } };
				return { id: createSemiosId("change"), forwards: [spawn], backwards: [remove] };
			}
			case "removeAppInstance": {
				const projection = this.projection();
				const instance = projection.appInstances.find((entry) => entry.id === command.instanceId);
				if (!instance) return null;
				const node = projection.mediaGraph.nodes.find((entry) => entry.instanceId === instance.id);
				const connectedEdges = projection.mediaGraph.edges.filter((edge) => edge.sourceNodeId === node?.id || edge.targetNodeId === node?.id);
				const forwards: SemiosStudioOperation[] = [{ op: "removeAppInstance", payload: { instanceId: instance.id } }];
				const backwards: SemiosStudioOperation[] = [
					{ op: "spawnAppInstance", payload: { instance, position: node ? { x: node.x, y: node.y } : { x: 0, y: 0 } } },
					...connectedEdges.map((edge) => ({ op: "connectMediaPorts" as const, payload: { edge } })),
				];
				return { id: createSemiosId("change"), forwards, backwards };
			}
			case "connectMediaPorts": {
				const edge: SemiosMediaGraphEdge = {
					id: createSemiosId("edge"),
					sourceNodeId: command.sourceNodeId,
					sourcePortId: command.sourcePortId,
					targetNodeId: command.targetNodeId,
					targetPortId: command.targetPortId,
				};
				const validation = validateMediaGraph({
					...this.projection().mediaGraph,
					edges: [...this.projection().mediaGraph.edges, edge],
				});
				if (!validation.ok) throw new Error(validation.errors.join("; "));
				return {
					id: createSemiosId("change"),
					forwards: [{ op: "connectMediaPorts", payload: { edge } }],
					backwards: [{ op: "disconnectMediaEdge", payload: { edgeId: edge.id } }],
				};
			}
			case "disconnectMediaEdge":
				return {
					id: createSemiosId("change"),
					forwards: [{ op: "disconnectMediaEdge", payload: { edgeId: command.edgeId } }],
					backwards: [{ op: "connectMediaPorts", payload: { edge: this.projection().mediaGraph.edges.find((edge) => edge.id === command.edgeId) } }],
				};
			case "moveMediaNode": {
				const node = this.projection().mediaGraph.nodes.find((entry) => entry.id === command.nodeId);
				if (!node) return null;
				return {
					id: createSemiosId("change"),
					forwards: [{ op: "moveMediaNode", payload: { nodeId: command.nodeId, x: command.x, y: command.y } }],
					backwards: [{ op: "moveMediaNode", payload: { nodeId: command.nodeId, x: node.x, y: node.y } }],
				};
			}
			case "patchAppSource": {
				const instance = this.projection().appInstances.find((entry) => entry.id === command.instanceId);
				if (!instance) return null;
				const previous = instance.sourceDocument.inline ?? "";
				return {
					id: createSemiosId("change"),
					forwards: [{ op: "patchAppSource", payload: { instanceId: command.instanceId, inline: command.inline } }],
					backwards: [{ op: "patchAppSource", payload: { instanceId: command.instanceId, inline: previous } }],
				};
			}
			case "patchAppInstances": {
				if (command.field !== "label" || typeof command.value !== "string") return null;
				const projection = this.projection();
				const forwards: SemiosStudioOperation[] = [];
				const backwards: SemiosStudioOperation[] = [];
				for (const instanceId of command.instanceIds) {
					const instance = projection.appInstances.find((entry) => entry.id === instanceId);
					if (!instance) continue;
					forwards.push({ op: "patchAppInstance", payload: { instanceId, label: command.value } });
					backwards.push({ op: "patchAppInstance", payload: { instanceId, label: instance.label } });
				}
				if (!forwards.length) return null;
				return { id: createSemiosId("change"), forwards, backwards };
			}
			case "applyAppOperation": {
				const instance = this.projection().appInstances.find((entry) => entry.id === command.instanceId);
				if (!instance) return null;
				const previous = { ...instance.sourceDocument };
				const nextSource = applyAppOperationToSource(instance.sourceDocument, command.forwards, command.backwards);
				return {
					id: createSemiosId("change"),
					forwards: [{ op: "applyAppOperation", payload: { instanceId: command.instanceId, nextSource } }],
					backwards: [{ op: "applyAppOperation", payload: { instanceId: command.instanceId, nextSource: previous } }],
				};
			}
		}
	}
}
//#endregion 🔖StudioStore

export { RustStudioStore } from "./rust-studio.ts";

//#region 🔖DevJsonBackbone
export interface StudioBackbonePort {
	readonly read: (uri: string) => string | null;
	readonly write: (uri: string, json: string) => void;
}

const defaultBrowserBackbonePort: StudioBackbonePort = {
	read(uri) {
		if (typeof localStorage === "undefined") return null;
		return localStorage.getItem(`semios:backbone:${uri}`);
	},
	write(uri, json) {
		if (typeof localStorage === "undefined") return;
		localStorage.setItem(`semios:backbone:${uri}`, json);
	},
};

export class DevJsonBackbone {
	private uri: string | null = null;
	private readonly port: StudioBackbonePort;

	constructor(port: StudioBackbonePort = defaultBrowserBackbonePort) {
		this.port = port;
	}

	attach(uri: string): void {
		this.uri = uri;
	}

	detach(): void {
		this.uri = null;
	}

	status(): { readonly attachedUri: string | null; readonly kind: "dev" } {
		return { attachedUri: this.uri, kind: "dev" };
	}

	load(json: string): SemiosStudioDocumentV1 {
		return semiosStudioDocumentFromJson(json);
	}

	loadAttached(): SemiosStudioDocumentV1 | null {
		if (!this.uri) return null;
		const json = this.port.read(this.uri);
		if (!json) return null;
		return this.load(json);
	}

	sync(document: SemiosStudioDocumentV1): string {
		const synced =
			this.uri != null ? { ...document, backbone: { kind: "dev" as const, uri: this.uri } } : document;
		const json = semiosStudioDocumentToJson(synced);
		if (this.uri) this.port.write(this.uri, json);
		return json;
	}
}

/** @emoji 💾 Local-first backbone stub (`local://`) mirroring dev JSON port shape. */
export class LocalJsonBackbone {
	private uri: string | null = null;
	private readonly port: StudioBackbonePort;

	constructor(port: StudioBackbonePort = defaultBrowserBackbonePort) {
		this.port = port;
	}

	attach(uri: string): void {
		if (!uri.startsWith("local://")) throw new Error(`expected local:// uri, got ${uri}`);
		this.uri = uri;
	}

	status(): { readonly attachedUri: string | null; readonly kind: "local" } {
		return { attachedUri: this.uri, kind: "local" };
	}

	loadAttached(): SemiosStudioDocumentV1 | null {
		if (!this.uri) return null;
		const json = this.port.read(this.uri);
		if (!json) return null;
		return semiosStudioDocumentFromJson(json);
	}

	sync(document: SemiosStudioDocumentV1): string {
		const synced =
			this.uri != null ? { ...document, backbone: { kind: "local" as const, uri: this.uri } } : document;
		const json = semiosStudioDocumentToJson(synced);
		if (this.uri) this.port.write(this.uri, json);
		return json;
	}
}

/** @emoji 🌐 Remote backbone stub (`remote://`) with conflict placeholder. */
export class RemoteJsonBackbone {
	private uri: string | null = null;
	private lastConflict: StudioConflict | null = null;

	attach(uri: string): void {
		if (!uri.startsWith("remote://")) throw new Error(`expected remote:// uri, got ${uri}`);
		this.uri = uri;
	}

	status(): { readonly attachedUri: string | null; readonly kind: "remote"; readonly conflict: StudioConflict | null } {
		return { attachedUri: this.uri, kind: "remote", conflict: this.lastConflict };
	}

	sync(_document: SemiosStudioDocumentV1): never {
		this.lastConflict = {
			kind: "studio-conflict",
			uri: this.uri ?? "remote://unknown",
			message: "remote backbone sync is not implemented",
		};
		throw new Error(this.lastConflict.message);
	}
}
//#endregion 🔖DevJsonBackbone

// #region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("semios studio", () => {
		it("spawns app instances through CQRS dispatch", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", sourceInline: "{}" });
			expect(store.projection().appInstances).toHaveLength(1);
			expect(store.projection().mediaGraph.nodes).toHaveLength(1);
		});

		it("patchAppInstances updates labels in batch", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", position: { x: 0, y: 0 } });
			store.dispatch({ kind: "spawnAppInstance", programId: "writer", appId: "writer", position: { x: 220, y: 0 } });
			const ids = store.projection().appInstances.map((row) => row.id);
			store.dispatch({ kind: "patchAppInstances", instanceIds: ids, field: "label", value: "Renamed" });
			expect(store.projection().appInstances.every((row) => row.label === "Renamed")).toBe(true);
		});

		it("validates resource-compatible media edges", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", position: { x: 0, y: 0 } });
			store.dispatch({ kind: "spawnAppInstance", programId: "writer", appId: "writer", position: { x: 200, y: 0 } });
			const nodes = store.projection().mediaGraph.nodes;
			expect(() =>
				store.dispatch({
					kind: "connectMediaPorts",
					sourceNodeId: nodes[0]!.id,
					sourcePortId: nodes[0]!.outputs[0]!.id,
					targetNodeId: nodes[1]!.id,
					targetPortId: nodes[1]!.inputs[0]!.id,
				}),
			).toThrow(/incompatible/u);
		});

		it("round-trips dev JSON backbone", () => {
			const backbone = new DevJsonBackbone();
			backbone.attach("dev://demo.semios.json");
			const original = createEmptyStudioDocument("demo", "Demo");
			const json = backbone.sync(original);
			const loaded = backbone.load(json);
			expect(loaded.id).toBe("demo");
			expect(loaded.backbone?.uri).toBe("dev://demo.semios.json");
		});

		it("supports undo after spawn", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw" });
			store.dispatch({ kind: "undo" });
			expect(store.projection().appInstances).toHaveLength(0);
		});

		it("connects compatible resource ports", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", position: { x: 0, y: 0 } });
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", position: { x: 220, y: 0 } });
			const nodes = store.projection().mediaGraph.nodes;
			store.dispatch({
				kind: "connectMediaPorts",
				sourceNodeId: nodes[0]!.id,
				sourcePortId: nodes[0]!.outputs[0]!.id,
				targetNodeId: nodes[1]!.id,
				targetPortId: nodes[1]!.inputs[0]!.id,
			});
			expect(store.projection().mediaGraph.edges).toHaveLength(1);
		});

		it("lists expanded program catalog", () => {
			const ids = listSemiosPrograms().map((program) => program.id);
			expect(ids).toContain("dag");
			expect(ids).toContain("procedural.2d");
			expect(ids).toContain("presentation");
		});

		it("resolves upstream resource handles on connected instances", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", position: { x: 0, y: 0 } });
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw", position: { x: 220, y: 0 } });
			const nodes = store.projection().mediaGraph.nodes;
			store.dispatch({
				kind: "connectMediaPorts",
				sourceNodeId: nodes[0]!.id,
				sourcePortId: nodes[0]!.outputs[0]!.id,
				targetNodeId: nodes[1]!.id,
				targetPortId: nodes[1]!.inputs[0]!.id,
			});
			const downstream = store.projection().appInstances[1]!;
			const bundle = appInstanceResourceProjection(store.projection().mediaGraph, store.projection().appInstances, downstream.id);
			expect(bundle?.upstreamInstanceId).toBe(store.projection().appInstances[0]!.id);
		});

		it("creates and switches studio alternatives", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "draw", appId: "draw" });
			store.dispatch({ kind: "createAlternative", name: "branch-a" });
			const altId = store.getDocument().vcs.alternatives[0]!.id;
			store.dispatch({ kind: "spawnAppInstance", programId: "writer", appId: "writer" });
			store.dispatch({ kind: "switchAlternative", alternativeId: altId });
			expect(store.projection().appInstances).toHaveLength(1);
		});

		it("round-trips applyAppOperation on json-backed app documents", () => {
			const store = new StudioStore(createEmptyStudioDocument());
			store.dispatch({ kind: "spawnAppInstance", programId: "flow", appId: "flow" });
			const instance = store.projection().appInstances[0]!;
			store.dispatch({
				kind: "applyAppOperation",
				instanceId: instance.id,
				forwards: [{ op: "setFlow", flow: { id: "patched" } }],
				backwards: [{ op: "setFlow", flow: {} }],
			});
			const updated = store.projection().appInstances[0]!;
			const projection = materializeAppInstanceProjection(updated) as { flow?: { id?: string } };
			expect(projection.flow?.id).toBe("patched");
			store.dispatch({ kind: "undo" });
			expect(store.projection().appInstances).toHaveLength(1);
		});

		it("rejects remote backbone sync with studio conflict", () => {
			const remote = new RemoteJsonBackbone();
			remote.attach("remote://studio");
			expect(() => remote.sync(createEmptyStudioDocument())).toThrow(/not implemented/u);
			expect(remote.status().conflict?.kind).toBe("studio-conflict");
		});

		it("spawns and materializes every technology program id", async () => {
			const [
				{ createDrawAppVcsHandler },
				{ createWriterAppVcsHandler },
				{ createRasterAppVcsHandler },
				{ createFormsAppVcsHandler },
				{ createFlowAppVcsHandler },
				{ createPresentationAppVcsHandler },
			] = await Promise.all([
				import("@semio-tech/draw-core"),
				import("@semio-tech/writer-core"),
				import("@semio-tech/raster-core"),
				import("@semio-tech/forms-core"),
				import("@semio-tech/flow-core"),
				import("@semio-tech/framework-presentation-core"),
			]);
			registerAppVcsHandler(createDrawAppVcsHandler());
			registerAppVcsHandler(createWriterAppVcsHandler());
			registerAppVcsHandler(createRasterAppVcsHandler());
			registerAppVcsHandler(createFormsAppVcsHandler());
			registerAppVcsHandler(createFlowAppVcsHandler());
			registerAppVcsHandler(createPresentationAppVcsHandler());
			const store = new StudioStore(createEmptyStudioDocument());
			const spawned: Array<{ programId: string; appId: string; instanceId: string; sourceFormat: string }> = [];
			for (const program of listSemiosPrograms()) {
				if (program.id === "semios.system") continue;
				const app = program.apps[0];
				if (!app) continue;
				store.dispatch({ kind: "spawnAppInstance", programId: program.id, appId: app.id, position: { x: 0, y: 0 } });
				const instance = store.projection().appInstances.at(-1)!;
				spawned.push({ programId: program.id, appId: app.id, instanceId: instance.id, sourceFormat: app.sourceFormat });
			}
			expect(spawned.length).toBeGreaterThanOrEqual(14);
			for (const row of spawned) {
				const instance = store.projection().appInstances.find((entry) => entry.id === row.instanceId)!;
				expect(() => materializeAppInstanceProjection(instance)).not.toThrow();
				const projection = materializeAppInstanceProjection(instance);
				expect(projection).not.toBeNull();
				expect(projection).not.toBeUndefined();
				if (row.sourceFormat === "forms.form/v1") {
					expect((projection as { schema?: string }).schema).toBe("forms.form/v1");
				}
			}
		});
	});
}
// #endregion 🧪Tests
