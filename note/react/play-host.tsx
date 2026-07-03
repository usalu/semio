// #region 🧲Header
/** @emoji 🛝 Note app renderer contribution — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import type { AppRendererContribution, UiNoteHostSurfaceNode } from "@semio-tech/framework-platform-core";
import type { OsAppInstance } from "@semio-tech/framework-os-core";
import { OsUpstreamBadge, useOsInstanceHostBridge, useOsInstanceMaterialization } from "@semio-tech/framework-os-renderer-react";
import { PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig, controllerBackedExampleContribution } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_CATALOG, CANVAS_HOVER_SOURCE_HIERARCHY } from "@semio-tech/framework-core";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import * as React from "react";
import {
  NOTE_PLAY_CATALOGUE_TAB_ID,
  NOTE_PLAY_CONTROLLER_ID,
  NOTE_PLAY_HIERARCHY_TAB_ID,
  NOTE_PLAY_PROPERTIES_TAB_ID,
  NOTE_PLAY_SURFACE_ID_COMPOSITE,
  NOTE_PLAY_SURFACE_ID_NAVIGATOR,
  NotePlayController,
  buildNotePlayCatalogueTree,
  buildNotePlayHierarchyTree,
  buildNotePlayInspectorTree,
  createNotePlayExampleHost,
  createNotePlayHierarchyTreeDragController,
  defaultNoteDocument,
  noteDocumentToJson,
  type NoteDocument,
  type NotePlayHostBridge,
  notePlayWindowBodies,
} from "@semio-tech/note-core";
import { NoteCanvas } from "./index.tsx";

const notePlayControllerRef: { current: NotePlayController | null } = { current: null };

function useNotePlayController(runtimeOverride?: Platform): NotePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => {
      const unsubscribeRuntime = runtime ? runtime.subscribe(listener) : () => {};
      const ctrl = runtime?.getActiveApp()?.controller as NotePlayController | undefined;
      const unsubscribeCtrl = ctrl?.subscribe(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeCtrl?.();
      };
    },
    () => {
      const generation = runtime?.generation ?? 0;
      const revision = (runtime?.getActiveApp()?.controller as NotePlayController | undefined)?.getInteractionRevision() ?? 0;
      return generation * 1_000_000 + revision;
    },
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as NotePlayController | undefined;
  notePlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function NotePlayFileBridge(): ReactElement | null {
  const ctrl = useNotePlayController();
  const loadInputRef = reactHostPort.useRef<HTMLInputElement | null>(null);
  const downloadFixture = reactHostPort.useCallback(async () => {
    if (!ctrl) return;
    const text = ctrl.getDocumentJson();
    const blob = new Blob([text], { type: "application/json" });
    const url = URL.createObjectURL(blob);
    const anchor = document.createElement("a");
    anchor.href = url;
    anchor.download = "semio.note.json";
    anchor.click();
    URL.revokeObjectURL(url);
    console.log("[DEBUG] note play exported document");
  }, [ctrl]);
  const handleLoadFile = reactHostPort.useCallback(
    (event: React.ChangeEvent<HTMLInputElement>) => {
      const file = event.target.files?.[0];
      event.target.value = "";
      if (!file || !ctrl) return;
      void file.text().then((text) => {
        ctrl.run("setFixtureJson", { json: text, resetInteraction: true });
        console.log("[DEBUG] note play imported document from file");
      });
    },
    [ctrl],
  );
  reactHostPort.useEffect(() => {
    if (!ctrl) return;
    const bridge: NotePlayHostBridge = {
      runHostCommand: (command) => {
        if (command === "saveDownload") {
          void downloadFixture();
          return;
        }
        if (command === "loadRequest") loadInputRef.current?.click();
      },
    };
    ctrl.setHostBridge(bridge);
    return () => ctrl.setHostBridge(null);
  }, [ctrl, downloadFixture]);
  return <input ref={loadInputRef} type="file" accept=".json,.note.json,application/json" className="hidden" onChange={handleLoadFile} />;
}

function NotePlayPaneSurfaceHost({ node }: { readonly node: UiNoteHostSurfaceNode }): ReactElement {
  const ctrl = useNotePlayController();
  const doc = ctrl?.getDocument();
  if (!doc) {
    return (
      <>
        {node.view !== "navigator" ? <NotePlayFileBridge /> : null}
        <div className="p-double text-sm text-muted-foreground">No note document</div>
      </>
    );
  }
  const common = {
    document: doc,
    selectedIds: ctrl?.getSelectedIds() ?? [],
    hoveredId: ctrl?.getHoveredId() ?? null,
    kindHover: ctrl?.getHoveredKind() ?? null,
    activeTool: doc.activeTool,
    camera: doc.camera,
    onHover: (payload: import("@semio-tech/note-core").NoteHoverPayload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CANVAS }),
    onSelect: (ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }),
    onCommit: (document: typeof doc, selectBlockId?: string) => ctrl?.run("commitDocument", { document, selectBlockId }),
    onCameraChange: (camera: typeof doc.camera) => ctrl?.run("setCamera", { camera }),
    className: "h-full",
  };
  return (
    <>
      {node.view !== "navigator" ? <NotePlayFileBridge /> : null}
      {node.view === "navigator" ? <NoteCanvas {...common} viewMode="navigator" /> : <NoteCanvas {...common} viewMode="composite" />}
    </>
  );
}

class NotePlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: NOTE_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = notePlayControllerRef.current;
          const doc = ctrl?.getDocument();
          const bus = new CommandBus();
          if (!doc) return { sections: [{ id: "note-empty", items: [{ id: "empty", label: "No document" }] }] };
          const treeNode = buildNotePlayHierarchyTree(
            doc,
            ctrl?.getSelectedIds() ?? [],
            ctrl?.getHoveredId() ?? null,
            ctrl?.getHoveredKind() ?? null,
            (payload) => ctrl?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_HIERARCHY }),
          );
          const config = uiTreeNodeToTreePanelConfig(treeNode, bus);
          return {
            ...config,
            dragAndDropController: createNotePlayHierarchyTreeDragController(() => notePlayControllerRef.current ?? undefined),
          };
        },
        () => {
          const ctrl = notePlayControllerRef.current;
          const doc = ctrl?.getDocument();
          if (!doc) return [];
          return [...(buildNotePlayHierarchyTree(doc, [], ctrl?.getHoveredId() ?? null, ctrl?.getHoveredKind() ?? null).highlightedIds ?? [])];
        },
      ),
    };
  }
}

class NotePlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: NOTE_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        const treeNode = buildNotePlayCatalogueTree((payload) =>
          notePlayControllerRef.current?.run("setHover", { id: payload.id, kind: payload.kind, sourceId: CANVAS_HOVER_SOURCE_CATALOG }),
        );
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

class NotePlayPropertiesPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: NOTE_PLAY_PROPERTIES_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = notePlayControllerRef.current;
        const doc = ctrl?.getDocument();
        const bus = new CommandBus();
        if (!doc) return { sections: [{ id: "note-props-empty", items: [{ id: "empty", label: "No document" }] }] };
        const treeNode = buildNotePlayInspectorTree(doc, ctrl?.getSelectedIds() ?? []);
        return uiTreeNodeToTreePanelConfig(treeNode, bus);
      }),
    };
  }
}

function NoteOsInstanceHost({ instance }: { readonly instance: OsAppInstance }): ReactElement {
  const bridge = useOsInstanceHostBridge();
  const bundle = useOsInstanceMaterialization(instance);
  const materialized = bundle.projection;
  const noteDoc = reactHostPort.useMemo(() => {
    if (materialized && typeof materialized === "object" && (materialized as NoteDocument).schema === "note.document") return materialized as NoteDocument;
    return defaultNoteDocument(instance.id);
  }, [instance.id, materialized]);
  const dispatchNote = reactHostPort.useCallback(
    (document: NoteDocument) => {
      bridge.dispatch({ kind: "patchAppSource", instanceId: instance.id, inline: noteDocumentToJson(document) });
    },
    [bridge, instance.id],
  );
  return (
    <div className="flex h-full min-h-0 flex-col overflow-hidden">
      <OsUpstreamBadge upstreamInstanceId={bundle.upstreamInstanceId} />
      <NoteCanvas document={noteDoc} onCommit={(document) => dispatchNote(document)} className="min-h-0 flex-1" viewMode="composite" />
    </div>
  );
}

/** @emoji 🛝 Note app renderer contribution for playground and OS shells. */
export const noteAppRenderer: AppRendererContribution = {
  windowBodies: notePlayWindowBodies,
  surfaceHosts: {
    [NOTE_PLAY_SURFACE_ID_COMPOSITE]: NotePlayPaneSurfaceHost,
    [NOTE_PLAY_SURFACE_ID_NAVIGATOR]: NotePlayPaneSurfaceHost,
  },
  panelTabs: {
    workbench: [new NotePlayHierarchyPanelDefinition(), new NotePlayCataloguePanelDefinition()],
    details: [new NotePlayPropertiesPanelDefinition()],
  },
  instanceHost: NoteOsInstanceHost,
  examples: controllerBackedExampleContribution(NOTE_PLAY_CONTROLLER_ID, createNotePlayExampleHost().options),
};
//#endregion 🔖NotePlayHost
