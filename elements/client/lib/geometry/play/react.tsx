// #region 🧲Header
// 💻 elements/client/lib/geometry/play/react.tsx — React runtime for geometry play: contexts, window body, controller, and React-specific tests.
// #endregion 🧲Header

import { App, LevelProvider, PureAppDefinition, getLevelBgClass, useApp, type AppConfig, type UIToolbarItem } from "@elements/ui";
import { BoxSelect, Move3d, Rotate3d, Scaling } from "lucide-react";
import { act, createContext, useContext, useEffect, useMemo, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";

import topologyJson from "./fixtures/topology.json";
import {
	ANALYZE_KINDS,
	GEOMETRY_PLAY_APP_ID,
	GEOMETRY_PLAY_DEFAULT_LAYOUT,
	GEOMETRY_PLAY_TRANSFORM_MODES,
	GEOMETRY_PLAY_WINDOW_ID,
	GEOMETRY_PLAY_WINDOW_LABEL,
	TopologicPlaySession,
	analyzeKindLabel,
	createAllKindsEnabled,
	entityAnalyzeKind,
	formatEnabledKindsLabel,
	geometryKindLabel,
	geometryPlayModeFromApp,
	isAnalyzeSelectableEntity,
	isAnalyzeEntitySelectable,
	isAnalyzeEntityVisible,
	isSelectableEntity,
	listAnalyzeSelectableEntities,
	listEnabledKinds,
	listSelectableEntities,
	setKindGroup,
	type AnalyzeKind,
} from "./index.tsx";
import { TopologicViewport, type TopologicTransformMode } from "../react/index.tsx";
import {
	TOPOLOGIC_KINDS,
	deriveAnalyzeTopologicFixtureV1,
	ensureTopologicWasmLoaded,
	loadTopologicFixtureV1,
	updateTopologicFixtureTransformKernelV1,
	type TopologicFixtureV1,
	type TopologicKind,
	type TopologicTransform,
} from "../wasm/index.ts";

interface GeometryPlayValue {
	readonly fixture: TopologicFixtureV1;
	readonly session: TopologicPlaySession;
	readonly analyzeFixture: TopologicFixtureV1;
	readonly analyzeSession: TopologicPlaySession;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly visibleKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly analyzeSelectableKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly analyzeVisibleKinds: Readonly<Record<AnalyzeKind, boolean>>;
	readonly selectedId: string | null;
	readonly transformMode: TopologicTransformMode;
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
	toggleGroup: (group: readonly AnalyzeKind[], enabled: boolean) => void,
): UIToolbarItem[] {
	const surfaceKinds = ANALYZE_KINDS.filter((kind) => kind.startsWith("surface.")) as readonly AnalyzeKind[];
	const partKinds = ANALYZE_KINDS.filter((kind) => kind.startsWith("part.")) as readonly AnalyzeKind[];
	const surfacesEnabled = surfaceKinds.every((kind) => kinds[kind]);
	const partsEnabled = partKinds.every((kind) => kinds[kind]);
	return [
		{
			id: `geometry.${prefix}.group.surface`,
			kind: "toggle",
			text: "Surfaces",
			onPressedChange: () => toggleGroup(surfaceKinds, !surfacesEnabled),
			order: 0,
			pressed: surfacesEnabled,
		},
		...geometryKindToolbarToggles(prefix, surfaceKinds, analyzeKindLabel, kinds, toggle).map((item) => ({ ...item, order: (item.order ?? 0) + 1 })),
		{ id: `geometry.${prefix}.group.surface.separator`, kind: "separator", order: surfaceKinds.length + 1 },
		{
			id: `geometry.${prefix}.group.part`,
			kind: "toggle",
			text: "Parts",
			onPressedChange: () => toggleGroup(partKinds, !partsEnabled),
			order: surfaceKinds.length + 2,
			pressed: partsEnabled,
		},
		...geometryKindToolbarToggles(prefix, partKinds, analyzeKindLabel, kinds, toggle).map((item) => ({ ...item, order: (item.order ?? 0) + surfaceKinds.length + 3 })),
		{ id: `geometry.${prefix}.group.part.separator`, kind: "separator", order: surfaceKinds.length + partKinds.length + 3 },
		...geometryKindToolbarToggles(prefix, ["solid"], analyzeKindLabel, kinds, toggle).map((item) => ({ ...item, order: (item.order ?? 0) + surfaceKinds.length + partKinds.length + 4 })),
	];
}

const GEOMETRY_PLAY_TRANSFORM_ICONS: Record<TopologicTransformMode, ReactElement> = {
	translate: <Move3d className="size-4" aria-hidden />,
	rotate: <Rotate3d className="size-4" aria-hidden />,
	scale: <Scaling className="size-4" aria-hidden />,
};

class GeometryPlayDefinition extends PureAppDefinition {
	constructor(
		private readonly analyzeSelectableKinds: Record<AnalyzeKind, boolean>,
		private readonly analyzeVisibleKinds: Record<AnalyzeKind, boolean>,
		private readonly selectableKinds: Record<TopologicKind, boolean>,
		private readonly transformMode: TopologicTransformMode,
		private readonly visibleKinds: Record<TopologicKind, boolean>,
		private readonly setAnalyzeSelectableKinds: React.Dispatch<React.SetStateAction<Record<AnalyzeKind, boolean>>>,
		private readonly setAnalyzeVisibleKinds: React.Dispatch<React.SetStateAction<Record<AnalyzeKind, boolean>>>,
		private readonly setSelectableKinds: React.Dispatch<React.SetStateAction<Record<TopologicKind, boolean>>>,
		private readonly setSelectedId: React.Dispatch<React.SetStateAction<string | null>>,
		private readonly setTransformMode: React.Dispatch<React.SetStateAction<TopologicTransformMode>>,
		private readonly setVisibleKinds: React.Dispatch<React.SetStateAction<Record<TopologicKind, boolean>>>,
	) {
		super();
	}

	resolveConfig(): AppConfig {
		const selectionKindOrderBase = TOPOLOGIC_KINDS.length;
		return {
			id: GEOMETRY_PLAY_APP_ID,
			label: "Geometry play",
			options: {
				selectableKinds: this.selectableKinds,
				visibleKinds: this.visibleKinds,
				analyzeSelectableKinds: this.analyzeSelectableKinds,
				analyzeVisibleKinds: this.analyzeVisibleKinds,
				transformMode: this.transformMode,
			},
			windowKinds: [{ id: GEOMETRY_PLAY_WINDOW_ID, label: GEOMETRY_PLAY_WINDOW_LABEL, component: GeometryPlayWindow }],
			defaultLayout: GEOMETRY_PLAY_DEFAULT_LAYOUT,
			defaultModeId: "edit",
			modes: [
				{
					id: "edit",
					label: "Edit",
					tools: {
						selection: [
							...geometryKindToolbarToggles("selection", TOPOLOGIC_KINDS, geometryKindLabel, this.selectableKinds, (kind) =>
								this.setSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
							),
							{ id: "geometry.selection.separator.clear", kind: "separator" as const, order: selectionKindOrderBase },
							{ id: "geometry.selection.clear", icon: <BoxSelect className="size-4" aria-hidden />, label: "Clear", onClick: () => this.setSelectedId(null), order: selectionKindOrderBase + 1 },
						],
						filter: geometryKindToolbarToggles("filter", TOPOLOGIC_KINDS, geometryKindLabel, this.visibleKinds, (kind) =>
							this.setVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
						),
						actions: GEOMETRY_PLAY_TRANSFORM_MODES.map((mode, order) => ({
							id: `geometry.transform.${mode}`,
							kind: "toggle" as const,
							icon: GEOMETRY_PLAY_TRANSFORM_ICONS[mode],
							label: mode.charAt(0).toUpperCase() + mode.slice(1),
							onPressedChange: (pressed: boolean) => {
								if (pressed) this.setTransformMode(mode);
							},
							order,
							pressed: this.transformMode === mode,
						})),
					},
				},
				{
					id: "analyze",
					label: "Analyze",
					tools: {
						selection: [
							...geometryAnalyzeToolbarToggles("selection", this.analyzeSelectableKinds, (kind) =>
								this.setAnalyzeSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
								(group, enabled) => this.setAnalyzeSelectableKinds((current) => ({ ...current, ...setKindGroup(current, group, enabled) })),
							),
							{ id: "geometry.analyze.selection.separator.clear", kind: "separator" as const, order: ANALYZE_KINDS.length + 4 },
							{ id: "geometry.analyze.selection.clear", icon: <BoxSelect className="size-4" aria-hidden />, label: "Clear", onClick: () => this.setSelectedId(null), order: ANALYZE_KINDS.length + 5 },
						],
						filter: geometryAnalyzeToolbarToggles(
							"filter",
							this.analyzeVisibleKinds,
							(kind) => this.setAnalyzeVisibleKinds((current) => ({ ...current, [kind]: !current[kind] })),
							(group, enabled) => this.setAnalyzeVisibleKinds((current) => ({ ...current, ...setKindGroup(current, group, enabled) })),
						),
					},
				},
			],
		};
	}
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
		const selectedStillValid = mode === "analyze" ? isAnalyzeSelectableEntity(activeSession, play.analyzeSelectableKinds, play.selectedId) : isSelectableEntity(activeSession, play.selectableKinds, play.selectedId);
		if (play.selectedId && !selectedStillValid) play.setSelectedId(null);
	}, [activeSession, mode, play]);
	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<span className="text-muted-foreground px-1 text-xs font-semibold uppercase tracking-wide" data-e2e-geometry-mode>{mode}</span>
				<span className="text-muted-foreground px-1 text-xs font-semibold uppercase tracking-wide" data-e2e-geometry-transform-mode>{mode === "edit" ? play.transformMode : "locked"}</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection-kinds>{mode === "analyze" ? formatEnabledKindsLabel(listEnabledKinds(ANALYZE_KINDS, play.analyzeSelectableKinds), ANALYZE_KINDS.length) : formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, play.selectableKinds), TOPOLOGIC_KINDS.length)}</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-visible-kinds>{mode === "analyze" ? formatEnabledKindsLabel(listEnabledKinds(ANALYZE_KINDS, play.analyzeVisibleKinds), ANALYZE_KINDS.length) : formatEnabledKindsLabel(listEnabledKinds(TOPOLOGIC_KINDS, play.visibleKinds), TOPOLOGIC_KINDS.length)}</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection>{activeSelectedEntity ? activeSelectedEntity.label ?? activeSelectedEntity.id : "—"}</span>
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

export function GeometryPlayController(): ReactElement {
	const [fixture, setFixture] = useState<TopologicFixtureV1 | null>(null);
	const [loadError, setLoadError] = useState<Error | null>(null);
	const session = useMemo(() => (fixture ? new TopologicPlaySession(fixture) : null), [fixture]);
	const analyzeFixture = useMemo(() => (fixture ? deriveAnalyzeTopologicFixtureV1(fixture) : null), [fixture]);
	const analyzeSession = useMemo(() => (analyzeFixture ? new TopologicPlaySession(analyzeFixture) : null), [analyzeFixture]);
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
					setSelectedId: (id) => {
						if (!id || isSelectableEntity(session, selectableKinds, id) || isAnalyzeSelectableEntity(analyzeSession, analyzeSelectableKinds, id)) {
							setSelectedId(id);
						}
					},
					setTransformMode,
					onTransformCommit: (id, transform) =>
						setFixture((current) => {
							if (!current) return current;
							return updateTopologicFixtureTransformKernelV1(current, id, transform) ?? current;
						}),
				}
				: null,
		[analyzeFixture, analyzeSelectableKinds, analyzeSession, analyzeVisibleKinds, fixture, selectableKinds, selectedId, session, transformMode, visibleKinds],
	);

	const apps = useMemo(
		() => [new GeometryPlayDefinition(analyzeSelectableKinds, analyzeVisibleKinds, selectableKinds, transformMode, visibleKinds, setAnalyzeSelectableKinds, setAnalyzeVisibleKinds, setSelectableKinds, setSelectedId, setTransformMode, setVisibleKinds)],
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

export function createGeometryPlayElement(): ReactElement {
	return <GeometryPlayApp />;
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("geometry play react runtime", () => {
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
	});
}