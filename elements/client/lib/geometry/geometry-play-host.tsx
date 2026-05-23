// #region 🧲Header
// 💻 elements/client/lib/geometry/geometry-play-host.tsx — Host adapter outside play bundle: icons, declarative window registration, Topologic surface, and mount entry.
// #endregion 🧲Header

import type { UiScene3DHostSurfaceNode } from "@elements/ui-shell";
import {
	Workbench,
	WorkbenchView,
	getLevelBgClass,
	registerDeclarativeWindowBody,
	registerElementIcon,
	registerUiScene3DSurfaceHost,
	useApp,
} from "@elements/ui";
import { BoxSelect, Move3d, Rotate3d, Scaling } from "lucide-react";
import * as React from "react";
import { act } from "react";
import { createRoot } from "react-dom/client";

import topologyJson from "./play/fixtures/topology.json";
import {
	GEOMETRY_PLAY_BODY_KEY,
	GEOMETRY_PLAY_CONTROLLER_ID,
	GEOMETRY_PLAY_ICON_BOX_SELECT,
	GEOMETRY_PLAY_ICON_MOVE_3D,
	GEOMETRY_PLAY_ICON_ROTATE_3D,
	GEOMETRY_PLAY_ICON_SCALE_3D,
	GEOMETRY_PLAY_SCENE3D_SURFACE_ID,
	GeometryPlayShellController,
	buildGeometryPlayDeclarativeBody,
	buildGeometryPlayWorkbenchApp,
	geometryPlayModeFromApp,
	isAnalyzeEntitySelectable,
	isAnalyzeEntityVisible,
} from "./play/index.ts";
import { TopologicViewport } from "../react/index.tsx";
import { ensureTopologicWasmLoaded, loadTopologicFixtureV1, type TopologicFixtureV1 } from "../wasm/index.ts";

let geometryPlayChromeRegistered = false;

function GeometryTopologicScene3DSurfaceHost({ node }: { readonly node: UiScene3DHostSurfaceNode }): React.ReactElement {
	const { workbench, activeModeId } = useApp();
	const shellGen = React.useSyncExternalStore(
		(onStoreChange) => workbench.subscribe(onStoreChange),
		() => workbench.generation,
		() => 0,
	);
	void shellGen;
	const app = workbench.getActiveApp();
	const ctrl = app?.controller as GeometryPlayShellController | undefined;
	if (!ctrl || node.controllerId !== GEOMETRY_PLAY_CONTROLLER_ID) {
		return <div className="p-2 text-xs text-muted-foreground">Invalid geometry viewport binding</div>;
	}
	let play: ReturnType<GeometryPlayShellController["getSnapshot"]> = null;
	try {
		play = ctrl.getSnapshot();
	} catch {
		return <div className="p-2 text-xs text-destructive">Geometry wasm error</div>;
	}
	if (!play) {
		return <div className={`flex h-full items-center justify-center text-sm text-muted-foreground ${getLevelBgClass("window")}`}>Loading geometry wasm…</div>;
	}
	const mode = geometryPlayModeFromApp(activeModeId ?? null);
	const activeFixture = mode === "analyze" ? play.analyzeFixture : play.fixture;
	return (
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
	);
}

function registerGeometryPlayChrome(): void {
	if (geometryPlayChromeRegistered) return;
	geometryPlayChromeRegistered = true;
	registerElementIcon(GEOMETRY_PLAY_ICON_BOX_SELECT, <BoxSelect className="size-4" aria-hidden />);
	registerElementIcon(GEOMETRY_PLAY_ICON_MOVE_3D, <Move3d className="size-4" aria-hidden />);
	registerElementIcon(GEOMETRY_PLAY_ICON_ROTATE_3D, <Rotate3d className="size-4" aria-hidden />);
	registerElementIcon(GEOMETRY_PLAY_ICON_SCALE_3D, <Scaling className="size-4" aria-hidden />);
	registerUiScene3DSurfaceHost(GEOMETRY_PLAY_SCENE3D_SURFACE_ID, GeometryTopologicScene3DSurfaceHost);
	registerDeclarativeWindowBody(GEOMETRY_PLAY_BODY_KEY, buildGeometryPlayDeclarativeBody);
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

/** @emoji 🚀 Vite host entry: mounts geometry play into `#root`. */
export async function mountGeometryPlay(): Promise<void> {
	const { getLevelBgClass, LevelProvider, mountReactApp, WorkbenchView } = await import("@elements/ui");
	const workbench = await bootstrapGeometryPlayWorkbench();
	mountReactApp(
		<LevelProvider>
			<WorkbenchView workbench={workbench} className={getLevelBgClass(0)} />
		</LevelProvider>,
	);
}

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
