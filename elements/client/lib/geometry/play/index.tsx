// #region 🧲Header
// 💻 elements/client/lib/geometry/play/index.tsx — Geometry play harness: Topologic all-kinds selector, single-window UI shell, and transform gumball editing for every entity kind.
// #endregion 🧲Header

import { App, LevelProvider, createDefaultLayout, getLevelBgClass, useApp, type AppConfig, type UIToolbarItem } from "@elements/ui";
import { BoxSelect, Move3d, Rotate3d, Scaling } from "lucide-react";
import { act, createContext, useContext, useEffect, useMemo, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";

import topologyJson from "./fixtures/topology.json";
import { TopologicViewport, type TopologicTransformMode } from "../react/index.tsx";
import {
	TOPOLOGIC_KINDS,
	TopologicWasmSession,
	deriveAnalyzeTopologicFixtureV1,
	ensureTopologicWasmLoaded,
	loadTopologicFixtureV1,
	topologicEntityLabel,
	updateTopologicFixtureTransform,
	type TopologicEntity,
	type TopologicFixtureV1,
	type TopologicKind,
	type TopologicTransform,
} from "../wasm/index.ts";
import "./globals.css";

//#region 🔖Ids
const GEOMETRY_PLAY_APP_ID = "elements-geometry-play";
const GEOMETRY_PLAY_WINDOW_ID = "geometry-topologic-window";
const GEOMETRY_PLAY_WINDOW_LABEL = "Topologic Playground";
const GEOMETRY_PLAY_DEFAULT_LAYOUT = createDefaultLayout([GEOMETRY_PLAY_WINDOW_ID], "row", [100], [GEOMETRY_PLAY_WINDOW_LABEL]);
const GEOMETRY_PLAY_TRANSFORM_MODES = ["translate", "rotate", "scale"] as const satisfies readonly TopologicTransformMode[];
const GEOMETRY_PLAY_MODES = ["edit", "analyze"] as const;
const GEOMETRY_PLAY_TRANSFORM_ICONS: Record<TopologicTransformMode, ReactElement> = {
	translate: <Move3d className="size-4" aria-hidden />,
	rotate: <Rotate3d className="size-4" aria-hidden />,
	scale: <Scaling className="size-4" aria-hidden />,
};
//#endregion 🔖Ids

//#region 🔖AnalyzeKinds
type GeometryPlayMode = (typeof GEOMETRY_PLAY_MODES)[number];
type AnalyzeSurfaceExposure = "external" | "internal";
type AnalyzeSurfaceStance = "horizontal" | "vertical";
type AnalyzePartOverlap = "none" | "difference" | "intersection";
type AnalyzeKind =
	| `surface.${AnalyzeSurfaceExposure}.${AnalyzeSurfaceStance}`
	| `part.${AnalyzePartOverlap}`
	| "solid";

const ANALYZE_KINDS = [
	"surface.external.horizontal",
	"surface.external.vertical",
	"surface.internal.horizontal",
	"surface.internal.vertical",
	"part.none",
	"part.difference",
	"part.intersection",
	"solid",
] as const satisfies readonly AnalyzeKind[];

const ANALYZE_SURFACE_KINDS = ANALYZE_KINDS.filter((kind) => kind.startsWith("surface.")) as readonly AnalyzeKind[];
const ANALYZE_PART_KINDS = ANALYZE_KINDS.filter((kind) => kind.startsWith("part.")) as readonly AnalyzeKind[];
const ANALYZE_KIND_SET = new Set<string>(ANALYZE_KINDS);
//#endregion 🔖AnalyzeKinds

//#region 🔖Context
interface GeometryPlayValue {
	readonly fixture: TopologicFixtureV1;
	readonly session: TopologicWasmSession;
	readonly analyzeFixture: TopologicFixtureV1;
	readonly analyzeSession: TopologicWasmSession;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly analyzeSelectableKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly analyzeVisibleKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly selectedId: string | null;
	readonly transformMode: TopologicTransformMode;
	readonly toggleSelectableKind: (kind: TopologicKind) => void;
	readonly toggleVisibleKind: (kind: TopologicKind) => void;
	readonly toggleAnalyzeSelectableKind: (kind: AnalyzeKind) => void;
	readonly toggleAnalyzeVisibleKind: (kind: AnalyzeKind) => void;
	readonly setSelectedId: (id: string | null) => void;
	readonly setTransformMode: (mode: TopologicTransformMode) => void;
	readonly onTransformCommit: (id: string, transform: TopologicTransform) => void;
}

const GeometryPlayContext = createContext<GeometryPlayValue | null>(null);

function useGeometryPlay(): GeometryPlayValue {
	const value = useContext(GeometryPlayContext);
	if (!value) throw new Error("GeometryPlayContext missing");
	return value;
}
//#endregion 🔖Context

//#region 🔖Controls
function geometryKindLabel(kind: TopologicKind): string {
	if (kind === "cellComplex") return "CellComplex";
	return kind.charAt(0).toUpperCase() + kind.slice(1);
}

function analyzeKindLabel(kind: AnalyzeKind): string {
	if (kind === "solid") return "Solid";
	if (kind.startsWith("surface.")) {
		const [, exposure, stance] = kind.split(".") as ["surface", AnalyzeSurfaceExposure, AnalyzeSurfaceStance];
		return `${exposure.charAt(0).toUpperCase() + exposure.slice(1)} ${stance.charAt(0).toUpperCase() + stance.slice(1)}`;
	}
	const [, overlap] = kind.split(".") as ["part", AnalyzePartOverlap];
	return overlap.charAt(0).toUpperCase() + overlap.slice(1);
}

function createAllKindsEnabled<TKind extends string>(order: readonly TKind[]): Record<TKind, boolean> {
	return Object.fromEntries(order.map((kind) => [kind, true])) as Record<TKind, boolean>;
}

function listEnabledKinds<TKind extends string>(order: readonly TKind[], kinds: Readonly<Record<TKind, boolean>>): TKind[] {
	return order.filter((kind) => kinds[kind]);
}

function formatEnabledKindsLabel(enabledKinds: readonly string[], totalCount: number): string {
	return enabledKinds.length === totalCount ? "all" : enabledKinds.join(",") || "none";
}

function geometryKindToolbarToggles<TKind extends string>(
	prefix: "selection" | "filter",
	order: readonly TKind[],
	labelForKind: (kind: TKind) => string,
	kinds: Record<TKind, boolean>,
	toggle: (kind: TKind) => void,
): UIToolbarItem[] {
	return order.map((kind, itemOrder) => ({
		id: `geometry.${prefix}.kind.${kind}`,
		kind: "toggle" as const,
		text: labelForKind(kind),
		onPressedChange: () => toggle(kind),
		order: itemOrder,
		pressed: kinds[kind],
	}));
}

function geometryAnalyzeToolbarToggles(
	prefix: "selection" | "filter",
	kinds: Record<AnalyzeKind, boolean>,
	toggle: (kind: AnalyzeKind) => void,
): UIToolbarItem[] {
	const items: UIToolbarItem[] = [
		...geometryKindToolbarToggles(prefix, ANALYZE_SURFACE_KINDS, analyzeKindLabel, kinds, toggle),
		{ id: `geometry.${prefix}.group.surface.separator`, kind: "separator", order: ANALYZE_SURFACE_KINDS.length },
		...geometryKindToolbarToggles(prefix, ANALYZE_PART_KINDS, analyzeKindLabel, kinds, toggle).map((item) => ({
			...item,
			order: (item.order ?? 0) + ANALYZE_SURFACE_KINDS.length + 1,
		})),
		{ id: `geometry.${prefix}.group.part.separator`, kind: "separator", order: ANALYZE_SURFACE_KINDS.length + ANALYZE_PART_KINDS.length + 1 },
		...geometryKindToolbarToggles(prefix, ["solid"], analyzeKindLabel, kinds, toggle).map((item) => ({
			...item,
			order: (item.order ?? 0) + ANALYZE_SURFACE_KINDS.length + ANALYZE_PART_KINDS.length + 2,
		})),
	];
	return items;
}

function entityAnalyzeKind(entity: TopologicEntity): AnalyzeKind | null {
	const kind = entity.metadata?.analyzeKind;
	return typeof kind === "string" && ANALYZE_KIND_SET.has(kind) ? (kind as AnalyzeKind) : null;
}

function isAnalyzeEntitySelectable(entity: TopologicEntity, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>): boolean {
	const kind = entityAnalyzeKind(entity);
	if (!kind) return false;
	return entity.metadata?.analyzeSelectable === true && selectableKinds[kind];
}

function isAnalyzeEntityVisible(entity: TopologicEntity, visibleKinds: Readonly<Record<AnalyzeKind, boolean>>): boolean {
	const kind = entityAnalyzeKind(entity);
	return kind ? visibleKinds[kind] : false;
}

function listSelectableEntities(session: TopologicWasmSession, selectableKinds: Readonly<Record<TopologicKind, boolean>>) {
	return session.fixture.topologies.filter((entity) => selectableKinds[entity.kind]);
}

function listAnalyzeSelectableEntities(session: TopologicWasmSession, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>) {
	return session.fixture.topologies.filter((entity) => isAnalyzeEntitySelectable(entity, selectableKinds));
}

function isSelectableEntity(
	session: TopologicWasmSession,
	selectableKinds: Readonly<Record<TopologicKind, boolean>>,
	id: string | null,
): boolean {
	if (!id) return false;
	const entity = session.getEntity(id);
	return Boolean(entity && selectableKinds[entity.kind]);
}

function isAnalyzeSelectableEntity(session: TopologicWasmSession, selectableKinds: Readonly<Record<AnalyzeKind, boolean>>, id: string | null): boolean {
	if (!id) return false;
	const entity = session.getEntity(id);
	return Boolean(entity && isAnalyzeEntitySelectable(entity, selectableKinds));
}

function geometryPlayModeFromApp(activeModeId: string | null): GeometryPlayMode {
	return activeModeId === "analyze" ? "analyze" : "edit";
}

function GeometryPlayWindow(): ReactElement {
	const play = useGeometryPlay();
	const { activeModeId } = useApp();
	const mode = geometryPlayModeFromApp(activeModeId);
	const activeSession = mode === "analyze" ? play.analyzeSession : play.session;
	const activeFixture = mode === "analyze" ? play.analyzeFixture : play.fixture;
	const activeSelectableEntities = mode === "analyze" ? listAnalyzeSelectableEntities(activeSession, play.analyzeSelectableKinds) : listSelectableEntities(activeSession, play.selectableKinds);
	const activeSelectedEntity = play.selectedId ? activeSession.getEntity(play.selectedId) : null;
	useEffect(() => {
		const selectedStillValid =
			mode === "analyze"
				? isAnalyzeSelectableEntity(activeSession, play.analyzeSelectableKinds, play.selectedId)
				: isSelectableEntity(activeSession, play.selectableKinds, play.selectedId);
		if (play.selectedId && !selectedStillValid) play.setSelectedId(null);
	}, [activeSession, mode, play, play.analyzeSelectableKinds, play.selectableKinds]);
	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<span className="text-muted-foreground px-1 text-xs font-semibold uppercase tracking-wide" data-e2e-geometry-mode>
					{mode}
				</span>
				<span className="text-muted-foreground px-1 text-xs font-semibold uppercase tracking-wide" data-e2e-geometry-transform-mode>
					{mode === "edit" ? play.transformMode : "locked"}
				</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection-kinds>
					{mode === "analyze"
						? formatEnabledKindsLabel(listEnabledKinds(ANALYZE_KINDS, play.analyzeSelectableKinds), ANALYZE_KINDS.length)
						: formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, play.selectableKinds), TOPOLOGIC_KINDS.length)}
				</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-visible-kinds>
					{mode === "analyze"
						? formatEnabledKindsLabel(listEnabledKinds(ANALYZE_KINDS, play.analyzeVisibleKinds), ANALYZE_KINDS.length)
						: formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, play.visibleKinds), TOPOLOGIC_KINDS.length)}
				</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection>{activeSelectedEntity ? topologicEntityLabel(activeSelectedEntity) : "—"}</span>
				<span className="text-muted-foreground px-1 text-xs">{activeSelectableEntities.length}</span>
			</div>
			<div className="relative min-h-0 flex-1">
				<TopologicViewport
					fixture={activeFixture}
					selectedId={play.selectedId}
					selectableKinds={mode === "edit" ? play.selectableKinds : undefined}
					visibleKinds={mode === "edit" ? play.visibleKinds : undefined}
					isEntitySelectable={mode === "analyze" ? (entity) => isAnalyzeEntitySelectable(entity, play.analyzeSelectableKinds) : undefined}
					isEntityVisible={mode === "analyze" ? (entity) => isAnalyzeEntityVisible(entity, play.analyzeVisibleKinds) : undefined}
					onSelect={play.setSelectedId}
					onTransformCommit={mode === "edit" ? play.onTransformCommit : undefined}
					transformMode={play.transformMode}
				/>
			</div>
		</div>
	);
}
//#endregion 🔖Controls

//#region 🔖Controller
function GeometryPlayController(): ReactElement {
	const [fixture, setFixture] = useState<TopologicFixtureV1 | null>(null);
	const [loadError, setLoadError] = useState<Error | null>(null);
	const session = useMemo(() => (fixture ? new TopologicWasmSession(fixture) : null), [fixture]);
	const analyzeFixture = useMemo(() => (fixture ? deriveAnalyzeTopologicFixtureV1(fixture) : null), [fixture]);
	const analyzeSession = useMemo(() => (analyzeFixture ? new TopologicWasmSession(analyzeFixture) : null), [analyzeFixture]);
	const [selectableKinds, setSelectableKinds] = useState<Record<TopologicKind, boolean>>(() => createAllKindsEnabled(TOPOLOGIC_KINDS));
	const [visibleKinds, setVisibleKinds] = useState<Record<TopologicKind, boolean>>(() => createAllKindsEnabled(TOPOLOGIC_KINDS));
	const [analyzeSelectableKinds, setAnalyzeSelectableKinds] = useState<Record<AnalyzeKind, boolean>>(() => createAllKindsEnabled(ANALYZE_KINDS));
	const [analyzeVisibleKinds, setAnalyzeVisibleKinds] = useState<Record<AnalyzeKind, boolean>>(() => createAllKindsEnabled(ANALYZE_KINDS));
	const [selectedId, setSelectedId] = useState<string | null>(null);
	const [transformMode, setTransformMode] = useState<TopologicTransformMode>("translate");

	useEffect(() => {
		let cancelled = false;
		void ensureTopologicWasmLoaded()
			.then(async () => {
				const parsedFixture = await loadTopologicFixtureV1(topologyJson as unknown);
				if (!parsedFixture) throw new Error("geometry topology fixture failed to parse");
				if (!cancelled) setFixture(parsedFixture);
			})
			.catch((error) => {
				if (!cancelled) setLoadError(error instanceof Error ? error : new Error(String(error)));
			});
		return () => {
			cancelled = true;
		};
	}, []);

	useEffect(() => {
		if (!session) return;
		if (!isSelectableEntity(session, selectableKinds, selectedId)) {
			setSelectedId(null);
		}
	}, [selectedId, selectableKinds, session]);

	const value = useMemo<GeometryPlayValue | null>(
		() =>
			fixture && session && analyzeFixture && analyzeSession
				? {
					fixture,
					session,
					analyzeFixture,
					analyzeSession,
					selectableKinds,
					visibleKinds,
					analyzeSelectableKinds,
					analyzeVisibleKinds,
					selectedId,
					transformMode,
					toggleSelectableKind: (kind) => setSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
					toggleVisibleKind: (kind) => setVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
					toggleAnalyzeSelectableKind: (kind) => setAnalyzeSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
					toggleAnalyzeVisibleKind: (kind) => setAnalyzeVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
					setSelectedId: (id) => {
						if (!id || isSelectableEntity(session, selectableKinds, id) || isAnalyzeSelectableEntity(analyzeSession, analyzeSelectableKinds, id)) {
							setSelectedId(id);
						}
					},
					setTransformMode,
					onTransformCommit: (id, transform) =>
						setFixture((current) => (current ? updateTopologicFixtureTransform(current, id, transform) : current)),
				}
				: null,
		[analyzeFixture, analyzeSelectableKinds, analyzeSession, analyzeVisibleKinds, fixture, selectableKinds, selectedId, session, transformMode, visibleKinds],
	);

	const selectionKindOrderBase = TOPOLOGIC_KINDS.length;
	const apps = useMemo<AppConfig[]>(
		() => [
			{
				id: GEOMETRY_PLAY_APP_ID,
				label: "Geometry play",
				options: { selectableKinds, visibleKinds, analyzeSelectableKinds, analyzeVisibleKinds, transformMode },
				windowKinds: [{ id: GEOMETRY_PLAY_WINDOW_ID, label: GEOMETRY_PLAY_WINDOW_LABEL, component: GeometryPlayWindow }],
				defaultLayout: GEOMETRY_PLAY_DEFAULT_LAYOUT,
				defaultModeId: "edit",
				modes: [
					{
						id: "edit",
						label: "Edit",
						tools: {
							selection: [
								...geometryKindToolbarToggles("selection", TOPOLOGIC_KINDS, geometryKindLabel, selectableKinds, (kind) =>
									setSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
								),
								{ id: "geometry.selection.separator.clear", kind: "separator" as const, order: selectionKindOrderBase },
								{
									id: "geometry.selection.clear",
									icon: <BoxSelect className="size-4" aria-hidden />,
									label: "Clear",
									onClick: () => setSelectedId(null),
									order: selectionKindOrderBase + 1,
								},
							],
							filter: geometryKindToolbarToggles("filter", TOPOLOGIC_KINDS, geometryKindLabel, visibleKinds, (kind) =>
								setVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
							),
							actions: GEOMETRY_PLAY_TRANSFORM_MODES.map((mode, order) => ({
								id: `geometry.transform.${mode}`,
								kind: "toggle" as const,
								icon: GEOMETRY_PLAY_TRANSFORM_ICONS[mode],
								label: mode.charAt(0).toUpperCase() + mode.slice(1),
								onPressedChange: (pressed: boolean) => {
									if (pressed) setTransformMode(mode);
								},
								order,
								pressed: transformMode === mode,
							})),
						},
					},
					{
						id: "analyze",
						label: "Analyze",
						tools: {
							selection: [
								...geometryAnalyzeToolbarToggles("selection", analyzeSelectableKinds, (kind) =>
									setAnalyzeSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
								),
								{ id: "geometry.analyze.selection.separator.clear", kind: "separator" as const, order: ANALYZE_KINDS.length + 2 },
								{
									id: "geometry.analyze.selection.clear",
									icon: <BoxSelect className="size-4" aria-hidden />,
									label: "Clear",
									onClick: () => setSelectedId(null),
									order: ANALYZE_KINDS.length + 3,
								},
							],
							filter: geometryAnalyzeToolbarToggles("filter", analyzeVisibleKinds, (kind) =>
								setAnalyzeVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
							),
						},
					},
				],
			},
		],
		[analyzeSelectableKinds, analyzeVisibleKinds, selectableKinds, transformMode, visibleKinds],
	);

	if (loadError) throw loadError;
	if (!value) {
		return <div className={`flex h-screen items-center justify-center text-sm text-muted-foreground ${getLevelBgClass("window")}`}>Loading geometry wasm…</div>;
	}

	return (
		<GeometryPlayContext.Provider value={value}>
			<App apps={apps} defaultAppId={GEOMETRY_PLAY_APP_ID} className={getLevelBgClass(0)} />
		</GeometryPlayContext.Provider>
	);
}

function GeometryPlayApp(): ReactElement {
	return (
		<LevelProvider>
			<GeometryPlayController />
		</LevelProvider>
	);
}

const rootElement = document.getElementById("root");
if (rootElement) {
	createRoot(rootElement).render(<GeometryPlayApp />);
}
//#endregion 🔖Controller

//#region 🧪Tests
if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("geometry play fixture", () => {
		it("enables selection and visibility for every kind by default", () => {
			expect(createAllKindsEnabled(TOPOLOGIC_KINDS)).toEqual({
				topology: true,
				vertex: true,
				edge: true,
				wire: true,
				face: true,
				shell: true,
				cell: true,
				cellComplex: true,
				cluster: true,
			});
			expect(createAllKindsEnabled(ANALYZE_KINDS)).toEqual({
				"surface.external.horizontal": true,
				"surface.external.vertical": true,
				"surface.internal.horizontal": true,
				"surface.internal.vertical": true,
				"part.none": true,
				"part.difference": true,
				"part.intersection": true,
				solid: true,
			});
		});

		it("renders through wasm fixture load without changing hook order", async () => {
			const container = document.createElement("div");
			document.body.appendChild(container);
			const root = createRoot(container);
			const errors: string[] = [];
			const originalError = console.error;
			const originalActEnvironment = (globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT;
			(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = true;
			console.error = (...args: unknown[]) => {
				errors.push(args.map((value) => String(value)).join(" "));
			};
			try {
				await act(async () => {
					root.render(<GeometryPlayController />);
					await Promise.resolve();
					await Promise.resolve();
				});
				expect(errors.some((entry) => entry.includes("change in the order of Hooks called by GeometryPlayController"))).toBe(false);
				expect(errors.some((entry) => entry.includes("Rendered more hooks than during the previous render"))).toBe(false);
			} finally {
				console.error = originalError;
				(globalThis as typeof globalThis & { IS_REACT_ACT_ENVIRONMENT?: boolean }).IS_REACT_ACT_ENVIRONMENT = originalActEnvironment;
				await act(async () => {
					root.unmount();
				});
				container.remove();
			}
		});

		it("ships at least one selectable entity for every topologic kind", async () => {
			const fixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1;
			const session = new TopologicWasmSession(fixture);
			for (const kind of TOPOLOGIC_KINDS) {
				expect(session.listByKind(kind).length).toBeGreaterThan(0);
			}
		});

		it("registers translate, rotate, and scale as toolbar transform tools", () => {
			expect([...GEOMETRY_PLAY_TRANSFORM_MODES]).toEqual(["translate", "rotate", "scale"]);
			expect(Object.keys(GEOMETRY_PLAY_TRANSFORM_ICONS)).toEqual(["translate", "rotate", "scale"]);
		});

		it("labels enabled kind sets for the status strip", () => {
			const all = createAllKindsEnabled(TOPOLOGIC_KINDS);
			expect(formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, all), TOPOLOGIC_KINDS.length)).toBe("all");
			expect(formatEnabledKindsLabel(["vertex", "edge"], TOPOLOGIC_KINDS.length)).toBe("vertex,edge");
			expect(formatEnabledKindsLabel([], TOPOLOGIC_KINDS.length)).toBe("none");
		});

		it("derives analyze solids, parts, and semantic surfaces from the shipped fixture", async () => {
			const fixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1;
			const analyzeFixture = deriveAnalyzeTopologicFixtureV1(fixture);
			const analyzeSession = new TopologicWasmSession(analyzeFixture);
			const selectable = analyzeFixture.topologies.filter((entity) => entity.metadata?.analyzeSelectable === true);
			expect(selectable.filter((entity) => entity.metadata?.analyzeGroup === "solid")).toHaveLength(3);
			expect(selectable.filter((entity) => entity.metadata?.analyzeKind === "part.difference")).toHaveLength(3);
			expect(selectable.filter((entity) => entity.metadata?.analyzeKind === "part.intersection")).toHaveLength(2);
			expect(selectable.filter((entity) => String(entity.metadata?.analyzeKind).startsWith("surface."))).not.toHaveLength(0);
			expect(isAnalyzeSelectableEntity(analyzeSession, createAllKindsEnabled(ANALYZE_KINDS), "analyze.part.1")).toBe(true);
			expect(isAnalyzeSelectableEntity(analyzeSession, createAllKindsEnabled(ANALYZE_KINDS), "analyze.part.1.face.1")).toBe(false);
		});
	});
}
//#endregion 🧪Tests