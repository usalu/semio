/** @emoji 🎮 Vite entry: geometry catalog + `BrepjsKernel` + `InteractionRepl` + `construct` query runner. */
import { StrictMode, useCallback, useEffect, useMemo, useState, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	DerivedViewService,
	listSpatialInteractions,
	loadSpatialInteraction,
	parseTopologyGraphJson,
	type InteractionRuntime,
	type InteractionSpec,
	type InteractionRuntimeOptions,
	type SpatialComputeMode,
	type ModelDocument,
	TopologyGraph,
} from "@spatial/js-core";
import { defaultConstructRunner } from "@spatial/js-query";
import geometryNakagin from "../../../fixtures/geometry.json" with { type: "json" };
import geometryLoom from "../../../fixtures/geometry-loom.json" with { type: "json" };
import geometryRoutes from "../../../fixtures/geometry-routes.json" with { type: "json" };
import geometrySmallBuilding from "../../../fixtures/small-building.topology.json" with { type: "json" };
import geometryTallBuilding from "../../../fixtures/tall-building.topology.json" with { type: "json" };
import geometryLargeBuilding from "../../../fixtures/large-building.topology.json" with { type: "json" };
import { BrepjsKernel } from "@spatial/js-kernel-brepjs";
import { statelyStateEngineProvider } from "@spatial/js-machine-stately";
import {
	DocumentHistory,
	InteractionRepl,
	r3fPreviewKernel,
	useDocumentHistory,
	useInteractionRuntime,
} from "../index.tsx";

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
function topologyVertexCount(json: Record<string, unknown>): number {
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
//#endregion

//#region 🔖PlaySession
interface PlaySessionProps {
	readonly interactions: ReturnType<typeof listSpatialInteractions>;
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly documentModel: ModelDocument;
	readonly history: DocumentHistory;
	readonly kernel: BrepjsKernel;
	readonly mode: SpatialComputeMode;
	readonly asideExtra: ReactNode;
	readonly sessionRestartNonce: number;
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
	mode,
	asideExtra,
	sessionRestartNonce,
}: PlaySessionProps) {
	const derived = useMemo(() => new DerivedViewService(kernel), [kernel]);
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
			geometry={documentModel.topology}
			derived={derived}
			asideExtra={asideWithQuery}
			sessionRestartNonce={sessionRestartNonce}
		/>
	);
}
//#endregion

//#region 🔖PlayApp
function PlayApp() {
	const interactions = useMemo(() => listSpatialInteractions(), []);
	const [interactionId, setInteractionId] = useState("");
	const [interactionBootId, setInteractionBootId] = useState(0);
	const [geometryAssetId, setGeometryAssetId] = useState("");
	const [mode, setMode] = useState<SpatialComputeMode>("fast");
	const spec = useMemo<InteractionSpec | null>(() => (interactionId ? loadSpatialInteraction(interactionId) : PLAY_REPL_SPEC), [interactionId]);

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

	const interactionTopo = useMemo(() => {
		const asset = GEOMETRY_ASSETS.find((g) => g.id === geometryAssetId);
		if (!asset) return new TopologyGraph();
		return parseTopologyGraphJson(asset.json) ?? new TopologyGraph();
	}, [geometryAssetId]);

	const documentModel = useMemo((): ModelDocument => {
		const topo = TopologyGraph.fromJSON(interactionTopo.toJSON());
		return { topology: topo, nodes: [] };
	}, [interactionTopo]);

	const history = useDocumentHistory();
	const kernel = useMemo(() => new BrepjsKernel(), []);

	useEffect(() => {
		history.clear();
	}, [history, geometryAssetId]);

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
					onChange={(e) => setGeometryAssetId(e.target.value)}
					style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
				>
					<option value="">No asset</option>
					{GEOMETRY_ASSETS.map((g) => (
						<option key={g.id} value={g.id}>
							[{g.key}] {g.label} ({topologyVertexCount(g.json)} verts)
						</option>
					))}
				</select>
			</label>
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
			mode={mode}
			asideExtra={asideExtra}
			sessionRestartNonce={interactionBootId}
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
