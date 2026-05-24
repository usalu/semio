/** @emoji 🎮 Vite entry: single command line + geometry catalog + `BrepjsKernel` + `@spatial/js-renderer-r3f`. */
import { StrictMode, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import {
	createFactoryRuntime,
	listKeyedFactoryTransitions,
	listSpatialFactoryPresets,
	loadSpatialFactoryPreset,
	parseTopologyGraphJson,
	resolveSpatialFactoryPresetKey,
	type FactoryEvent,
	type FactoryKeybindRow,
	type FactoryRuntime,
	type FactorySpec,
	TopologyGraph,
	type Vec3,
} from "@spatial/js-core";
import geometryNakagin from "../../../fixtures/geometry.json" with { type: "json" };
import geometryLoom from "../../../fixtures/geometry-loom.json" with { type: "json" };
import geometryRoutes from "../../../fixtures/geometry-routes.json" with { type: "json" };
import { BrepjsKernel } from "@spatial/js-kernel-brepjs";
import { FactoryCanvas, FactorySpatialView, useFactorySnapshot, type MeshPreview } from "../index.tsx";

//#region 🔖GeometryCatalog
const GEOMETRY_ASSETS = [
	{ id: "nakagin-slice", key: "a", label: "Nakagin capsule (8 verts)", json: geometryNakagin as Record<string, unknown> },
	{ id: "geometry-loom", key: "l", label: "Loom deck + pent loop + rail", json: geometryLoom as Record<string, unknown> },
	{ id: "geometry-routes", key: "r", label: "Multi-route lattice (24 verts)", json: geometryRoutes as Record<string, unknown> },
] as const;
//#endregion

//#region 🔖CommandParsing
type PlaySuggestKind = "factory" | "transition" | "host";

interface PlaySuggestion {
	readonly kind: PlaySuggestKind;
	readonly key: string;
	readonly label: string;
	readonly detail: string;
	readonly transition?: FactoryKeybindRow;
	readonly factoryId?: string;
	readonly onRun: () => void;
}

function firstWireId(topo: TopologyGraph): string | null {
	const ks = Object.keys(topo.wires);
	return ks.length ? topo.wires[ks[0]!]!.id : null;
}

function firstFaceId(topo: TopologyGraph): string | null {
	const ks = Object.keys(topo.faces);
	return ks.length ? topo.faces[ks[0]!]!.id : null;
}

function buildDispatchEvent(
	row: FactoryKeybindRow,
	opts: {
		readonly factoryId: string;
		readonly topo: TopologyGraph;
	},
): FactoryEvent | null {
	const { factoryId, topo } = opts;
	if (row.eventKind === "set.height") {
		return null;
	}
	if (row.eventKind === "set.distance") {
		return null;
	}
	if (row.eventKind === "set.footprint") {
		return null;
	}
	if (row.eventKind === "selection.changed") {
		if (factoryId === "feature.extrudeWire") {
			const wid = firstWireId(topo);
			if (!wid) return null;
			return { kind: "selection.changed", wireId: wid, modifiers: {} };
		}
		if (factoryId === "feature.offsetSurface") {
			const fid = firstFaceId(topo);
			if (!fid) return null;
			return { kind: "selection.changed", surfaceId: fid, modifiers: {} };
		}
		return { kind: "selection.changed", modifiers: {} };
	}
	return { kind: row.eventKind, modifiers: {} };
}

function tryParseValueCommand(line: string, spec: FactorySpec, state: string): FactoryEvent | null {
	const t = line.trim();
	const m = t.match(/^(\S+)\s+(.+)$/);
	if (!m) return null;
	const head = m[1]!.toLowerCase();
	const tail = m[2]!.trim();
	const rows = listKeyedFactoryTransitions(spec, state);
	for (const row of rows) {
		if (row.eventKind === "set.height") {
			if (head !== row.key.toLowerCase() && head !== "height") continue;
			const v = Number(tail);
			if (!Number.isFinite(v) || v <= 0) return null;
			return { kind: "set.height", value: v, modifiers: {} };
		}
		if (row.eventKind === "set.distance") {
			if (head !== row.key.toLowerCase() && head !== "dist" && head !== "distance") continue;
			const v = Number(tail);
			if (!Number.isFinite(v)) return null;
			return { kind: "set.distance", value: v, modifiers: {} };
		}
		if (row.eventKind === "set.footprint") {
			if (head !== row.key.toLowerCase() && head !== "footprint" && head !== "lw") continue;
			const parts = tail.split(/\s+/);
			const L = Number(parts[0]);
			const W = Number(parts[1]);
			if (!Number.isFinite(L) || !Number.isFinite(W)) return null;
			return { kind: "set.footprint", value: { length: L, width: W }, modifiers: {} };
		}
	}
	return null;
}

function suggestionHaystack(s: PlaySuggestion): string {
	return `${s.key} ${s.label} ${s.detail}`.toLowerCase();
}

function filterSuggestions(query: string, all: readonly PlaySuggestion[]): PlaySuggestion[] {
	const q = query.trim().toLowerCase();
	if (!q) return [...all];
	return all.filter((s) => suggestionHaystack(s).includes(q));
}
//#endregion

//#region 🔖PlaySession
interface PlaySessionProps {
	readonly presets: ReturnType<typeof listSpatialFactoryPresets>;
	readonly factoryId: string;
	readonly spec: FactorySpec;
	readonly onFactoryId: (id: string) => void;
	readonly interactionTopo: TopologyGraph;
	readonly geometryAssetId: string;
	readonly onGeometryAssetId: (id: string) => void;
}

function PlaySession({
	presets,
	factoryId,
	spec,
	onFactoryId,
	interactionTopo,
	geometryAssetId,
	onGeometryAssetId,
}: PlaySessionProps) {
	const kernel = useMemo(() => new BrepjsKernel(), []);
	const documentModel = useMemo(() => ({ topology: new TopologyGraph(), nodes: [] }), []);
	const rt = useMemo<FactoryRuntime>(
		() => createFactoryRuntime(spec, { kernel, document: documentModel }),
		[spec, kernel, documentModel],
	);
	const snapshot = useFactorySnapshot(rt);
	const [committedMesh, setCommittedMesh] = useState<MeshPreview | null>(null);
	const [lastCell, setLastCell] = useState<string | null>(null);
	const [cmdLine, setCmdLine] = useState("");
	const [suggestOpen, setSuggestOpen] = useState(true);
	const [activeIndex, setActiveIndex] = useState(0);
	const cmdRef = useRef<HTMLInputElement>(null);

	useEffect(() => {
		console.log("[DEBUG] snapshot", snapshot.state, snapshot.revision, snapshot.capabilities);
	}, [snapshot]);

	const onCommit = useCallback(async () => {
		const cell = await rt.commit();
		setLastCell(cell);
		if (cell) {
			const m = await kernel.tessellate(cell, 1e-3);
			setCommittedMesh(m);
			console.log("[DEBUG] committed cell", cell, "triangles", m.indices.length / 3);
		}
	}, [rt, kernel]);

	const dispatchTransition = useCallback(
		(row: FactoryKeybindRow) => {
			const ev = buildDispatchEvent(row, { factoryId: spec.id, topo: interactionTopo });
			if (ev) void rt.send(ev);
		},
		[rt, spec.id, interactionTopo],
	);

	const allSuggestions = useMemo((): PlaySuggestion[] => {
		const st = snapshot.state;
		const rows = listKeyedFactoryTransitions(spec, st);
		const out: PlaySuggestion[] = [];
		for (const p of presets) {
			out.push({
				kind: "factory",
				key: p.key,
				label: p.label,
				detail: p.id,
				factoryId: p.id,
				onRun: () => onFactoryId(p.id),
			});
		}
		for (const row of rows) {
			out.push({
				kind: "transition",
				key: row.key,
				label: row.label,
				detail: row.eventKind,
				transition: row,
				onRun: () => dispatchTransition(row),
			});
		}
		out.push({
			kind: "host",
			key: "m",
			label: "Commit solid",
			detail: "host",
			onRun: () => void onCommit(),
		});
		out.push({
			kind: "host",
			key: "r",
			label: "Undo",
			detail: "host",
			onRun: () => rt.undo(),
		});
		return out;
	}, [presets, spec, snapshot.state, onFactoryId, dispatchTransition, onCommit, rt]);

	const filtered = useMemo(() => filterSuggestions(cmdLine, allSuggestions), [cmdLine, allSuggestions]);

	useEffect(() => {
		setActiveIndex((i) => (filtered.length ? Math.min(i, filtered.length - 1) : 0));
	}, [filtered.length, cmdLine]);

	const runSuggestion = useCallback(
		(s: PlaySuggestion) => {
			s.onRun();
			setCmdLine("");
			setSuggestOpen(true);
			setActiveIndex(0);
		},
		[],
	);

	const trySubmitLine = useCallback((): boolean => {
		const raw = cmdLine.trim();
		if (!raw) return false;
		const valEv = tryParseValueCommand(raw, spec, rt.getSnapshot().state);
		if (valEv) {
			void rt.send(valEv);
			setCmdLine("");
			return true;
		}
		const rows = listKeyedFactoryTransitions(spec, rt.getSnapshot().state);
		for (const row of rows) {
			if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") {
				continue;
			}
			if (row.key.toLowerCase() === raw.toLowerCase() || row.eventKind.toLowerCase() === raw.toLowerCase()) {
				dispatchTransition(row);
				setCmdLine("");
				return true;
			}
		}
		const presetHit = resolveSpatialFactoryPresetKey(raw);
		if (presetHit) {
			onFactoryId(presetHit.id);
			setCmdLine("");
			return true;
		}
		if (raw.toLowerCase() === "m") {
			void onCommit();
			setCmdLine("");
			return true;
		}
		if (raw.toLowerCase() === "r") {
			rt.undo();
			setCmdLine("");
			return true;
		}
		return false;
	}, [cmdLine, spec, rt, dispatchTransition, onFactoryId, onCommit]);

	const onInputKeyDown = useCallback(
		(e: React.KeyboardEvent<HTMLInputElement>) => {
			if (e.key === "Escape") {
				e.preventDefault();
				setCmdLine("");
				setSuggestOpen(true);
				return;
			}
			if (e.key === "ArrowDown" && filtered.length) {
				e.preventDefault();
				setSuggestOpen(true);
				setActiveIndex((i) => (i + 1) % filtered.length);
				return;
			}
			if (e.key === "ArrowUp" && filtered.length) {
				e.preventDefault();
				setSuggestOpen(true);
				setActiveIndex((i) => (i - 1 + filtered.length) % filtered.length);
				return;
			}
			if (e.key === "Tab" && filtered.length) {
				e.preventDefault();
				setSuggestOpen(true);
				runSuggestion(filtered[activeIndex]!);
				return;
			}
			if (e.key === "Enter") {
				e.preventDefault();
				if (trySubmitLine()) return;
				if (filtered.length) runSuggestion(filtered[activeIndex]!);
				return;
			}
		},
		[filtered, activeIndex, suggestOpen, runSuggestion, trySubmitLine],
	);

	useEffect(() => {
		const onWin = (e: KeyboardEvent) => {
			const t = e.target as HTMLElement | null;
			if (t && (t.tagName === "INPUT" || t.tagName === "TEXTAREA") && t !== cmdRef.current) return;
			if (e.key === "m" || e.key === "M") {
				e.preventDefault();
				void onCommit();
				return;
			}
			if (e.key === "r" || e.key === "R") {
				e.preventDefault();
				rt.undo();
			}
		};
		window.addEventListener("keydown", onWin);
		return () => window.removeEventListener("keydown", onWin);
	}, [rt, onCommit]);

	const onGroundPick = useCallback(
		(_p: Vec3, _ev: FactoryEvent) => {
			const st = rt.getSnapshot().state;
			const hi = rt.getSnapshot().spatialInteraction.heightConfirmState;
			if (hi && st === hi) {
				void rt.send({ kind: "confirm", modifiers: {} });
				return;
			}
			void rt.send({ kind: "pointer.down", point: _p, modifiers: {} });
		},
		[rt],
	);

	const onScenePointerMove = useCallback(
		(p: Vec3) => {
			void rt.send({ kind: "pointer.move", point: p, modifiers: {} });
		},
		[rt],
	);

	const pointerMoveActive = useMemo(() => {
		const si = snapshot.spatialInteraction;
		return (
			si.spatialGroundPick &&
			(si.groundPointerMoveStates.includes(snapshot.state) ||
				si.heightDragStates.includes(snapshot.state) ||
				si.verticalRodStates.includes(snapshot.state))
		);
	}, [snapshot.state, snapshot.spatialInteraction]);

	const pickPlaneOn = snapshot.spatialInteraction.spatialGroundPick
		? !snapshot.spatialInteraction.pickDisabledStates.includes(snapshot.state)
		: false;

	const kindLabel = (k: PlaySuggestKind) => (k === "factory" ? "Factory" : k === "transition" ? "Transition" : "Host");

	return (
		<div style={{ display: "flex", height: "100vh", fontFamily: "system-ui", color: "#e8e8f0" }}>
			<div style={{ flex: 1, minWidth: 0 }} key={factoryId}>
				<FactoryCanvas>
					<FactorySpatialView
						snapshot={snapshot}
						onGroundPick={onGroundPick}
						onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
						pickEnabled={pickPlaneOn}
						committedMesh={committedMesh}
						geometry={interactionTopo}
					/>
				</FactoryCanvas>
			</div>
			<aside
				style={{
					width: 360,
					padding: 12,
					background: "#12121c",
					borderLeft: "1px solid #2a2a3a",
					display: "flex",
					flexDirection: "column",
					gap: 10,
					overflow: "auto",
				}}
			>
				<strong>Spatial play</strong>
				<label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12 }}>
					Geometry asset
					<select
						value={geometryAssetId}
						onChange={(e) => onGeometryAssetId(e.target.value)}
						style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
					>
						{GEOMETRY_ASSETS.map((g) => (
							<option key={g.id} value={g.id}>
								[{g.key}] {g.label}
							</option>
						))}
					</select>
				</label>
				<div style={{ fontSize: 12, opacity: 0.85 }}>
					Factory <code>{factoryId}</code> · state <code>{snapshot.state}</code> · rev {snapshot.revision}
				</div>
				<div style={{ fontSize: 12 }}>Can commit {String(snapshot.capabilities.canCommit)} · undo {String(snapshot.capabilities.canUndo)}</div>
				<div style={{ position: "relative" }}>
					<label style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
						<span>Command (factories first in palette; Tab/Enter run highlighted)</span>
						<input
							ref={cmdRef}
							type="text"
							autoComplete="off"
							value={cmdLine}
							onChange={(e) => {
								setCmdLine(e.target.value);
								setSuggestOpen(true);
							}}
							onFocus={() => setSuggestOpen(true)}
							onBlur={() => {
								window.setTimeout(() => setSuggestOpen(false), 120);
							}}
							onKeyDown={onInputKeyDown}
							placeholder="Filter or type a command…"
							style={{
								width: "100%",
								boxSizing: "border-box",
								padding: 8,
								borderRadius: 6,
								background: "#0e0e16",
								color: "#e8e8f0",
								border: "1px solid #2a2a3a",
							}}
						/>
					</label>
					{suggestOpen && filtered.length ? (
						<div
							style={{
								position: "absolute",
								left: 0,
								right: 0,
								top: "100%",
								marginTop: 4,
								maxHeight: 280,
								overflowY: "auto",
								background: "#0c0c14",
								border: "1px solid #3a3a55",
								borderRadius: 6,
								zIndex: 20,
								boxShadow: "0 8px 24px rgba(0,0,0,0.45)",
							}}
						>
							{filtered.map((s, idx) => (
								<button
									key={`${s.kind}-${s.key}-${s.detail}-${idx}`}
									type="button"
									onMouseDown={(e) => e.preventDefault()}
									onClick={() => runSuggestion(s)}
									style={{
										display: "block",
										width: "100%",
										textAlign: "left",
										padding: "6px 8px",
										border: "none",
										borderBottom: "1px solid #1e1e2e",
										background: idx === activeIndex ? "#1f2f4a" : "transparent",
										color: "#e8e8f0",
										cursor: "pointer",
										fontSize: 12,
									}}
									onMouseEnter={() => setActiveIndex(idx)}
								>
									<span style={{ opacity: 0.65 }}>{kindLabel(s.kind)}</span>{" "}
									<span style={{ textDecoration: "underline", fontWeight: 700 }}>{s.key}</span>{" "}
									{s.label}
									<span style={{ opacity: 0.55, marginLeft: 6 }}>{s.detail}</span>
								</button>
							))}
						</div>
					) : null}
				</div>
				<div style={{ fontSize: 11, opacity: 0.75, lineHeight: 1.45 }}>
					Factories use keys <u>q</u> <u>j</u> <u>k</u> (listed first). Value-style transitions:{" "}
					<code>h 2.5</code>, <code>n 0.4</code>, <code>w 2 1.5</code>. Geometry assets use keys{" "}
					{GEOMETRY_ASSETS.map((g) => (
						<code key={g.id}>
							{g.key}
						</code>
					))}{" "}
					in the dropdown only. Global <u>m</u> commit / <u>r</u> undo when focus is outside inputs.
				</div>
				{lastCell ? <div style={{ fontSize: 12 }}>Last cell: {lastCell}</div> : null}
			</aside>
		</div>
	);
}
//#endregion

//#region 🔖PlayApp
function PlayApp() {
	const presets = useMemo(() => listSpatialFactoryPresets(), []);
	const [factoryId, setFactoryId] = useState(() => presets[0]?.id ?? "");
	const [geometryAssetId, setGeometryAssetId] = useState<string>(GEOMETRY_ASSETS[0]!.id);
	const spec = useMemo<FactorySpec | null>(() => (factoryId ? loadSpatialFactoryPreset(factoryId) : null), [factoryId]);

	const interactionTopo = useMemo(() => {
		const asset = GEOMETRY_ASSETS.find((g) => g.id === geometryAssetId) ?? GEOMETRY_ASSETS[0]!;
		return parseTopologyGraphJson(asset.json) ?? new TopologyGraph();
	}, [geometryAssetId]);

	useEffect(() => {
		if (!factoryId && presets[0]) setFactoryId(presets[0].id);
	}, [factoryId, presets]);

	if (!presets.length) {
		return <div style={{ padding: 16, color: "#f88" }}>No spatial factory presets registered.</div>;
	}
	if (!spec) {
		return (
			<div style={{ padding: 16, color: "#f88" }}>
				Unknown factory <code>{factoryId}</code>.
				<button type="button" onClick={() => setFactoryId(presets[0]!.id)}>
					Reset
				</button>
			</div>
		);
	}

	return (
		<PlaySession
			key={factoryId}
			presets={presets}
			factoryId={factoryId}
			spec={spec}
			onFactoryId={setFactoryId}
			interactionTopo={interactionTopo}
			geometryAssetId={geometryAssetId}
			onGeometryAssetId={setGeometryAssetId}
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
