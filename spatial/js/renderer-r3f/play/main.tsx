/** @emoji 🎮 Vite entry: multi-factory play + `BrepjsKernel` + `@spatial/js-renderer-r3f` viewport. */
import { StrictMode, useCallback, useEffect, useMemo, useRef, useState } from "react";
import { createRoot } from "react-dom/client";
import {
	createFactoryRuntime,
	listKeyedFactoryTransitions,
	listSpatialFactoryPresets,
	loadSpatialFactoryPreset,
	parseTopologyGraphJson,
	type FactoryEvent,
	type FactoryKeybindRow,
	type FactoryRuntime,
	type FactorySpec,
	TopologyGraph,
	type Vec3,
} from "@spatial/js-core";
import geometryJson from "../../../fixtures/geometry.json" with { type: "json" };
import { BrepjsKernel } from "@spatial/js-kernel-brepjs";
import {
	FactoryCanvas,
	FactorySpatialView,
	useFactorySnapshot,
	type MeshPreview,
} from "../index.tsx";

function keyMatchesHotkey(keySpec: string, e: React.KeyboardEvent<HTMLInputElement>): boolean {
	if (keySpec === "Enter") return e.key === "Enter";
	if (keySpec.length === 1) return e.key.length === 1 && e.key.toLowerCase() === keySpec.toLowerCase();
	return e.key === keySpec;
}

function buildDispatchEvent(
	row: FactoryKeybindRow,
	opts: {
		readonly factoryId: string;
		readonly heightStr: string;
		readonly lwLen: string;
		readonly lwWid: string;
		readonly dimStr: string;
	},
): FactoryEvent | null {
	const { factoryId, heightStr, lwLen, lwWid, dimStr } = opts;
	if (row.eventKind === "set.height") {
		const v = Number(heightStr);
		if (!Number.isFinite(v) || v <= 0) return null;
		return { kind: "set.height", value: v, modifiers: {} };
	}
	if (row.eventKind === "set.distance") {
		const v = Number(dimStr);
		if (!Number.isFinite(v)) return null;
		return { kind: "set.distance", value: v, modifiers: {} };
	}
	if (row.eventKind === "set.footprint") {
		const L = Number(lwLen);
		const W = Number(lwWid);
		if (!Number.isFinite(L) || !Number.isFinite(W)) return null;
		return { kind: "set.footprint", value: { length: L, width: W }, modifiers: {} };
	}
	if (row.eventKind === "selection.changed") {
		if (factoryId === "feature.extrudeWire") {
			return { kind: "selection.changed", wireId: "stub-wire", modifiers: {} };
		}
		if (factoryId === "feature.offsetSurface") {
			return { kind: "selection.changed", surfaceId: "stub-surface", modifiers: {} };
		}
		return { kind: "selection.changed", modifiers: {} };
	}
	return { kind: row.eventKind, modifiers: {} };
}

function KeybindChip({
	row,
	onActivate,
}: {
	readonly row: FactoryKeybindRow;
	readonly onActivate: () => void;
}): React.ReactNode {
	return (
		<button
			type="button"
			onClick={() => onActivate()}
			style={{
				fontSize: 12,
				padding: "4px 8px",
				borderRadius: 6,
				border: "1px solid #3a3a55",
				background: "#1a1a28",
				color: "#e8e8f0",
				cursor: "pointer",
			}}
		>
			<span style={{ textDecoration: "underline", fontWeight: 700 }}>{row.key}</span>
			<span style={{ marginLeft: 4 }}>{row.label}</span>
		</button>
	);
}

interface PlaySessionProps {
	readonly presets: ReadonlyArray<{ readonly id: string; readonly label: string }>;
	readonly factoryId: string;
	readonly spec: FactorySpec;
	readonly onFactoryId: (id: string) => void;
}

function PlaySession({ presets, factoryId, spec, onFactoryId }: PlaySessionProps) {
	const kernel = useMemo(() => new BrepjsKernel(), []);
	const geometry = useMemo(() => parseTopologyGraphJson(geometryJson), []);
	const documentModel = useMemo(() => ({ topology: new TopologyGraph(), nodes: [] }), []);
	const rt = useMemo<FactoryRuntime>(
		() => createFactoryRuntime(spec, { kernel, document: documentModel }),
		[spec, kernel, documentModel],
	);
	const snapshot = useFactorySnapshot(rt);
	const [heightInput, setHeightInput] = useState("1.5");
	const [lwLen, setLwLen] = useState("2");
	const [lwWid, setLwWid] = useState("1.5");
	const [dimInput, setDimInput] = useState("1");
	const [committedMesh, setCommittedMesh] = useState<MeshPreview | null>(null);
	const [lastCell, setLastCell] = useState<string | null>(null);
	const cmdRef = useRef<HTMLInputElement>(null);

	useEffect(() => {
		console.log("[DEBUG] snapshot", snapshot.state, snapshot.revision, snapshot.capabilities);
	}, [snapshot]);

	const keybinds = useMemo(
		() => [...listKeyedFactoryTransitions(spec, snapshot.state)],
		[spec, snapshot.state],
	);

	const dispatchRow = useCallback(
		(row: FactoryKeybindRow) => {
			const ev = buildDispatchEvent(row, {
				factoryId: spec.id,
				heightStr: heightInput,
				lwLen,
				lwWid,
				dimStr: dimInput,
			});
			if (ev) void rt.send(ev);
		},
		[rt, spec, heightInput, lwLen, lwWid, dimInput],
	);

	const onGroundPick = useCallback(
		(_p: Vec3) => {
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

	const onCommit = useCallback(async () => {
		const cell = await rt.commit();
		setLastCell(cell);
		if (cell) {
			const m = await kernel.tessellate(cell, 1e-3);
			setCommittedMesh(m);
			console.log("[DEBUG] committed cell", cell, "triangles", m.indices.length / 3);
		}
	}, [rt, kernel]);

	const onCommandKeyDown = useCallback(
		(e: React.KeyboardEvent<HTMLInputElement>) => {
			const k = e.key;
			if (k === "Escape") {
				e.preventDefault();
				e.currentTarget.value = "";
				return;
			}
			for (const row of listKeyedFactoryTransitions(spec, rt.getSnapshot().state)) {
				if (!keyMatchesHotkey(row.key, e)) continue;
				e.preventDefault();
				dispatchRow(row);
				e.currentTarget.value = "";
				return;
			}
		},
		[rt, spec, dispatchRow],
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
						geometry={geometry}
					/>
				</FactoryCanvas>
			</div>
			<aside
				style={{
					width: 340,
					padding: 12,
					background: "#12121c",
					borderLeft: "1px solid #2a2a3a",
					display: "flex",
					flexDirection: "column",
					gap: 10,
				}}
			>
				<strong>Spatial / factory play</strong>
				<label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12 }}>
					Factory
					<select
						value={factoryId}
						onChange={(e) => {
							onFactoryId(e.target.value);
							setCommittedMesh(null);
							setLastCell(null);
						}}
						style={{ padding: 6, borderRadius: 6, background: "#1a1a28", color: "#e8e8f0" }}
					>
						{presets.map((p) => (
							<option key={p.id} value={p.id}>
								{p.label}
							</option>
						))}
					</select>
				</label>
				<div>State: {snapshot.state}</div>
				<div>Revision: {snapshot.revision}</div>
				<div>Can commit: {String(snapshot.capabilities.canCommit)}</div>
				<div>Can undo: {String(snapshot.capabilities.canUndo)}</div>
				<div style={{ display: "flex", flexWrap: "wrap", gap: 6, alignItems: "center" }}>
					{keybinds.map((row) => (
						<KeybindChip key={`${row.eventKind}-${row.key}-${row.label}`} row={row} onActivate={() => dispatchRow(row)} />
					))}
				</div>
				<label style={{ display: "flex", flexDirection: "column", gap: 4, fontSize: 12 }}>
					Command (type shortcut; Enter = literal Enter)
					<input
						ref={cmdRef}
						type="text"
						autoComplete="off"
						placeholder="Keys…"
						onKeyDown={onCommandKeyDown}
						style={{ padding: 6, borderRadius: 6, background: "#0e0e16", color: "#e8e8f0", border: "1px solid #2a2a3a" }}
					/>
				</label>
				<div style={{ fontSize: 11, opacity: 0.75 }}>
					Host: <u>m</u> Commit solid · <u>r</u> Undo (global). Spatial picking follows each factory&apos;s{" "}
					<code>interaction</code> block.
				</div>
				{spec.id === "primitive.box" ? (
					<>
						<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
							Height (set.height / <u>h</u>)
							<input value={heightInput} onChange={(e) => setHeightInput(e.target.value)} />
						</label>
						{snapshot.state === "first_corner_length_prompt" ? (
							<div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
								<label>
									Length
									<input value={lwLen} onChange={(e) => setLwLen(e.target.value)} />
								</label>
								<label>
									Width
									<input value={lwWid} onChange={(e) => setLwWid(e.target.value)} />
								</label>
							</div>
						) : null}
					</>
				) : null}
				{(spec.id === "feature.extrudeWire" || spec.id === "feature.offsetSurface") &&
				(snapshot.state === "setDistance" || snapshot.state === "selectWire" || snapshot.state === "selectSurface") ? (
					<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
						Distance / offset (<u>n</u> apply)
						<input value={dimInput} onChange={(e) => setDimInput(e.target.value)} />
					</label>
				) : null}
				<button type="button" disabled={!snapshot.capabilities.canCommit} onClick={() => void onCommit()}>
					Commit (<u>m</u>)
				</button>
				<button type="button" disabled={!snapshot.capabilities.canUndo} onClick={() => rt.undo()}>
					Undo (<u>r</u>)
				</button>
				{lastCell ? <div>Last cell: {lastCell}</div> : null}
			</aside>
		</div>
	);
}

function PlayApp() {
	const presets = useMemo(() => listSpatialFactoryPresets(), []);
	const [factoryId, setFactoryId] = useState(() => presets[0]?.id ?? "");
	const spec = useMemo<FactorySpec | null>(() => (factoryId ? loadSpatialFactoryPreset(factoryId) : null), [factoryId]);

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

	return <PlaySession key={factoryId} presets={presets} factoryId={factoryId} spec={spec} onFactoryId={setFactoryId} />;
}

const el = document.getElementById("root");
if (el) {
	createRoot(el).render(
		<StrictMode>
			<PlayApp />
		</StrictMode>,
	);
}
