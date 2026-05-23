// #region 🧲Header
// 💻 elements/client/lib/geometry/play/index.tsx — Geometry play harness: Topologic all-kinds selector, single-window UI shell, and transform gumball editing for every entity kind.
// #endregion 🧲Header

import { App, LevelProvider, createDefaultLayout, getLevelBgClass, type AppConfig } from "@elements/ui";
import { BoxSelect, Move3d, Rotate3d, Scaling } from "lucide-react";
import { act, createContext, useContext, useEffect, useMemo, useState, type ReactElement } from "react";
import { createRoot } from "react-dom/client";

import topologyJson from "../fixtures/topology.json";
import { TopologicViewport, type TopologicTransformMode } from "../react/index.tsx";
import {
	TOPOLOGIC_KINDS,
	TopologicWasmSession,
	ensureTopologicWasmLoaded,
	loadTopologicFixtureV1,
	topologicEntityLabel,
	updateTopologicFixtureTransform,
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
const GEOMETRY_PLAY_TRANSFORM_ICONS: Record<TopologicTransformMode, ReactElement> = {
	translate: <Move3d className="size-4" aria-hidden />,
	rotate: <Rotate3d className="size-4" aria-hidden />,
	scale: <Scaling className="size-4" aria-hidden />,
};
//#endregion 🔖Ids

//#region 🔖Context
interface GeometryPlayValue {
	readonly fixture: TopologicFixtureV1;
	readonly session: TopologicWasmSession;
	readonly selectableKinds: Readonly<Record<TopologicKind, boolean>>;
	readonly selectedId: string | null;
	readonly transformMode: TopologicTransformMode;
	readonly toggleSelectableKind: (kind: TopologicKind) => void;
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

function createSelectableKinds(): Record<TopologicKind, boolean> {
	return Object.fromEntries(TOPOLOGIC_KINDS.map((kind) => [kind, true])) as Record<TopologicKind, boolean>;
}

function listSelectableEntities(session: TopologicWasmSession, selectableKinds: Readonly<Record<TopologicKind, boolean>>) {
	return session.fixture.topologies.filter((entity) => selectableKinds[entity.kind]);
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

function GeometryPlayWindow(): ReactElement {
	const play = useGeometryPlay();
	const selectableEntities = listSelectableEntities(play.session, play.selectableKinds);
	const enabledKinds = TOPOLOGIC_KINDS.filter((kind) => play.selectableKinds[kind]);
	const selectedEntity = play.selectedId ? play.session.getEntity(play.selectedId) : null;
	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<span className="text-muted-foreground px-1 text-xs font-semibold uppercase tracking-wide" data-e2e-geometry-transform-mode>
					{play.transformMode}
				</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-kind>{enabledKinds.length === TOPOLOGIC_KINDS.length ? "all" : enabledKinds.join(",") || "none"}</span>
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection>{selectedEntity ? topologicEntityLabel(selectedEntity) : "—"}</span>
				<span className="text-muted-foreground px-1 text-xs">{selectableEntities.length}</span>
			</div>
			<div className="relative min-h-0 flex-1">
				<TopologicViewport
					fixture={play.fixture}
					selectedId={play.selectedId}
					onSelect={play.setSelectedId}
					onTransformCommit={play.onTransformCommit}
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
	const [selectableKinds, setSelectableKinds] = useState<Record<TopologicKind, boolean>>(() => createSelectableKinds());
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
			fixture && session
				? {
					fixture,
					session,
					selectableKinds,
					selectedId,
					transformMode,
					toggleSelectableKind: (kind) => setSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
					setSelectedId: (id) => {
						if (!id || isSelectableEntity(session, selectableKinds, id)) {
							setSelectedId(id);
						}
					},
					setTransformMode,
					onTransformCommit: (id, transform) =>
						setFixture((current) => (current ? updateTopologicFixtureTransform(current, id, transform) : current)),
				}
				: null,
		[fixture, selectableKinds, selectedId, session, transformMode],
	);

	const apps = useMemo<AppConfig[]>(
		() => [
			{
				id: GEOMETRY_PLAY_APP_ID,
				label: "Geometry play",
				options: { selectableKinds, transformMode },
				tools: {
					filter: TOPOLOGIC_KINDS.map((kind, order) => ({
						id: `geometry.kind.${kind}`,
						kind: "toggle" as const,
						text: geometryKindLabel(kind),
						onPressedChange: () => setSelectableKinds((current) => ({ ...current, [kind]: !current[kind] })),
						order,
						pressed: selectableKinds[kind],
					})),
					selection: [
						{
							id: "geometry.selection.clear",
							icon: <BoxSelect className="size-4" aria-hidden />,
							label: "Clear",
							onClick: () => setSelectedId(null),
							order: 0,
						},
					],
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
				windowKinds: [{ id: GEOMETRY_PLAY_WINDOW_ID, label: GEOMETRY_PLAY_WINDOW_LABEL, component: GeometryPlayWindow }],
				defaultLayout: GEOMETRY_PLAY_DEFAULT_LAYOUT,
			},
		],
		[selectableKinds, transformMode],
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
		it("enables selection for every kind by default", () => {
			expect(createSelectableKinds()).toEqual({
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
	});
}
//#endregion 🧪Tests