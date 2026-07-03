// #region 🧲Header
/** @emoji 🛝 Playground play host for Shooting — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, useApp, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiShootingSurfaceHost, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import * as React from "react";
import type { UiShootingHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
    SHOOTING_PLAY_APP_ID,
    SHOOTING_PLAY_CATALOGUE_TAB_ID,
    SHOOTING_PLAY_HIERARCHY_TAB_ID,
    SHOOTING_PLAY_INSPECTION_TAB_ID,
    SHOOTING_PLAY_SURFACE_ID_ICON,
    SHOOTING_PLAY_SURFACE_ID_MODEL,
    ShootingPlayController,
    buildShootingPlayCatalogueTree,
    buildShootingPlayHierarchyTree,
    buildShootingPlayInspectorTree,
    registerShootingPlayDeclarativeBodies,
    type ShootingPlayHostBridge
} from "@semio-tech/shooting-core";

const shootingPlayControllerRef: { current: ShootingPlayController | null } = { current: null };
let shootingPlayChromeRegistered = false;


function useShootingPlayController(runtimeOverride?: Platform): ShootingPlayController | undefined {
	const appCtx = reactHostPort.useContext(PlaygroundContext);
	const runtime = runtimeOverride ?? appCtx?.runtime;
	reactHostPort.useSyncExternalStore(
		(listener) => (runtime ? runtime.subscribe(listener) : () => {}),
		() => runtime?.generation ?? 0,
		() => 0,
	);
	const ctrl = runtime?.getActiveApp()?.controller as ShootingPlayController | undefined;
	shootingPlayControllerRef.current = ctrl ?? null;
	return ctrl;
}

function ShootingPlayFileBridge(): ReactElement | null {
	const ctrl = useShootingPlayController();
	const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
	const assetInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
	const downloadFixture = reactHostPort.useCallback(async () => {
		if (!ctrl) return;
		const text = ctrl.getFixtureJson();
		const blob = new Blob([`${text}\n`], { type: "application/json" });
		const url = URL.createObjectURL(blob);
		const anchor = document.createElement("a");
		anchor.href = url;
		anchor.download = "shooting.fixture.json";
		anchor.click();
		URL.revokeObjectURL(url);
	}, [ctrl]);
	const downloadShot = reactHostPort.useCallback(
		async (shotId?: string) => {
			if (!ctrl) return;
			const fixture = ctrl.getFixture();
			const shots = shotId ? fixture.shots.filter((shot) => shot.id === shotId) : fixture.shots;
			for (const shot of shots) {
				const result = await renderShootingShot(fixture, shot);
				const extension = shot.format === "svg" ? "svg" : "png";
				const anchor = document.createElement("a");
				anchor.href = result.dataUrl;
				anchor.download = `${shot.id}.${extension}`;
				anchor.click();
				console.log(`[DEBUG] shooting exported shot ${shot.id}.${extension}`);
			}
		},
		[ctrl],
	);
	const handleLoadFile = reactHostPort.useCallback(
		(event: React.ChangeEvent<HTMLInputElement>) => {
			const file = event.target.files?.[0];
			event.target.value = "";
			if (!file || !ctrl) return;
			void file.text().then((text) => {
				ctrl.run("setFixtureJson", { json: text });
				console.log("[DEBUG] shooting play loaded fixture from file");
			});
		},
		[ctrl],
	);
	const handleImportAsset = reactHostPort.useCallback(
		(event: React.ChangeEvent<HTMLInputElement>) => {
			const file = event.target.files?.[0];
			event.target.value = "";
			if (!file || !ctrl) return;
			const objectUrl = URL.createObjectURL(file);
			const id = file.name.replace(/\.[^.]+$/, "").replace(/[^\w-]+/g, "-") || `asset_${Date.now()}`;
			ctrl.run("importAsset", {
				asset: { id, name: file.name, url: objectUrl, format: "glb" },
			});
		},
		[ctrl],
	);
	reactHostPort.useEffect(() => {
		if (!ctrl) return;
		const bridge: ShootingPlayHostBridge = {
			getToolbarState: () => ({
				hasStoredFixture: ctrl.hasStoredFixture(),
				activeShotId: ctrl.getFixture().activeShotId ?? resolveActiveShot(ctrl.getFixture())?.id ?? null,
			}),
			runHostCommand: (command) => {
				if (command === "saveDownload") {
					void downloadFixture();
					return;
				}
				if (command === "loadRequest") {
					loadInputRef.current?.click();
					return;
				}
				if (command === "importAssetRequest") {
					assetInputRef.current?.click();
					return;
				}
				if (command === "exportActiveShot") {
					const active = resolveActiveShot(ctrl.getFixture());
					if (active) void downloadShot(active.id);
					return;
				}
				if (command === "exportAllShots") {
					void downloadShot();
				}
			},
		};
		ctrl.setHostBridge(bridge);
		return () => ctrl.setHostBridge(null);
	}, [ctrl, downloadFixture, downloadShot]);
	return (
		<>
			<input ref={loadInputRef} type="file" accept=".json,application/json" className="hidden" onChange={handleLoadFile} />
			<input ref={assetInputRef} type="file" accept=".glb,model/gltf-binary" className="hidden" onChange={handleImportAsset} />
		</>
	);
}

function ShootingModelSurfaceHost({ node }: { readonly node: UiShootingHostSurfaceNode }): ReactElement {
	const ctrl = useShootingPlayController();
	const fixture = ctrl?.getFixture();
	if (!fixture || node.view !== "model") {
		return <div className="absolute inset-0 min-h-0 min-w-0" />;
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<ShootingModelCanvas
				fixture={fixture}
				className="h-full w-full"
				centerModel={ctrl?.getCenterModel() ?? true}
				fitRevision={ctrl?.getFitRevision() ?? 0}
				onCamera={(camera) => ctrl?.run("setCamera", { camera })}
			/>
		</div>
	);
}

function ShootingIconSurfaceHost({ node }: { readonly node: UiShootingHostSurfaceNode }): ReactElement {
	const { runtime } = useApp();
	const ctrl = useShootingPlayController();
	const revision = ctrl?.getRenderRevision() ?? 0;
	void runtime.generation;
	const fixture = ctrl?.getFixture();
	if (!fixture || node.view !== "icon") {
		return <div className="absolute inset-0 min-h-0 min-w-0" />;
	}
	return (
		<div className="absolute inset-0 min-h-0 min-w-0">
			<ShootingIconCanvas fixture={fixture} className="h-full w-full" renderRevision={revision} />
		</div>
	);
}

function useShootingPlayInteractionRevision(runtime: Platform): number {
	return reactHostPort.useSyncExternalStore(
		(listener) => {
			const ctrl = runtime.getActiveApp()?.controller as ShootingPlayController | undefined;
			shootingPlayControllerRef.current = ctrl ?? null;
			const unsubscribeRuntime = runtime.subscribe(listener);
			const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
			return () => {
				unsubscribeRuntime();
				unsubscribeSnapshot?.();
			};
		},
		() => (runtime.getActiveApp()?.controller as ShootingPlayController | undefined)?.getInteractionRevision() ?? 0,
		() => 0,
	);
}

class ShootingPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: SHOOTING_PLAY_HIERARCHY_TAB_ID,
			icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
			name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => {
				const ctrl = shootingPlayControllerRef.current;
				const bus = new CommandBus();
				const fixture = ctrl?.getFixture();
				if (!fixture) {
					return [{ id: "shooting-play-hierarchy.loading", label: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, items: [{ id: "loading", label: "…" }] }];
				}
				const treeNode = buildShootingPlayHierarchyTree(fixture, ctrl?.getSelectedShotIds() ?? [], ctrl?.getSelectedAssetIds() ?? []);
				return uiTreeNodeToTreePanelConfig(treeNode, bus);
			}),
		};
	}
}

class ShootingPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: SHOOTING_PLAY_CATALOGUE_TAB_ID,
			icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
			name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
			order: 1,
			tree: new CallbackTreePanelDefinition(() => {
				const bus = new CommandBus();
				return uiTreeNodeToTreePanelConfig(buildShootingPlayCatalogueTree(), bus);
			}),
		};
	}
}

class ShootingPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
	buildTab(): SidePanelTabConfig {
		return {
			id: SHOOTING_PLAY_INSPECTION_TAB_ID,
			icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
			name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
			order: 0,
			tree: new CallbackTreePanelDefinition(() => {
				const ctrl = shootingPlayControllerRef.current;
				const bus = new CommandBus();
				const fixture = ctrl?.getFixture();
				if (!fixture) {
					return [{ id: "shooting-play-inspector.loading", label: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, items: [{ id: "loading", label: "…" }] }];
				}
				const treeNode = buildShootingPlayInspectorTree(fixture, ctrl?.getSelectedShotIds() ?? [], ctrl?.getSelectedAssetIds() ?? []);
				return uiTreeNodeToTreePanelConfig(treeNode, bus);
			}),
		};
	}
}

function ShootingPlayInner({ playground }: { readonly playground: Playground }): ReactElement {
	const interactionRevision = useShootingPlayInteractionRevision(playground.runtime);
	const ctrl = useShootingPlayController(playground.runtime);
	shootingPlayControllerRef.current = ctrl ?? null;
	const shootingPlayHierarchyPanel = reactHostPort.useMemo(() => new ShootingPlayHierarchyPanelDefinition(), []);
	const shootingPlayCataloguePanel = reactHostPort.useMemo(() => new ShootingPlayCataloguePanelDefinition(), []);
	const shootingPlayInspectionPanel = reactHostPort.useMemo(() => new ShootingPlayInspectionPanelDefinition(), []);
	const augmentPanelTabs = reactHostPort.useMemo(
		() => ({
			workbench: [shootingPlayHierarchyPanel, shootingPlayCataloguePanel],
			details: [shootingPlayInspectionPanel],
		}),
		[interactionRevision, shootingPlayCataloguePanel, shootingPlayHierarchyPanel, shootingPlayInspectionPanel],
	);
	return (
		<>
			<ShootingPlayFileBridge />
			<PlaygroundView runtime={playground.runtime} defaultAppId={SHOOTING_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} playgroundKeybindings={playground.keybindings} />
		</>
	);
}

export function registerShootingPlaySurfaceHosts(): void {
	if (shootingPlayChromeRegistered) return;
	shootingPlayChromeRegistered = true;
	registerUiShootingSurfaceHost(SHOOTING_PLAY_SURFACE_ID_MODEL, ShootingModelSurfaceHost);
	registerUiShootingSurfaceHost(SHOOTING_PLAY_SURFACE_ID_ICON, ShootingIconSurfaceHost);
	registerShootingPlayDeclarativeBodies();
}

function ShootingPlayChrome({ playground }: { readonly playground: Playground }): ReactElement {
	return <ShootingPlayInner playground={playground} />;
}

export function mountShootingPlayChrome(playground: Playground, rootId = "root"): void {
	mountPlaygroundApp(<ShootingPlayChrome playground={playground} />, rootId);
}

const shootingPlayChromeBoot: PlaygroundChromeBoot = {
	registerHosts: registerShootingPlaySurfaceHosts,
	mount: mountShootingPlayChrome,
};

export function bootShootingPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, shootingPlayChromeBoot, rootId);
}
//#endregion 🔖ShootingPlayHost