// #region 🧲Header
/** @emoji 🛝 Playground play host for Writer — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiWriterSurfaceHost, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig } from "@semio-tech/framework-playground-core";
import type { UiWriterHostSurfaceNode } from "@semio-tech/framework-platform-core";
import { createWorkerLspTransport as createWriterPlayWorkerLspTransport, createWriterDocument as createWriterPlayDocument } from "@semio-tech/writer-core";

import {
  WRITER_PLAY_APP_ID,
  WRITER_PLAY_CONTROLLER_ID,
  WRITER_PLAY_SURFACE_ID,
  WriterPlayController,
  buildWriterPlayCatalogueTree,
  buildWriterPlayHierarchyTree,
  buildWriterPlayInspectorTree,
  registerWriterPlayDeclarativeBodies,
} from "@semio-tech/writer-core";

let writerPlayChromeRegistered = false;
const writerPlayControllerRef: { current: WriterPlayController | null } = { current: null };

function useWriterPlayController(runtimeOverride?: Platform): WriterPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribe(listener) : () => {}),
    () => runtime?.generation ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as WriterPlayController | undefined;
  writerPlayControllerRef.current = ctrl ?? null;
  return ctrl;
}

function WriterPlaySurfaceHost({ node: _node }: { readonly node: UiWriterHostSurfaceNode }): ReactElement {
  const ctrl = useWriterPlayController();
  const document = ctrl?.getDocument() ?? createWriterPlayDocument({ id: "jack", languageId: "jack", text: "" });
  const formatSignal = ctrl?.getFormatSignal() ?? 0;
  const lintSignal = ctrl?.getLintSignal() ?? 0;
  const editorSelection = ctrl?.getEditorSelection();
  const editorSelectionSignal = ctrl?.getEditorSelectionSignal() ?? 0;
  const externalHoverRange = ctrl?.getHoveredAstSpan() ?? null;
  const externalHoverSignal = ctrl?.getExternalHoverSignal() ?? 0;
  const editorSettings = ctrl?.getEditorSettings();
  const jackLspWorkerRef = reactHostPort.useRef<typeof import("@semio-tech/trinity-react").createJackLspWorker | null>(null);
  const [jackLspReady, setJackLspReady] = reactHostPort.useState(false);
  reactHostPort.useEffect(() => {
    void import("@semio-tech/trinity-react").then(({ createJackLspWorker }) => {
      jackLspWorkerRef.current = createJackLspWorker;
      setJackLspReady(true);
    });
  }, []);
  const createLspTransport = reactHostPort.useCallback(() => {
    if (!jackLspWorkerRef.current) throw new Error("Jack LSP worker not loaded yet");
    return createWriterPlayWorkerLspTransport(jackLspWorkerRef.current());
  }, []);
  const onChange = reactHostPort.useCallback((next: import("@semio-tech/writer-core").WriterDocument) => {
    const ctrl = writerPlayControllerRef.current;
    if (!ctrl) return;
    const prev = ctrl.getDocument();
    if (prev.id === next.id && prev.languageId === next.languageId && prev.uri === next.uri && prev.schema === next.schema) {
      ctrl.run("setText", { text: next.text });
      return;
    }
    ctrl.run("setDocument", { document: next });
  }, []);
  const onLintMessages = reactHostPort.useCallback((messages: readonly string[]) => {
    writerPlayControllerRef.current?.setLintMessages(messages);
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { readonly start: number; readonly end: number }) => {
    writerPlayControllerRef.current?.run("setEditorSelection", range);
  }, []);
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    writerPlayControllerRef.current?.run("setEditorHover", { offset });
  }, []);
  if (!jackLspReady) {
    return <div className="h-full" />;
  }
  return (
    <WriterCanvas
      document={document}
      onChange={onChange}
      createLspTransport={createLspTransport}
      formatSignal={formatSignal}
      lintSignal={lintSignal}
      onLintMessages={onLintMessages}
      externalSelection={editorSelection}
      externalSelectionSignal={editorSelectionSignal}
      externalHoverRange={externalHoverRange}
      externalHoverSignal={externalHoverSignal}
      onSelectionChange={onSelectionChange}
      onHoverChange={onHoverChange}
      editorSettings={editorSettings}
      className="h-full"
    />
  );
}

export function registerWriterPlaySurfaceHosts(): void {
  if (writerPlayChromeRegistered) return;
  writerPlayChromeRegistered = true;
  registerUiWriterSurfaceHost(WRITER_PLAY_SURFACE_ID, WriterPlaySurfaceHost);
  registerWriterPlayDeclarativeBodies();
}

class WriterPlayHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "framework.panel.hierarchy",
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(
        () => {
          const ctrl = writerPlayControllerRef.current;
          const bus = new CommandBus();
          return uiTreeNodeToTreePanelConfig(
            buildWriterPlayHierarchyTree(
              ctrl?.getDocument() ?? createWriterPlayDocument({ id: "jack", languageId: "jack" }),
              ctrl?.getSelectedAstIds() ?? [],
              ctrl?.getHoveredAstId() ?? null,
              (id) => ctrl?.run("setAstHover", { id }),
            ),
            bus,
          );
        },
        () => {
          const ctrl = writerPlayControllerRef.current;
          const hovered = ctrl?.getHoveredAstId();
          return hovered ? [hovered] : [];
        },
      ),
    };
  }
}

class WriterPlayCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "framework.panel.catalogue",
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => uiTreeNodeToTreePanelConfig(buildWriterPlayCatalogueTree(), new CommandBus())),
    };
  }
}

class WriterPlayInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: "framework.panel.inspection",
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = writerPlayControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildWriterPlayInspectorTree(
            ctrl?.getDocument() ?? createWriterPlayDocument({ id: "jack", languageId: "jack" }),
            ctrl?.getLintMessages() ?? [],
          ),
          bus,
        );
      }),
    };
  }
}

function WriterPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useWriterPlayController(runtime);
  const hierarchy = reactHostPort.useMemo(() => new WriterPlayHierarchyPanelDefinition(), []);
  const catalogue = reactHostPort.useMemo(() => new WriterPlayCataloguePanelDefinition(), []);
  const inspection = reactHostPort.useMemo(() => new WriterPlayInspectionPanelDefinition(), []);
  return (
    <PlaygroundView runtime={runtime} defaultAppId={WRITER_PLAY_APP_ID} augmentPanelTabs={{ workbench: [hierarchy, catalogue], details: [inspection] }} />
  );
}

export function mountWriterPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<WriterPlayInner runtime={playground.runtime} />, rootId);
}

const writerPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerWriterPlaySurfaceHosts,
  mount: mountWriterPlayChrome,
};

export function bootWriterPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, writerPlayChromeBoot, rootId);
}
//#endregion 🔖WriterPlayHost