/** @emoji 🎮 Vite entry: spatial.shape catalog + `BrepjsKernel` + `InteractionRepl` + `construct` query runner. */
import { StrictMode, useCallback, useEffect, useMemo, useRef, useState, type ChangeEvent, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	DocumentHistory,
	SHAPE_MODEL_DEFINITION_ID,
	applyTransformation,
	isShapeModelDefinition,
	isInteractionSessionActive,
	isEmptyModelDiff,
	listModelDefinitionManifests,
	listTransformationsFromModelDefinition,
	listTransformationsIntoModelDefinition,
	listSpatialInteractionsForModelDefinition,
	loadSpatialInteraction,
	countViewObjectsForModelDefinition,
	modelDefinitionSelectionEntityKinds,
	modelDefinitionUsesGeometryPicking,
	parseModelJson,
	qualifiedTransformationId,
	resolveModelDefinitionScope,
	typologyObjectPascalFromLabel,
	type InteractionSnapshot,
	type InteractionRuntime,
	type InteractionSpec,
	type InteractionRuntimeOptions,
	type SpatialComputeMode,
	type ModelDocument,
	type SelectionTarget,
	type TransformationSpec,
	Model,
	ModelSpace,
} from "@spatial/js-core";
import { defaultConstructRunner } from "@spatial/js-query";
import geometryNakagin from "../../../fixtures/geometry.json";
import geometryLoom from "../../../fixtures/geometry-loom.json";
import geometryRoutes from "../../../fixtures/geometry-routes.json";
import geometrySmallBuilding from "../../../fixtures/small-building.model.json";
import geometryTallBuilding from "../../../fixtures/tall-building.model.json";
import geometryLargeBuilding from "../../../fixtures/large-building.model.json";
import { BrepjsKernel } from "@spatial/js-kernel-brepjs";
import { statelyStateEngineProvider } from "@spatial/js-machine-stately";
import {
	InteractionRepl,
	SelectionAttributesPanel,
	SelectionPropertiesPanel,
	replDisplayedSelectionTargets,
	r3fPreviewKernel,
	useDocumentHistory,
	useInteractionRuntime,
	type SpatialInteractionSelectionByState,
	type SpatialRendererSelectionByModel,
} from "../index";

//#region 🔖ConstructQueryPanel
function defaultConstructQueryForModelDefinition(modelDefinitionId: string): string {
	if (isShapeModelDefinition(modelDefinitionId)) {
		return "MATCH (o:Object {typology: 'spatial.shape.primitive.box'}) RETURN o.id LIMIT 8";
	}
	const scope = resolveModelDefinitionScope(modelDefinitionId);
	const typology = scope.typologies[0];
	if (typology) return `MATCH (o:Object {typology: '${typology.id}'}) RETURN o.id LIMIT 8`;
	return "MATCH (o:Object) RETURN o.id LIMIT 8";
}

/** @emoji 🔍 Play-only `construct` runner scoped to the active model definition. */
function ConstructQueryPanel({
	runtime,
	activeModelDefinitionId,
}: {
	readonly runtime: InteractionRuntime;
	readonly activeModelDefinitionId: string;
}) {
	const defaultQuery = useMemo(
		() => defaultConstructQueryForModelDefinition(activeModelDefinitionId),
		[activeModelDefinitionId],
	);
	const [text, setText] = useState(defaultQuery);
	const [out, setOut] = useState("");
	const [busy, setBusy] = useState(false);
	useEffect(() => {
		setText(defaultQuery);
		setOut("");
	}, [defaultQuery]);
	const run = useCallback(async () => {
		const q = text.trim();
		if (!q) return;
		setBusy(true);
		try {
			const r = await runtime.query(q);
			setOut(JSON.stringify({ rows: r.rows, ...(r.data !== undefined ? { data: r.data } : {}), ...(r.diff ? { diff: r.diff } : {}) }, null, 2));
		} catch (e) {
			setOut(String(e));
		} finally {
			setBusy(false);
		}
	}, [runtime, text]);
	return (
		<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12, marginTop: 8 }}>
			<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Construct query</span>
			<textarea
				value={text}
				onChange={(e) => setText(e.target.value)}
				rows={4}
				spellCheck={false}
				style={{
					padding: 8,
					borderRadius: 6,
					background: "#12121c",
					color: "#e8e8f0",
					border: "1px solid #2a2a3c",
					fontFamily: "ui-monospace, monospace",
					fontSize: 11,
				}}
			/>
			<button
				type="button"
				disabled={busy}
				onClick={() => void run()}
				style={{ padding: "6px 10px", borderRadius: 6, alignSelf: "flex-start", cursor: busy ? "wait" : "pointer" }}
			>
				{busy ? "Running…" : "Run"}
			</button>
			{out ? (
				<pre
					style={{
						margin: 0,
						maxHeight: 200,
						overflow: "auto",
						padding: 8,
						borderRadius: 6,
						background: "#0e0e16",
						color: "#a8d8a8",
						fontSize: 11,
						whiteSpace: "pre-wrap",
						wordBreak: "break-word",
					}}
				>
					{out}
				</pre>
			) : null}
		</div>
	);
}
//#endregion

//#region 🔖GeometryCatalog
function modelVertexCount(json: Record<string, unknown>): number {
	const modelSpace = parseModelSpaceJson(json);
	if (modelSpace) return Object.values(modelSpace.models).reduce((count, model) => count + Object.keys(model.vertices).length, 0);
	const model = parseModelJson(json);
	if (model) return Object.keys(model.vertices).length;
	const geo = json.geometry;
	if (geo && typeof geo === "object") {
		const nested = (geo as Record<string, unknown>).vertices;
		if (Array.isArray(nested)) return nested.length;
	}
	const verts = json.vertices;
	return Array.isArray(verts) ? verts.length : 0;
}

const SHAPE_ASSETS = [
	{ id: "nakagin-slice", key: "a", label: "Nakagin capsule", json: geometryNakagin as Record<string, unknown> },
	{ id: "geometry-loom", key: "l", label: "Loom deck + pent loop + rail", json: geometryLoom as Record<string, unknown> },
	{ id: "geometry-routes", key: "r", label: "Multi-route lattice", json: geometryRoutes as Record<string, unknown> },
	{ id: "small-building", key: "s", label: "Small building", json: geometrySmallBuilding as Record<string, unknown> },
	{ id: "tall-building", key: "t", label: "Tall building", json: geometryTallBuilding as Record<string, unknown> },
	{ id: "large-building", key: "b", label: "Large building", json: geometryLargeBuilding as Record<string, unknown> },
] as const;

const PLAY_REPL_SPEC: InteractionSpec = {
	schema: "spatial.interaction/v1",
	id: "",
	version: "1.0.0",
	label: "Play",
	machine: {
		initial: "idle",
		states: [{ name: "idle" }],
	},
	display: {
		states: [{ state: "idle", items: [] }],
	},
	commit: {
		fromStates: [],
		operation: { kind: "action", action: "play.repl.noop" },
	},
};

type ModelJsonSnapshot = ReturnType<Model["toJSON"]>;
type ModelSpaceJsonSnapshot = ReturnType<ModelSpace["toJSON"]>;

interface SpatialExchangeBundle {
	readonly model?: ModelJsonSnapshot;
	readonly modelSpace?: ModelSpaceJsonSnapshot;
	readonly activeModelDefinitionId?: string;
}

interface SaveFilePickerTypeOption {
	readonly description?: string;
	readonly accept: Record<string, readonly string[]>;
}

interface SaveFilePickerOptionsLike {
	readonly suggestedName?: string;
	readonly types?: readonly SaveFilePickerTypeOption[];
	readonly excludeAcceptAllOption?: boolean;
}

interface FileSystemWritableFileStreamLike {
	write(data: string): Promise<void>;
	close(): Promise<void>;
}

interface FileSystemFileHandleLike {
	createWritable(): Promise<FileSystemWritableFileStreamLike>;
}

interface SavePickerWindow extends Window {
	showSaveFilePicker?: (options?: SaveFilePickerOptionsLike) => Promise<FileSystemFileHandleLike>;
}

function emptyPlayModels(): Record<string, Model> {
	return { [SHAPE_MODEL_DEFINITION_ID]: new Model() };
}

function ensurePlayShapeModel(models: Readonly<Record<string, Model>>): Record<string, Model> {
	if (models[SHAPE_MODEL_DEFINITION_ID]) return { ...models };
	return { ...models, [SHAPE_MODEL_DEFINITION_ID]: new Model() };
}

function parseModelSpaceJson(raw: unknown): ModelSpace | null {
	if (!raw || typeof raw !== "object") return null;
	const row = raw as Record<string, unknown>;
	if (row.schema !== "spatial.modelspace/v1" || !Array.isArray(row.models)) return null;
	return ModelSpace.fromJSON(row as ModelSpaceJsonSnapshot);
}

function fileStem(name: string): string {
	const trimmed = name.trim();
	if (!trimmed) return "spatial";
	return trimmed
		.replace(/\.analytic\.spatial\.json$/i, "")
		.replace(/\.raw\.spatial\.json$/i, "")
		.replace(/\.spatial\.json$/i, "")
		.replace(/\.json$/i, "")
		.replace(/[^a-z0-9._-]+/gi, "-")
		.replace(/^-+|-+$/g, "") || "spatial";
}

function selectRawModel(model: Model, selection: readonly SelectionTarget[]): ModelJsonSnapshot {
	const selectedModel = new Model();
	selectedModel.revision = model.revision;
	const anchors = new Set<string>();
	const vertices = new Set<string>();
	const edges = new Set<string>();
	const wires = new Set<string>();
	const faces = new Set<string>();
	const shells = new Set<string>();
	const solids = new Set<string>();
	const visitById = (id: string): void => {
		if (model.anchors[id]) {
			visitAnchor(id);
			return;
		}
		if (model.vertices[id]) {
			visitVertex(id);
			return;
		}
		if (model.edges[id]) {
			visitEdge(id);
			return;
		}
		if (model.wires[id]) {
			visitWire(id);
			return;
		}
		if (model.faces[id]) {
			visitFace(id);
			return;
		}
		if (model.shells[id]) {
			visitShell(id);
			return;
		}
		if (model.solids[id]) {
			visitSolid(id);
			return;
		}
	};

	const visitAnchor = (id: string): void => {
		if (anchors.has(id)) return;
		const rec = model.anchors[id];
		if (!rec) return;
		anchors.add(id);
		visitById(rec.attachment.id);
	};

	const visitVertex = (id: string): void => {
		if (vertices.has(id) || !model.vertices[id]) return;
		vertices.add(id);
	};

	const visitEdge = (id: string): void => {
		if (edges.has(id)) return;
		const rec = model.edges[id];
		if (!rec) return;
		edges.add(id);
		for (const vertexId of rec.vertexIds) visitVertex(vertexId);
	};

	const visitWire = (id: string): void => {
		if (wires.has(id)) return;
		const rec = model.wires[id];
		if (!rec) return;
		wires.add(id);
		for (const edgeId of rec.edgeIds) visitEdge(edgeId);
	};

	const visitFace = (id: string): void => {
		if (faces.has(id)) return;
		const rec = model.faces[id];
		if (!rec) return;
		faces.add(id);
		for (const wireId of rec.wireIds) visitWire(wireId);
	};

	const visitShell = (id: string): void => {
		if (shells.has(id)) return;
		const rec = model.shells[id];
		if (!rec) return;
		shells.add(id);
		for (const faceId of rec.faceIds) visitFace(faceId);
	};

	const visitSolid = (id: string): void => {
		if (solids.has(id)) return;
		const rec = model.solids[id];
		if (!rec) return;
		solids.add(id);
		for (const shellId of rec.shellIds) visitShell(shellId);
	};

	for (const target of selection) {
		switch (target.kind) {
			case "object": {
				const object = model.objects[target.id];
				if (!object) break;
				selectedModel.objects[object.id] = object;
				for (const primitiveId of Object.values(object.primitives)) visitById(primitiveId);
				break;
			}
			case "anchor":
				visitAnchor(target.id);
				break;
			case "vertex":
				visitVertex(target.id);
				break;
			case "edge":
				visitEdge(target.id);
				break;
			case "wire":
				visitWire(target.id);
				break;
			case "face":
				visitFace(target.id);
				break;
			case "shell":
				visitShell(target.id);
				break;
			case "solid":
				visitSolid(target.id);
				break;
			default:
				break;
		}
	}

	const sortIds = (ids: Set<string>) => [...ids].sort((a, b) => a.localeCompare(b));
	selectedModel.anchors = Object.fromEntries(sortIds(anchors).map((id) => [id, model.anchors[id]!])) as typeof selectedModel.anchors;
	selectedModel.vertices = Object.fromEntries(sortIds(vertices).map((id) => [id, model.vertices[id]!])) as typeof selectedModel.vertices;
	selectedModel.edges = Object.fromEntries(sortIds(edges).map((id) => [id, model.edges[id]!])) as typeof selectedModel.edges;
	selectedModel.wires = Object.fromEntries(sortIds(wires).map((id) => [id, model.wires[id]!])) as typeof selectedModel.wires;
	selectedModel.faces = Object.fromEntries(sortIds(faces).map((id) => [id, model.faces[id]!])) as typeof selectedModel.faces;
	selectedModel.shells = Object.fromEntries(sortIds(shells).map((id) => [id, model.shells[id]!])) as typeof selectedModel.shells;
	selectedModel.solids = Object.fromEntries(sortIds(solids).map((id) => [id, model.solids[id]!])) as typeof selectedModel.solids;
	selectedModel.metadata.loadSnapshot(model.metadata.toJSON(), false);
	return selectedModel.toJSON();
}

async function writeTextFile(
	name: string,
	text: string,
	types: readonly SaveFilePickerTypeOption[],
	fallbackMime = "application/octet-stream",
): Promise<void> {
	const pickerWindow = window as SavePickerWindow;
	if (pickerWindow.showSaveFilePicker) {
		const handle = await pickerWindow.showSaveFilePicker({ suggestedName: name, types });
		const writable = await handle.createWritable();
		await writable.write(text);
		await writable.close();
		return;
	}
	const href = URL.createObjectURL(new Blob([text], { type: fallbackMime }));
	const link = document.createElement("a");
	link.href = href;
	link.download = name;
	link.click();
	URL.revokeObjectURL(href);
}

async function writeJsonFile(name: string, payload: SpatialExchangeBundle): Promise<void> {
	await writeTextFile(
		name,
		`${JSON.stringify(payload, null, 2)}\n`,
		[{ description: "Spatial JSON", accept: { "application/json": [".json", ".spatial.json"] } }],
		"application/json",
	);
}

async function writeStepFile(name: string, stepText: string): Promise<void> {
	await writeTextFile(
		name,
		stepText,
		[{ description: "STEP AP242", accept: { "application/step": [".stp", ".step"], "model/step": [".stp", ".step"] } }],
		"application/step",
	);
}

function sanitizeModelDefinitionFileStem(modelDefinitionId: string): string {
	return modelDefinitionId.replace(/[^a-z0-9._-]+/gi, "-").replace(/^-+|-+$/g, "") || "model";
}

function modelsFromSpatialJson(json: unknown): Record<string, Model> {
	const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
	const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
	if (modelSpace) return ensurePlayShapeModel(recordFromModelSpace(modelSpace));
	return ensurePlayShapeModel({
		[SHAPE_MODEL_DEFINITION_ID]: parseModelJson(bundle?.model ?? json) ?? new Model(),
	});
}

function activeModelDefinitionIdFromSpatialJson(json: unknown): string {
	const bundle = json && typeof json === "object" ? (json as SpatialExchangeBundle) : null;
	if (typeof bundle?.activeModelDefinitionId === "string") return bundle.activeModelDefinitionId;
	const modelSpace = parseModelSpaceJson(bundle?.modelSpace ?? json);
	return Object.keys(modelSpace?.models ?? {})[0] ?? SHAPE_MODEL_DEFINITION_ID;
}

function flushModelsRecord(models: Readonly<Record<string, Model>>, activeId: string, live: Model): Record<string, Model> {
	return { ...models, [activeId]: Model.fromJSON(live.toJSON()) };
}

function modelSpaceFromRecord(models: Readonly<Record<string, Model>>): ModelSpace {
	const space = new ModelSpace();
	for (const id of Object.keys(models).sort()) space.link(id, models[id]!);
	return space;
}

function recordFromModelSpace(space: ModelSpace): Record<string, Model> {
	const out: Record<string, Model> = {};
	for (const id of Object.keys(space.models).sort()) {
		const model = space.models[id];
		if (model) out[id] = Model.fromJSON(model.toJSON());
	}
	return out;
}

function ensureDerivedModelInSpace(models: Readonly<Record<string, Model>>, definitionId: string): Record<string, Model> {
	const withShape = ensurePlayShapeModel(models);
	if (withShape[definitionId]) return withShape;
	if (isShapeModelDefinition(definitionId)) return withShape;
	const fromShape = listTransformationsIntoModelDefinition(definitionId).find((row) =>
		isShapeModelDefinition(row.source.modelDefinition),
	);
	const shape = withShape[SHAPE_MODEL_DEFINITION_ID];
	if (!fromShape || !shape) return withShape;
	return { ...withShape, [definitionId]: applyTransformation(fromShape, shape) };
}

function pickShapeForModelDefinition(
	models: Readonly<Record<string, Model>>,
	activeModelDefinitionId: string,
	liveModel: Model,
): Model {
	if (isShapeModelDefinition(activeModelDefinitionId)) {
		return models[SHAPE_MODEL_DEFINITION_ID] ?? liveModel;
	}
	if (modelDefinitionUsesGeometryPicking(activeModelDefinitionId)) {
		return models[activeModelDefinitionId] ?? models[SHAPE_MODEL_DEFINITION_ID] ?? liveModel;
	}
	return liveModel;
}

const PLAY_SELECT_STYLE = { padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" } as const;

//#region 🔖PlayModelSpacePanel
interface PlayModelSpacePanelProps {
	readonly activeModelDefinitionId: string;
	readonly modelSpaceCount: number;
	readonly viewObjectCount: number;
	readonly onActiveModelDefinitionId: (value: string) => void;
	readonly onApplyTransformation: (spec: TransformationSpec) => void;
}

/** @emoji 🌌 Play aside: active model definition + transform to/from dropdowns. */
function PlayModelSpacePanel({
	activeModelDefinitionId,
	modelSpaceCount,
	viewObjectCount,
	onActiveModelDefinitionId,
	onApplyTransformation,
}: PlayModelSpacePanelProps) {
	const modelDefinitions = useMemo(() => listModelDefinitionManifests(), []);
	const scope = useMemo(() => resolveModelDefinitionScope(activeModelDefinitionId), [activeModelDefinitionId]);
	const transformsTo = useMemo(
		() => listTransformationsFromModelDefinition(activeModelDefinitionId),
		[activeModelDefinitionId],
	);
	const transformsFrom = useMemo(
		() => listTransformationsIntoModelDefinition(activeModelDefinitionId),
		[activeModelDefinitionId],
	);
	return (
		<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
			<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
				<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Model definition</span>
				<select
					value={activeModelDefinitionId}
					onChange={(e) => onActiveModelDefinitionId(e.target.value || SHAPE_MODEL_DEFINITION_ID)}
					style={PLAY_SELECT_STYLE}
				>
					{modelDefinitions.map((row) => (
						<option key={row.id} value={row.id}>
							{row.label} ({row.id})
						</option>
					))}
				</select>
			</label>
			<span style={{ opacity: 0.75, lineHeight: 1.4 }}>
				{scope.typologies.length} typolog{scope.typologies.length === 1 ? "y" : "ies"}
				{" · "}
				{scope.interactions.length} interaction{scope.interactions.length === 1 ? "" : "s"}
				{" · "}
				{scope.attributeDefinitions.length} attribute{scope.attributeDefinitions.length === 1 ? "" : "s"}
				{" · "}
				{modelSpaceCount} linked model{modelSpaceCount === 1 ? "" : "s"}
			</span>
			{!isShapeModelDefinition(activeModelDefinitionId) && viewObjectCount > 0 ? (
				<span style={{ opacity: 0.75 }}>
					{viewObjectCount} object{viewObjectCount === 1 ? "" : "s"} in view
				</span>
			) : null}
			{transformsTo.length ? (
				<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
					<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Transform To</span>
					<select
						defaultValue=""
						onChange={(e) => {
							const qid = e.target.value;
							if (!qid) return;
							const spec = transformsTo.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
							if (spec) onApplyTransformation(spec);
							e.target.value = "";
						}}
						style={PLAY_SELECT_STYLE}
					>
						<option value="">Select target model…</option>
						{transformsTo.map((row) => (
							<option key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
								{row.target.modelDefinition} — {row.label}
							</option>
						))}
					</select>
				</label>
			) : null}
			{transformsFrom.length ? (
				<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
					<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Transform From</span>
					<select
						defaultValue=""
						onChange={(e) => {
							const qid = e.target.value;
							if (!qid) return;
							const spec = transformsFrom.find((row) => qualifiedTransformationId(row.modelDefinitionId, row.id) === qid);
							if (spec) onApplyTransformation(spec);
							e.target.value = "";
						}}
						style={PLAY_SELECT_STYLE}
					>
						<option value="">Select source model…</option>
						{transformsFrom.map((row) => (
							<option key={qualifiedTransformationId(row.modelDefinitionId, row.id)} value={qualifiedTransformationId(row.modelDefinitionId, row.id)}>
								{row.source.modelDefinition} — {row.label}
							</option>
						))}
					</select>
				</label>
			) : null}
		</div>
	);
}
//#endregion

//#region 🔖PlaySession
interface PlaySessionProps {
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly documentModel: ModelDocument;
	readonly history: DocumentHistory;
	readonly kernel: InteractionRuntimeOptions["kernel"];
	readonly mode: SpatialComputeMode;
	readonly asideExtra: ReactNode;
	readonly sessionRestartNonce: number;
	readonly activeModelDefinitionId: string;
	readonly onActiveModelDefinitionId: (value: string) => void;
	readonly rendererSelectionByModel: SpatialRendererSelectionByModel;
	readonly onRendererSelectionByModel: (value: SpatialRendererSelectionByModel) => void;
	readonly interactionSelectionByState: SpatialInteractionSelectionByState;
	readonly onInteractionSelectionByState: (value: SpatialInteractionSelectionByState) => void;
	readonly modelDefinitionRevision: number;
	readonly onModelDefinitionRevision: (revision: number) => void;
	readonly onApplyTransformation: (spec: TransformationSpec) => void;
	readonly pickGeometry: Model;
	readonly onDocumentModelChange: (model: Model) => void;
	readonly onSnapshot: (snapshot: InteractionSnapshot) => void;
}

/** @emoji 🎮 Hosts `useInteractionRuntime` + `InteractionRepl`; same-interaction restarts use `sessionRestartNonce` without remounting GL. */
function PlaySession({
	interactionId,
	spec,
	onInteractionId,
	documentModel,
	history,
	kernel,
	mode,
	asideExtra,
	sessionRestartNonce,
	activeModelDefinitionId,
	onActiveModelDefinitionId,
	rendererSelectionByModel,
	onRendererSelectionByModel,
	interactionSelectionByState,
	onInteractionSelectionByState,
	modelDefinitionRevision,
	onModelDefinitionRevision,
	onApplyTransformation,
	pickGeometry,
	onDocumentModelChange,
	onSnapshot,
}: PlaySessionProps) {
	const rtOpts = useMemo(
		(): InteractionRuntimeOptions => ({
			kernel,
			previewKernel: r3fPreviewKernel,
			mode,
			document: documentModel,
			history,
			stateEngine: statelyStateEngineProvider,
			query: defaultConstructRunner,
			activeModelDefinitionId,
		}),
		[kernel, mode, documentModel, history, activeModelDefinitionId],
	);
	const rt = useInteractionRuntime(spec, rtOpts);
	useEffect(() => {
		return rt.subscribe(() => {
			const snap = rt.getSnapshot();
			onSnapshot(snap);
			const res = snap.lastResponse;
			if (res?.ok && res.diff && !isEmptyModelDiff(res.diff)) {
				onDocumentModelChange(Model.fromJSON(documentModel.model.toJSON()));
			}
		});
	}, [rt, documentModel, onSnapshot, onDocumentModelChange]);
	const asideWithQuery = useMemo(
		() => (
			<>
				{asideExtra}
				<ConstructQueryPanel runtime={rt} activeModelDefinitionId={activeModelDefinitionId} />
			</>
		),
		[asideExtra, rt, activeModelDefinitionId],
	);
	return (
		<InteractionRepl
			interactionId={interactionId}
			spec={spec}
			onInteractionId={onInteractionId}
			runtime={rt}
			history={history}
			document={documentModel}
			geometry={documentModel.model}
			pickGeometry={pickGeometry}
			onDocumentModelChange={onDocumentModelChange}
			asideExtra={asideWithQuery}
			sessionRestartNonce={sessionRestartNonce}
			activeModelDefinitionId={activeModelDefinitionId}
			onActiveModelDefinitionIdChange={onActiveModelDefinitionId}
			rendererSelectionByModel={rendererSelectionByModel}
			onRendererSelectionByModelChange={onRendererSelectionByModel}
			interactionSelectionByState={interactionSelectionByState}
			onInteractionSelectionByStateChange={onInteractionSelectionByState}
			modelDefinitionRevision={modelDefinitionRevision}
			onModelDefinitionRevisionChange={onModelDefinitionRevision}
			onApplyTransformation={onApplyTransformation}
			hideModelDefinitionControls
			onSnapshotChange={onSnapshot}
		/>
	);
}
//#endregion

//#region 🔖PlayApp
function PlayApp() {
	const [activeModelDefinitionId, setActiveModelDefinitionId] = useState(SHAPE_MODEL_DEFINITION_ID);
	const scopedInteractions = useMemo(
		() => listSpatialInteractionsForModelDefinition(activeModelDefinitionId),
		[activeModelDefinitionId],
	);
	const [interactionId, setInteractionId] = useState("");
	const [interactionBootId, setInteractionBootId] = useState(0);
	const [shapeAssetId, setShapeAssetId] = useState("");
	const [modelsByDefinitionId, setModelsByDefinitionId] = useState<Record<string, Model>>(emptyPlayModels);
	const [loadedRawName, setLoadedRawName] = useState("");
	const [mode, setMode] = useState<SpatialComputeMode>("fast");
	const [rendererSelectionByModel, setRendererSelectionByModel] = useState<SpatialRendererSelectionByModel>({});
	const [interactionSelectionByState, setInteractionSelectionByState] = useState<SpatialInteractionSelectionByState>({});
	const [modelDefinitionRevision, setModelDefinitionRevision] = useState(0);
	const [snapshot, setSnapshot] = useState<InteractionSnapshot | null>(null);
	const [fileStatus, setFileStatus] = useState<string>("");
	const loadInputRef = useRef<HTMLInputElement>(null);
	const spec = useMemo<InteractionSpec | null>(() => (interactionId ? loadSpatialInteraction(interactionId) : PLAY_REPL_SPEC), [interactionId]);
	const history = useDocumentHistory();
	const brepjsKernel = useMemo(() => new BrepjsKernel(), []);
	const kernel = useMemo<InteractionRuntimeOptions["kernel"]>(
		() => brepjsKernel as unknown as InteractionRuntimeOptions["kernel"],
		[brepjsKernel],
	);

	useEffect(() => {
		if (!interactionId) return;
		if (!scopedInteractions.some((row) => row.id === interactionId)) setInteractionId("");
	}, [activeModelDefinitionId, interactionId, scopedInteractions]);

	useEffect(() => {
		setModelsByDefinitionId((prev) => ensureDerivedModelInSpace(prev, activeModelDefinitionId));
	}, [activeModelDefinitionId]);

	const handleInteractionPick = useCallback(
		(id: string) => {
			if (id === interactionId) setInteractionBootId((b) => b + 1);
			else {
				setInteractionId(id);
				setInteractionBootId(0);
			}
		},
		[interactionId],
	);

	const handleShapeAssetChange = useCallback((id: string) => {
		setShapeAssetId(id);
		setLoadedRawName("");
		setFileStatus("");
		if (!id) {
			setModelsByDefinitionId(emptyPlayModels());
			setActiveModelDefinitionId(SHAPE_MODEL_DEFINITION_ID);
		} else {
			const asset = SHAPE_ASSETS.find((candidate) => candidate.id === id);
			if (!asset) return;
			setModelsByDefinitionId(modelsFromSpatialJson(asset.json));
			setActiveModelDefinitionId(activeModelDefinitionIdFromSpatialJson(asset.json));
		}
		setModelDefinitionRevision((r) => r + 1);
	}, []);

	const modelsForActiveDefinition = useMemo(
		() => ensureDerivedModelInSpace(modelsByDefinitionId, activeModelDefinitionId),
		[activeModelDefinitionId, modelsByDefinitionId],
	);

	const activeModel = useMemo(() => {
		const resolved = modelsForActiveDefinition[activeModelDefinitionId];
		if (resolved) return resolved;
		if (isShapeModelDefinition(activeModelDefinitionId)) {
			return modelsForActiveDefinition[SHAPE_MODEL_DEFINITION_ID] ?? new Model();
		}
		throw new Error(`Play model space missing model for ${activeModelDefinitionId}.`);
	}, [activeModelDefinitionId, modelsForActiveDefinition]);

	const documentModel = useMemo((): ModelDocument => {
		const model = Model.fromJSON(activeModel.toJSON());
		return { model: model, nodes: [] };
	}, [activeModel, modelDefinitionRevision]);
	const liveModel = documentModel.model;

	const flushedModelsByDefinitionId = useMemo(() => {
		const flushed = flushModelsRecord(modelsByDefinitionId, activeModelDefinitionId, liveModel);
		return ensureDerivedModelInSpace(flushed, activeModelDefinitionId);
	}, [activeModelDefinitionId, liveModel, modelsByDefinitionId]);

	const playModelSpace = useMemo(
		() => modelSpaceFromRecord(flushedModelsByDefinitionId),
		[flushedModelsByDefinitionId],
	);

	const visibleExportModel = useMemo(
		() => flushedModelsByDefinitionId[activeModelDefinitionId] ?? liveModel,
		[activeModelDefinitionId, flushedModelsByDefinitionId, liveModel],
	);

	const pickGeometry = useMemo(
		() => pickShapeForModelDefinition(flushedModelsByDefinitionId, activeModelDefinitionId, liveModel),
		[activeModelDefinitionId, flushedModelsByDefinitionId, liveModel],
	);

	const handleActiveModelDefinitionChange = useCallback(
		(nextId: string) => {
			setModelsByDefinitionId((prev) => {
				const flushed = flushModelsRecord(prev, activeModelDefinitionId, liveModel);
				return ensureDerivedModelInSpace(flushed, nextId);
			});
			setActiveModelDefinitionId(nextId);
			setModelDefinitionRevision((r) => r + 1);
		},
		[activeModelDefinitionId, liveModel],
	);

	const handleModelAttributesChange = useCallback(
		(model: Model) => {
			setModelsByDefinitionId((prev) => ({ ...prev, [activeModelDefinitionId]: Model.fromJSON(model.toJSON()) }));
			setModelDefinitionRevision((r) => r + 1);
		},
		[activeModelDefinitionId],
	);

	const interactionActive = useMemo(
		() => Boolean(snapshot) && isInteractionSessionActive(spec ?? PLAY_REPL_SPEC, snapshot?.state ?? "idle"),
		[spec, snapshot],
	);
	const boundInteractionSession = Boolean(interactionId) && interactionActive;
	const handleSnapshotChange = useCallback((next: InteractionSnapshot) => {
		setSnapshot((prev) => {
			if (prev && prev.revision === next.revision && prev.state === next.state) return prev;
			return next;
		});
	}, []);
	const currentSelection = useMemo(
		() =>
			replDisplayedSelectionTargets(
				boundInteractionSession,
				activeModelDefinitionId,
				snapshot?.state ?? "idle",
				rendererSelectionByModel,
				interactionSelectionByState,
			),
		[
			boundInteractionSession,
			activeModelDefinitionId,
			snapshot?.state,
			rendererSelectionByModel,
			interactionSelectionByState,
		],
	);
	const selectionKinds = useMemo(
		() => new Set(modelDefinitionSelectionEntityKinds(activeModelDefinitionId)),
		[activeModelDefinitionId],
	);
	const viewObjectCount = useMemo(
		() => countViewObjectsForModelDefinition(liveModel, activeModelDefinitionId),
		[liveModel, activeModelDefinitionId, modelDefinitionRevision],
	);

	const selectionInScope = useMemo(
		() =>
			currentSelection.filter((target) => {
				if (target.kind === "object" && target.editable === false) return selectionKinds.has("object");
				return selectionKinds.has(target.kind);
			}),
		[currentSelection, selectionKinds],
	);

	const exportBaseName = useMemo(() => {
		if (loadedRawName) return fileStem(loadedRawName);
		const asset = SHAPE_ASSETS.find((g) => g.id === shapeAssetId);
		return fileStem(asset?.id ?? "spatial");
	}, [shapeAssetId, loadedRawName]);

	const handleApplyTransformation = useCallback(
		(spec: TransformationSpec) => {
			const space = modelSpaceFromRecord(flushModelsRecord(modelsByDefinitionId, activeModelDefinitionId, liveModel));
			try {
				space.transform(spec.source.modelDefinition, spec.target.modelDefinition, spec);
			} catch (error) {
				setFileStatus(`Transform failed: ${String(error)}`);
				return;
			}
			setModelsByDefinitionId(recordFromModelSpace(space));
			setActiveModelDefinitionId(spec.target.modelDefinition);
			setModelDefinitionRevision((r) => r + 1);
			setFileStatus(`Transformed ${spec.source.modelDefinition} → ${spec.target.modelDefinition}.`);
		},
		[activeModelDefinitionId, liveModel, modelsByDefinitionId],
	);

	useEffect(() => {
		history.clear();
		setSnapshot(null);
	}, [history, modelDefinitionRevision]);

	const saveBundle = useCallback(
		async (name: string, payload: SpatialExchangeBundle, message: string) => {
			try {
				await writeJsonFile(name, payload);
				setFileStatus(message);
			} catch (error) {
				setFileStatus(`Save failed: ${String(error)}`);
			}
		},
		[],
	);

	const handleSaveSelected = useCallback(async () => {
		const selectedModel = Model.fromJSON(selectRawModel(liveModel, selectionInScope));
		const selectedModelSpace = new ModelSpace();
		selectedModelSpace.link(activeModelDefinitionId, selectedModel);
		await saveBundle(
			`${exportBaseName}.selected.spatial.json`,
			{ model: selectedModel.toJSON(), modelSpace: selectedModelSpace.toJSON(), activeModelDefinitionId },
			`Saved ${selectionInScope.length} selected item(s) for ${activeModelDefinitionId}.`,
		);
	}, [activeModelDefinitionId, exportBaseName, liveModel, saveBundle, selectionInScope]);

	const handleSaveInPlay = useCallback(async () => {
		try {
			const stepText = await brepjsKernel.exportModelSpaceToStep(playModelSpace, exportBaseName);
			await writeStepFile(`${exportBaseName}.modelspace.stp`, stepText);
			setFileStatus(`Saved model space (${Object.keys(playModelSpace.models).length} model(s)) to STEP.`);
		} catch (error) {
			setFileStatus(`Save failed: ${String(error)}`);
		}
	}, [brepjsKernel, exportBaseName, playModelSpace]);

	const handleSaveCurrent = useCallback(async () => {
		try {
			const modelId = activeModelDefinitionId;
			const stepText = await brepjsKernel.exportModelToStep(visibleExportModel, modelId);
			const stem = sanitizeModelDefinitionFileStem(modelId);
			await writeStepFile(`${exportBaseName}.${stem}.stp`, stepText);
			setFileStatus(`Saved ${modelId} to STEP.`);
		} catch (error) {
			setFileStatus(`Save failed: ${String(error)}`);
		}
	}, [activeModelDefinitionId, brepjsKernel, exportBaseName, visibleExportModel]);

	const handleLoadRawRequest = useCallback(() => {
		loadInputRef.current?.click();
	}, []);

	const handleLoadRaw = useCallback(async (event: ChangeEvent<HTMLInputElement>) => {
		const file = event.target.files?.[0];
		if (!file) return;
		try {
			const parsed = JSON.parse(await file.text()) as unknown;
			const envelope = parsed as Record<string, unknown>;
			const snapshot =
				envelope && typeof envelope === "object" && "modelSpace" in envelope
					? envelope.modelSpace
					: envelope && typeof envelope === "object" && "model" in envelope
					? envelope.model
					: envelope && typeof envelope === "object" && "raw" in envelope
						? envelope.raw
						: parsed;
			const modelSpace = parseModelSpaceJson(snapshot);
			if (modelSpace) {
				const nextActiveModelDefinitionId =
					typeof envelope.activeModelDefinitionId === "string" && modelSpace.get(envelope.activeModelDefinitionId)
						? envelope.activeModelDefinitionId
						: activeModelDefinitionIdFromSpatialJson(snapshot);
				setShapeAssetId("");
				setLoadedRawName(file.name);
				setModelsByDefinitionId(recordFromModelSpace(modelSpace));
				setActiveModelDefinitionId(nextActiveModelDefinitionId);
				setModelDefinitionRevision((r) => r + 1);
				setFileStatus(`Loaded model space from ${file.name}.`);
				return;
			}
			const model = parseModelJson(snapshot);
			if (!model) throw new Error("No spatial model found in file.");
			setShapeAssetId("");
			setLoadedRawName(file.name);
			setModelsByDefinitionId(modelsFromSpatialJson(model.toJSON()));
			setActiveModelDefinitionId(SHAPE_MODEL_DEFINITION_ID);
			setModelDefinitionRevision((r) => r + 1);
			setFileStatus(`Loaded model from ${file.name}.`);
		} catch (error) {
			setFileStatus(`Load failed: ${String(error)}`);
		} finally {
			event.target.value = "";
		}
	}, []);

	const asideExtra: ReactNode = (
		<>
			<PlayModelSpacePanel
				activeModelDefinitionId={activeModelDefinitionId}
				modelSpaceCount={Object.keys(playModelSpace.models).length}
				viewObjectCount={viewObjectCount}
				onActiveModelDefinitionId={handleActiveModelDefinitionChange}
				onApplyTransformation={handleApplyTransformation}
			/>
			<SelectionAttributesPanel
				model={liveModel}
				activeModelDefinitionId={activeModelDefinitionId}
				selection={selectionInScope}
				selectionCount={selectionInScope.length}
				onModelChange={handleModelAttributesChange}
			/>
			<SelectionPropertiesPanel
				model={liveModel}
				kernel={brepjsKernel}
				activeModelDefinitionId={activeModelDefinitionId}
				selection={selectionInScope}
				selectionCount={selectionInScope.length}
			/>
			<div style={{ display: "flex", gap: 6, fontSize: 12 }}>
				<span style={{ fontWeight: 600, color: "#c8c8e0", alignSelf: "center" }}>Compute</span>
				<button
					type="button"
					onClick={() => setMode("fast")}
					style={{
						flex: 1,
						padding: "6px 10px",
						borderRadius: 6,
						border: "1px solid #2a2a3c",
						background: mode === "fast" ? "#3a4a6a" : "#1a1a28",
						color: "#e8e8f0",
						cursor: "pointer",
					}}
				>
					Fast
				</button>
				<button
					type="button"
					onClick={() => setMode("precise")}
					style={{
						flex: 1,
						padding: "6px 10px",
						borderRadius: 6,
						border: "1px solid #2a2a3c",
						background: mode === "precise" ? "#3a4a6a" : "#1a1a28",
						color: "#e8e8f0",
						cursor: "pointer",
					}}
				>
					Precise
				</button>
			</div>
			{isShapeModelDefinition(activeModelDefinitionId) ? (
				<label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12 }}>
					<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Shape asset</span>
					<select
						value={shapeAssetId}
						onChange={(e) => handleShapeAssetChange(e.target.value)}
						style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
					>
						<option value="">No asset</option>
						{SHAPE_ASSETS.map((g) => (
							<option key={g.id} value={g.id}>
								[{g.key}] {g.label} ({modelVertexCount(g.json)} verts)
							</option>
						))}
					</select>
				</label>
			) : (
				<span style={{ fontSize: 12, opacity: 0.75, lineHeight: 1.4 }}>
					Shape assets apply to <code style={{ color: "#e8e8f0" }}>spatial.shape</code>. Switch model definition to spatial.shape to
					change source shape; derived models share it via transforms.
				</span>
			)}
			<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
				<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Files</span>
				<div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
					<button
						type="button"
						onClick={() => void handleSaveSelected()}
						disabled={selectionInScope.length === 0}
						style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}
					>
						Save (Selected)
					</button>
					<button type="button" onClick={() => void handleSaveInPlay()} style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}>
						Save in play
					</button>
					<button type="button" onClick={() => void handleSaveCurrent()} style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}>
						Save (Current)
					</button>
					<button type="button" onClick={handleLoadRawRequest} style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}>
						Load
					</button>
				</div>
				<input ref={loadInputRef} type="file" accept=".json,.spatial.json" hidden onChange={(event) => void handleLoadRaw(event)} />
				{fileStatus ? <span style={{ color: fileStatus.startsWith("Load failed") || fileStatus.startsWith("Save failed") ? "#ff9a9a" : "#a8d8a8" }}>{fileStatus}</span> : null}
			</div>
		</>
	);

	if (!spec) {
		return (
			<div style={{ padding: 16, color: "#f88" }}>
				Unknown interaction <code>{interactionId}</code>.
				<button type="button" onClick={() => setInteractionId("")}>
					Reset
				</button>
			</div>
		);
	}

	return (
		<PlaySession
			interactionId={interactionId}
			spec={spec}
			onInteractionId={handleInteractionPick}
			documentModel={documentModel}
			history={history}
			kernel={kernel}
			mode={mode}
			asideExtra={asideExtra}
			sessionRestartNonce={interactionBootId}
			activeModelDefinitionId={activeModelDefinitionId}
			onActiveModelDefinitionId={handleActiveModelDefinitionChange}
			rendererSelectionByModel={rendererSelectionByModel}
			onRendererSelectionByModel={setRendererSelectionByModel}
			interactionSelectionByState={interactionSelectionByState}
			onInteractionSelectionByState={setInteractionSelectionByState}
			modelDefinitionRevision={modelDefinitionRevision}
			onModelDefinitionRevision={setModelDefinitionRevision}
			onApplyTransformation={handleApplyTransformation}
			pickGeometry={pickGeometry}
			onDocumentModelChange={handleModelAttributesChange}
			onSnapshot={handleSnapshotChange}
		/>
	);
}
//#endregion

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, it, expect } = import.meta.vitest;

	describe("spatial play typology chrome", () => {
		it("lists energy typologies from model definition scope", () => {
			const scope = resolveModelDefinitionScope("aec.building.energy");
			const labels = scope.typologies.map((row) => typologyObjectPascalFromLabel(row.label));
			expect(labels).toContain("BasePlate");
			expect(labels).toContain("ExternalWall");
			expect(labels).toContain("Roof");
		});
	});

	describe("spatial play model bootstrap", () => {
		it("emptyPlayModels always seeds spatial.shape", () => {
			expect(emptyPlayModels()[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
		});

		it("modelsFromSpatialJson on empty model space still seeds spatial.shape", () => {
			const models = modelsFromSpatialJson(new ModelSpace().toJSON());
			expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
		});

		it("modelsFromSpatialJson loads fixture models under spatial.shape", () => {
			const models = modelsFromSpatialJson(geometrySmallBuilding);
			expect(models[SHAPE_MODEL_DEFINITION_ID]?.objects).not.toEqual({});
		});

		it("ensureDerivedModelInSpace keeps spatial.shape for shape definition", () => {
			const models = ensureDerivedModelInSpace({}, SHAPE_MODEL_DEFINITION_ID);
			expect(models[SHAPE_MODEL_DEFINITION_ID]).toBeInstanceOf(Model);
		});
	});
}
//#endregion

const el = document.getElementById("root");
if (el && !import.meta.vitest) {
	createRoot(el).render(
		<StrictMode>
			<PlayApp />
		</StrictMode>,
	);
}
