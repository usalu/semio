// #region 🧲Header
// 💻 elements/client/lib/geometry/play/react.tsx — React runtime for geometry play: contexts, window body, controller, and React-specific tests.
// #endregion 🧲Header

import { App, AppContext, LevelProvider, PureAppDefinition, getLevelBgClass, type AppConfig, type UIToolbarItem } from "@elements/ui";
import { BoxSelect, Move3d, Rotate3d, Scaling } from "lucide-react";
import * as React from "react";
import { act } from "react";
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

interface GeometryPlayToolbarState {
	readonly analyzeSelectableKinds: Record<AnalyzeKind, boolean>;
	readonly analyzeVisibleKinds: Record<AnalyzeKind, boolean>;
	readonly selectableKinds: Record<TopologicKind, boolean>;
	readonly transformMode: TopologicTransformMode;
	readonly visibleKinds: Record<TopologicKind, boolean>;
}

interface GeometryPlayActions {
	readonly toggleSelectableKind: (kind: TopologicKind) => void;
	readonly toggleVisibleKind: (kind: TopologicKind) => void;
	readonly toggleAnalyzeSelectableKind: (kind: AnalyzeKind) => void;
	readonly toggleAnalyzeVisibleKind: (kind: AnalyzeKind) => void;
	readonly setAnalyzeSelectableGroup: (group: readonly AnalyzeKind[], enabled: boolean) => void;
	readonly setAnalyzeVisibleGroup: (group: readonly AnalyzeKind[], enabled: boolean) => void;
	readonly setSelectedId: (id: string | null) => void;
	readonly setTransformMode: (mode: TopologicTransformMode) => void;
}

interface GeometryPlayControllerState extends GeometryPlayToolbarState {
	readonly fixture: TopologicFixtureV1 | null;
	readonly loadError: Error | null;
	readonly selectedId: string | null;
}

const GeometryPlayContext = React.createContext<GeometryPlayValue | null>(null);

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
		private readonly stateSnapshot: GeometryPlayToolbarState,
		private readonly actions: GeometryPlayActions,
	) {
		super();
	}

	resolveConfig(): AppConfig {
		const selectionKindOrderBase = TOPOLOGIC_KINDS.length;
		return {
			id: GEOMETRY_PLAY_APP_ID,
			label: "Geometry play",
			options: {
				selectableKinds: this.stateSnapshot.selectableKinds,
				visibleKinds: this.stateSnapshot.visibleKinds,
				analyzeSelectableKinds: this.stateSnapshot.analyzeSelectableKinds,
				analyzeVisibleKinds: this.stateSnapshot.analyzeVisibleKinds,
				transformMode: this.stateSnapshot.transformMode,
			},
			windowKinds: [{ id: GEOMETRY_PLAY_WINDOW_ID, label: GEOMETRY_PLAY_WINDOW_LABEL, component: GeometryPlayWindowHost }],
			defaultLayout: GEOMETRY_PLAY_DEFAULT_LAYOUT,
			defaultModeId: "edit",
			modes: [
				{
					id: "edit",
					label: "Edit",
					tools: {
						selection: [
							...geometryKindToolbarToggles("selection", TOPOLOGIC_KINDS, geometryKindLabel, this.stateSnapshot.selectableKinds, (kind) =>
								this.actions.toggleSelectableKind(kind),
							),
							{ id: "geometry.selection.separator.clear", kind: "separator" as const, order: selectionKindOrderBase },
							{ id: "geometry.selection.clear", icon: <BoxSelect className="size-4" aria-hidden />, label: "Clear", onClick: () => this.actions.setSelectedId(null), order: selectionKindOrderBase + 1 },
						],
						filter: geometryKindToolbarToggles("filter", TOPOLOGIC_KINDS, geometryKindLabel, this.stateSnapshot.visibleKinds, (kind) =>
							this.actions.toggleVisibleKind(kind),
						),
						actions: GEOMETRY_PLAY_TRANSFORM_MODES.map((mode, order) => ({
							id: `geometry.transform.${mode}`,
							kind: "toggle" as const,
							icon: GEOMETRY_PLAY_TRANSFORM_ICONS[mode],
							label: mode.charAt(0).toUpperCase() + mode.slice(1),
							onPressedChange: (pressed: boolean) => {
								if (pressed) this.actions.setTransformMode(mode);
							},
							order,
							pressed: this.stateSnapshot.transformMode === mode,
						})),
					},
				},
				{
					id: "analyze",
					label: "Analyze",
					tools: {
						selection: [
							...geometryAnalyzeToolbarToggles("selection", this.stateSnapshot.analyzeSelectableKinds, (kind) =>
								this.actions.toggleAnalyzeSelectableKind(kind),
								(group, enabled) => this.actions.setAnalyzeSelectableGroup(group, enabled),
							),
							{ id: "geometry.analyze.selection.separator.clear", kind: "separator" as const, order: ANALYZE_KINDS.length + 4 },
							{ id: "geometry.analyze.selection.clear", icon: <BoxSelect className="size-4" aria-hidden />, label: "Clear", onClick: () => this.actions.setSelectedId(null), order: ANALYZE_KINDS.length + 5 },
						],
						filter: geometryAnalyzeToolbarToggles(
							"filter",
							this.stateSnapshot.analyzeVisibleKinds,
							(kind) => this.actions.toggleAnalyzeVisibleKind(kind),
							(group, enabled) => this.actions.setAnalyzeVisibleGroup(group, enabled),
						),
					},
				},
			],
		};
	}
}

class GeometryPlayWindow extends React.Component<{ readonly play: GeometryPlayValue }> {
	static contextType = AppContext;
	declare context: React.ContextType<typeof AppContext>;

	componentDidMount(): void {
		this.ensureSelectionValidity();
	}

	componentDidUpdate(prevProps: Readonly<{ readonly play: GeometryPlayValue }>): void {
		if (prevProps.play !== this.props.play || this.context?.activeModeId !== undefined) {
			this.ensureSelectionValidity();
		}
	}

	private ensureSelectionValidity(): void {
		const { play } = this.props;
		const mode = geometryPlayModeFromApp(this.context?.activeModeId ?? null);
		const activeSession = mode === "analyze" ? play.analyzeSession : play.session;
		const selectedStillValid =
			mode === "analyze"
				? isAnalyzeSelectableEntity(activeSession, play.analyzeSelectableKinds, play.selectedId)
				: isSelectableEntity(activeSession, play.selectableKinds, play.selectedId);
		if (play.selectedId && !selectedStillValid) {
			play.setSelectedId(null);
		}
	}

	render(): React.ReactElement {
		const { play } = this.props;
		const mode = geometryPlayModeFromApp(this.context?.activeModeId ?? null);
		const activeSession = mode === "analyze" ? play.analyzeSession : play.session;
		const activeFixture = mode === "analyze" ? play.analyzeFixture : play.fixture;
		const activeSelectableEntities = mode === "analyze" ? listAnalyzeSelectableEntities(activeSession, play.analyzeSelectableKinds) : listSelectableEntities(activeSession, play.selectableKinds);
		const activeSelectedEntity = play.selectedId ? activeSession.getEntity(play.selectedId) : null;
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
}

class GeometryPlayWindowHost extends React.Component {
	render(): React.ReactElement | null {
		return <GeometryPlayContext.Consumer>{(play) => (play ? <GeometryPlayWindow play={play} /> : null)}</GeometryPlayContext.Consumer>;
	}
}

export class GeometryPlayController extends React.Component<{}, GeometryPlayControllerState> {
	state: GeometryPlayControllerState = {
		fixture: null,
		loadError: null,
		selectableKinds: createAllKindsEnabled(TOPOLOGIC_KINDS),
		visibleKinds: createAllKindsEnabled(TOPOLOGIC_KINDS),
		analyzeSelectableKinds: createAllKindsEnabled(ANALYZE_KINDS),
		analyzeVisibleKinds: createAllKindsEnabled(ANALYZE_KINDS),
		selectedId: null,
		transformMode: "translate",
	};

	private cancelled = false;

	private readonly actions: GeometryPlayActions = {
		toggleSelectableKind: (kind) => {
			this.setState((current) => ({ selectableKinds: { ...current.selectableKinds, [kind]: !current.selectableKinds[kind] } }));
		},
		toggleVisibleKind: (kind) => {
			this.setState((current) => ({ visibleKinds: { ...current.visibleKinds, [kind]: !current.visibleKinds[kind] } }));
		},
		toggleAnalyzeSelectableKind: (kind) => {
			this.setState((current) => ({ analyzeSelectableKinds: { ...current.analyzeSelectableKinds, [kind]: !current.analyzeSelectableKinds[kind] } }));
		},
		toggleAnalyzeVisibleKind: (kind) => {
			this.setState((current) => ({ analyzeVisibleKinds: { ...current.analyzeVisibleKinds, [kind]: !current.analyzeVisibleKinds[kind] } }));
		},
		setAnalyzeSelectableGroup: (group, enabled) => {
			this.setState((current) => ({ analyzeSelectableKinds: { ...current.analyzeSelectableKinds, ...setKindGroup(current.analyzeSelectableKinds, group, enabled) } }));
		},
		setAnalyzeVisibleGroup: (group, enabled) => {
			this.setState((current) => ({ analyzeVisibleKinds: { ...current.analyzeVisibleKinds, ...setKindGroup(current.analyzeVisibleKinds, group, enabled) } }));
		},
		setSelectedId: (id) => {
			const value = this.createPlayValue();
			if (!value || !id || isSelectableEntity(value.session, value.selectableKinds, id) || isAnalyzeSelectableEntity(value.analyzeSession, value.analyzeSelectableKinds, id)) {
				this.setState({ selectedId: id });
			}
		},
		setTransformMode: (mode) => {
			this.setState({ transformMode: mode });
		},
	};

	componentDidMount(): void {
		void this.loadFixture();
	}

	componentDidUpdate(_prevProps: {}, prevState: Readonly<GeometryPlayControllerState>): void {
		const currentValue = this.createPlayValue();
		if (!currentValue) return;
		if (
			prevState.selectedId !== this.state.selectedId ||
			prevState.fixture !== this.state.fixture ||
			prevState.selectableKinds !== this.state.selectableKinds
		) {
			if (!isSelectableEntity(currentValue.session, currentValue.selectableKinds, this.state.selectedId)) {
				if (this.state.selectedId !== null) {
					this.setState({ selectedId: null });
				}
			}
		}
	}

	componentWillUnmount(): void {
		this.cancelled = true;
	}

	private async loadFixture(): Promise<void> {
		try {
			await ensureTopologicWasmLoaded();
			const parsedFixture = await loadTopologicFixtureV1(topologyJson as unknown);
			if (!parsedFixture) throw new Error("geometry topology fixture failed to parse");
			if (!this.cancelled) {
				this.setState({ fixture: parsedFixture, loadError: null });
			}
		} catch (error) {
			if (!this.cancelled) {
				this.setState({ loadError: error instanceof Error ? error : new Error(String(error)) });
			}
		}
	}

	private createPlayValue(): GeometryPlayValue | null {
		if (!this.state.fixture) return null;
		const session = new TopologicPlaySession(this.state.fixture);
		const analyzeFixture = deriveAnalyzeTopologicFixtureV1(this.state.fixture);
		const analyzeSession = new TopologicPlaySession(analyzeFixture);
		return {
			fixture: this.state.fixture,
			session,
			analyzeFixture,
			analyzeSession,
			selectableKinds: this.state.selectableKinds,
			visibleKinds: this.state.visibleKinds,
			analyzeSelectableKinds: this.state.analyzeSelectableKinds,
			analyzeVisibleKinds: this.state.analyzeVisibleKinds,
			selectedId: this.state.selectedId,
			transformMode: this.state.transformMode,
			setSelectedId: this.actions.setSelectedId,
			setTransformMode: this.actions.setTransformMode,
			onTransformCommit: (id, transform) => {
				this.setState((current) => ({
					fixture: current.fixture ? updateTopologicFixtureTransformKernelV1(current.fixture, id, transform) ?? current.fixture : current.fixture,
				}));
			},
		};
	}

	render(): React.ReactElement {
		if (this.state.loadError) throw this.state.loadError;
		const value = this.createPlayValue();
		if (!value) {
			return <div className={`flex h-screen items-center justify-center text-sm text-muted-foreground ${getLevelBgClass("window")}`}>Loading geometry wasm…</div>;
		}
		const apps = [
			new GeometryPlayDefinition(
				{
					analyzeSelectableKinds: this.state.analyzeSelectableKinds,
					analyzeVisibleKinds: this.state.analyzeVisibleKinds,
					selectableKinds: this.state.selectableKinds,
					transformMode: this.state.transformMode,
					visibleKinds: this.state.visibleKinds,
				},
				this.actions,
			),
		];
		return (
			<GeometryPlayContext.Provider value={value}>
				<App apps={apps} defaultAppId={GEOMETRY_PLAY_APP_ID} className={getLevelBgClass(0)} />
			</GeometryPlayContext.Provider>
		);
	}
}

class GeometryPlayApp extends React.Component {
	render(): React.ReactElement {
		return (
			<LevelProvider>
				<GeometryPlayController />
			</LevelProvider>
		);
	}
}

export function createGeometryPlayElement(): React.ReactElement {
	return <GeometryPlayApp />;
}

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("geometry play react runtime", () => {
		it("renders through wasm fixture load without hook-based controller logic", async () => {
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
				expect(container.textContent?.includes("Loading geometry wasm")).toBe(true);
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