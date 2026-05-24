import { Button, Input, getLevelBgClass, useApp } from "@elements/ui";
import { OrbitControls } from "@react-three/drei";
import { Canvas, useThree, type ThreeEvent } from "@react-three/fiber";
import * as React from "react";
import { act } from "react";
import { createRoot } from "react-dom/client";
import { Matrix4, Plane, Raycaster, Vector2, Vector3 } from "three";

import {
	TOPOLOGIC_KINDS,
	buildSpatialDetailsPanelState,
	buildSpatialModel,
	buildSpatialWorkbenchPanelState,
	listSpatialViewportRenderables,
	parseTopologicFixtureV1,
	spatialKindLabel,
	transformProps,
	type SpatialRenderable,
	type SpatialSurfaceKindFilter,
	type SpatialSurfaceSnapshot,
	type TopologicTransform,
	type TopologicKind,
} from "@elements/geometry-spatial-js";

//#region 🔖Helpers
function nextSpatialTransformPosition(transform: TopologicTransform | undefined, position: readonly [number, number, number]): TopologicTransform {
	return { ...transform, position };
}

function selectedColor(color: string | undefined, selected: boolean, fallback: string): string {
	if (selected) return "#fb7185";
	return color ?? fallback;
}

interface SpatialDragStartArgs {
	readonly id: string;
	readonly event: ThreeEvent<PointerEvent>;
	readonly group: THREE.Group | null;
}

interface SpatialDragState {
	readonly id: string;
	readonly pointerId: number;
	readonly plane: Plane;
	readonly offset: Vector3;
	readonly parentWorldInverse: Matrix4;
}

//#endregion 🔖Helpers

//#region 🔖Scene
function SpatialRenderableNode(props: {
	readonly renderable: SpatialRenderable;
	readonly draggingId: string | null;
	readonly selectedId: string | null;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly onSelect: (id: string | null) => void;
	readonly onDragStart: (args: SpatialDragStartArgs) => void;
}): React.ReactElement {
	const { position, quaternion, scale } = transformProps(props.renderable.transform);
	const groupRef = React.useRef<THREE.Group | null>(null);
	const selected = props.renderable.id === props.selectedId;
	const selectable = props.selectableKinds[props.renderable.kind];
	const fill = props.renderable.fill;
	const edges = props.renderable.edges;
	const point = props.renderable.point;
	const select = selectable
		? (event: ThreeEvent<PointerEvent>) => {
			event.stopPropagation();
			props.onSelect(props.renderable.id);
			props.onDragStart({ id: props.renderable.id, event, group: groupRef.current });
		}
		: undefined;
	return (
		<group ref={groupRef} position={position} quaternion={quaternion} scale={scale}>
			{fill ? (
				<mesh onPointerDown={select}>
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
				<lineSegments onPointerDown={select}>
					<bufferGeometry>
						<bufferAttribute attach="attributes-position" array={edges.position} itemSize={3} count={edges.position.length / 3} />
					</bufferGeometry>
					<lineBasicMaterial color={selectedColor(props.renderable.style?.edgeColor ?? props.renderable.style?.color, selected, "#e2e8f0")} />
				</lineSegments>
			) : null}
			{point ? (
				<mesh position={point.position} onPointerDown={select}>
					<sphereGeometry args={[point.radius, 20, 20]} />
					<meshStandardMaterial color={selectedColor(props.renderable.style?.color, selected, "#f8fafc")} />
				</mesh>
			) : null}
			{props.renderable.children?.map((child) => (
				<SpatialRenderableNode
					key={child.id}
					renderable={child}
					draggingId={props.draggingId}
					selectedId={props.selectedId}
					selectableKinds={props.selectableKinds}
					onSelect={props.onSelect}
					onDragStart={props.onDragStart}
				/>
			))}
		</group>
	);
}

function SpatialViewportScene(props: {
	readonly snapshot: SpatialSurfaceSnapshot;
	readonly renderables: readonly SpatialRenderable[];
	readonly onSelect: (id: string | null) => void;
}): React.ReactElement {
	const { camera, gl } = useThree();
	const [draggingId, setDraggingId] = React.useState<string | null>(null);
	const dragStateRef = React.useRef<SpatialDragState | null>(null);
	const pointerRef = React.useRef(new Vector2());
	const raycasterRef = React.useRef(new Raycaster());
	const planePointRef = React.useRef(new Vector3());
	const planeNormalRef = React.useRef(new Vector3());
	const worldOriginRef = React.useRef(new Vector3());

	const endDrag = React.useCallback((pointerId?: number) => {
		if (pointerId !== undefined && dragStateRef.current?.pointerId !== pointerId) return;
		dragStateRef.current = null;
		setDraggingId(null);
	}, []);

	const beginDrag = React.useCallback(
		(args: SpatialDragStartArgs) => {
			const { event, group, id } = args;
			if (!group) return;
			group.updateWorldMatrix(true, false);
			group.getWorldPosition(worldOriginRef.current);
			camera.getWorldDirection(planeNormalRef.current).normalize();
			const plane = new Plane().setFromNormalAndCoplanarPoint(planeNormalRef.current.clone(), worldOriginRef.current.clone());
			const hitPoint = event.ray.intersectPlane(plane, planePointRef.current.clone());
			if (!hitPoint) return;
			const parentWorldInverse = group.parent ? group.parent.matrixWorld.clone().invert() : new Matrix4();
			dragStateRef.current = {
				id,
				pointerId: event.pointerId,
				plane,
				offset: hitPoint.clone().sub(worldOriginRef.current),
				parentWorldInverse,
			};
			setDraggingId(id);
			try {
				if (event.nativeEvent.target instanceof Element) {
					event.nativeEvent.target.setPointerCapture?.(event.pointerId);
				}
			} catch {
				// Ignore pointer capture failures from synthetic or unsupported targets.
			}
		},
		[camera],
	);

	React.useEffect(() => {
		if (!draggingId) return;
		const ownerDocument = gl.domElement.ownerDocument;
		const handlePointerMove = (event: PointerEvent) => {
			const dragState = dragStateRef.current;
			if (!dragState || dragState.pointerId !== event.pointerId) return;
			const rect = gl.domElement.getBoundingClientRect();
			pointerRef.current.set(((event.clientX - rect.left) / rect.width) * 2 - 1, -((event.clientY - rect.top) / rect.height) * 2 + 1);
			raycasterRef.current.setFromCamera(pointerRef.current, camera);
			const hitPoint = raycasterRef.current.ray.intersectPlane(dragState.plane, planePointRef.current);
			if (!hitPoint) return;
			const localPoint = hitPoint.clone().sub(dragState.offset).applyMatrix4(dragState.parentWorldInverse);
			const nextPosition: readonly [number, number, number] = [localPoint.x, localPoint.y, localPoint.z];
			const currentTransform = props.snapshot.model?.get(dragState.id)?.transform;
			props.snapshot.setEntityTransform(dragState.id, nextSpatialTransformPosition(currentTransform, nextPosition));
		};
		const handlePointerEnd = (event: PointerEvent) => endDrag(event.pointerId);
		ownerDocument.addEventListener("pointermove", handlePointerMove);
		ownerDocument.addEventListener("pointerup", handlePointerEnd);
		ownerDocument.addEventListener("pointercancel", handlePointerEnd);
		return () => {
			ownerDocument.removeEventListener("pointermove", handlePointerMove);
			ownerDocument.removeEventListener("pointerup", handlePointerEnd);
			ownerDocument.removeEventListener("pointercancel", handlePointerEnd);
		};
	}, [camera, draggingId, endDrag, gl.domElement, props.snapshot]);

	return (
		<>
			<color attach="background" args={["#020617"]} />
			<ambientLight intensity={0.75} />
			<directionalLight intensity={1.1} position={[8, 14, 10]} />
			<gridHelper args={[24, 24, "#1e293b", "#0f172a"]} position={[0, -2.25, 0]} />
			{props.renderables.map((renderable) => (
				<SpatialRenderableNode
					key={renderable.id}
					renderable={renderable}
					draggingId={draggingId}
					selectedId={props.snapshot.selectedId}
					selectableKinds={props.snapshot.selectableKinds}
					onSelect={props.onSelect}
					onDragStart={beginDrag}
				/>
			))}
			<OrbitControls enabled={!draggingId} makeDefault />
		</>
	);
}

export function SpatialViewport(props: {
	readonly snapshot: SpatialSurfaceSnapshot;
	readonly onSelect: (id: string | null) => void;
}): React.ReactElement {
	const renderables = React.useMemo(
		() => listSpatialViewportRenderables({ model: props.snapshot.model, focusedKind: props.snapshot.focusedKind, visibleKinds: props.snapshot.visibleKinds }),
		[props.snapshot],
	);
	return (
		<Canvas camera={{ position: [10, 10, 12], fov: 45 }} onPointerMissed={() => props.onSelect(null)}>
			<SpatialViewportScene snapshot={props.snapshot} renderables={renderables} onSelect={props.onSelect} />
		</Canvas>
	);
}
//#endregion 🔖Scene

//#region 🔖Surface
export function SpatialSurface(props: {
	readonly snapshot: SpatialSurfaceSnapshot;
	readonly disableViewport?: boolean;
}): React.ReactElement {
	return (
		<div className={`absolute inset-0 min-h-0 min-w-0 overflow-hidden ${getLevelBgClass("window")} bg-[radial-gradient(circle_at_top,rgba(148,163,184,0.12),transparent_28%),linear-gradient(180deg,rgba(15,23,42,0.98),rgba(2,6,23,1))]`}>
			{props.snapshot.model && props.snapshot.status === "ready" && !props.disableViewport ? (
				<SpatialViewport snapshot={props.snapshot} onSelect={props.snapshot.setSelectedId} />
			) : (
				<div className="flex h-full items-center justify-center text-sm text-muted-foreground">
					{props.snapshot.status === "ready" ? "Viewport disabled for this surface." : "Preparing scene…"}
				</div>
			)}
		</div>
	);
}

const EMPTY_KINDS = Object.fromEntries(TOPOLOGIC_KINDS.map((kind) => [kind, true])) as Record<TopologicKind, boolean>;

const EMPTY_SNAPSHOT: SpatialSurfaceSnapshot = {
	status: "loading",
	fixtureLabel: undefined,
	model: null,
	focusedKind: "all",
	selectedId: null,
	query: "",
	error: null,
	selectableKinds: EMPTY_KINDS,
	visibleKinds: EMPTY_KINDS,
	setSelectedId: () => undefined,
	setEntityTransform: () => undefined,
	workbenchPanel: buildSpatialWorkbenchPanelState({
		fixtureLabel: undefined,
		model: null,
		focusedKind: "all",
		selectedId: null,
		query: "",
		selectableKinds: EMPTY_KINDS,
		visibleKinds: EMPTY_KINDS,
		setFocusedKind: () => undefined,
		setSelectedId: () => undefined,
		setQuery: () => undefined,
	}),
	detailsPanel: buildSpatialDetailsPanelState({
		status: "loading",
		model: null,
		focusedKind: "all",
		selectedId: null,
		query: "",
	}),
};

export const SpatialPlayWindowBody: React.FC = () => {
	const { workbench } = useApp();
	const generation = React.useSyncExternalStore(
		(onStoreChange) => workbench.subscribe(onStoreChange),
		() => workbench.generation,
		() => 0,
	);
	void generation;
	const app = workbench.getActiveApp();
	const controller = app?.controller as { getSnapshot(): SpatialSurfaceSnapshot } | undefined;
	return <SpatialSurface snapshot={controller?.getSnapshot() ?? EMPTY_SNAPSHOT} />;
};

export function SpatialWorkbenchPanel(props: { readonly snapshot: SpatialSurfaceSnapshot }): React.ReactElement {
	const panel = props.snapshot.workbenchPanel;
	return (
		<div className="flex h-full min-h-0 flex-col gap-3 p-3" data-e2e-spatial-workbench-panel>
			<div className="grid gap-2 text-xs text-muted-foreground">
				<div className="rounded-lg border border-border bg-background px-3 py-2" data-e2e-spatial-visible-kinds>
					<span className="font-medium text-foreground">Visible</span>: {panel.visibleKindsLabel}
				</div>
				<div className="rounded-lg border border-border bg-background px-3 py-2" data-e2e-spatial-selectable-kinds>
					<span className="font-medium text-foreground">Selectable</span>: {panel.selectableKindsLabel}
				</div>
			</div>
			<div>
				<div className="mb-2 text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">Filter</div>
				<Input data-e2e-spatial-query placeholder="Filter ids, labels, and kinds" value={panel.query} onChange={(event) => panel.setQuery(event.target.value)} />
			</div>
			<div>
				<div className="mb-2 text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">Focus</div>
				<div className="grid grid-cols-2 gap-2">
					{panel.focusOptions.map((option) => (
						<Button className="justify-start rounded-lg" key={option.kind} onClick={() => panel.setFocusedKind(option.kind)} size="sm" variant={option.active ? "default" : "outline"}>
							{option.label}
						</Button>
					))}
				</div>
			</div>
			<div className="min-h-0 flex-1 overflow-auto">
				<div className="mb-3 flex items-center justify-between text-xs text-muted-foreground">
					<span>{panel.fixtureLabel ?? "topology"}</span>
					<span className="rounded-full border border-border bg-muted px-2 py-0.5" data-e2e-spatial-entity-count>{panel.entityCount}</span>
				</div>
				<div className="space-y-2">
					{panel.entities.map((entity) => (
						<button
							className={`w-full rounded-lg border px-3 py-3 text-left transition-colors ${entity.selected ? "border-primary bg-primary/10 shadow-sm" : "border-border bg-background hover:bg-muted/60"}`}
							data-e2e-spatial-entity={entity.id}
							key={entity.id}
							onClick={() => panel.setSelectedId(entity.id)}
							type="button"
						>
							<div className="flex items-center justify-between gap-3">
								<div className="text-sm font-medium text-foreground">{entity.label}</div>
								<div className="text-[11px] uppercase tracking-[0.18em] text-muted-foreground">{entity.kindLabel}</div>
							</div>
							<div className="mt-1 text-xs text-muted-foreground">{entity.id}</div>
						</button>
					))}
					{panel.entities.length === 0 ? <p className="text-sm text-muted-foreground">No entities match the current filter.</p> : null}
				</div>
			</div>
		</div>
	);
}

export function SpatialDetailsPanel(props: { readonly snapshot: SpatialSurfaceSnapshot }): React.ReactElement {
	const details = props.snapshot.detailsPanel;
	return (
		<div className="flex h-full min-h-0 flex-col gap-3 p-3" data-e2e-spatial-details-panel>
			<div className="rounded-lg border border-border bg-background px-3 py-3 text-sm">
				<div className="text-xs uppercase tracking-[0.18em] text-muted-foreground">Selection</div>
				<div className="mt-2 font-medium text-foreground" data-e2e-spatial-selection-label>{details.selectedLabel}</div>
				<div className="mt-1 text-xs text-muted-foreground" data-e2e-spatial-selection-kind>{details.selectedKindLabel}</div>
				{details.description ? <p className="mt-2 text-xs text-muted-foreground">{details.description}</p> : null}
			</div>
			<div className="rounded-lg border border-border bg-background px-3 py-3 text-xs text-muted-foreground">
				<div><span className="font-medium text-foreground">Status</span>: {details.status}</div>
				<div className="mt-1"><span className="font-medium text-foreground">Focus</span>: {details.focusedKindLabel}</div>
				<div className="mt-1"><span className="font-medium text-foreground">Query</span>: {details.query}</div>
			</div>
		</div>
	);
}

export const SpatialWorkbenchPanelBody: React.FC = () => {
	const { workbench } = useApp();
	const generation = React.useSyncExternalStore(
		(onStoreChange) => workbench.subscribe(onStoreChange),
		() => workbench.generation,
		() => 0,
	);
	void generation;
	const app = workbench.getActiveApp();
	const controller = app?.controller as { getSnapshot(): SpatialSurfaceSnapshot } | undefined;
	return <SpatialWorkbenchPanel snapshot={controller?.getSnapshot() ?? EMPTY_SNAPSHOT} />;
};

export const SpatialDetailsPanelBody: React.FC = () => {
	const { workbench } = useApp();
	const generation = React.useSyncExternalStore(
		(onStoreChange) => workbench.subscribe(onStoreChange),
		() => workbench.generation,
		() => 0,
	);
	void generation;
	const app = workbench.getActiveApp();
	const controller = app?.controller as { getSnapshot(): SpatialSurfaceSnapshot } | undefined;
	return <SpatialDetailsPanel snapshot={controller?.getSnapshot() ?? EMPTY_SNAPSHOT} />;
};
//#endregion 🔖Surface

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;
	const reactTestFixture = parseTopologicFixtureV1({
		schema: "elements.geometry.topologic.fixture/v1",
		label: "React panel harness",
		roots: ["topology-root"],
		topologies: [
			{ id: "topology-root", kind: "topology", members: ["cell-a", "cell-b"] },
			{ id: "cell-a", kind: "cell", shells: [] },
			{ id: "cell-b", kind: "cell", shells: [] },
		],
	});

	describe("spatial react surface", () => {
		it("uses dedicated workbench and details panels for filtering and selection", async () => {
			const fixture = reactTestFixture;
			expect(fixture).not.toBeNull();
			const model = buildSpatialModel(fixture!);
			const selectedCellId = model.listByKind("cell")[0]?.id ?? null;
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);
			const originalActEnvironment = (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT;
			(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
			let setFocusedKindRef: ((kind: SpatialSurfaceKindFilter) => void) | null = null;
			let setSelectedIdRef: ((id: string | null) => void) | null = null;

			function Harness(): React.ReactElement {
				const [focusedKind, setFocusedKind] = React.useState<SpatialSurfaceKindFilter>("all");
				const [selectedId, setSelectedId] = React.useState<string | null>(null);
				const [query, setQuery] = React.useState("");
				const setFocusedKindCommand = (kind: SpatialSurfaceKindFilter) => setFocusedKind(kind);
				const setSelectedIdCommand = (id: string | null) => setSelectedId(id);
				const setQueryCommand = (nextQuery: string) => setQuery(nextQuery);
				const snapshot: SpatialSurfaceSnapshot = {
					status: "ready",
					fixtureLabel: fixture?.label,
					model,
					focusedKind,
					selectedId,
					query,
					error: null,
					selectableKinds: EMPTY_KINDS,
					visibleKinds: EMPTY_KINDS,
					setSelectedId: setSelectedIdCommand,
					setEntityTransform: () => undefined,
					workbenchPanel: buildSpatialWorkbenchPanelState({
						fixtureLabel: fixture?.label,
						model,
						focusedKind,
						selectedId,
						query,
						selectableKinds: EMPTY_KINDS,
						visibleKinds: EMPTY_KINDS,
						setFocusedKind: setFocusedKindCommand,
						setSelectedId: setSelectedIdCommand,
						setQuery: setQueryCommand,
					}),
					detailsPanel: buildSpatialDetailsPanelState({
						status: "ready",
						model,
						focusedKind,
						selectedId,
						query,
					}),
				};
				setFocusedKindRef = setFocusedKind;
				setSelectedIdRef = setSelectedId;
				return (
					<div>
						<SpatialSurface disableViewport snapshot={snapshot} />
						<SpatialWorkbenchPanel snapshot={snapshot} />
						<SpatialDetailsPanel snapshot={snapshot} />
					</div>
				);
			}

			try {
				await act(async () => {
					root.render(<Harness />);
					await Promise.resolve();
				});
				expect(container.textContent).toContain("Viewport disabled for this surface.");
				const countBefore = Number(container.querySelector("[data-e2e-spatial-entity-count]")?.textContent ?? "0");
				await act(async () => {
					setFocusedKindRef?.("cell");
					await Promise.resolve();
				});
				const countAfter = Number(container.querySelector("[data-e2e-spatial-entity-count]")?.textContent ?? "0");
				expect(countAfter).toBeLessThan(countBefore);
				expect(selectedCellId).not.toBeNull();
				await act(async () => {
					setSelectedIdRef?.(selectedCellId);
					await Promise.resolve();
				});
				expect(container.querySelector("[data-e2e-spatial-selection-label]")?.textContent).not.toBe("No entity selected");
			} finally {
				(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = originalActEnvironment;
				await act(async () => {
					root.unmount();
				});
				container.remove();
			}
		});
	});
}
//#endregion 🧪Tests
