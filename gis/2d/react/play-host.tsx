// #region 🧲Header
/** @emoji 🛝 Playground play host for Map — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiGisMapSurfaceHost, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, useControllerStore, useShellWindowInstance, registerWindowBody } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort, Select } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import type { UiGisMapHostSurfaceNode } from "@semio-tech/framework-platform-core";
import {
    GIS_MAP_PLAY_APP_ID,
    GIS_MAP_PLAY_BODY_KEY_MAIN,
    GIS_MAP_PLAY_CATALOGUE_TAB_ID,
    GIS_MAP_PLAY_HIERARCHY_TAB_ID,
    GIS_MAP_PLAY_IDLE_SNAPSHOT,
    GIS_MAP_PLAY_INSPECTION_TAB_ID,
    GIS_MAP_PLAY_STORE_ID,
    GIS_MAP_PLAY_SURFACE_ID,
    GIS_MAP_PLAY_WINDOW_KIND_ID,
    buildMapPlayCatalogueTree,
    buildMapPlayHierarchyTree,
    buildMapPlayInspectorTree,
    buildMapPlayMainDeclarativeBody,
    parseGisMapFixture,
    type MapPlayController
} from "@semio-tech/gis-2d-core";

let mapPlayChromeRegistered = false;
const mapPlayControllerRef: { current: MapPlayController | null } = { current: null };

function useMapPlayController(runtimeOverride?: Platform): MapPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as MapPlayController | undefined;
  mapPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useMapPlayInteractionRevision(runtime: Platform): number {
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = runtime.getActiveApp()?.controller as MapPlayController | undefined;
      mapPlayControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = runtime.subscribe(listener);
      const unsubscribeSnapshot =
        ctrl && typeof ctrl.subscribeSnapshot === "function" ? ctrl.subscribeSnapshot(listener) : undefined;
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (runtime.getActiveApp()?.controller as MapPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useMapPlaySnapshot() {
  const ctrl = useMapPlayController();
  return useControllerStore(ctrl, GIS_MAP_PLAY_STORE_ID) ?? GIS_MAP_PLAY_IDLE_SNAPSHOT;
}

function buildMapPlayContextMenuItems(ctrl: MapPlayController | null | undefined, context: MapContextMenuContext): ContextMenuItem[] {
  if (!ctrl) {
    return [];
  }
  const { feature } = context;
  if (feature) {
    const selected =
      feature.kind === "position"
        ? ctrl.getSelectedPositionIds().includes(feature.id)
        : ctrl.getSelectedRouteIds().includes(feature.id);
    const items: ContextMenuItem[] = [
      {
        id: "gis-map.ctx.select",
        label: "Select",
        onSelect: () => ctrl.run("setSelection", { positions: feature.kind === "position" ? [feature.id] : [], routes: feature.kind === "route" ? [feature.id] : [], mode: "default" }),
      },
    ];
    if (selected) {
      items.push({
        id: "gis-map.ctx.deselect",
        label: "Deselect",
        onSelect: () => ctrl.run("deselect", { featureId: feature.id, featureKind: feature.kind }),
      });
    }
    items.push({
      id: "gis-map.ctx.focus",
      label: "Focus / zoom to",
      onSelect: () => ctrl.run("focusFeature", { featureId: feature.id, featureKind: feature.kind }),
    });
    if (feature.kind === "position") {
      const position = ctrl.getActiveFixture()?.positions.find((row) => row.id === feature.id);
      if (position?.sourceUrl) {
        items.push({
          id: "gis-map.ctx.source",
          label: "Open source",
          onSelect: () => ctrl.run("openSource", { featureId: feature.id }),
        });
      }
    }
    return items;
  }
  return [
    {
      id: "gis-map.ctx.select-all",
      label: "Select all",
      onSelect: () => ctrl.run("selectAll"),
    },
    {
      id: "gis-map.ctx.clear",
      label: "Clear selection",
      disabled: ctrl.getSelectedPositionIds().length + ctrl.getSelectedRouteIds().length === 0,
      onSelect: () => ctrl.run("clearSelection"),
    },
    {
      id: "gis-map.ctx.fit-world",
      label: "Fit world",
      onSelect: () => ctrl.run("fitWorld"),
    },
  ];
}

function MapPlayPaneSurfaceHost({ node: _node }: { readonly node: UiGisMapHostSurfaceNode }): ReactElement {
  const shellInstance = useShellWindowInstance();
  const scopeId = shellWindowScopeId(shellInstance, GIS_MAP_PLAY_WINDOW_KIND_ID);
  const ctrl = useMapPlayController();
  const snapshot = useMapPlaySnapshot();
  const activeFixture = snapshot.activeFixture ?? ctrl?.getActiveFixture() ?? null;
  const selectedPositionIds = snapshot.selectedPositionIds ?? ctrl?.getSelectedPositionIds() ?? [];
  const selectedRouteIds = snapshot.selectedRouteIds ?? ctrl?.getSelectedRouteIds() ?? [];
  const hoveredFeature = snapshot.hoveredFeature ?? ctrl?.getHoveredFeature() ?? null;
  const selectionMethod = snapshot.selectionMethod ?? ctrl?.getSelectionMethod() ?? "rectangle";
  const fitWorldRevision = snapshot.fitWorldRevision ?? ctrl?.getFitWorldRevision() ?? 0;
  const renderMode = ctrl?.getRenderModeForScope(scopeId) ?? snapshot.renderModeByInstance[scopeId] ?? snapshot.renderMode;
  const vectorStyle = ctrl?.getVectorStyleForScope(scopeId) ?? snapshot.vectorStyleByInstance[scopeId] ?? snapshot.vectorStyle;
  const lodMode = ctrl?.getLodModeForScope(scopeId) ?? snapshot.lodModeByInstance[scopeId] ?? snapshot.lodMode;
  const layerVisibility = ctrl?.getLayerVisibilityForScope(scopeId) ?? snapshot.layerVisibilityByInstance[scopeId] ?? snapshot.layerVisibility;
  const layerStrokeScale = ctrl?.getLayerStrokeScaleForScope(scopeId) ?? snapshot.layerStrokeScaleByInstance[scopeId] ?? snapshot.layerStrokeScale;
  const reportEffectiveLod = reactHostPort.useCallback(
    (lodId: GisMapLodId) => {
      ctrl?.run("setEffectiveLod", { lod: lodId, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  const handleSelect = reactHostPort.useCallback(
    (payload: MapSelectPayload) => {
      ctrl?.run("setSelection", {
        positions: [...payload.positions],
        routes: [...payload.routes],
        mode: payload.mode,
      });
    },
    [ctrl],
  );
  const handleHoverChange = reactHostPort.useCallback(
    (feature: MapHoveredFeature | null) => {
      ctrl?.run("setHover", {
        featureId: feature?.id ?? null,
        featureKind: feature?.kind ?? null,
      });
    },
    [ctrl],
  );
  const getContextMenuItems = reactHostPort.useCallback(
    (context: MapContextMenuContext) => buildMapPlayContextMenuItems(ctrl, context),
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!activeFixture) {
      return;
    }
    console.log(
      `[DEBUG] gis map fixture loaded: ${activeFixture.positions.length} positions, ${activeFixture.routes.length} routes`,
    );
  }, [activeFixture]);
  return (
    <MapCanvas
      renderMode={renderMode}
      vectorStyle={vectorStyle}
      lodMode={lodMode}
      layerVisibility={layerVisibility}
      layerStrokeScale={layerStrokeScale}
      onEffectiveLodChange={reportEffectiveLod}
      selectedPositionIds={selectedPositionIds}
      selectedRouteIds={selectedRouteIds}
      hoveredFeature={hoveredFeature}
      selectionMethod={selectionMethod}
      onSelect={handleSelect}
      onHoverChange={handleHoverChange}
      getContextMenuItems={getContextMenuItems}
      fitWorldRevision={fitWorldRevision}
    >
      {activeFixture?.positions.map((position) => (
        <Position
          key={position.id}
          id={position.id}
          lon={position.lon}
          lat={position.lat}
          label={position.label}
          name={position.name}
          icon={position.icon}
          sourceUrl={position.sourceUrl}
          kind={position.kind}
        />
      ))}
      {activeFixture?.routes.map((route) => (
        <Route key={route.id} id={route.id} points={route.points} />
      ))}
    </MapCanvas>
  );
}

export function registerMapPlaySurfaceHosts(): void {
  if (mapPlayChromeRegistered) return;
  mapPlayChromeRegistered = true;
  registerUiGisMapSurfaceHost(GIS_MAP_PLAY_SURFACE_ID, MapPlayPaneSurfaceHost);
  registerWindowBody(GIS_MAP_PLAY_BODY_KEY_MAIN, buildMapPlayMainDeclarativeBody);
}

class MapPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: GIS_MAP_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = mapPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildMapPlayHierarchyTree(
          ctrl?.getActiveFixture() ?? null,
          ctrl?.getSelectedPositionIds() ?? [],
          ctrl?.getSelectedRouteIds() ?? [],
          ctrl?.getHoveredFeature() ?? null,
          (payload) => ctrl?.run("setHover", payload),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class MapPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: GIS_MAP_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildMapPlayCatalogueTree(), bus);
      }),
    };
  }
}

class MapPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: GIS_MAP_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = mapPlayControllerRef.current;
        const bus = new CommandBus();
        const treeNode = buildMapPlayInspectorTree(
          ctrl?.getActiveFixture() ?? null,
          ctrl?.getSelectedPositionIds() ?? [],
          ctrl?.getSelectedRouteIds() ?? [],
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function MapPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useMapPlayController(runtime);
  const interactionRevision = useMapPlayInteractionRevision(runtime);
  const mapPlayHierarchyPanel = reactHostPort.useMemo(() => new MapPlayHierarchyPanelDefinition(), []);
  const mapPlayCataloguePanel = reactHostPort.useMemo(() => new MapPlayCataloguePanelDefinition(), []);
  const mapPlayInspectionPanel = reactHostPort.useMemo(() => new MapPlayInspectionPanelDefinition(), []);
  const augmentPanelTabs = reactHostPort.useMemo(
    () => ({
      workbench: [mapPlayHierarchyPanel, mapPlayCataloguePanel],
      details: [mapPlayInspectionPanel],
    }),
    [interactionRevision, mapPlayCataloguePanel, mapPlayHierarchyPanel, mapPlayInspectionPanel],
  );
  return <PlaygroundView runtime={runtime} defaultAppId={GIS_MAP_PLAY_APP_ID} augmentPanelTabs={augmentPanelTabs} />;
}

function MapPlayChrome({ runtime }: { readonly runtime: Platform }): ReactElement {
  return <MapPlayInner runtime={runtime} />;
}

export function mountMapPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<MapPlayChrome runtime={playground.runtime} />, rootId);
}

const mapPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerMapPlaySurfaceHosts,
  mount: mountMapPlayChrome,
};

export function bootMapPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, mapPlayChromeBoot, rootId);
}
//#endregion 🔖MapPlayHost