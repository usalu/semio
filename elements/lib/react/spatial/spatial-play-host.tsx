// #region 🧲Header
/** @emoji 🚀 Spatial play React host — mounts {@link @elements/playground} via {@link @elements/framework-react} (not imported by framework-free play core). */
// #endregion 🧲Header

import { type UiScene3DHostSurfaceNode, type UiTableHostSurfaceNode } from "@elements/framework";
import { LevelProvider, ProductView, getLevelBgClass, mountReactApp, registerElementIcon, registerUiScene3DSurfaceHost, registerUiTableSurfaceHost, useApp } from "@elements/framework-react";
import { SpatialDetailsPanel, SpatialSurface, SpatialWorkbenchPanel, type SpatialSurfaceSnapshot } from "@elements/spatial-react";
import { ListFilter, ScanSearch } from "lucide-react";
import * as React from "react";

import {
  SPATIAL_PLAY_CONTROLLER_ID,
  SPATIAL_PLAY_DETAILS_ICON_ID,
  SPATIAL_PLAY_PANEL_DETAILS_SURFACE_ID,
  SPATIAL_PLAY_PANEL_WORKBENCH_SURFACE_ID,
  SPATIAL_PLAY_SCENE3D_SURFACE_ID,
  SPATIAL_PLAY_WORKBENCH_ICON_ID,
  SpatialPlayShellController,
  bootstrapSpatialPlayWorkbench,
} from "./play/index.ts";

import "./play/globals.css";

const EMPTY_KINDS = Object.fromEntries(["topology", "vertex", "edge", "wire", "face", "shell", "cell", "cellComplex", "cluster"].map((kind) => [kind, true])) as SpatialSurfaceSnapshot["selectableKinds"];

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
  setSelectedId: () => undefined,
  setEntityTransform: () => undefined,
  workbenchPanel: {
    fixtureLabel: undefined,
    visibleKindsLabel: "",
    selectableKindsLabel: "",
    query: "",
    focusOptions: [],
    entityCount: 0,
    entities: [],
    setFocusedKind: () => undefined,
    setSelectedId: () => undefined,
    setQuery: () => undefined,
  },
  detailsPanel: {
    selectedLabel: "No entity selected",
    selectedKindLabel: "all",
    status: "loading",
    focusedKindLabel: "All",
    query: "none",
  },
};

function useSpatialPlaySnapshot(): SpatialSurfaceSnapshot {
  const { runtime } = useApp();
  const generation = React.useSyncExternalStore(
    (onStoreChange) => runtime.subscribe(onStoreChange),
    () => runtime.generation,
    () => 0,
  );
  void generation;
  const ctrl = runtime.getActiveApp()?.controller as SpatialPlayShellController | undefined;
  return ctrl?.getSnapshot() ?? EMPTY_SNAPSHOT;
}

function SpatialScene3DSurfaceHost({ node }: { readonly node: UiScene3DHostSurfaceNode }): React.ReactElement {
  if (node.controllerId !== SPATIAL_PLAY_CONTROLLER_ID) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid spatial viewport binding</div>;
  }
  return <SpatialSurface snapshot={useSpatialPlaySnapshot()} />;
}

function SpatialWorkbenchPanelHost({ node }: { readonly node: UiTableHostSurfaceNode }): React.ReactElement {
  if (node.controllerId !== SPATIAL_PLAY_CONTROLLER_ID) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid spatial workbench panel binding</div>;
  }
  const snapshot = useSpatialPlaySnapshot();
  const { runtime } = useApp();
  const bus = runtime.commandBus;
  const panel = snapshot.workbenchPanel;
  const panelSnapshot = {
    ...panel,
    setFocusedKind: (kind: (typeof panel.focusOptions)[number]["kind"]) => bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "setFocusedKind", { kind }),
    setSelectedId: (id: string | null) => bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "setSelectedId", { id }),
    setQuery: (query: string) => bus.dispatch(SPATIAL_PLAY_CONTROLLER_ID, "setQuery", { query }),
  };
  return <SpatialWorkbenchPanel snapshot={{ ...snapshot, workbenchPanel: panelSnapshot }} />;
}

function SpatialDetailsPanelHost({ node }: { readonly node: UiTableHostSurfaceNode }): React.ReactElement {
  if (node.controllerId !== SPATIAL_PLAY_CONTROLLER_ID) {
    return <div className="p-2 text-xs text-muted-foreground">Invalid spatial details panel binding</div>;
  }
  return <SpatialDetailsPanel snapshot={useSpatialPlaySnapshot()} />;
}

let spatialPlayChromeRegistered = false;

function registerSpatialPlayChrome(): void {
  if (spatialPlayChromeRegistered) return;
  spatialPlayChromeRegistered = true;
  registerElementIcon(SPATIAL_PLAY_WORKBENCH_ICON_ID, <ListFilter className="size-4" aria-hidden />);
  registerElementIcon(SPATIAL_PLAY_DETAILS_ICON_ID, <ScanSearch className="size-4" aria-hidden />);
  registerUiScene3DSurfaceHost(SPATIAL_PLAY_SCENE3D_SURFACE_ID, SpatialScene3DSurfaceHost);
  registerUiTableSurfaceHost(SPATIAL_PLAY_PANEL_WORKBENCH_SURFACE_ID, SpatialWorkbenchPanelHost);
  registerUiTableSurfaceHost(SPATIAL_PLAY_PANEL_DETAILS_SURFACE_ID, SpatialDetailsPanelHost);
}

/** @emoji 🚀 Vite host entry: mounts spatial play into `#root`. */
export async function mountSpatialPlay(): Promise<void> {
  registerSpatialPlayChrome();
  const runtime = bootstrapSpatialPlayWorkbench();
  mountReactApp(
    <LevelProvider>
      <ProductView runtime={runtime} className={getLevelBgClass(0)} defaultAppId={runtime.apps[0]?.id} initialPanelVisibility={{ leftSidePanel: true, rightSidePanel: true }} />
    </LevelProvider>,
  );
}
