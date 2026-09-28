#!/usr/bin/env python3
"""🗂️ F3 session 14c region (2) — panel bodies leave the shell reducer: `windowUi.panelUiByKey` (BuiltNode per panel)
becomes `panelBodyStoreByKey` (one stable `UiDocumentStore` per panel key, from the shell's BuiltNode store cache, key
`panel:<record key>`). A refresh publishes each body into its store OUTSIDE render (`publishBuiltNodesV1`); the record only
changes when a panel gains its store, so a body that changes on every keystroke (writer's Inspection) re-renders its own
mounted panel tree, never the shell. The ToolRun task feed subscribes to the ToolRun panel's store itself.
Idempotent: exits 0 when already applied. usage: python3 f3-panel-body-stores.py [--dry-run]
"""
import sys

ROOT = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
SHELL = f"{ROOT}/🧱️elements/🐚️Shell/🟦️.tsx"
HOST = f"{ROOT}/🧱️elements/🏛️ShellHost/🟦️.tsx"
HELPERS = f"{ROOT}/🧱️elements/🛠️ShellHelpers/🟦️.tsx"
CONTRACT = f"{ROOT}/🧪️tests/🔬️engine-contract/🟦️.ts"
DRY = "--dry-run" in sys.argv
files = {path: open(path, encoding="utf-8").read() for path in (SHELL, HOST, HELPERS, CONTRACT)}
if "panelBodyStoreByKey" in files[SHELL]:
    print("already applied")
    sys.exit(0)
problems = []


def rep(path: str, old: str, new: str, count: int = 1) -> None:
    found = files[path].count(old)
    if found != count:
        problems.append(f"{path.rsplit('/', 3)[-2]} {found}x (want {count}): {old[:110]!r}")
        return
    files[path] = files[path].replace(old, new)


# 🐚️ Shell reducer
rep(SHELL, """import type { WindowFault } from "../🏛️ShellHost/🩺️fault/🟦️.ts";
""", """import type { WindowFault } from "../🏛️ShellHost/🩺️fault/🟦️.ts";
import type { UiDocumentStore } from "../📃️UiDocumentStore/🟦️.tsx";
""")
rep(SHELL, """  readonly panelUiByKey: Readonly<Record<string, BuiltNode>>;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;""", """  /** 🗂️ One body store per panel key (a spawned program's under its prefixed key). The record moves only when a panel
   * gains its store; a body that changes is loaded into that store outside render and re-renders only the mounted
   * panel tree (ticket 26/09/23 F3: writer's Inspection body re-rendered the whole shell on every keystroke). */
  readonly panelBodyStoreByKey: Readonly<Record<string, UiDocumentStore>>;
  readonly appLabelsOverlay: PluginAppLabelsOverlay;""")
rep(SHELL, """  | { readonly type: "SET_PANEL_UI_BY_KEY"; readonly value: Updatable<Readonly<Record<string, BuiltNode>>> }""",
    """  | { readonly type: "SET_PANEL_BODY_STORE_BY_KEY"; readonly value: Updatable<Readonly<Record<string, UiDocumentStore>>> }""")
rep(SHELL, """    case "SET_PANEL_UI_BY_KEY":
      return withField(state, "panelUiByKey", resolveUpdatable(action.value, state.panelUiByKey));""",
    """    case "SET_PANEL_BODY_STORE_BY_KEY":
      return withField(state, "panelBodyStoreByKey", resolveUpdatable(action.value, state.panelBodyStoreByKey));""")
rep(SHELL, "toolMeasuresByToolId: {}, panelUiByKey: {}, appLabelsOverlay:", "toolMeasuresByToolId: {}, panelBodyStoreByKey: {}, appLabelsOverlay:")

# 🏛️ ShellHost — the store cache answers a store's node, and a reload sets the node before it notifies
rep(HOST, """  readonly storeOf: (key: string) => UiDocumentStore | null;
  readonly flushPendingReloads: () => void;""", """  readonly storeOf: (key: string) => UiDocumentStore | null;
  /** 🧾️ The authored node `store` currently holds — set before a reload notifies, so a subscriber reads the new one. */
  readonly nodeOf: (store: UiDocumentStore) => BuiltNode | null;
  readonly flushPendingReloads: () => void;""")
rep(HOST, """  const stores = new Map<string, { node: BuiltNode; revision: number; readonly store: UiDocumentStore }>();
  const pending = new Map<string, BuiltNode>();
  return {
    storeFor: (key, node) => {
      const existing = stores.get(key);
      if (!existing) {
        const store = new UiDocumentStore(key);
        store.loadSnapshot(builtNodeToSnapshot(key, node));
        stores.set(key, { node, revision: 0, store });
        return store;
      }""", """  const stores = new Map<string, { node: BuiltNode; revision: number; readonly store: UiDocumentStore }>();
  const byStore = new WeakMap<UiDocumentStore, { node: BuiltNode }>();
  const pending = new Map<string, BuiltNode>();
  return {
    storeFor: (key, node) => {
      const existing = stores.get(key);
      if (!existing) {
        const store = new UiDocumentStore(key);
        store.loadSnapshot(builtNodeToSnapshot(key, node));
        const entry = { node, revision: 0, store };
        stores.set(key, entry);
        byStore.set(store, entry);
        return store;
      }""")
rep(HOST, """    storeOf: (key) => stores.get(key)?.store ?? null,
    flushPendingReloads: () => {
      for (const [key, node] of pending) {
        const entry = stores.get(key);
        if (!entry) continue;
        entry.revision += 1;
        entry.store.loadSnapshot(builtNodeToSnapshot(key, node, entry.revision));
        entry.node = node;
      }""", """    storeOf: (key) => stores.get(key)?.store ?? null,
    nodeOf: (store) => byStore.get(store)?.node ?? null,
    flushPendingReloads: () => {
      for (const [key, node] of pending) {
        const entry = stores.get(key);
        if (!entry) continue;
        entry.revision += 1;
        entry.node = node;
        entry.store.loadSnapshot(builtNodeToSnapshot(key, node, entry.revision));
      }""")
rep(HOST, "const { windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId, toolMeasuresByToolId, panelUiByKey, appLabelsOverlay, appCatalogue } = shellState.windowUi;",
    "const { windowUiByWindowId, windowEngagementsByWindowId, windowMeasuresByWindowId, toolMeasuresByToolId, panelBodyStoreByKey, appLabelsOverlay, appCatalogue } = shellState.windowUi;")
rep(HOST, "it changes the guest body, which arrives as a new `panelUiByKey` entry the memos already watch. */",
    "it changes the guest body, which the panel's own body store reloads in place. */")
rep(HOST, "/** 🧬️ Per-window/panel `📃️UiDocumentStore`s, keyed the same way `windowUiByWindowId`/`panelUiByKey` already are",
    "/** 🧬️ Per-window/panel `📃️UiDocumentStore`s, keyed the way `windowUiByWindowId`/`panelBodyStoreByKey` are")
rep(HOST, "   * flattened into `buildUiRefreshRequest`) and the result was cached in `panelUiByKey` and then never",
    "   * flattened into `buildUiRefreshRequest`) and the result was cached as a panel body and then never")
rep(HOST, "  const focusedPanelUiByKey = useMemo(() => programEntriesV1(panelUiByKey, focusedSpawnedId, spawnedIds), [panelUiByKey, focusedSpawnedId, spawnedIds]);",
    """  const focusedPanelBodyStores = useMemo(() => programEntriesV1(panelBodyStoreByKey, focusedSpawnedId, spawnedIds), [panelBodyStoreByKey, focusedSpawnedId, spawnedIds]);
  /** ⏯️ The focused program's ToolRun panel body — the task feed and the reveal follow THIS store, not every panel body. */
  const toolRunPanelStore = focusedPanelBodyStores[FRAMEWORK_PANEL_TAB_TOOL_RUN_ID] ?? null;
  const subscribeToolRunPanel = useCallback((listener: () => void) => toolRunPanelStore?.subscribeRevision(listener) ?? (() => undefined), [toolRunPanelStore]);
  const toolRunPanelNode = useSyncExternalStore(subscribeToolRunPanel, () => (toolRunPanelStore === null ? null : builtNodeStoreCacheRef.current.nodeOf(toolRunPanelStore)), () => null) ?? undefined;""")
rep(HOST, """      dispatch({
        type: "SET_PANEL_UI_BY_KEY",
        value: (current) => {
          const entries = panelTabLeaves
            .filter((tab) => tab.bodyKey)
            .map((tab) => [panelTabKindId(tab.kind), (cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined) ?? current[panelTabKindId(tab.kind)] ?? pendingPanelUiNode()] as const);
          if (!replaceBodies) return mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, null, spawnedIdsRef.current, entries));
          const next: Record<string, BuiltNode> = { ...current };
          for (const [id, node] of entries) next[id] = node;
          return next;
        },
      });""", """      const panelStores = publishPanelBodiesV1(
        builtNodeStoreCacheRef.current,
        panelTabLeaves.filter((tab) => tab.bodyKey).map((tab) => [panelTabKindId(tab.kind), cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined] as const),
      );
      dispatch({ type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, null, spawnedIdsRef.current, panelStores)) });""")
rep(HOST, """      const panelBodies = programKeyedEntriesV1(
        spawned.id,
        Object.fromEntries(panelTabLeaves.filter((tab) => tab.bodyKey).map((tab) => [panelTabKindId(tab.kind), (cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined) ?? pendingPanelUiNode()] as const)),
      );
      dispatch({ type: "SET_PANEL_UI_BY_KEY", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, spawned.id, spawnedIdsRef.current, Object.entries(panelBodies))) });""",
    """      const panelBodies = programKeyedEntriesV1(
        spawned.id,
        Object.fromEntries(panelTabLeaves.filter((tab) => tab.bodyKey).map((tab) => [panelTabKindId(tab.kind), cache.get(`panel:${panelTabKindId(tab.kind)}`)?.value as BuiltNode | undefined] as const)),
      );
      const panelStores = publishPanelBodiesV1(builtNodeStoreCacheRef.current, Object.entries(panelBodies));
      dispatch({ type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, spawned.id, spawnedIdsRef.current, panelStores)) });""")
rep(HOST, """//#endregion 🔖️BuiltNodeStoreCache

/** ⌨️ How many animation frames""", """
/** 🗂️ Publishes panel bodies into their stores (`panel:<record key>`) outside render and answers each key's store: a body
 * the refresh did not re-serialize keeps its store's content, a panel with no store yet starts on the pending body. */
export function publishPanelBodiesV1(cache: BuiltNodeStoreCacheV1, bodies: readonly (readonly [string, BuiltNode | undefined])[]): (readonly [string, UiDocumentStore])[] {
  publishBuiltNodesV1(
    cache,
    bodies.flatMap(([key, node]) => (node !== undefined ? [[`panel:${key}`, node] as const] : cache.storeOf(`panel:${key}`) === null ? [[`panel:${key}`, pendingPanelUiNode()] as const] : [])),
  );
  return bodies.map(([key]) => [key, cache.storeOf(`panel:${key}`)!] as const);
}
//#endregion 🔖️BuiltNodeStoreCache

/** ⌨️ How many animation frames""")
rep(HOST, "    const runs = toolRunPanelTasksV1(focusedPanelUiByKey[FRAMEWORK_PANEL_TAB_TOOL_RUN_ID]);", "    const runs = toolRunPanelTasksV1(toolRunPanelNode);")
rep(HOST, "  }, [agentBridge.conversation, installingPluginIds, focusedPanelUiByKey, focusedProgram?.pluginId, focusedProgram?.spawnedId]);",
    "  }, [agentBridge.conversation, installingPluginIds, toolRunPanelNode, focusedProgram?.pluginId, focusedProgram?.spawnedId]);")
rep(HOST, "toolRunPanelReveal(toolRunPanelRunsRef.current, focusedPanelUiByKey[FRAMEWORK_PANEL_TAB_TOOL_RUN_ID]);", "toolRunPanelReveal(toolRunPanelRunsRef.current, toolRunPanelNode);")
rep(HOST, "  }, [focusedPanelUiByKey, dock]);", "  }, [toolRunPanelNode, dock]);")
count = files[HOST].count("focusedPanelUiByKey")
if count != 6:
    problems.append(f"ShellHost focusedPanelUiByKey sites: {count} (want 6: three builders + three deps)")
files[HOST] = files[HOST].replace("focusedPanelUiByKey", "focusedPanelBodyStores")

# 🛠️ ShellHelpers — a panel tab's tree hosts its body store
rep(HELPERS, """export function panelTabDefinitionToNode(
  tab: AppPanelTabDefinition,
  group: string,
  panelUiByKey: Readonly<Record<string, BuiltNode>>,""", """export function panelTabDefinitionToNode(
  tab: AppPanelTabDefinition,
  group: string,
  panelBodyStoreByKey: Readonly<Record<string, UiDocumentStore>>,""")
rep(HELPERS, "children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction,",
    "children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelBodyStoreByKey, onAction,")
rep(HELPERS, """        ? cachedTreePanelConfigV1(cache, tabId, panelUiByKey[tabId] ?? pendingPanelUiNodeV1(), tab.bodyKey ?? tabId, onAction, treeWindows)""",
    """        ? cachedBodyStoreTreePanelConfigV1(cache, tabId, panelBodyStoreByKey[tabId], tab.bodyKey ?? tabId, onAction, treeWindows)""")
rep(HELPERS, " * call, and the `panelUiByKey` `useMemo`s in `ShellHost` list the whole record as a dependency — so one",
    " * call, and the panel-body `useMemo`s in `ShellHost` list the whole record as a dependency — so one")
rep(HELPERS, """/** 🎭️ {@link cachedTreePanelConfigV1} for an actor-rendered panel: the tree hosts the actor's retained store itself,
 * so a patch updates the mounted panel in place and the config is rebuilt only when the store, the intent route or
 * the tree-window inputs move. */
function cachedActorTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, actorPanels: BrowserActorPanelHostV1, bodyKey: string, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  const store = actorPanels.stores.get(tabId);
  if (store === undefined) return cachedTreePanelConfigV1(cache, tabId, pendingPanelUiNodeV1(), bodyKey, actorPanels.onAction, treeWindows);
  const openSignature = treeWindows ? treeWindowOpenSignatureV1(treeWindows.openStatesFor(bodyKey)) : "";
  const entry = cache?.get(tabId);
  if (entry && entry.source === store && entry.onAction === actorPanels.onIntent && entry.bodyKey === bodyKey && entry.treeWindows === treeWindows && entry.openSignature === openSignature) return entry.config;
  const config = interpretedTreePanelConfigV1(store, tabId, actorPanels.onAction, (intent) => actorPanels.onIntent(tabId, intent), bodyKey, treeWindows);
  cache?.set(tabId, { source: store, onAction: actorPanels.onIntent, bodyKey, treeWindows, openSignature, config });
  return config;
}""", """/** 🗂️ A tree that hosts a retained body store itself — the shell's own panel body store or an actor's — so a reload or a
 * patch updates the mounted panel in place; the config is rebuilt only when the store, the intent `route` or the
 * tree-window inputs move. */
function cachedStoreTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, store: UiDocumentStore, bodyKey: string, treeWindows: TreeWindowHostV1 | null, route: unknown, build: () => TreePanelConfig): TreePanelConfig {
  const openSignature = treeWindows ? treeWindowOpenSignatureV1(treeWindows.openStatesFor(bodyKey)) : "";
  const entry = cache?.get(tabId);
  if (entry && entry.source === store && entry.onAction === route && entry.bodyKey === bodyKey && entry.treeWindows === treeWindows && entry.openSignature === openSignature) return entry.config;
  const config = build();
  cache?.set(tabId, { source: store, onAction: route, bodyKey, treeWindows, openSignature, config });
  return config;
}

/** 🗂️ A shell-rendered panel over its body store ({@link cachedStoreTreePanelConfigV1}); a tab whose body has no store yet
 * shows the pending body. */
function cachedBodyStoreTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, store: UiDocumentStore | undefined, bodyKey: string, onAction: (action: ActionDescriptor) => void, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  if (store === undefined) return cachedTreePanelConfigV1(cache, tabId, pendingPanelUiNodeV1(), bodyKey, onAction, treeWindows);
  return cachedStoreTreePanelConfigV1(cache, tabId, store, bodyKey, treeWindows, onAction, () => interpretedTreePanelConfigV1(store, tabId, onAction, (intent) => onAction(uiIntentToActionDescriptor(intent)), bodyKey, treeWindows));
}

/** 🎭️ An actor-rendered panel over the actor's retained store ({@link cachedStoreTreePanelConfigV1}). */
function cachedActorTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, actorPanels: BrowserActorPanelHostV1, bodyKey: string, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  const store = actorPanels.stores.get(tabId);
  if (store === undefined) return cachedTreePanelConfigV1(cache, tabId, pendingPanelUiNodeV1(), bodyKey, actorPanels.onAction, treeWindows);
  return cachedStoreTreePanelConfigV1(cache, tabId, store, bodyKey, treeWindows, actorPanels.onIntent, () => interpretedTreePanelConfigV1(store, tabId, actorPanels.onAction, (intent) => actorPanels.onIntent(tabId, intent), bodyKey, treeWindows));
}""")

# 🔬️ engine-contract laws
rep(CONTRACT, """    expect(shellReducer(state, { type: "SET_PANEL_UI_BY_KEY", value: (current) => current })).toBe(state);""",
    """    expect(shellReducer(state, { type: "SET_PANEL_BODY_STORE_BY_KEY", value: (current) => current })).toBe(state);""")
rep(CONTRACT, """    const node = panelTabDefinitionToNode(historyTab, "settings", { "framework.panel.history": historyUiNode }, () => {}, 1, emptyAppLabelsOverlay);""",
    """    const historyStore = new UiDocumentStore("panel:framework.panel.history");
    historyStore.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", historyUiNode));
    const node = panelTabDefinitionToNode(historyTab, "settings", { "framework.panel.history": historyStore }, () => {}, 1, emptyAppLabelsOverlay);""")
rep(CONTRACT, "    // cached in `panelUiByKey`, and then dropped before any dock node existed — unreachable in every anchor.",
    "    // cached as a panel body, and then dropped before any dock node existed — unreachable in every anchor.")

for path, text in files.items():
    for leftover in ("panelUiByKey", "SET_PANEL_UI_BY_KEY"):
        if leftover in text:
            problems.append(f"{path.rsplit('/', 3)[-2]} still names {leftover}")
if problems:
    print("PROBLEMS:\n" + "\n".join(problems))
    sys.exit(1)
if DRY:
    print("dry run clean")
    sys.exit(0)
for path, text in files.items():
    open(path, "w", encoding="utf-8").write(text)
print("applied")
