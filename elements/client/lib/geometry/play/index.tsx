// #region 🧲Header
// 💻 elements/client/lib/geometry/play/index.tsx — Geometry play harness: Topologic all-kinds selector, single-window UI shell, and transform gumball editing for every entity kind.
// #endregion 🧲Header

import {
	LevelProvider,
	ToolbarDivider,
	ToolbarGroup,
	ToolbarItem,
	ToolbarZone,
	UI,
	createDefaultLayout,
	getLevelBgClass,
	type UIAppConfig,
} from "@elements/ui";
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
//#endregion 🔖Ids

//#region 🔖Context
interface GeometryPlayValue {
	readonly fixture: TopologicFixtureV1;
	readonly session: TopologicWasmSession;
	readonly selectedKind: TopologicKind;
	readonly selectedId: string | null;
	readonly transformMode: TopologicTransformMode;
	readonly setSelectedKind: (kind: TopologicKind) => void;
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
function geometryToolbarToggleClass(active: boolean): string {
	return [
		"inline-flex shrink-0 items-center justify-center rounded px-2 py-1 text-xs font-medium transition-colors",
		active ? "bg-accent text-accent-foreground border border-element" : "text-muted-foreground hover:bg-hover-panel border border-transparent",
	].join(" ");
}

function geometryKindLabel(kind: TopologicKind): string {
	if (kind === "cellComplex") return "CellComplex";
	return kind.charAt(0).toUpperCase() + kind.slice(1);
}

function GeometryPlayWindow(): ReactElement {
	const play = useGeometryPlay();
	const entitiesOfKind = play.session.listByKind(play.selectedKind);
	const selectedEntity = play.selectedId ? play.session.getEntity(play.selectedId) : null;
	return (
		<div className="flex h-full w-full flex-col">
			<div className="flex shrink-0 gap-2 border-b border-border bg-muted/40 p-2">
				<ToolbarZone>
					<ToolbarGroup>
						<ToolbarItem>
							<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Kinds</span>
						</ToolbarItem>
						{TOPOLOGIC_KINDS.map((kind) => (
							<ToolbarItem key={kind}>
								<button type="button" className={geometryToolbarToggleClass(play.selectedKind === kind)} title={`Show ${geometryKindLabel(kind)} entities`} onClick={() => play.setSelectedKind(kind)}>
									<span className="px-0.5">{geometryKindLabel(kind)}</span>
								</button>
							</ToolbarItem>
						))}
					</ToolbarGroup>
					<ToolbarDivider />
					<ToolbarGroup>
						<ToolbarItem>
							<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Transform</span>
						</ToolbarItem>
						<ToolbarItem>
							<button type="button" className={geometryToolbarToggleClass(play.transformMode === "translate")} title="Translate selection" onClick={() => play.setTransformMode("translate")}>
								<Move3d className="size-4" aria-hidden />
							</button>
						</ToolbarItem>
						<ToolbarItem>
							<button type="button" className={geometryToolbarToggleClass(play.transformMode === "rotate")} title="Rotate selection" onClick={() => play.setTransformMode("rotate")}>
								<Rotate3d className="size-4" aria-hidden />
							</button>
						</ToolbarItem>
						<ToolbarItem>
							<button type="button" className={geometryToolbarToggleClass(play.transformMode === "scale")} title="Scale selection" onClick={() => play.setTransformMode("scale")}>
								<Scaling className="size-4" aria-hidden />
							</button>
						</ToolbarItem>
					</ToolbarGroup>
					<ToolbarDivider />
					<ToolbarGroup>
						<ToolbarItem>
							<span className="text-muted-foreground pr-1 text-[10px] font-semibold uppercase tracking-wide">Selection</span>
						</ToolbarItem>
						<ToolbarItem>
							<button type="button" className={geometryToolbarToggleClass(false)} title="Clear current selection" onClick={() => play.setSelectedId(null)}>
								<BoxSelect className="size-4" aria-hidden />
							</button>
						</ToolbarItem>
						<ToolbarItem>
							<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-kind>{play.selectedKind}</span>
						</ToolbarItem>
						<ToolbarItem>
							<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection>{selectedEntity ? topologicEntityLabel(selectedEntity) : "—"}</span>
						</ToolbarItem>
						<ToolbarItem>
							<span className="text-muted-foreground px-1 text-xs">{entitiesOfKind.length}</span>
						</ToolbarItem>
					</ToolbarGroup>
				</ToolbarZone>
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
	const [selectedKind, setSelectedKind] = useState<TopologicKind>("topology");
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
		const selected = selectedId ? session.getEntity(selectedId) : null;
		if (!selected || selected.kind !== selectedKind) {
			setSelectedId(session.listByKind(selectedKind)[0]?.id ?? null);
		}
	}, [selectedId, selectedKind, session]);

	const value = useMemo<GeometryPlayValue | null>(
		() =>
			fixture && session
				? {
					fixture,
					session,
					selectedKind,
					selectedId,
					transformMode,
					setSelectedKind,
					setSelectedId,
					setTransformMode,
					onTransformCommit: (id, transform) =>
						setFixture((current) => (current ? updateTopologicFixtureTransform(current, id, transform) : current)),
				}
				: null,
		[fixture, selectedId, selectedKind, session, transformMode],
	);

	const apps = useMemo<UIAppConfig[]>(
		() => [
			{
				id: GEOMETRY_PLAY_APP_ID,
				label: "Geometry play",
				windowKinds: [{ id: GEOMETRY_PLAY_WINDOW_ID, label: GEOMETRY_PLAY_WINDOW_LABEL, component: GeometryPlayWindow }],
				defaultLayout: createDefaultLayout([GEOMETRY_PLAY_WINDOW_ID], "row", [100], [GEOMETRY_PLAY_WINDOW_LABEL]),
			},
		],
		[],
	);

	if (loadError) throw loadError;
	if (!value) {
		return <div className={`flex h-screen items-center justify-center text-sm text-muted-foreground ${getLevelBgClass("window")}`}>Loading geometry wasm…</div>;
	}

	return (
		<GeometryPlayContext.Provider value={value}>
			<UI apps={apps} defaultAppId={GEOMETRY_PLAY_APP_ID} className={getLevelBgClass(0)} />
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
	});
}
//#endregion 🧪Tests