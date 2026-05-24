/** @emoji 🎮 Vite entry: single command line + geometry catalog + `BrepjsKernel` + `@spatial/js-renderer-r3f`. */
import { StrictMode, useCallback, useEffect, useMemo, useRef, useState, type ReactNode } from "react";
import { createRoot } from "react-dom/client";
import {
	createCommandRuntime,
	isEmptyTopologyDiff,
	listKeyedCommandTransitions,
	listSpatialCommandPresets,
	type SpatialCommandPreset,
	loadSpatialCommandPreset,
	parseTopologyGraphJson,
	resolveSpatialCommandPresetKey,
	type CommandEvent,
	type CommandKeybindRow,
	type CommandRuntime,
	type CommandSpec,
	type ModelDocument,
	TopologyGraph,
	type TopologyEntityKind,
	type Vec3,
} from "@spatial/js-core";
import geometryNakagin from "../../../fixtures/geometry.json" with { type: "json" };
import geometryLoom from "../../../fixtures/geometry-loom.json" with { type: "json" };
import geometryRoutes from "../../../fixtures/geometry-routes.json" with { type: "json" };
import geometrySmallBuilding from "../../../fixtures/small-building.topology.json" with { type: "json" };
import geometryTallBuilding from "../../../fixtures/tall-building.topology.json" with { type: "json" };
import geometryLargeBuilding from "../../../fixtures/large-building.topology.json" with { type: "json" };
import { BrepjsKernel } from "@spatial/js-kernel-brepjs";
import { statelyStateEngineProvider } from "@spatial/js-machine-stately";
import { CommandCanvas, CommandSpatialView, useCommandSnapshot } from "../index.tsx";

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

//#region 🔖CommandParsing
type PlaySuggestKind = "preset" | "transition" | "host";

interface PlaySuggestion {
	readonly kind: PlaySuggestKind;
	readonly key: string;
	readonly label: string;
	readonly detail: string;
	readonly transition?: CommandKeybindRow;
	readonly commandId?: string;
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
	row: CommandKeybindRow,
	opts: {
		readonly commandId: string;
		readonly topo: TopologyGraph;
	},
): CommandEvent | null {
	const { commandId, topo } = opts;
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
		if (commandId === "feature.extrudeWire") {
			const wid = firstWireId(topo);
			if (!wid) return null;
			return { kind: "selection.changed", targets: [{ kind: "wire", id: wid, editable: true }], modifiers: {} };
		}
		if (commandId === "feature.offsetSurface") {
			const fid = firstFaceId(topo);
			if (!fid) return null;
			return { kind: "selection.changed", targets: [{ kind: "face", id: fid, editable: true }], modifiers: {} };
		}
		return null;
	}
	return { kind: row.eventKind, modifiers: {} };
}

function tryParseValueCommand(line: string, spec: CommandSpec, state: string): CommandEvent | null {
	const t = line.trim();
	const m = t.match(/^(\S+)\s+(.+)$/);
	if (!m) return null;
	const head = m[1]!.toLowerCase();
	const tail = m[2]!.trim();
	const rows = listKeyedCommandTransitions(spec, state);
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

function presetSuggestionsFrom(all: readonly PlaySuggestion[]): PlaySuggestion[] {
	return all.filter((s) => s.kind === "preset");
}

/** @emoji 🧭 Palette rows: command presets stay visible; filter narrows the rest without hiding presets. */
function paletteRows(cmdLine: string, all: readonly PlaySuggestion[]): PlaySuggestion[] {
	const fac = presetSuggestionsFrom(all);
	const hit = filterSuggestions(cmdLine, all);
	if (!cmdLine.trim()) return hit;
	const rest = hit.filter((s) => s.kind !== "preset");
	const seen = new Set<string>();
	const out: PlaySuggestion[] = [];
	for (const s of [...fac, ...rest]) {
		const k = `${s.kind}:${s.key}:${s.detail}`;
		if (seen.has(k)) continue;
		seen.add(k);
		out.push(s);
	}
	return out;
}

function isTextTypingTarget(t: EventTarget | null): boolean {
	if (!t || !(t instanceof HTMLElement)) return false;
	if (t.isContentEditable) return true;
	const tag = t.tagName;
	if (tag === "TEXTAREA" || tag === "SELECT") return true;
	if (tag !== "INPUT") return false;
	const ty = (t as HTMLInputElement).type;
	return !["button", "checkbox", "radio", "range", "reset", "submit"].includes(ty);
}

/** @emoji 🔤 One underlined activation key glued to the human label (palette row). */
function presentationWithUnderlinedKey(key: string, label: string): ReactNode {
	return (
		<>
			<span style={{ textDecoration: "underline", fontWeight: 700 }}>{key}</span>
			{label}
		</>
	);
}

function commandPresetFromShortcutKey(evKey: string, presets: readonly SpatialCommandPreset[]): SpatialCommandPreset | null {
	if (evKey.length !== 1) return null;
	const k = evKey.toLowerCase();
	for (const p of presets) {
		if (p.key.toLowerCase() === k) return p;
	}
	return null;
}
//#endregion

//#region 🔖PlaySession
interface PlaySessionProps {
	readonly presets: ReturnType<typeof listSpatialCommandPresets>;
	readonly commandId: string;
	readonly spec: CommandSpec;
	readonly onCommandId: (id: string) => void;
	readonly documentModel: ModelDocument;
	readonly geometryAssetId: string;
	readonly onGeometryAssetId: (id: string) => void;
}

function PlaySession({
	presets,
	commandId,
	spec,
	onCommandId,
	documentModel,
	geometryAssetId,
	onGeometryAssetId,
}: PlaySessionProps) {
	const kernel = useMemo(() => new BrepjsKernel(), []);
	const rt = useMemo<CommandRuntime>(
		() =>
			createCommandRuntime(spec, {
				kernel,
				document: documentModel,
				stateEngine: statelyStateEngineProvider,
			}),
		[spec, kernel, documentModel],
	);
	const snapshot = useCommandSnapshot(rt);
	const [lastCommitLine, setLastCommitLine] = useState<string | null>(null);
	const [cmdLine, setCmdLine] = useState("");
	const [suggestOpen, setSuggestOpen] = useState(true);
	const [activeIndex, setActiveIndex] = useState(0);
	const cmdRef = useRef<HTMLInputElement>(null);
	const setCmdLineRef = useRef(setCmdLine);
	useEffect(() => {
		setCmdLineRef.current = setCmdLine;
	}, [setCmdLine]);

	useEffect(() => {
		const snap = rt.getSnapshot();
		const initial = spec.machine.initial;
		if (snap.state !== initial) return;
		const onMap = spec.machine.states[snap.state]?.on;
		if (!onMap || !Object.prototype.hasOwnProperty.call(onMap, "start")) return;
		void rt.send({ kind: "start", modifiers: {} });
	}, [rt, spec]);

	useEffect(() => {
		console.log("[DEBUG] snapshot", snapshot.state, snapshot.revision, snapshot.capabilities);
	}, [snapshot]);

	const onSpatialCommandEvent = useCallback(
		(ev: CommandEvent) => {
			if (ev.kind === "pointer.down") {
				const st = rt.getSnapshot().state;
				const hi = rt.getSnapshot().spatialInteraction.heightConfirmState;
				const snap = (ev as { snap?: { kind: string; id: string } }).snap;
				if (hi && st === hi && !snap) {
					void rt.send({ kind: "confirm", modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {} });
					return;
				}
				const accept = rt.listActiveSelectionAccept() as readonly TopologyEntityKind[];
				if (snap && accept.length > 0 && accept.includes(snap.kind as TopologyEntityKind)) {
					void rt.send({
						kind: "selection.changed",
						targets: [{ kind: snap.kind as TopologyEntityKind, id: snap.id, editable: true }],
						modifiers: (ev as { modifiers?: Record<string, unknown> }).modifiers ?? {},
					});
					return;
				}
			}
			if (ev.kind === "pointer.down" || ev.kind === "pointer.move") {
				void rt.send(ev);
			}
		},
		[rt],
	);

	const onCommit = useCallback(async () => {
		const res = await rt.commit();
		if (res.ok && res.data != null) {
			setLastCommitLine(`data: ${JSON.stringify(res.data)}`);
			console.log("[DEBUG] commit response data", res.data);
		} else if (!isEmptyTopologyDiff(res.diff)) {
			const fc = Object.keys(res.diff.faces?.added ?? {}).length;
			setLastCommitLine(`diff (e.g. faces.added count=${fc})`);
			console.log("[DEBUG] commit topology diff", res.diff);
		} else if (!res.ok) {
			setLastCommitLine(res.errors.map((e) => e.message).join("; ") || "commit failed");
		} else {
			setLastCommitLine("ok (empty diff, no data)");
			console.log("[DEBUG] commit ok empty", res);
		}
	}, [rt]);

	const dispatchTransition = useCallback(
		(row: CommandKeybindRow) => {
			const ev = buildDispatchEvent(row, { commandId: spec.id, topo: documentModel.topology });
			if (ev) void rt.send(ev);
		},
		[rt, spec.id, documentModel.topology],
	);

	const allSuggestions = useMemo((): PlaySuggestion[] => {
		const st = snapshot.state;
		const rows = listKeyedCommandTransitions(spec, st);
		const out: PlaySuggestion[] = [];
		for (const p of presets) {
			out.push({
				kind: "preset",
				key: p.key,
				label: p.label,
				detail: p.id,
				commandId: p.id,
				onRun: () => onCommandId(p.id),
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
	}, [presets, spec, snapshot.state, onCommandId, dispatchTransition, onCommit, rt]);

	const filtered = useMemo(() => paletteRows(cmdLine, allSuggestions), [cmdLine, allSuggestions]);

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
		const presetHit = resolveSpatialCommandPresetKey(raw);
		if (presetHit) {
			onCommandId(presetHit.id);
			setCmdLine("");
			return true;
		}
		const rows = listKeyedCommandTransitions(spec, rt.getSnapshot().state);
		for (const row of rows) {
			if (row.eventKind === "set.height" || row.eventKind === "set.distance" || row.eventKind === "set.footprint") {
				continue;
			}
			if (row.key === raw || row.key.toLowerCase() === raw.toLowerCase() || row.eventKind.toLowerCase() === raw.toLowerCase()) {
				dispatchTransition(row);
				setCmdLine("");
				return true;
			}
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
	}, [cmdLine, spec, rt, dispatchTransition, onCommandId, onCommit]);

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
		[filtered, activeIndex, runSuggestion, trySubmitLine],
	);

	useEffect(() => {
		const onWinCapture = (e: KeyboardEvent) => {
			if (e.defaultPrevented || e.isComposing) return;
			if (e.ctrlKey || e.metaKey || e.altKey) return;
			const t = e.target;
			const one = e.key.length === 1 ? e.key : "";
			if (one) {
				const fac = commandPresetFromShortcutKey(one, presets);
				if (fac) {
					if (isTextTypingTarget(t) && t !== cmdRef.current) return;
					e.preventDefault();
					e.stopPropagation();
					onCommandId(fac.id);
					setCmdLineRef.current("");
					return;
				}
			}
			if (isTextTypingTarget(t) && t !== cmdRef.current) return;
			if (e.key === "m" || e.key === "M") {
				e.preventDefault();
				e.stopPropagation();
				void onCommit();
				return;
			}
			if (e.key === "r" || e.key === "R") {
				e.preventDefault();
				e.stopPropagation();
				rt.undo();
			}
		};
		window.addEventListener("keydown", onWinCapture, true);
		return () => window.removeEventListener("keydown", onWinCapture, true);
	}, [rt, onCommit, presets, onCommandId]);

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

	const kindLabel = (k: PlaySuggestKind) => (k === "preset" ? "Preset" : k === "transition" ? "Transition" : "Host");

	return (
		<div style={{ display: "flex", height: "100vh", fontFamily: "system-ui", color: "#e8e8f0" }}>
			<div style={{ flex: 1, minWidth: 0 }} key={commandId}>
				<CommandCanvas>
					<CommandSpatialView
						snapshot={snapshot}
						onCommandEvent={onSpatialCommandEvent}
						onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
						pickEnabled={pickPlaneOn}
						geometry={documentModel.topology}
					/>
				</CommandCanvas>
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
					position: "relative",
					zIndex: 2,
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
					Command <code>{commandId}</code> · state <code>{snapshot.state}</code> · rev {snapshot.revision}
				</div>
				<div style={{ fontSize: 12 }}>Can commit {String(snapshot.capabilities.canCommit)} · undo {String(snapshot.capabilities.canUndo)}</div>
				<div style={{ position: "relative" }}>
					<label style={{ display: "flex", flexDirection: "column", gap: 6, fontSize: 12 }}>
						<span>Command (presets first in palette; Tab/Enter run highlighted)</span>
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
							onPointerDown={(e) => e.stopPropagation()}
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
								zIndex: 10050,
								boxShadow: "0 8px 24px rgba(0,0,0,0.45)",
							}}
						>
							{filtered.map((s, idx) => (
								<button
									key={`${s.kind}-${s.key}-${s.detail}-${idx}`}
									type="button"
									onPointerDown={(e) => {
										e.preventDefault();
										e.stopPropagation();
										runSuggestion(s);
									}}
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
									{presentationWithUnderlinedKey(s.key, s.label)}
									<span style={{ opacity: 0.55, marginLeft: 6 }}>{s.detail}</span>
								</button>
							))}
						</div>
					) : null}
				</div>
				<div style={{ fontSize: 11, opacity: 0.75, lineHeight: 1.45 }}>
					Keys <u>q</u>/<u>j</u>/<u>k</u>/<u>d</u>/<u>a</u> switch command preset from anywhere (capture phase, clears the filter). <u>m</u> commits, <u>r</u> undoes, except while typing in other text fields. Choosing the same preset again restarts its session; presets that begin in <code>idle</code> with a <code>start</code> transition auto-enter picking. Value-style commands:{" "}
					<code>h 2.5</code>, <code>n 0.4</code>, <code>w 2 1.5</code>. Geometry asset uses the dropdown above.
				</div>
				{lastCommitLine ? <div style={{ fontSize: 12 }}>Last commit: {lastCommitLine}</div> : null}
			</aside>
		</div>
	);
}
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

	useEffect(() => {
		if (!commandId && presets[0]) setCommandId(presets[0].id);
	}, [commandId, presets]);

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
		<PlaySession
			key={`${commandId}:${commandBootId}`}
			presets={presets}
			commandId={commandId}
			spec={spec}
			onCommandId={handleCommandPick}
			documentModel={documentModel}
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
