/** @emoji 🎮 Vite entry: box factory + `BrepjsKernel` + `@spatial/js-renderer-r3f` viewport. */
import { StrictMode, useCallback, useEffect, useMemo, useState } from "react";
import { createRoot } from "react-dom/client";
import {
	buildBoxFactorySpec,
	createFactoryRuntime,
	type FactoryRuntime,
	TopologyGraph,
	type Vec3,
} from "@spatial/js-core";
import { BrepjsKernel } from "@spatial/js-kernel-brepjs";
import {
	FactoryCanvas,
	FactorySpatialView,
	useFactorySnapshot,
	type MeshPreview,
} from "../index.tsx";

function PlayApp() {
	const spec = useMemo(() => buildBoxFactorySpec(), []);
	const kernel = useMemo(() => new BrepjsKernel(), []);
	const documentModel = useMemo(() => ({ topology: new TopologyGraph(), nodes: [] }), []);
	const rt = useMemo<FactoryRuntime>(
		() => createFactoryRuntime(spec, { kernel, document: documentModel }),
		[spec, kernel, documentModel],
	);
	const snapshot = useFactorySnapshot(rt);
	const [heightInput, setHeightInput] = useState("1.5");
	const [committedMesh, setCommittedMesh] = useState<MeshPreview | null>(null);
	const [lastCell, setLastCell] = useState<string | null>(null);

	useEffect(() => {
		console.log("[DEBUG] snapshot", snapshot.state, snapshot.revision, snapshot.capabilities);
	}, [snapshot]);

	const onGroundPick = useCallback(
		(_p: Vec3) => {
			const st = rt.getSnapshot().state;
			if (st === "pickingHeight") {
				void rt.send({ kind: "confirm" });
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

	const onCommit = useCallback(async () => {
		const cell = await rt.commit();
		setLastCell(cell);
		if (cell) {
			const m = await kernel.tessellate(cell, 1e-3);
			setCommittedMesh(m);
			console.log("[DEBUG] committed cell", cell, "triangles", m.indices.length / 3);
		}
	}, [rt, kernel]);

	const picking =
		snapshot.state === "pickingFirstCorner" ||
		snapshot.state === "pickingSecondCorner" ||
		snapshot.state === "pickingHeight";
	const pickEnabled = picking;
	const pointerMoveActive =
		snapshot.state === "pickingFirstCorner" ||
		snapshot.state === "pickingSecondCorner" ||
		snapshot.state === "pickingHeight";

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
					/>
				</FactoryCanvas>
			</div>
			<aside
				style={{
					width: 300,
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
				<div style={{ fontSize: 12, opacity: 0.85 }}>
					Start → cyan dot follows the cursor on the grid; click to place the first corner; move to rubber-band second corner;
					second click fixes corner. Adjust height (teal wall at the second corner along Z, or Apply height), then click the grid
					or <strong>Accept height</strong>. Use <strong>right mouse drag</strong> to orbit if a click starts orbiting instead of
					hitting the grid. In Ready, Commit creates the solid.
				</div>
				<button type="button" disabled={snapshot.state !== "pickingHeight"} onClick={onAcceptHeight}>
					Accept height → Ready
				</button>
				<label style={{ display: "flex", flexDirection: "column", gap: 4 }}>
					Height
					<input value={heightInput} onChange={(e) => setHeightInput(e.target.value)} />
				</label>
				<button type="button" onClick={onApplyHeight}>
					Apply height
				</button>
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
