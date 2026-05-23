import { OrbitControls } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import { createRoot } from "react-dom/client";
import * as React from "react";
import topologyJson from "../play/fixtures/topology.json";
import "./globals.css";
import {
	TOPOLOGIC_KINDS,
	buildSpatialModel,
	ensureSpatialKernelLoaded,
	listRenderablesByKind,
	loadTopologicFixtureV1,
	transformProps,
	type SpatialRenderable,
	type SpatialModel,
	type TopologicKind,
} from "./index.ts";

//#region 🔖State
type SpatialStatus = "idle" | "loading" | "ready" | "error";

interface SpatialSnapshot {
	readonly status: SpatialStatus;
	readonly model: SpatialModel | null;
	readonly activeKind: TopologicKind;
	readonly selectedId: string | null;
	readonly error: string | null;
}

/** @emoji 🎛️ Imperative play store for the spatial viewer and fixture lifecycle. */
export class SpatialPlayStore {
	private readonly listeners = new Set<() => void>();
	private snapshot: SpatialSnapshot = {
		status: "idle",
		model: null,
		activeKind: "cellComplex",
		selectedId: null,
		error: null,
	};

	subscribe(listener: () => void): () => void {
		this.listeners.add(listener);
		return () => this.listeners.delete(listener);
	}

	getSnapshot(): SpatialSnapshot {
		return this.snapshot;
	}

	private emit(): void {
		for (const listener of this.listeners) listener();
	}

	setKind(kind: TopologicKind): void {
		this.snapshot = { ...this.snapshot, activeKind: kind, selectedId: null };
		this.emit();
	}

	setSelectedId(selectedId: string | null): void {
		this.snapshot = { ...this.snapshot, selectedId };
		this.emit();
	}

	async load(raw: unknown): Promise<void> {
		this.snapshot = { ...this.snapshot, status: "loading", error: null };
		this.emit();
		try {
			await ensureSpatialKernelLoaded();
			const fixture = await loadTopologicFixtureV1(raw);
			if (!fixture) throw new Error("Spatial fixture failed to parse.");
			this.snapshot = { ...this.snapshot, status: "ready", model: buildSpatialModel(fixture), selectedId: null, error: null };
		} catch (error) {
			this.snapshot = {
				...this.snapshot,
				status: "error",
				error: error instanceof Error ? error.message : String(error),
			};
		}
		this.emit();
	}
}
//#endregion 🔖State

//#region 🔖Scene
function selectedColor(color: string | undefined, selected: boolean, fallback: string): string {
	if (selected) return "#fb7185";
	return color ?? fallback;
	}

function SpatialRenderableNode(props: {
	renderable: SpatialRenderable;
	selectedId: string | null;
	onSelect: (id: string | null) => void;
}): React.ReactElement {
	const { position, quaternion, scale } = transformProps(props.renderable.transform);
	const selected = props.renderable.id === props.selectedId;
	const fill = props.renderable.fill;
	const edges = props.renderable.edges;
	const point = props.renderable.point;
	return (
		<group position={position} quaternion={quaternion} scale={scale}>
			{fill ? (
				<mesh
					onPointerDown={(event) => {
						event.stopPropagation();
						props.onSelect(props.renderable.id);
					}}
				>
					<bufferGeometry>
						<bufferAttribute attach="attributes-position" array={fill.position} itemSize={3} count={fill.position.length / 3} />
						{fill.normal.length > 0 ? (
							<bufferAttribute attach="attributes-normal" array={fill.normal} itemSize={3} count={fill.normal.length / 3} />
						) : null}
						<bufferAttribute attach="index" array={fill.index} itemSize={1} count={fill.index.length} />
					</bufferGeometry>
					<meshStandardMaterial
						color={selectedColor(props.renderable.style?.color, selected, "#7dd3fc")}
						opacity={props.renderable.style?.opacity ?? 0.65}
						transparent
						polygonOffset
						polygonOffsetFactor={1}
						polygonOffsetUnits={1}
						side={2}
					/>
				</mesh>
			) : null}
			{edges ? (
				<lineSegments
					onPointerDown={(event) => {
						event.stopPropagation();
						props.onSelect(props.renderable.id);
					}}
				>
					<bufferGeometry>
						<bufferAttribute attach="attributes-position" array={edges.position} itemSize={3} count={edges.position.length / 3} />
					</bufferGeometry>
					<lineBasicMaterial color={selectedColor(props.renderable.style?.edgeColor ?? props.renderable.style?.color, selected, "#e2e8f0")} />
				</lineSegments>
			) : null}
			{point ? (
				<mesh
					position={point.position}
					onPointerDown={(event) => {
						event.stopPropagation();
						props.onSelect(props.renderable.id);
					}}
				>
					<sphereGeometry args={[point.radius, 20, 20]} />
					<meshStandardMaterial color={selectedColor(props.renderable.style?.color, selected, "#f8fafc")} />
				</mesh>
			) : null}
			{props.renderable.children?.map((child) => (
				<SpatialRenderableNode key={child.id} renderable={child} selectedId={props.selectedId} onSelect={props.onSelect} />
			))}
		</group>
	);
}

export function SpatialViewport(props: {
	readonly model: SpatialModel;
	readonly activeKind: TopologicKind;
	readonly selectedId: string | null;
	readonly onSelect: (id: string | null) => void;
}): React.ReactElement {
	const renderables = React.useMemo(() => listRenderablesByKind(props.model, props.activeKind), [props.activeKind, props.model]);
	return (
		<Canvas camera={{ position: [10, 10, 12], fov: 45 }} onPointerMissed={() => props.onSelect(null)}>
			<color attach="background" args={["#020617"]} />
			<ambientLight intensity={0.75} />
			<directionalLight intensity={1.1} position={[8, 14, 10]} />
			<gridHelper args={[24, 24, "#1e293b", "#0f172a"]} position={[0, -2.25, 0]} />
			{renderables.map((renderable) => (
				<SpatialRenderableNode key={renderable.id} renderable={renderable} selectedId={props.selectedId} onSelect={props.onSelect} />
			))}
			<OrbitControls makeDefault />
		</Canvas>
	);
}
//#endregion 🔖Scene

//#region 🔖App
export function SpatialPlayApp(props: { readonly store?: SpatialPlayStore; readonly disableViewport?: boolean }): React.ReactElement {
	const store = React.useMemo(() => props.store ?? new SpatialPlayStore(), [props.store]);
	const snapshot = React.useSyncExternalStore(
		(listener) => store.subscribe(listener),
		() => store.getSnapshot(),
		() => store.getSnapshot(),
	);

	React.useEffect(() => {
		if (snapshot.status === "idle") void store.load(topologyJson as unknown);
	}, [snapshot.status, store]);

	const entities = snapshot.model ? snapshot.model.listByKind(snapshot.activeKind) : [];
	const selected = snapshot.selectedId ? snapshot.model?.get(snapshot.selectedId) : null;

	return (
		<div className="flex h-screen w-screen overflow-hidden bg-slate-950 text-slate-100">
			<aside className="flex w-80 shrink-0 flex-col border-r border-white/10 bg-slate-950/80 backdrop-blur">
				<div className="border-b border-white/10 p-4">
					<p className="text-[11px] font-semibold uppercase tracking-[0.28em] text-cyan-300">Spatial</p>
					<h1 className="mt-2 text-2xl font-semibold text-white">Topologic via brepjs</h1>
					<p className="mt-2 text-sm text-slate-400">Fixture-compatible scene graph, clean imperative state, and a lightweight React viewer.</p>
				</div>
				<div className="grid grid-cols-3 gap-2 p-4">
					{TOPOLOGIC_KINDS.map((kind) => (
						<button
							className={`rounded-xl border px-3 py-2 text-left text-xs transition ${snapshot.activeKind === kind ? "border-cyan-400 bg-cyan-400/15 text-cyan-100" : "border-white/10 bg-white/5 text-slate-300 hover:border-white/20"}`}
							key={kind}
							onClick={() => store.setKind(kind)}
							type="button"
						>
							{kind}
						</button>
					))}
				</div>
				<div className="min-h-0 flex-1 overflow-auto px-4 pb-4">
					{snapshot.status === "loading" ? <p className="text-sm text-slate-400">Loading brep kernel…</p> : null}
					{snapshot.status === "error" ? <p className="text-sm text-rose-300">{snapshot.error}</p> : null}
					{snapshot.status === "ready" ? (
						<div className="space-y-2">
							{entities.map((entity) => (
								<button
									className={`w-full rounded-2xl border px-3 py-3 text-left transition ${snapshot.selectedId === entity.id ? "border-cyan-400 bg-cyan-400/10" : "border-white/10 bg-white/5 hover:border-white/20"}`}
									key={entity.id}
									onClick={() => store.setSelectedId(entity.id)}
									type="button"
								>
									<div className="text-sm font-medium text-white">{entity.label}</div>
									<div className="mt-1 text-xs text-slate-400">{entity.id}</div>
								</button>
							))}
						</div>
					) : null}
				</div>
				<div className="border-t border-white/10 p-4 text-sm text-slate-400">
					<div>{selected ? selected.label : "No entity selected"}</div>
					<div className="mt-1 text-xs uppercase tracking-[0.2em] text-slate-500">{selected?.kind ?? snapshot.activeKind}</div>
				</div>
			</aside>
			<main className="relative min-w-0 flex-1">
				{snapshot.model && !props.disableViewport ? (
					<SpatialViewport model={snapshot.model} activeKind={snapshot.activeKind} selectedId={snapshot.selectedId} onSelect={(id) => store.setSelectedId(id)} />
				) : (
					<div className="flex h-full items-center justify-center text-sm text-slate-400">{snapshot.status === "ready" ? "Viewport disabled for this surface." : "Preparing scene…"}</div>
				)}
			</main>
		</div>
	);
}

const rootElement = typeof document === "undefined" ? null : document.getElementById("root");
if (rootElement) createRoot(rootElement).render(<SpatialPlayApp />);
//#endregion 🔖App

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("spatial react surface", () => {
		it(
			"loads the fixture into the imperative store and renders the shell without a WebGL viewport",
			async () => {
				const container = document.createElement("div");
				document.body.appendChild(container);
				const root = createRoot(container);
				const store = new SpatialPlayStore();
				const originalActEnvironment = (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT;
				(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
				await store.load(topologyJson as unknown);
				await React.act(async () => {
					root.render(<SpatialPlayApp store={store} disableViewport />);
					await Promise.resolve();
				});
				expect(container.textContent).toContain("Topologic via brepjs");
				expect(store.getSnapshot().status).toBe("ready");
				expect(container.textContent).toContain("cellComplex");
				await React.act(async () => {
					root.unmount();
				});
				(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = originalActEnvironment;
				container.remove();
			},
			60000,
		);
	});
}
