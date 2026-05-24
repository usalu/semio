/** @emoji 🎮 Vite entry: geometry catalog + `BrepjsKernel` + `InteractionRepl`. */
import { StrictMode, useCallback, useEffect, useMemo, useState, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	listSpatialInteractionPresets,
	loadSpatialInteractionPreset,
	parseTopologyGraphJson,
	type InteractionSpec,
	type InteractionRuntimeOptions,
	type ModelDocument,
	TopologyGraph,
} from "@spatial/js-core";
import geometryNakagin from "../../../fixtures/geometry.json" with { type: "json" };
import geometryLoom from "../../../fixtures/geometry-loom.json" with { type: "json" };
import geometryRoutes from "../../../fixtures/geometry-routes.json" with { type: "json" };
import geometrySmallBuilding from "../../../fixtures/small-building.topology.json" with { type: "json" };
import geometryTallBuilding from "../../../fixtures/tall-building.topology.json" with { type: "json" };
import geometryLargeBuilding from "../../../fixtures/large-building.topology.json" with { type: "json" };
import { BrepjsKernel } from "@spatial/js-kernel-brepjs";
import { statelyStateEngineProvider } from "@spatial/js-machine-stately";
import {
	ArchivedBoxLayout,
	DocumentHistory,
	InteractionRepl,
	useDocumentHistory,
	useInteractionRuntime,
} from "../index.tsx";

//#region 🔖GeometryCatalog
const GEOMETRY_ASSETS = [
	{ id: "nakagin-slice", key: "a", label: "Nakagin capsule (8 verts)", json: geometryNakagin as Record<string, unknown> },
	{ id: "geometry-loom", key: "l", label: "Loom deck + pent loop + rail", json: geometryLoom as Record<string, unknown> },
	{ id: "geometry-routes", key: "r", label: "Multi-route lattice (24 verts)", json: geometryRoutes as Record<string, unknown> },
	{ id: "small-building", key: "s", label: "Small building (264 verts)", json: geometrySmallBuilding as Record<string, unknown> },
	{ id: "tall-building", key: "t", label: "Tall building (680 verts)", json: geometryTallBuilding as Record<string, unknown> },
	{ id: "large-building", key: "b", label: "Large building (12,370 verts)", json: geometryLargeBuilding as Record<string, unknown> },
] as const;
//#endregion

//#region 🔖PlaySession
interface PlaySessionProps {
	readonly presets: ReturnType<typeof listSpatialInteractionPresets>;
	readonly interactionId: string;
	readonly spec: InteractionSpec;
	readonly onInteractionId: (id: string) => void;
	readonly documentModel: ModelDocument;
	readonly history: DocumentHistory;
	readonly kernel: BrepjsKernel;
	readonly asideExtra: ReactNode;
	readonly archivedBoxLayouts: readonly ArchivedBoxLayout[];
	readonly onArchiveCommittedBox: (layout: ArchivedBoxLayout) => void;
	readonly sessionRestartNonce: number;
}

/** @emoji 🎮 Hosts `useInteractionRuntime` + `InteractionRepl`; same-preset restarts use `sessionRestartNonce` without remounting GL. */
function PlaySession({
	presets,
	interactionId,
	spec,
	onInteractionId,
	documentModel,
	history,
	kernel,
	asideExtra,
	archivedBoxLayouts,
	onArchiveCommittedBox,
	sessionRestartNonce,
}: PlaySessionProps) {
	const rtOpts = useMemo(
		(): InteractionRuntimeOptions => ({
			kernel,
			document: documentModel,
			history,
			stateEngine: statelyStateEngineProvider,
		}),
		[kernel, documentModel, history],
	);
	const rt = useInteractionRuntime(spec, rtOpts);
	return (
		<InteractionRepl
			presets={presets}
			interactionId={interactionId}
			spec={spec}
			onInteractionId={onInteractionId}
			runtime={rt}
			history={history}
			document={documentModel}
			geometry={documentModel.topology}
			asideExtra={asideExtra}
			archivedBoxLayouts={archivedBoxLayouts}
			onArchiveCommittedBox={onArchiveCommittedBox}
			sessionRestartNonce={sessionRestartNonce}
		/>
	);
}
//#endregion

//#region 🔖PlayApp
function PlayApp() {
	const presets = useMemo(() => listSpatialInteractionPresets(), []);
	const [interactionId, setInteractionId] = useState(() => presets[0]?.id ?? "");
	const [interactionBootId, setInteractionBootId] = useState(0);
	const [geometryAssetId, setGeometryAssetId] = useState<string>(GEOMETRY_ASSETS[0]!.id);
	const spec = useMemo<InteractionSpec | null>(() => (interactionId ? loadSpatialInteractionPreset(interactionId) : null), [interactionId]);

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
		const asset = GEOMETRY_ASSETS.find((g) => g.id === geometryAssetId) ?? GEOMETRY_ASSETS[0]!;
		return parseTopologyGraphJson(asset.json) ?? new TopologyGraph();
	}, [geometryAssetId]);

	const documentModel = useMemo((): ModelDocument => {
		const topo = TopologyGraph.fromJSON(interactionTopo.toJSON());
		return { topology: topo, nodes: [] };
	}, [interactionTopo]);

	const history = useDocumentHistory();
	const kernel = useMemo(() => new BrepjsKernel(), []);
	const [archivedBoxLayouts, setArchivedBoxLayouts] = useState<ArchivedBoxLayout[]>([]);
	const appendArchivedBox = useCallback((layout: ArchivedBoxLayout) => {
		setArchivedBoxLayouts((xs) => [...xs, layout]);
	}, []);

	useEffect(() => {
		setArchivedBoxLayouts([]);
	}, [geometryAssetId, interactionId]);

	const asideExtra: ReactNode = (
		<label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12 }}>
			Geometry asset
			<select
				value={geometryAssetId}
				onChange={(e) => setGeometryAssetId(e.target.value)}
				style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
			>
				{GEOMETRY_ASSETS.map((g) => (
					<option key={g.id} value={g.id}>
						[{g.key}] {g.label}
					</option>
				))}
			</select>
		</label>
	);

	if (!presets.length) {
		return <div style={{ padding: 16, color: "#f88" }}>No spatial interaction presets registered.</div>;
	}
	if (!spec) {
		return (
			<div style={{ padding: 16, color: "#f88" }}>
				Unknown interaction <code>{interactionId}</code>.
				<button type="button" onClick={() => setInteractionId(presets[0]!.id)}>
					Reset
				</button>
			</div>
		);
	}

	return (
		<PlaySession
			key={interactionId}
			presets={presets}
			interactionId={interactionId}
			spec={spec}
			onInteractionId={handleInteractionPick}
			documentModel={documentModel}
			history={history}
			kernel={kernel}
			asideExtra={asideExtra}
			archivedBoxLayouts={archivedBoxLayouts}
			onArchiveCommittedBox={appendArchivedBox}
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
