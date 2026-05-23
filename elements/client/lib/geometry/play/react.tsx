// #region 🧲Header
// 💻 elements/client/lib/geometry/play/react.tsx — React adapter: registers window body + icons, builds {@link Workbench}, and hosts geometry play tests.
// #endregion 🧲Header

import {
	Workbench,
	WorkbenchView,
	getLevelBgClass,
	registerElementIcon,
	registerWindowBody,
	useApp,
} from "@elements/ui";
import { BoxSelect, Move3d, Rotate3d, Scaling } from "lucide-react";
import * as React from "react";
import { act } from "react";
import { createRoot } from "react-dom/client";

import topologyJson from "./fixtures/topology.json";
import {
	ANALYZE_KINDS,
	GEOMETRY_PLAY_BODY_KEY,
	GEOMETRY_PLAY_ICON_BOX_SELECT,
	GEOMETRY_PLAY_ICON_MOVE_3D,
	GEOMETRY_PLAY_ICON_ROTATE_3D,
	GEOMETRY_PLAY_ICON_SCALE_3D,
	GeometryPlayShellController,
	buildGeometryPlayWorkbenchApp,
	formatEnabledKindsLabel,
	geometryPlayModeFromApp,
	isAnalyzeEntitySelectable,
	isAnalyzeEntityVisible,
	isAnalyzeSelectableEntity,
	isSelectableEntity,
	listAnalyzeSelectableEntities,
	listEnabledKinds,
	listSelectableEntities,
} from "./index.tsx";
import { TopologicViewport } from "../react/index.tsx";
import { TOPOLOGIC_KINDS, ensureTopologicWasmLoaded, loadTopologicFixtureV1, type TopologicFixtureV1 } from "../wasm/index.ts";

let geometryPlayChromeRegistered = false;

function registerGeometryPlayChrome(): void {
	if (geometryPlayChromeRegistered) return;
	geometryPlayChromeRegistered = true;
	registerElementIcon(GEOMETRY_PLAY_ICON_BOX_SELECT, <BoxSelect className="size-4" aria-hidden />);
	registerElementIcon(GEOMETRY_PLAY_ICON_MOVE_3D, <Move3d className="size-4" aria-hidden />);
	registerElementIcon(GEOMETRY_PLAY_ICON_ROTATE_3D, <Rotate3d className="size-4" aria-hidden />);
	registerElementIcon(GEOMETRY_PLAY_ICON_SCALE_3D, <Scaling className="size-4" aria-hidden />);
	registerWindowBody(GEOMETRY_PLAY_BODY_KEY, GeometryPlayWindowBody);
}

/** @emoji 🧭 Loads wasm + fixture, registers React chrome, returns a mounted-ready {@link Workbench}. */
export async function bootstrapGeometryPlayWorkbench(): Promise<Workbench> {
	registerGeometryPlayChrome();
	await ensureTopologicWasmLoaded();
	const parsedFixture = (await loadTopologicFixtureV1(topologyJson as unknown)) as TopologicFixtureV1 | null;
	if (!parsedFixture) throw new Error("geometry topology fixture failed to parse");
	const wb = new Workbench();
	const ctrl = new GeometryPlayShellController(wb.commandBus, () => wb.notify(), parsedFixture);
	wb.addApp(buildGeometryPlayWorkbenchApp(ctrl));
	return wb;
}

const GeometryPlayWindowBody: React.FC = () => {
	const { workbench, activeModeId } = useApp();
	const shellGen = React.useSyncExternalStore(
		(onStoreChange) => workbench.subscribe(onStoreChange),
		() => workbench.generation,
		() => 0,
	);
	void shellGen;
	const app = workbench.getActiveApp();
	const ctrl = app?.controller as GeometryPlayShellController | undefined;
	const play = ctrl?.getSnapshot() ?? null;

	React.useEffect(() => {
		if (!ctrl || !play) return;
		const mode = geometryPlayModeFromApp(activeModeId ?? null);
		const activeSession = mode === "analyze" ? play.analyzeSession : play.session;
		const selectedStillValid =
			mode === "analyze"
				? isAnalyzeSelectableEntity(activeSession, play.analyzeSelectableKinds, play.selectedId)
				: isSelectableEntity(activeSession, play.selectableKinds, play.selectedId);
		if (play.selectedId && !selectedStillValid) play.setSelectedId(null);
	}, [ctrl, play, activeModeId, shellGen]);

	if (!play) {
		return <div className={`flex h-full items-center justify-center text-sm text-muted-foreground ${getLevelBgClass("window")}`}>Loading geometry wasm…</div>;
	}
	const mode = geometryPlayModeFromApp(activeModeId ?? null);
	const activeSession = mode === "analyze" ? play.analyzeSession : play.session;
	const activeFixture = mode === "analyze" ? play.analyzeFixture : play.fixture;
	const activeSelectableEntities =
		mode === "analyze" ? listAnalyzeSelectableEntities(activeSession, play.analyzeSelectableKinds) : listSelectableEntities(activeSession, play.selectableKinds);
	const activeSelectedEntity = play.selectedId ? activeSession.getEntity(play.selectedId) : null;
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
				<span className="text-muted-foreground px-1 text-xs" data-e2e-geometry-selection>
					{activeSelectedEntity ? (activeSelectedEntity.label ?? activeSelectedEntity.id) : "—"}
				</span>
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
};

if (import.meta.vitest) {
	const { describe, expect, it } = import.meta.vitest;

	describe("geometry play react runtime", () => {
		it("renders through wasm fixture load without hook-order regressions", async () => {
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
					const wb = await bootstrapGeometryPlayWorkbench();
					root.render(<WorkbenchView workbench={wb} />);
					await Promise.resolve();
					await Promise.resolve();
				});
				expect(errors.some((entry) => entry.includes("change in the order of Hooks"))).toBe(false);
				expect(errors.some((entry) => entry.includes("Rendered more hooks than during the previous render"))).toBe(false);
				expect(container.textContent?.length).toBeGreaterThan(0);
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
