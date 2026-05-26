/** @emoji 🎮 Vite entry: geometry catalog + `BrepjsKernel` + `InteractionRepl` + `construct` query runner. */
import { StrictMode, useCallback, useEffect, useMemo, useRef, useState, type ChangeEvent, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	DerivedViewService,
	DocumentHistory,
	isInteractionSessionActive,
	listSpatialInteractions,
	loadSpatialInteraction,
	parseModelJson,
	type InteractionSnapshot,
	type InteractionRuntime,
	type InteractionSpec,
	type InteractionRuntimeOptions,
	type SpatialComputeMode,
	type ModelDocument,
	type SelectionTarget,
	type SurfaceView,
	type PartView,
	type VolumeView,
	Model,
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
	replDisplayedSelectionTargets,
	r3fPreviewKernel,
	useDocumentHistory,
	useInteractionRuntime,
	type SpatialPickViewKind,
} from "../index";

//#region 🔖ConstructQueryPanel
/** @emoji 🔍 Play-only `construct` runner bound to the live `InteractionRuntime`. */
function ConstructQueryPanel({ runtime }: { readonly runtime: InteractionRuntime }) {
	const [text, setText] = useState("MATCH (v:Vertex) RETURN v.id LIMIT 8");
	const [out, setOut] = useState("");
	const [busy, setBusy] = useState(false);
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
	const verts = json.vertices;
	return Array.isArray(verts) ? verts.length : 0;
}

const GEOMETRY_ASSETS = [
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

interface SpatialAnalyticBundle {
	readonly surfaces: readonly SurfaceView[];
	readonly parts: readonly PartView[];
	readonly volumes: readonly VolumeView[];
}

interface SpatialExchangeBundle {
	readonly raw?: ModelJsonSnapshot;
	readonly analytic?: SpatialAnalyticBundle;
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

function emptyModelJson(): ModelJsonSnapshot {
	return new Model().toJSON();
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
	const anchors = new Set<string>();
	const vertices = new Set<string>();
	const edges = new Set<string>();
	const wires = new Set<string>();
	const faces = new Set<string>();
	const shells = new Set<string>();
	const cells = new Set<string>();
	const cellComplexes = new Set<string>();
	const clusters = new Set<string>();

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
		if (model.cells[id]) {
			visitCell(id);
			return;
		}
		if (model.cellComplexes[id]) {
			visitCellComplex(id);
			return;
		}
		if (model.clusters[id]) visitCluster(id);
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

	const visitCell = (id: string): void => {
		if (cells.has(id)) return;
		const rec = model.cells[id];
		if (!rec) return;
		cells.add(id);
		for (const shellId of rec.shellIds) visitShell(shellId);
	};

	const visitCellComplex = (id: string): void => {
		if (cellComplexes.has(id)) return;
		const rec = model.cellComplexes[id];
		if (!rec) return;
		cellComplexes.add(id);
		for (const cellId of rec.cellIds) visitCell(cellId);
	};

	const visitCluster = (id: string): void => {
		if (clusters.has(id)) return;
		const rec = model.clusters[id];
		if (!rec) return;
		clusters.add(id);
		for (const memberId of rec.memberIds) visitById(memberId);
	};

	for (const target of selection) {
		switch (target.kind) {
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
			case "cell":
				visitCell(target.id);
				break;
			case "cellComplex":
				visitCellComplex(target.id);
				break;
			case "cluster":
				visitCluster(target.id);
				break;
			default:
				break;
		}
	}

	const sortIds = (ids: Set<string>) => [...ids].sort((a, b) => a.localeCompare(b));
	return {
		schema: "spatial.model/v1",
		revision: model.revision,
		anchors: sortIds(anchors).map((id) => model.anchors[id]!),
		vertices: sortIds(vertices).map((id) => model.vertices[id]!),
		edges: sortIds(edges).map((id) => model.edges[id]!),
		wires: sortIds(wires).map((id) => model.wires[id]!),
		faces: sortIds(faces).map((id) => model.faces[id]!),
		shells: sortIds(shells).map((id) => model.shells[id]!),
		cells: sortIds(cells).map((id) => model.cells[id]!),
		cellComplexes: sortIds(cellComplexes).map((id) => model.cellComplexes[id]!),
		clusters: sortIds(clusters).map((id) => model.clusters[id]!),
	};
}

function createAnalyticBundle(
	derived: DerivedViewService,
	topo: Model,
	selection?: readonly SelectionTarget[],
): SpatialAnalyticBundle {
	const allSurfaces = derived.computeSurfaces(model);
	const allParts = derived.computeParts(model);
	const allVolumes = derived.computeVolumes(model);
	if (!selection) return { surfaces: allSurfaces, parts: allParts, volumes: allVolumes };
	const selected = new Set(selection.map((target) => `${target.kind}:${target.id}`));
	return {
		surfaces: allSurfaces.filter((surface) => selected.has(`surface:${String(surface.id)}`)),
		parts: allParts.filter((part) => selected.has(`part:${String(part.id)}`)),
		volumes: allVolumes.filter((volume) => selected.has(`volume:${String(volume.id)}`)),
	};
}

async function writeJsonFile(name: string, payload: SpatialExchangeBundle): Promise<void> {
	const text = `${JSON.stringify(payload, null, 2)}\n`;
	const pickerWindow = window as SavePickerWindow;
	if (pickerWindow.showSaveFilePicker) {
		const handle = await pickerWindow.showSaveFilePicker({
			suggestedName: name,
			types: [{ description: "Spatial JSON", accept: { "application/json": [".json", ".spatial.json"] } }],
		});
		const writable = await handle.createWritable();
		await writable.write(text);
		await writable.close();
		return;
	}
	const href = URL.createObjectURL(new Blob([text], { type: "application/json" }));
	const link = document.createElement("a");
	link.href = href;
	link.download = name;
	link.click();
	URL.revokeObjectURL(href);
}
//#endregion

//#region 🔖PlaySession
interface PlaySessionProps {
	readonly interactions: ReturnType<typeof listSpatialInteractions>;
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly documentModel: ModelDocument;
	readonly history: DocumentHistory;
	readonly kernel: InteractionRuntimeOptions["kernel"];
	readonly derived: DerivedViewService;
	readonly mode: SpatialComputeMode;
	readonly asideExtra: ReactNode;
	readonly sessionRestartNonce: number;
	readonly pickViewKind: SpatialPickViewKind;
	readonly onPickViewKind: (value: SpatialPickViewKind) => void;
	readonly rendererSelection: readonly SelectionTarget[];
	readonly onRendererSelection: (value: readonly SelectionTarget[]) => void;
	readonly interactionSelection: readonly SelectionTarget[];
	readonly onInteractionSelection: (value: readonly SelectionTarget[]) => void;
	readonly derivedRevision: number;
	readonly onDerivedRevision: (revision: number) => void;
	readonly onSnapshot: (snapshot: InteractionSnapshot) => void;
}

/** @emoji 🎮 Hosts `useInteractionRuntime` + `InteractionRepl`; same-interaction restarts use `sessionRestartNonce` without remounting GL. */
function PlaySession({
	interactions,
	interactionId,
	spec,
	onInteractionId,
	documentModel,
	history,
	kernel,
	derived,
	mode,
	asideExtra,
	sessionRestartNonce,
	pickViewKind,
	onPickViewKind,
	rendererSelection,
	onRendererSelection,
	interactionSelection,
	onInteractionSelection,
	derivedRevision,
	onDerivedRevision,
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
			derived,
		}),
		[kernel, mode, documentModel, history, derived],
	);
	const rt = useInteractionRuntime(spec, rtOpts);
	const asideWithQuery = useMemo(
		() => (
			<>
				{asideExtra}
				<ConstructQueryPanel runtime={rt} />
			</>
		),
		[asideExtra, rt],
	);
	return (
		<InteractionRepl
			interactions={interactions}
			interactionId={interactionId}
			spec={spec}
			onInteractionId={onInteractionId}
			runtime={rt}
			history={history}
			document={documentModel}
			geometry={documentModel.model}
			derived={derived}
			asideExtra={asideWithQuery}
			sessionRestartNonce={sessionRestartNonce}
			pickViewKind={pickViewKind}
			onPickViewKindChange={onPickViewKind}
			rendererSelection={rendererSelection}
			onRendererSelectionChange={onRendererSelection}
			interactionSelection={interactionSelection}
			onInteractionSelectionChange={onInteractionSelection}
			derivedRevision={derivedRevision}
			onDerivedRevisionChange={onDerivedRevision}
			onSnapshotChange={onSnapshot}
		/>
	);
}
//#endregion

//#region 🔖PlayApp
function PlayApp() {
	const interactions = useMemo(() => listSpatialInteractions(), []);
	const [interactionId, setInteractionId] = useState("");
	const [interactionBootId, setInteractionBootId] = useState(0);
	const [geometryAssetId, setGeometryAssetId] = useState("small-building");
	const [modelJson, setModelJson] = useState<unknown>(() => {
		const asset = GEOMETRY_ASSETS.find((g) => g.id === "small-building");
		return asset?.json ?? emptyModelJson();
	});
	const [loadedRawName, setLoadedRawName] = useState("");
	const [mode, setMode] = useState<SpatialComputeMode>("fast");
	const [pickViewKind, setPickViewKind] = useState<SpatialPickViewKind>("raw");
	const [rendererSelection, setRendererSelection] = useState<readonly SelectionTarget[]>([]);
	const [interactionSelection, setInteractionSelection] = useState<readonly SelectionTarget[]>([]);
	const [derivedRevision, setDerivedRevision] = useState(0);
	const [snapshot, setSnapshot] = useState<InteractionSnapshot | null>(null);
	const [fileStatus, setFileStatus] = useState<string>("");
	const loadInputRef = useRef<HTMLInputElement>(null);
	const spec = useMemo<InteractionSpec | null>(() => (interactionId ? loadSpatialInteraction(interactionId) : PLAY_REPL_SPEC), [interactionId]);
	const history = useDocumentHistory();
	const kernel = useMemo<InteractionRuntimeOptions["kernel"]>(() => new BrepjsKernel() as unknown as InteractionRuntimeOptions["kernel"], []);
	const derived = useMemo(() => new DerivedViewService(kernel), [kernel]);

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

	const handleGeometryAssetChange = useCallback((id: string) => {
		setGeometryAssetId(id);
		setLoadedRawName("");
		setFileStatus("");
		const asset = GEOMETRY_ASSETS.find((candidate) => candidate.id === id);
		setModelJson(asset?.json ?? emptyModelJson());
	}, []);

	const interactionModel = useMemo(() => parseModelJson(modelJson) ?? new Model(), [modelJson]);

	const documentModel = useMemo((): ModelDocument => {
		const model = Model.fromJSON(interactionModel.toJSON());
		return { model: model, nodes: [] };
	}, [interactionModel]);
	const liveModel = documentModel.model;

	const interactionActive = useMemo(
		() => Boolean(snapshot) && isInteractionSessionActive(spec ?? PLAY_REPL_SPEC, snapshot?.state ?? "idle"),
		[spec, snapshot],
	);
	const handleSnapshotChange = useCallback((next: InteractionSnapshot) => {
		setSnapshot((prev) => {
			if (prev && prev.revision === next.revision && prev.state === next.state) return prev;
			return next;
		});
	}, []);
	const currentSelection = useMemo(
		() => replDisplayedSelectionTargets(interactionActive, pickViewKind, rendererSelection, interactionSelection),
		[interactionActive, pickViewKind, rendererSelection, interactionSelection],
	);
	const selectedRaw = useMemo(
		() => currentSelection.filter((target) => target.kind !== "surface" && target.kind !== "part" && target.kind !== "volume"),
		[currentSelection],
	);
	const selectedAnalytic = useMemo(
		() => currentSelection.filter((target) => target.kind === "surface" || target.kind === "part" || target.kind === "volume"),
		[currentSelection],
	);
	const exportBaseName = useMemo(() => {
		if (loadedRawName) return fileStem(loadedRawName);
		const asset = GEOMETRY_ASSETS.find((g) => g.id === geometryAssetId);
		return fileStem(asset?.id ?? "spatial");
	}, [geometryAssetId, loadedRawName]);

	useEffect(() => {
		history.clear();
		setSnapshot(null);
		setRendererSelection([]);
		setInteractionSelection([]);
		setDerivedRevision(0);
	}, [history, modelJson]);

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

	const createLiveAnalyticBundle = useCallback(
		async (selection?: readonly SelectionTarget[]) => {
			await derived.refresh(liveModel);
			return createAnalyticBundle(derived, liveModel, selection);
		},
		[derived, liveModel],
	);

	const handleSaveSelected = useCallback(async () => {
		if (pickViewKind === "raw") {
			await saveBundle(
				`${exportBaseName}.selected.raw.spatial.json`,
				{ raw: selectRawModel(liveModel, selectedRaw) },
				`Saved ${selectedRaw.length} selected raw item(s).`,
			);
			return;
		}
		await saveBundle(
			`${exportBaseName}.selected.analytic.spatial.json`,
			{ analytic: await createLiveAnalyticBundle(selectedAnalytic) },
			`Saved ${selectedAnalytic.length} selected analytic item(s).`,
		);
	}, [createLiveAnalyticBundle, exportBaseName, liveModel, pickViewKind, saveBundle, selectedAnalytic, selectedRaw]);

	const handleSaveView = useCallback(async () => {
		if (pickViewKind === "raw") {
			await saveBundle(
				`${exportBaseName}.raw.spatial.json`,
				{ raw: liveModel.toJSON() },
				"Saved the raw view.",
			);
			return;
		}
		await saveBundle(
			`${exportBaseName}.analytic.spatial.json`,
			{ analytic: await createLiveAnalyticBundle() },
			"Saved the analytic view.",
		);
	}, [createLiveAnalyticBundle, exportBaseName, liveModel, pickViewKind, saveBundle]);

	const handleSaveAll = useCallback(async () => {
		await saveBundle(
			`${exportBaseName}.spatial.json`,
			{ raw: liveModel.toJSON(), analytic: await createLiveAnalyticBundle() },
			"Saved raw and analytic data.",
		);
	}, [createLiveAnalyticBundle, exportBaseName, liveModel, saveBundle]);

	const handleLoadRawRequest = useCallback(() => {
		loadInputRef.current?.click();
	}, []);

	const handleLoadRaw = useCallback(async (event: ChangeEvent<HTMLInputElement>) => {
		const file = event.target.files?.[0];
		if (!file) return;
		try {
			const parsed = JSON.parse(await file.text()) as unknown;
			const raw =
				parsed && typeof parsed === "object" && "raw" in (parsed as Record<string, unknown>)
					? (parsed as Record<string, unknown>).raw
					: parsed;
			const model = parseModelJson(raw);
			if (!model) throw new Error("No raw spatial model found in file.");
			setGeometryAssetId("");
			setLoadedRawName(file.name);
			setModelJson(model.toJSON());
			setFileStatus(`Loaded raw model from ${file.name}.`);
		} catch (error) {
			setFileStatus(`Load failed: ${String(error)}`);
		} finally {
			event.target.value = "";
		}
	}, []);

	const asideExtra: ReactNode = (
		<>
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
			<label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12 }}>
				Geometry asset
				<select
					value={geometryAssetId}
					onChange={(e) => handleGeometryAssetChange(e.target.value)}
					style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
				>
					<option value="">No asset</option>
					{GEOMETRY_ASSETS.map((g) => (
						<option key={g.id} value={g.id}>
							[{g.key}] {g.label} ({modelVertexCount(g.json)} verts)
						</option>
					))}
				</select>
			</label>
			<div style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
				<span style={{ fontWeight: 600, color: "#c8c8e0" }}>Files</span>
				<div style={{ display: "flex", gap: 6, flexWrap: "wrap" }}>
					<button
						type="button"
						onClick={() => void handleSaveSelected()}
						disabled={pickViewKind === "raw" ? selectedRaw.length === 0 : selectedAnalytic.length === 0}
						style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}
					>
						Save (Selected)
					</button>
					<button type="button" onClick={() => void handleSaveView()} style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}>
						Save (View)
					</button>
					<button type="button" onClick={() => void handleSaveAll()} style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}>
						Save (All)
					</button>
					<button type="button" onClick={handleLoadRawRequest} style={{ padding: "6px 10px", borderRadius: 6, cursor: "pointer" }}>
						Load (Raw)
					</button>
				</div>
				<input ref={loadInputRef} type="file" accept=".json,.spatial.json,.raw.spatial.json" hidden onChange={(event) => void handleLoadRaw(event)} />
				{fileStatus ? <span style={{ color: fileStatus.startsWith("Load failed") || fileStatus.startsWith("Save failed") ? "#ff9a9a" : "#a8d8a8" }}>{fileStatus}</span> : null}
			</div>
		</>
	);

	if (!interactions.length) {
		return <div style={{ padding: 16, color: "#f88" }}>No spatial interactions registered.</div>;
	}
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
			interactions={interactions}
			interactionId={interactionId}
			spec={spec}
			onInteractionId={handleInteractionPick}
			documentModel={documentModel}
			history={history}
			kernel={kernel}
			derived={derived}
			mode={mode}
			asideExtra={asideExtra}
			sessionRestartNonce={interactionBootId}
			pickViewKind={pickViewKind}
			onPickViewKind={setPickViewKind}
			rendererSelection={rendererSelection}
			onRendererSelection={setRendererSelection}
			interactionSelection={interactionSelection}
			onInteractionSelection={setInteractionSelection}
			derivedRevision={derivedRevision}
			onDerivedRevision={setDerivedRevision}
			onSnapshot={handleSnapshotChange}
		/>
	);
}
//#endregion

const el = document.getElementById("root");
if (el) {
	createRoot(el).render(
		<StrictMode>
			<PlayApp />
		</StrictMode>,
	);
}
