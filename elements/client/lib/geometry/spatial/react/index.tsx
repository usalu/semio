import { Button, Input, ToolbarGroup, ToolbarItem, ToolbarZone, getLevelBgClass, useApp } from "@elements/ui";
import { OrbitControls } from "@react-three/drei";
import { Canvas } from "@react-three/fiber";
import * as React from "react";
import { act } from "react";
import { createRoot } from "react-dom/client";

import {
	TOPOLOGIC_KINDS,
	buildSpatialModel,
	listRenderablesByKind,
	parseTopologicFixtureV1,
	transformProps,
	type SpatialModel,
	type SpatialRenderable,
	type TopologicKind,
	type Topology,
} from "../js/index.ts";

//#region 🔖Kinds
export type SpatialStatus = "loading" | "ready" | "error";
export type SpatialSurfaceKindFilter = TopologicKind | "all";

export interface SpatialSurfaceSnapshot {
	readonly status: SpatialStatus;
	readonly fixtureLabel?: string;
	readonly model: SpatialModel | null;
	readonly focusedKind: SpatialSurfaceKindFilter;
	readonly selectedId: string | null;
	readonly query: string;
	readonly error: string | null;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly setFocusedKind: (kind: SpatialSurfaceKindFilter) => void;
	readonly setSelectedId: (id: string | null) => void;
	readonly setQuery: (query: string) => void;
}
//#endregion 🔖Kinds

//#region 🔖Helpers
export function kindLabel(kind: SpatialSurfaceKindFilter): string {
	if (kind === "all") return "All";
	if (kind === "cellComplex") return "CellComplex";
	return kind.charAt(0).toUpperCase() + kind.slice(1);
}

export function formatEnabledKindsLabel(kinds: Readonly<Record<TopologicKind, boolean>>): string {
	const enabled = TOPOLOGIC_KINDS.filter((kind) => kinds[kind]);
	return enabled.length === TOPOLOGIC_KINDS.length ? "all" : enabled.join(", ") || "none";
}

function matchesSurfaceQuery(node: Topology, query: string): boolean {
	const value = query.trim().toLowerCase();
	if (!value) return true;
	const haystack = [node.id, node.label, node.kind, node.entity.description]
		.filter((entry): entry is string => Boolean(entry))
		.join(" ")
		.toLowerCase();
	return haystack.includes(value);
}

export function listPanelNodes(snapshot: SpatialSurfaceSnapshot): readonly Topology[] {
	if (!snapshot.model) return [];
	const nodes = snapshot.focusedKind === "all" ? snapshot.model.nodes : snapshot.model.listByKind(snapshot.focusedKind);
	return nodes.filter((node) => snapshot.visibleKinds[node.kind] && matchesSurfaceQuery(node, snapshot.query));
}

function filterRenderableForest(renderable: SpatialRenderable, visibleKinds: Readonly<Record<TopologicKind, boolean>>): readonly SpatialRenderable[] {
	const children = renderable.children?.flatMap((child) => filterRenderableForest(child, visibleKinds)) ?? [];
	if (!visibleKinds[renderable.kind]) return children;
	return [{ ...renderable, children: children.length > 0 ? children : undefined }];
}

function listViewportRenderables(snapshot: SpatialSurfaceSnapshot): readonly SpatialRenderable[] {
	if (!snapshot.model) return [];
	if (snapshot.focusedKind === "all") {
		return snapshot.model.rootNodes().flatMap((node) => filterRenderableForest(node.toRenderable(snapshot.model!), snapshot.visibleKinds));
	}
	if (!snapshot.visibleKinds[snapshot.focusedKind]) return [];
	return listRenderablesByKind(snapshot.model, snapshot.focusedKind);
}

function selectedColor(color: string | undefined, selected: boolean, fallback: string): string {
	if (selected) return "#fb7185";
	return color ?? fallback;
}

function statusToneClass(status: SpatialStatus): string {
	if (status === "error") return "border-rose-500/30 bg-rose-500/10 text-rose-200";
	if (status === "ready") return "border-emerald-500/30 bg-emerald-500/10 text-emerald-200";
	return "border-border bg-muted text-muted-foreground";
}

function snapshotSelection(snapshot: SpatialSurfaceSnapshot): Topology | null {
	return snapshot.selectedId && snapshot.model ? snapshot.model.get(snapshot.selectedId) ?? null : null;
}
//#endregion 🔖Helpers

//#region 🔖Scene
function SpatialRenderableNode(props: {
	readonly renderable: SpatialRenderable;
	readonly selectedId: string | null;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly onSelect: (id: string | null) => void;
}): React.ReactElement {
	const { position, quaternion, scale } = transformProps(props.renderable.transform);
	const selected = props.renderable.id === props.selectedId;
	const selectable = props.selectableKinds[props.renderable.kind];
	const fill = props.renderable.fill;
	const edges = props.renderable.edges;
	const point = props.renderable.point;
	const select = selectable
		? (event: { stopPropagation(): void }) => {
			event.stopPropagation();
			props.onSelect(props.renderable.id);
		}
		: undefined;
	return (
		<group position={position} quaternion={quaternion} scale={scale}>
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
				<SpatialRenderableNode key={child.id} renderable={child} selectedId={props.selectedId} selectableKinds={props.selectableKinds} onSelect={props.onSelect} />
			))}
		</group>
	);
}

export function SpatialViewport(props: {
	readonly snapshot: SpatialSurfaceSnapshot;
	readonly onSelect: (id: string | null) => void;
}): React.ReactElement {
	const renderables = React.useMemo(() => listViewportRenderables(props.snapshot), [props.snapshot]);
	return (
		<Canvas camera={{ position: [10, 10, 12], fov: 45 }} onPointerMissed={() => props.onSelect(null)}>
			<color attach="background" args={["#020617"]} />
			<ambientLight intensity={0.75} />
			<directionalLight intensity={1.1} position={[8, 14, 10]} />
			<gridHelper args={[24, 24, "#1e293b", "#0f172a"]} position={[0, -2.25, 0]} />
			{renderables.map((renderable) => (
				<SpatialRenderableNode
					key={renderable.id}
					renderable={renderable}
					selectedId={props.snapshot.selectedId}
					selectableKinds={props.snapshot.selectableKinds}
					onSelect={props.onSelect}
				/>
			))}
			<OrbitControls makeDefault />
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
		<div className={`relative flex h-full w-full min-w-0 flex-col ${getLevelBgClass("window")}`}>
			<main className="relative flex min-w-0 flex-1 flex-col bg-background">
				<div className="flex shrink-0 gap-2 border-b border-border bg-muted/20 px-3 py-2 text-xs text-muted-foreground">
					<ToolbarZone>
						<ToolbarGroup>
							<ToolbarItem>
								<span className="font-medium text-foreground">Status:</span>
								<span className={`rounded-full border px-2 py-0.5 ${statusToneClass(props.snapshot.status)}`}>{props.snapshot.status}</span>
							</ToolbarItem>
							<ToolbarItem>
								<span className="font-medium text-foreground">Focus:</span>
								<span>{kindLabel(props.snapshot.focusedKind)}</span>
							</ToolbarItem>
							<ToolbarItem>
								<span className="font-medium text-foreground">Query:</span>
								<span>{props.snapshot.query.trim() || "none"}</span>
							</ToolbarItem>
						</ToolbarGroup>
					</ToolbarZone>
				</div>
				<div className="relative min-h-0 flex-1 overflow-hidden bg-[radial-gradient(circle_at_top,rgba(148,163,184,0.12),transparent_28%),linear-gradient(180deg,rgba(15,23,42,0.98),rgba(2,6,23,1))]">
					{props.snapshot.model && props.snapshot.status === "ready" && !props.disableViewport ? (
						<SpatialViewport snapshot={props.snapshot} onSelect={props.snapshot.setSelectedId} />
					) : (
						<div className="flex h-full items-center justify-center text-sm text-muted-foreground">
							{props.snapshot.status === "ready" ? "Viewport disabled for this surface." : "Preparing scene…"}
						</div>
					)}
				</div>
			</main>
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
	setFocusedKind: () => undefined,
	setSelectedId: () => undefined,
	setQuery: () => undefined,
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
	const snapshot = props.snapshot;
	const entities = React.useMemo(() => listPanelNodes(snapshot), [snapshot]);
	return (
		<div className="flex h-full min-h-0 flex-col gap-3 p-3" data-e2e-spatial-workbench-panel>
			<div className="grid gap-2 text-xs text-muted-foreground">
				<div className="rounded-lg border border-border bg-background px-3 py-2" data-e2e-spatial-visible-kinds>
					<span className="font-medium text-foreground">Visible</span>: {formatEnabledKindsLabel(snapshot.visibleKinds)}
				</div>
				<div className="rounded-lg border border-border bg-background px-3 py-2" data-e2e-spatial-selectable-kinds>
					<span className="font-medium text-foreground">Selectable</span>: {formatEnabledKindsLabel(snapshot.selectableKinds)}
				</div>
			</div>
			<div>
				<div className="mb-2 text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">Filter</div>
				<Input data-e2e-spatial-query placeholder="Filter ids, labels, and kinds" value={snapshot.query} onChange={(event) => snapshot.setQuery(event.target.value)} />
			</div>
			<div>
				<div className="mb-2 text-xs font-medium uppercase tracking-[0.18em] text-muted-foreground">Focus</div>
				<div className="grid grid-cols-2 gap-2">
					{(["all", ...TOPOLOGIC_KINDS] as const).map((kind) => (
						<Button className="justify-start rounded-lg" key={kind} onClick={() => snapshot.setFocusedKind(kind)} size="sm" variant={snapshot.focusedKind === kind ? "default" : "outline"}>
							{kindLabel(kind)}
						</Button>
					))}
				</div>
			</div>
			<div className="min-h-0 flex-1 overflow-auto">
				<div className="mb-3 flex items-center justify-between text-xs text-muted-foreground">
					<span>{snapshot.fixtureLabel ?? "topology"}</span>
					<span className="rounded-full border border-border bg-muted px-2 py-0.5" data-e2e-spatial-entity-count>{entities.length}</span>
				</div>
				<div className="space-y-2">
					{entities.map((entity) => (
						<button
							className={`w-full rounded-lg border px-3 py-3 text-left transition-colors ${snapshot.selectedId === entity.id ? "border-primary bg-primary/10 shadow-sm" : "border-border bg-background hover:bg-muted/60"}`}
							data-e2e-spatial-entity={entity.id}
							key={entity.id}
							onClick={() => snapshot.setSelectedId(entity.id)}
							type="button"
						>
							<div className="flex items-center justify-between gap-3">
								<div className="text-sm font-medium text-foreground">{entity.label}</div>
								<div className="text-[11px] uppercase tracking-[0.18em] text-muted-foreground">{entity.kind}</div>
							</div>
							<div className="mt-1 text-xs text-muted-foreground">{entity.id}</div>
						</button>
					))}
					{entities.length === 0 ? <p className="text-sm text-muted-foreground">No entities match the current filter.</p> : null}
				</div>
			</div>
		</div>
	);
}

export function SpatialDetailsPanel(props: { readonly snapshot: SpatialSurfaceSnapshot }): React.ReactElement {
	const snapshot = props.snapshot;
	const selected = snapshotSelection(snapshot);
	return (
		<div className="flex h-full min-h-0 flex-col gap-3 p-3" data-e2e-spatial-details-panel>
			<div className="rounded-lg border border-border bg-background px-3 py-3 text-sm">
				<div className="text-xs uppercase tracking-[0.18em] text-muted-foreground">Selection</div>
				<div className="mt-2 font-medium text-foreground" data-e2e-spatial-selection-label>{selected ? selected.label : "No entity selected"}</div>
				<div className="mt-1 text-xs text-muted-foreground" data-e2e-spatial-selection-kind>{selected?.kind ?? snapshot.focusedKind}</div>
				{selected?.entity.description ? <p className="mt-2 text-xs text-muted-foreground">{selected.entity.description}</p> : null}
			</div>
			<div className="rounded-lg border border-border bg-background px-3 py-3 text-xs text-muted-foreground">
				<div><span className="font-medium text-foreground">Status</span>: {snapshot.status}</div>
				<div className="mt-1"><span className="font-medium text-foreground">Focus</span>: {kindLabel(snapshot.focusedKind)}</div>
				<div className="mt-1"><span className="font-medium text-foreground">Query</span>: {snapshot.query.trim() || "none"}</div>
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
	const topologyJson = (await import("../../play/fixtures/topology.json")).default;

	describe("spatial react surface", () => {
		it("uses dedicated workbench and details panels for filtering and selection", async () => {
			const fixture = parseTopologicFixtureV1(topologyJson);
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
					setFocusedKind,
					setSelectedId,
					setQuery,
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
