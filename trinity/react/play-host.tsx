// #region 🧲Header
/** @emoji 🛝 Playground play host for Trinity — loaded only via `./play` subpath. */
// #endregion 🧲Header

import type { ReactElement } from "react";
import { type Playground, type PlaygroundChromeBoot, bootPlayground, mountPlaygroundApp, PlaygroundView, PlaygroundContext, PureSidePanelTabDefinition, CallbackTreePanelDefinition, registerUiPuzzle2dSurfaceHost, registerUiTrinitySurfaceHost, registerUiTableSurfaceHost, registerUiFormsSurfaceHost, registerUiWriterSurfaceHost, Platform, CommandBus, FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL, FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, uiTreeNodeToTreePanelConfig } from "@semio-tech/framework-playground-renderer-react";
import { shellTabIconComponent } from "@semio-tech/framework-platform-renderer-react";
import { reactHostPort } from "@semio-tech/ui-react";
import { type SidePanelTabConfig, UiTableHostSurfaceNode, UiPuzzle2dHostSurfaceNode } from "@semio-tech/framework-playground-core";
import {
  TRINITY_JACK_PLAY_CONTROLLER_ID,
  TRINITY_JACK_PLAY_APP_ID,
  TRINITY_JACK_PLAY_CATALOGUE_TAB_ID,
  TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON,
  TRINITY_JACK_PLAY_DEFAULT_QUERY,
  TRINITY_JACK_PLAY_EDITOR_SURFACE_ID,
  TRINITY_JACK_PLAY_HIERARCHY_TAB_ID,
  TRINITY_JACK_PLAY_INSPECTION_TAB_ID,
  TRINITY_JACK_PLAY_RESULTS_SURFACE_ID,
  TRINITY_JACK_PLAY_SURFACE_ID,
  TRINITY_JACK_PLAY_WINDOW_KIND_ID,
  TrinityJackPlayController,
  buildTrinityJackPlayCatalogueTree,
  registerTrinityJackPlayDeclarativeBodies,
} from "@semio-tech/trinity-jack-host-core";

import {
  TRINITY_REWRITE_PLAY_CONTROLLER_ID,
  TRINITY_REWRITE_PLAY_APP_ID,
  TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER,
  TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE,
  TRINITY_REWRITE_PLAY_SURFACE_ID_JACK,
  TRINITY_REWRITE_PLAY_SURFACE_ID_LHS,
  TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS,
  TRINITY_REWRITE_PLAY_SURFACE_ID_RHS,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_LHS,
  TRINITY_REWRITE_PLAY_WINDOW_KIND_RHS,
  TrinityRewritePlayController,
  REWRITE_DEFAULT_LHS_FIXTURE,
  REWRITE_DEFAULT_LHS_FIXTURE_JSON,
  REWRITE_DEFAULT_RHS_FIXTURE,
  REWRITE_DEFAULT_RHS_FIXTURE_JSON,
  rewriteLhsKindCatalogs,
  rewriteRhsKindCatalogs,
  parseRewriteGraphFixtureJson,
  registerTrinityRewritePlayDeclarativeBodies,
} from "@semio-tech/trinity-rewrite-core";

import { createWorkerLspTransport as createTrinityWriterLspTransport, createWriterDocument as createTrinityWriterDocument } from "@semio-tech/writer-core";
import { WriterCanvas as TrinityWriterCanvas } from "@semio-tech/writer-react";

let trinityPlayChromeRegistered = false;
const trinityJackControllerRef: { current: TrinityJackPlayController | null } = { current: null };
const trinityRewriteControllerRef: { current: TrinityRewritePlayController | null } = { current: null };

function useTrinityJackController(runtimeOverride?: Platform): TrinityJackPlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as TrinityJackPlayController | undefined;
  trinityJackControllerRef.current = ctrl ?? null;
  return ctrl;
}

function useTrinityJackInteractionRevision(runtime?: Platform): number {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const resolved = runtime ?? appCtx?.runtime;
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = resolved?.getActiveApp()?.controller as TrinityJackPlayController | undefined;
      trinityJackControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = resolved ? resolved.subscribe(listener) : () => {};
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (resolved?.getActiveApp()?.controller as TrinityJackPlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useTrinityRewriteInteractionRevision(runtime?: Platform): number {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const resolved = runtime ?? appCtx?.runtime;
  return reactHostPort.useSyncExternalStore(
    (listener) => {
      const ctrl = resolved?.getActiveApp()?.controller as TrinityRewritePlayController | undefined;
      trinityRewriteControllerRef.current = ctrl ?? null;
      const unsubscribeRuntime = resolved ? resolved.subscribe(listener) : () => {};
      const unsubscribeSnapshot = ctrl?.subscribeSnapshot(listener);
      return () => {
        unsubscribeRuntime();
        unsubscribeSnapshot?.();
      };
    },
    () => (resolved?.getActiveApp()?.controller as TrinityRewritePlayController | undefined)?.getInteractionRevision() ?? 0,
    () => 0,
  );
}

function useTrinityRewriteController(runtimeOverride?: Platform): TrinityRewritePlayController | undefined {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const runtime = runtimeOverride ?? appCtx?.runtime;
  reactHostPort.useSyncExternalStore(
    (listener) => (runtime ? runtime.subscribeChrome(listener) : () => {}),
    () => runtime?.chromeGeneration ?? 0,
    () => 0,
  );
  const ctrl = runtime?.getActiveApp()?.controller as TrinityRewritePlayController | undefined;
  trinityRewriteControllerRef.current = ctrl ?? null;
  return ctrl;
}

function TrinityJackPlaySurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiTrinityHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityJackInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityJackController();
  const scopeId = node.paneId ?? TRINITY_JACK_PLAY_WINDOW_KIND_ID;
  const lodProps = trinityLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? TRINITY_LOD_MODE_AUTOMATIC);
  const onFixtureChange = reactHostPort.useCallback((json: string) => ctrl?.run("setFixtureJson", { json }), [ctrl]);
  const onJackDispatchComplete = reactHostPort.useCallback((resultJson: string) => ctrl?.onJackDispatchComplete(resultJson), [ctrl]);
  const onVcsApplied = reactHostPort.useCallback((generation: number) => ctrl?.onVcsApplied(generation), [ctrl]);
  const onSelectionChange = reactHostPort.useCallback((ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }), [ctrl]);
  const onLodChange = reactHostPort.useCallback(
    (lod: TrinityDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  void revision;
  return (
    <TrinityCanvas
      fixtureJson={ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      jackDispatch={ctrl?.getJackDispatch()}
      vcsRequest={ctrl?.getVcsRequest()}
      onFixtureChange={onFixtureChange}
      onJackDispatchComplete={onJackDispatchComplete}
      onVcsApplied={onVcsApplied}
      onSelectionChange={onSelectionChange}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function TrinityJackEditorSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityJackInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityJackController();
  void revision;
  const fixtureJson = ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON;
  const document = ctrl?.getWriterDocument() ?? createTrinityWriterDocument({ id: "jack-query", languageId: "jack", text: TRINITY_JACK_PLAY_DEFAULT_QUERY });
  const createLspTransport = reactHostPort.useCallback(() => createTrinityWriterLspTransport(createJackLspWorker(fixtureJson)), [fixtureJson]);
  const onChange = reactHostPort.useCallback((next: import("@semio-tech/writer-core").WriterDocument) => {
    trinityJackControllerRef.current?.run("setJackQuery", { value: next.text });
  }, []);
  const onSubmit = reactHostPort.useCallback(() => {
    trinityJackControllerRef.current?.run("runJackQuery");
  }, []);
  return (
    <TrinityWriterCanvas
      document={document}
      onChange={onChange}
      onSubmit={onSubmit}
      createLspTransport={createLspTransport}
      fixtureJsonForLsp={fixtureJson}
      placeholder={TRINITY_JACK_PLAY_DEFAULT_QUERY}
      className="h-full"
    />
  );
}

function TrinityJackResultsSurfaceHost({ node }: { readonly node: UiTableHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityJackInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityJackController();
  const result = reactHostPort.useMemo(() => {
    try {
      return JSON.parse(ctrl?.getJackResultJson() || '{"kind":"table","columns":[],"rows":[]}') as {
        kind?: "table" | "graph";
        columns: string[];
        rows: unknown[][];
        graphFixture?: import("@semio-tech/trinity-react").TrinityFixture;
      };
    } catch {
      return { kind: "table" as const, columns: ["error"], rows: [["Invalid result json"]] };
    }
  }, [ctrl, revision]);
  if (result.kind === "graph" && result.graphFixture) {
    return <TrinityCanvas fixtureJson={JSON.stringify(result.graphFixture)} className="h-full min-h-0" />;
  }
  return (
    <div className="h-full min-h-0 overflow-auto p-2">
      {result.columns.length === 0 ? (
        <div className="text-xs text-muted-foreground">Run a Jack query to see results.</div>
      ) : (
        <table className="w-full border-collapse text-xs">
          <thead>
            <tr>
              {result.columns.map((column) => (
                <th key={column} className="border-b border-border px-2 py-1 text-left font-medium text-muted-foreground">
                  {column}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            {result.rows.map((row, rowIndex) => (
              <tr key={rowIndex}>
                {row.map((cell, cellIndex) => (
                  <td key={cellIndex} className="border-b border-border px-2 py-1 font-mono text-foreground">
                    {cell == null ? "" : String(cell)}
                  </td>
                ))}
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
}

function TrinityRewriteBeforeSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiTrinityHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const scopeId = node.paneId ?? TRINITY_REWRITE_PLAY_WINDOW_KIND_BEFORE;
  const lodProps = trinityLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? TRINITY_LOD_MODE_AUTOMATIC);
  const onFixtureChange = reactHostPort.useCallback((json: string) => ctrl?.run("setFixtureJson", { json }), [ctrl]);
  const onJackDispatchComplete = reactHostPort.useCallback((resultJson: string) => ctrl?.onBeforeJackDispatchComplete(resultJson), [ctrl]);
  const onVcsApplied = reactHostPort.useCallback((generation: number) => ctrl?.onVcsApplied(generation), [ctrl]);
  const onSelectionChange = reactHostPort.useCallback((ids: readonly string[]) => ctrl?.run("setSelection", { ids: [...ids] }), [ctrl]);
  const onLodChange = reactHostPort.useCallback(
    (lod: TrinityDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <TrinityCanvas
      fixtureJson={ctrl?.getBeforeFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON}
      reorganize={ctrl?.getReorganize()}
      jackDispatch={ctrl?.getBeforeJackDispatch()}
      vcsRequest={ctrl?.getVcsRequest()}
      highlightedNodeIds={ctrl?.getBeforeHighlightedNodeIds()}
      highlightedNodeIdsSignal={ctrl?.getHoverEpoch() + ctrl?.getSelectEpoch()}
      onFixtureChange={onFixtureChange}
      onJackDispatchComplete={onJackDispatchComplete}
      onVcsApplied={onVcsApplied}
      onSelectionChange={onSelectionChange}
      {...lodProps}
      onLodChange={onLodChange}
    />
  );
}

function TrinityRewriteAfterSurfaceHost({ node }: { readonly node: import("@semio-tech/framework-platform-core").UiTrinityHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const scopeId = node.paneId ?? TRINITY_REWRITE_PLAY_WINDOW_KIND_AFTER;
  const lodProps = trinityLodCanvasProps(ctrl?.lodModeForScope(scopeId) ?? TRINITY_LOD_MODE_AUTOMATIC);
  const onLodChange = reactHostPort.useCallback(
    (lod: TrinityDrawLodKind) => {
      ctrl?.run("setEffectiveLod", { lod, instanceId: scopeId });
    },
    [ctrl, scopeId],
  );
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <TrinityCanvas
      fixtureJson={ctrl?.getAfterFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON}
      highlightedNodeIds={ctrl?.getAfterHighlightedNodeIds()}
      highlightedNodeIdsSignal={ctrl?.getHoverEpoch() + ctrl?.getSelectEpoch()}
      {...lodProps}
      onLodChange={onLodChange}
      className="h-full min-h-0"
    />
  );
}

function TrinityRewriteLhsSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiPuzzle2dHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const kindCatalogs = reactHostPort.useMemo(() => rewriteLhsKindCatalogs(), []);
  const fixture = reactHostPort.useMemo(() => {
    return parseRewriteGraphFixtureJson(ctrl?.getLhsFixtureJson() ?? REWRITE_DEFAULT_LHS_FIXTURE_JSON) ?? REWRITE_DEFAULT_LHS_FIXTURE;
  }, [ctrl, revision]);
  const declarativeSceneDescriptor = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(fixture), [fixture]);
  const onDragEnd = reactHostPort.useCallback(
    (payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
      if (!payload.moves.length) return;
      const current = parseRewriteGraphFixtureJson(trinityRewriteControllerRef.current?.getLhsFixtureJson() ?? REWRITE_DEFAULT_LHS_FIXTURE_JSON);
      if (!current) return;
      const byId = new Map(payload.moves.map((move) => [move.id, move]));
      trinityRewriteControllerRef.current?.run("setLhsFixtureJson", {
        json: JSON.stringify({
          ...current,
          nodes: current.nodes.map((entry) => {
            const move = byId.get(entry.id);
            return move ? { ...entry, x: move.x, y: move.y } : entry;
          }),
        }),
      });
    },
    [],
  );
  const onHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => {
    trinityRewriteControllerRef.current?.run("setLhsGraphHover", { id: payload.id });
  }, []);
  const onSelect = reactHostPort.useCallback((snapshot: { ids: readonly string[] }) => {
    trinityRewriteControllerRef.current?.run("setLhsGraphSelect", { ids: [...snapshot.ids] });
  }, []);
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <Puzzle2dCanvas
      declarativeSceneDescriptor={declarativeSceneDescriptor}
      camera={fixture.camera}
      kindCatalogs={kindCatalogs}
      fixtureDragDrop
      hoveredId={ctrl?.getLhsHoveredNodeId() ?? null}
      preselection={ctrl?.getLhsVarPreselection()}
      selection={ctrl?.getLhsVarSelection()}
      onDragEnd={onDragEnd}
      onHover={onHover}
      onSelect={onSelect}
      className="h-full min-h-0"
    />
  );
}

function TrinityRewriteRhsSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiPuzzle2dHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  const kindCatalogs = reactHostPort.useMemo(() => rewriteRhsKindCatalogs(), []);
  const fixture = reactHostPort.useMemo(() => {
    return parseRewriteGraphFixtureJson(ctrl?.getRhsFixtureJson() ?? REWRITE_DEFAULT_RHS_FIXTURE_JSON) ?? REWRITE_DEFAULT_RHS_FIXTURE;
  }, [ctrl, revision]);
  const declarativeSceneDescriptor = reactHostPort.useMemo(() => buildPuzzle2dSceneDescriptorFromFixture(fixture), [fixture]);
  const onDragEnd = reactHostPort.useCallback(
    (payload: { moves: Array<{ id: string; x: number; y: number }> }) => {
      if (!payload.moves.length) return;
      const current = parseRewriteGraphFixtureJson(trinityRewriteControllerRef.current?.getRhsFixtureJson() ?? REWRITE_DEFAULT_RHS_FIXTURE_JSON);
      if (!current) return;
      const byId = new Map(payload.moves.map((move) => [move.id, move]));
      trinityRewriteControllerRef.current?.run("setRhsFixtureJson", {
        json: JSON.stringify({
          ...current,
          nodes: current.nodes.map((entry) => {
            const move = byId.get(entry.id);
            return move ? { ...entry, x: move.x, y: move.y } : entry;
          }),
        }),
      });
    },
    [],
  );
  const onHover = reactHostPort.useCallback((payload: Puzzle2dHoverPayload) => {
    trinityRewriteControllerRef.current?.run("setRhsGraphHover", { id: payload.id });
  }, []);
  const onSelect = reactHostPort.useCallback((snapshot: { ids: readonly string[] }) => {
    trinityRewriteControllerRef.current?.run("setRhsGraphSelect", { ids: [...snapshot.ids] });
  }, []);
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  return (
    <Puzzle2dCanvas
      declarativeSceneDescriptor={declarativeSceneDescriptor}
      camera={fixture.camera}
      kindCatalogs={kindCatalogs}
      fixtureDragDrop
      hoveredId={ctrl?.getRhsHoveredNodeId() ?? null}
      preselection={ctrl?.getRhsVarPreselection()}
      selection={ctrl?.getRhsVarSelection()}
      onDragEnd={onDragEnd}
      onHover={onHover}
      onSelect={onSelect}
      className="h-full min-h-0"
    />
  );
}

function TrinityRewriteJackSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiWriterHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  void revision;
  void ctrl?.getHoverEpoch();
  void ctrl?.getSelectEpoch();
  const document = ctrl?.getWriterDocumentJack() ?? createTrinityWriterDocument({ id: "rewrite-jack", languageId: "jack", text: "" });
  const onHoverChange = reactHostPort.useCallback((offset: number | null) => {
    trinityRewriteControllerRef.current?.run("setJackHover", { offset });
  }, []);
  const onSelectionChange = reactHostPort.useCallback((range: { start: number; end: number }) => {
    trinityRewriteControllerRef.current?.run("setJackSelect", range);
  }, []);
  return (
    <TrinityWriterCanvas
      document={document}
      className="h-full"
      placeholder="Generated Jack query"
      onHoverChange={onHoverChange}
      onSelectionChange={onSelectionChange}
      externalHoverOccurrences={ctrl?.getJackHoverOccurrences()}
      externalHoverOccurrencesSignal={ctrl?.getHoverEpoch()}
      externalSelectionOccurrences={ctrl?.getJackSelectOccurrences()}
      externalSelectionOccurrencesSignal={ctrl?.getSelectEpoch()}
    />
  );
}

function TrinityRewriteParametersSurfaceHost({ node: _node }: { readonly node: import("@semio-tech/framework-platform-core").UiFormsHostSurfaceNode }): ReactElement {
  const appCtx = reactHostPort.useContext(PlaygroundContext);
  const revision = useTrinityRewriteInteractionRevision(appCtx?.runtime);
  const ctrl = useTrinityRewriteController();
  void revision;
  const spec = ctrl?.getParameterFormSpec();
  const values = ctrl?.getParameterValues() ?? {};
  if (!spec || spec.steps[0]?.questions.length === 0) {
    return <div className="p-double text-sm text-muted-foreground">No parameters declared on RHS.</div>;
  }
  return (
    <FormRenderer
      spec={spec}
      values={values}
      className="h-full"
      onChange={(next) => ctrl?.run("setParameterValues", { values: next })}
    />
  );
}

export function registerTrinityJackPlaySurfaceHosts(): void {
  if (trinityPlayChromeRegistered) return;
  trinityPlayChromeRegistered = true;
  registerUiTrinitySurfaceHost(TRINITY_JACK_PLAY_SURFACE_ID, TrinityJackPlaySurfaceHost);
  registerUiWriterSurfaceHost(TRINITY_JACK_PLAY_EDITOR_SURFACE_ID, TrinityJackEditorSurfaceHost);
  registerUiTableSurfaceHost(TRINITY_JACK_PLAY_RESULTS_SURFACE_ID, TrinityJackResultsSurfaceHost);
  registerTrinityJackPlayDeclarativeBodies();
}

export function registerTrinityRewritePlaySurfaceHosts(): void {
  registerUiTrinitySurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_BEFORE, TrinityRewriteBeforeSurfaceHost);
  registerUiTrinitySurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_AFTER, TrinityRewriteAfterSurfaceHost);
  registerUiPuzzle2dSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_LHS, TrinityRewriteLhsSurfaceHost);
  registerUiPuzzle2dSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_RHS, TrinityRewriteRhsSurfaceHost);
  registerUiWriterSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_JACK, TrinityRewriteJackSurfaceHost);
  registerUiFormsSurfaceHost(TRINITY_REWRITE_PLAY_SURFACE_ID_PARAMETERS, TrinityRewriteParametersSurfaceHost);
  registerTrinityRewritePlayDeclarativeBodies();
}

class TrinityJackHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityJackControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayHierarchyTree(ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []),
          bus,
        );
      }),
    };
  }
}

class TrinityJackCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityJackControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildTrinityJackPlayCatalogueTree(ctrl?.getActiveExampleId()), bus);
      }),
    };
  }
}

class TrinityJackInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityJackControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayInspectorTree(
            ctrl?.getFixtureJson() ?? TRINITY_JACK_PLAY_DEFAULT_FIXTURE_JSON,
            ctrl?.getSelectedNodeIds() ?? [],
            TRINITY_JACK_PLAY_CONTROLLER_ID,
          ),
          bus,
        );
      }),
    };
  }
}

class TrinityRewriteHierarchyPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_HIERARCHY_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_HIERARCHY_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_HIERARCHY_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityRewriteControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayHierarchyTree(ctrl?.getBeforeFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON, ctrl?.getSelectedNodeIds() ?? []),
          bus,
        );
      }),
    };
  }
}

class TrinityRewriteCataloguePanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_CATALOGUE_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_CATALOGUE_ICON_ID, "workbench"),
      name: FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
      order: 1,
      tree: new CallbackTreePanelDefinition(() => {
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(buildTrinityPlayCatalogueTree(), bus);
      }),
    };
  }
}

class TrinityRewriteInspectionPanelDefinition extends PureSidePanelTabDefinition {
  buildTab(): SidePanelTabConfig {
    return {
      id: TRINITY_JACK_PLAY_INSPECTION_TAB_ID,
      icon: shellTabIconComponent(FRAMEWORK_PANEL_TAB_INSPECTION_ICON_ID, "details"),
      name: FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
      order: 0,
      tree: new CallbackTreePanelDefinition(() => {
        const ctrl = trinityRewriteControllerRef.current;
        const bus = new CommandBus();
        return uiTreeNodeToTreePanelConfig(
          buildTrinityPlayInspectorTree(
            ctrl?.getBeforeFixtureJson() ?? TRINITY_DEFAULT_FIXTURE_JSON,
            ctrl?.getSelectedNodeIds() ?? [],
            TRINITY_REWRITE_PLAY_CONTROLLER_ID,
          ),
          bus,
        );
      }),
    };
  }
}

function TrinityJackPlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useTrinityJackController(runtime);
  const hierarchy = reactHostPort.useMemo(() => new TrinityJackHierarchyPanelDefinition(), []);
  const catalogue = reactHostPort.useMemo(() => new TrinityJackCataloguePanelDefinition(), []);
  const inspection = reactHostPort.useMemo(() => new TrinityJackInspectionPanelDefinition(), []);
  return (
    <PlaygroundView
      runtime={runtime}
      defaultAppId={TRINITY_JACK_PLAY_APP_ID}
      augmentPanelTabs={{ workbench: [hierarchy, catalogue], details: [inspection] }}
    />
  );
}

function TrinityRewritePlayInner({ runtime }: { readonly runtime: Platform }): ReactElement {
  useTrinityRewriteController(runtime);
  const hierarchy = reactHostPort.useMemo(() => new TrinityRewriteHierarchyPanelDefinition(), []);
  const catalogue = reactHostPort.useMemo(() => new TrinityRewriteCataloguePanelDefinition(), []);
  const inspection = reactHostPort.useMemo(() => new TrinityRewriteInspectionPanelDefinition(), []);
  return (
    <PlaygroundView
      runtime={runtime}
      defaultAppId={TRINITY_REWRITE_PLAY_APP_ID}
      augmentPanelTabs={{ workbench: [hierarchy, catalogue], details: [inspection] }}
    />
  );
}

export function mountTrinityJackPlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<TrinityJackPlayInner runtime={playground.runtime} />, rootId);
}

export function mountTrinityRewritePlayChrome(playground: Playground, rootId = "root"): void {
  mountPlaygroundApp(<TrinityRewritePlayInner runtime={playground.runtime} />, rootId);
}

const trinityJackPlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerTrinityJackPlaySurfaceHosts,
  mount: mountTrinityJackPlayChrome,
};

const trinityRewritePlayChromeBoot: PlaygroundChromeBoot = {
  registerHosts: registerTrinityRewritePlaySurfaceHosts,
  mount: mountTrinityRewritePlayChrome,
};

export function bootTrinityJackPlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, trinityJackPlayChromeBoot, rootId);
}

export function bootTrinityRewritePlay(playground: Playground, rootId = "root"): void {
  bootPlayground(playground, trinityRewritePlayChromeBoot, rootId);
}
//#endregion 🔖TrinityPlayHost