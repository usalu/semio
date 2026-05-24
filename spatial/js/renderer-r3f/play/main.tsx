/** @emoji 🎮 Vite entry: geometry catalog + `BrepjsKernel` + `CommandRepl`. */
import { StrictMode, useCallback, useMemo, useState, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	listSpatialCommandPresets,
	loadSpatialCommandPreset,
	parseTopologyGraphJson,
	type CommandSpec,
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
import { CommandRepl, useCommandRuntime, useDocumentHistory } from "../index.tsx";

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

//#region 🔖PlayApp
function PlayApp() {
	const presets = useMemo(() => listSpatialCommandPresets(), []);
	const [commandId, setCommandId] = useState(() => presets[0]?.id ?? "");
	const [commandBootId, setCommandBootId] = useState(0);
	const [geometryAssetId, setGeometryAssetId] = useState<string>(GEOMETRY_ASSETS[0]!.id);
	const spec = useMemo<CommandSpec | null>(() => (commandId ? loadSpatialCommandPreset(commandId) : null), [commandId]);

	const handleCommandPick = useCallback(
		(id: string) => {
			if (id === commandId) setCommandBootId((b) => b + 1);
			else {
				setCommandId(id);
				setCommandBootId(0);
			}
		},
		[commandId],
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
	const fallbackSpec = useMemo(() => loadSpatialCommandPreset(presets[0]!.id)!, [presets]);
	const activeSpec = spec ?? fallbackSpec;

	const rtOpts = useMemo(
		() => ({
			kernel,
			document: documentModel,
			history,
			stateEngine: statelyStateEngineProvider,
		}),
		[kernel, documentModel, history],
	);

	const rt = useCommandRuntime(activeSpec, rtOpts);

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
		return <div style={{ padding: 16, color: "#f88" }}>No spatial command presets registered.</div>;
	}
	if (!spec) {
		return (
			<div style={{ padding: 16, color: "#f88" }}>
				Unknown command <code>{commandId}</code>.
				<button type="button" onClick={() => setCommandId(presets[0]!.id)}>
					Reset
				</button>
			</div>
		);
	}

	return (
		<CommandRepl
			key={`${commandId}:${commandBootId}`}
			presets={presets}
			commandId={commandId}
			spec={spec}
			onCommandId={handleCommandPick}
			runtime={rt}
			history={history}
			document={documentModel}
			geometry={documentModel.topology}
			asideExtra={asideExtra}
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
