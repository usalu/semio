/** @emoji 🎮 Vite entry: box factory + `BrepjsKernel` + `@spatial/js-renderer-r3f` viewport. */
import { StrictMode, useCallback, useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import {
	buildBoxFactorySpec,
	createFactoryRuntime,
	parseTopologyGraphJson,
	type FactoryRuntime,
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

const PICK_DISABLED = new Set([
	"idle",
	"ready",
	"committed",
	"cancelled",
	"first_corner_length_prompt",
]);

const POINTER_MOVE_ACTIVE = new Set([
	"first_corner",
	"first_corner_other_or_length",
	"first_corner_height",
	"diagonal_rubber",
	"cube_diagonal_rubber",
	"cube_other_corner",
	"three_point_edge",
	"three_point_width",
	"vertical_end",
	"vertical_width",
	"center_corner",
	"cube_center_corner",
]);

function PlayApp() {
	const spec = useMemo(() => buildBoxFactorySpec(), []);
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
	const [committedMesh, setCommittedMesh] = useState<MeshPreview | null>(null);
	const [lastCell, setLastCell] = useState<string | null>(null);

	useEffect(() => {
		console.log("[DEBUG] snapshot", snapshot.state, snapshot.revision, snapshot.capabilities);
	}, [snapshot]);

	const onGroundPick = useCallback(
		(_p: Vec3, event?: { readonly kind: string; readonly [key: string]: unknown }) => {
			const st = rt.getSnapshot().state;
			if (st === "first_corner_height") {
				void rt.send({ kind: "confirm" });
				return;
			}
			void rt.send(event ?? { kind: "pointer.down", point: _p, modifiers: {} });
		},
		[rt],
	);

	const onScenePointerMove = useCallback(
		(p: Vec3, event?: { readonly kind: string; readonly [key: string]: unknown }) => {
			void rt.send(event ?? { kind: "pointer.move", point: p, modifiers: {} });
		},
		[rt],
	);

	const onAcceptHeight = useCallback(() => {
		void rt.send({ kind: "confirm" });
	}, [rt]);

	const onStart = useCallback(() => {
		void rt.send({ kind: "start" });
	}, [rt]);

	const onApplyHeight = useCallback(() => {
		const v = Number(heightInput);
		if (!Number.isFinite(v) || v <= 0) return;
		void rt.send({ kind: "set.height", value: v });
	}, [rt, heightInput]);

	const onApplyFootprint = useCallback(() => {
		const L = Number(lwLen);
		const W = Number(lwWid);
		if (!Number.isFinite(L) || !Number.isFinite(W)) return;
		void rt.send({ kind: "set.footprint", value: { length: L, width: W } });
	}, [rt, lwLen, lwWid]);

	const onCommit = useCallback(async () => {
		const cell = await rt.commit();
		setLastCell(cell);
		if (cell) {
			const m = await kernel.tessellate(cell, 1e-3);
			setCommittedMesh(m);
			console.log("[DEBUG] committed cell", cell, "triangles", m.indices.length / 3);
		}
	}, [rt, kernel]);

	const sendMode = useCallback(
		(kind: string) => {
			void rt.send({ kind, modifiers: {} });
		},
		[rt],
	);

	const pickEnabled = !PICK_DISABLED.has(snapshot.state);
	const pointerMoveActive = POINTER_MOVE_ACTIVE.has(snapshot.state);

	return (
		<div style={{ display: "flex", height: "100vh", fontFamily: "system-ui", color: "#e8e8f0" }}>
			<div style={{ flex: 1, minWidth: 0 }}>
				<FactoryCanvas>
					<FactorySpatialView
						snapshot={snapshot}
						onGroundPick={onGroundPick}
						onScenePointerMove={pointerMoveActive ? onScenePointerMove : undefined}
						pickEnabled={pickEnabled}
						committedMesh={committedMesh}
						geometry={geometry}
					/>
				</FactoryCanvas>
			</div>
			<aside
				style={{
					width: 320,
					padding: 12,
					background: "#12121c",
					borderLeft: "1px solid #2a2a3a",
					display: "flex",
					flexDirection: "column",
					gap: 10,
				}}
			>
				<strong>Spatial / box factory</strong>
				<div>State: {snapshot.state}</div>
				<div>Revision: {snapshot.revision}</div>
				<div>Can commit: {String(snapshot.capabilities.canCommit)}</div>
				<div>Can undo: {String(snapshot.capabilities.canUndo)}</div>
				<button type="button" onClick={onStart}>
					Start
				</button>
				{snapshot.state === "first_corner" ? (
					<div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
						<span style={{ width: "100%", fontSize: 11, opacity: 0.8 }}>Footprint mode</span>
						<button type="button" onClick={() => sendMode("mode.point")}>
							Point
						</button>
						<button type="button" onClick={() => sendMode("mode.diagonal")}>
							Diagonal
						</button>
						<button type="button" onClick={() => sendMode("mode.threePoint")}>
							3Point
						</button>
						<button type="button" onClick={() => sendMode("mode.vertical")}>
							Vertical
						</button>
						<button type="button" onClick={() => sendMode("mode.center")}>
							Center
						</button>
					</div>
				) : null}
				{(snapshot.state === "first_corner_other_or_length" ||
					snapshot.state === "diagonal_rubber" ||
					snapshot.state === "center_corner") ? (
					<div style={{ display: "flex", flexWrap: "wrap", gap: 6 }}>
						<button type="button" onClick={() => sendMode("mode.cube")}>
							Cube
						</button>
						{snapshot.state === "first_corner_other_or_length" ? (
							<button type="button" onClick={() => sendMode("mode.length")}>
								Length
							</button>
						) : null}
					</div>
				) : null}
				<div style={{ fontSize: 12, opacity: 0.85 }}>
					Workflow: <strong>Start</strong> → pick mode (default Point) → click construction plane (XY @ Z=0). Branch
					modes match the box statechart (diagonal / 3-point / vertical / center / cube variants). Height step: teal
					wall or numeric height, then plane click or <strong>Accept height</strong>. <strong>Right-drag</strong> orbits.
					<strong> Commit</strong> in Ready.
				</div>
				<button type="button" disabled={snapshot.state !== "first_corner_height"} onClick={onAcceptHeight}>
					Accept height → Ready
				</button>
				<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
					Height
					<input value={heightInput} onChange={(e) => setHeightInput(e.target.value)} />
				</label>
				<button type="button" onClick={onApplyHeight}>
					Apply height
				</button>
				{snapshot.state === "first_corner_length_prompt" ? (
					<div style={{ display: "flex", flexDirection: "column", gap: 6 }}>
						<span style={{ fontSize: 11, opacity: 0.85 }}>Length / width (+X / +Y from origin)</span>
						<label>
							Length
							<input value={lwLen} onChange={(e) => setLwLen(e.target.value)} />
						</label>
						<label>
							Width
							<input value={lwWid} onChange={(e) => setLwWid(e.target.value)} />
						</label>
						<button type="button" onClick={onApplyFootprint}>
							Apply L×W → height
						</button>
					</div>
				) : null}
				<button type="button" disabled={!snapshot.capabilities.canCommit} onClick={() => void onCommit()}>
					Commit
				</button>
				<button type="button" disabled={!snapshot.capabilities.canUndo} onClick={() => rt.undo()}>
					Undo
				</button>
				{lastCell ? <div>Last cell: {lastCell}</div> : null}
			</aside>
		</div>
	);
}

const el = document.getElementById("root");
if (el) {
	createRoot(el).render(
		<StrictMode>
			<PlayApp />
		</StrictMode>,
	);
}
