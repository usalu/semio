#!/usr/bin/env python3
"""🪟️ F3 (session 14b) — a spawned program's window BODIES live only in the shell's per-window UiDocumentStores.

The shell reducer kept every spawned body (`spawnedWindowUiByWindowId`), so a guest refresh that changed body CONTENT
only (a puzzle3d hover moves the selection lane's hash inside all three scene bodies) re-rendered FrameworkOsShellInner
and, through its unmemoized return tree, the whole shell: 2 shell renders / 69 ms of React render per hover transition
(`generated/f3-render-census-puzzle-before1.json`). After this patch the reducer holds only which windows have a body
and its activity; `refreshSpawnedUi` loads the bodies into `BuiltNodeStoreCacheV1` outside render and the mounted
`UiNodeView`s update in place.

Idempotent: every hunk is (old -> new); an already-applied hunk is skipped. `--dry-run` reports without writing.
usage: python3 f3-spawned-bodies-in-stores.py [--dry-run]
"""
import sys

REPO = "/Users/ueli/Documents/semio"
ELEMENTS = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
SHELL = f"{ELEMENTS}/🐚️Shell/🟦️.tsx"
HOST = f"{ELEMENTS}/🏛️ShellHost/🟦️.tsx"
CONTRACT = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts"

HUNKS = {
    SHELL: [
        (
            """  /** 🪟️ The focused spawned program's window bodies, keyed by the SHELL window instance id
   * (`${spawnedId}::${windowKindId}`) — one entry per window kind the spawned app declares, so a
   * spawned program projects every window it owns onto the canvas exactly as the session's own app
   * does. It was a single `BuiltNode` while a spawned app was allowed only its canvas body. */
  readonly spawnedWindowUiByWindowId: Readonly<Record<string, BuiltNode>>;
  /** 🩺️ Why `spawnedWindowUiByWindowId` went empty, when it went empty because the guest faulted""",
            """  /** 🪟️ Which of the focused spawned program's windows have a body, and that body's activity, keyed by the SHELL
   * window instance id (`${spawnedId}::${windowKindId}`) — one entry per window kind the spawned app declares. The
   * bodies themselves live only in the shell's per-window `📃️UiDocumentStore`s (`BuiltNodeStoreCacheV1`), loaded
   * outside render, so a refresh that changes body CONTENT alone updates the mounted `UiNodeView`s and never
   * re-renders the shell (ticket 26/09/23 F3: a puzzle3d hover re-rendered the whole shell twice, 69 ms per
   * transition). */
  readonly spawnedWindowActivityByWindowId: Readonly<Record<string, BuiltNode["activity"]>>;
  /** 🩺️ Why `spawnedWindowActivityByWindowId` went empty, when it went empty because the guest faulted""",
        ),
        (
            """  | { readonly type: "SET_SPAWNED_WINDOW_UI"; readonly value: Updatable<Readonly<Record<string, BuiltNode>>>; readonly fault?: WindowFault | null }""",
            """  | { readonly type: "SET_SPAWNED_WINDOW_ACTIVITY"; readonly value: Updatable<Readonly<Record<string, BuiltNode["activity"]>>>; readonly fault?: WindowFault | null }""",
        ),
        (
            """    case "SET_SPAWNED_WINDOW_UI": {
      const spawnedWindowUiByWindowId = resolveUpdatable(action.value, state.spawnedWindowUiByWindowId);
      const spawnedWindowFault = action.fault ?? null;
      return Object.is(spawnedWindowUiByWindowId, state.spawnedWindowUiByWindowId) && Object.is(spawnedWindowFault, state.spawnedWindowFault) ? state : { ...state, spawnedWindowUiByWindowId, spawnedWindowFault };
    }""",
            """    case "SET_SPAWNED_WINDOW_ACTIVITY": {
      const spawnedWindowActivityByWindowId = resolveUpdatable(action.value, state.spawnedWindowActivityByWindowId);
      const spawnedWindowFault = action.fault ?? null;
      return Object.is(spawnedWindowActivityByWindowId, state.spawnedWindowActivityByWindowId) && Object.is(spawnedWindowFault, state.spawnedWindowFault) ? state : { ...state, spawnedWindowActivityByWindowId, spawnedWindowFault };
    }""",
        ),
        (
            """    spawnedWindow: { spawnedWindowUiByWindowId: {}, spawnedWindowFault: null, spawnedWindowEngagements: {}, spawnedWindowMeasures: {} },""",
            """    spawnedWindow: { spawnedWindowActivityByWindowId: {}, spawnedWindowFault: null, spawnedWindowEngagements: {}, spawnedWindowMeasures: {} },""",
        ),
    ],
    HOST: [
        (
            """  const { spawnedWindowUiByWindowId, spawnedWindowFault, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;""",
            """  const { spawnedWindowActivityByWindowId, spawnedWindowFault, spawnedWindowEngagements, spawnedWindowMeasures } = shellState.spawnedWindow;""",
        ),
        (
            """export type BuiltNodeStoreCacheV1 = {
  readonly storeFor: (key: string, node: BuiltNode) => UiDocumentStore;""",
            """export type BuiltNodeStoreCacheV1 = {
  readonly storeFor: (key: string, node: BuiltNode) => UiDocumentStore;
  /** 🔎️ The store already minted for `key`, without offering a node — what a render reads for a body it does not own
   * (a spawned window's body is loaded outside render, {@link BuiltNodeStoreCacheV1.flushPendingReloads}). */
  readonly storeOf: (key: string) => UiDocumentStore | null;""",
        ),
        (
            """      if (existing.node === node) pending.delete(key);
      else pending.set(key, node);
      return existing.store;
    },
    flushPendingReloads: () => {""",
            """      if (existing.node === node) pending.delete(key);
      else pending.set(key, node);
      return existing.store;
    },
    storeOf: (key) => stores.get(key)?.store ?? null,
    flushPendingReloads: () => {""",
        ),
        (
            """  /** 🧬️ Per-window/panel `📃️UiDocumentStore`s, keyed the same way `windowUiByWindowId`/`panelUiByKey`/
   * `spawnedWindowUiByWindowId` already are — one stable store instance per key, reloaded via""",
            """  /** 🧬️ Per-window/panel `📃️UiDocumentStore`s, keyed the same way `windowUiByWindowId`/`panelUiByKey` already are
   * (a spawned window's store, `spawned:<windowId>`, is the ONLY holder of its body) — one stable store instance per key, reloaded via""",
        ),
        (
            """        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        dispatch({
          type: "SET_SPAWNED_WINDOW_UI",
          // 🦴 A `BuiltNode` needs its full field set (layout/style/accessibility/…) — reuse
          // `pendingWindowUiNode()`'s already-valid defaults rather than hand-authoring them, and
          // override just the component (a plain text node) and activity (no longer "loading").
          value: { [spawnedWindowInstanceIdV1(spawned.id, "main")]: { ...pendingWindowUiNode(), activity: "idle", component: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}`, emphasize: null, dataAttributes: null } } satisfies BuiltNode },
        });""",
            """        console.warn("[os-shell] refreshSpawnedUi: plugin/app unavailable", { pluginId: spawned.pluginId, appId: spawned.appId });
        const unavailableWindowId = spawnedWindowInstanceIdV1(spawned.id, "main");
        publishBuiltNodesV1(builtNodeStoreCacheRef.current, [[`spawned:${unavailableWindowId}`, { ...pendingWindowUiNode(), activity: "idle", component: { type: "text", value: `Plugin unavailable: ${spawned.pluginId}/${spawned.appId}`, emphasize: null, dataAttributes: null } } satisfies BuiltNode]]);
        dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: { [unavailableWindowId]: "idle" } });""",
        ),
        (
            """      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: (current: Readonly<Record<string, BuiltNode>>) => mergeRecordPreservingIdentity(current, Object.entries(uiByWindowId)) });""",
            """      publishBuiltNodesV1(builtNodeStoreCacheRef.current, Object.entries(uiByWindowId).map(([windowId, node]) => [`spawned:${windowId}`, node] as const));
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(uiByWindowId).map(([windowId, node]) => [windowId, node.activity] as const)) });""",
        ),
        (
            """    if (!hostMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: {} });""",
            """    if (!hostMode || !session) {
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {} });""",
        ),
        (
            """    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: {} });""",
            """    if (!activeSpawned) {
      dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {} });""",
        ),
        (
            """      if (fault !== null) dispatch({ type: "SET_SPAWNED_WINDOW_UI", value: {}, fault });""",
            """      if (fault !== null) dispatch({ type: "SET_SPAWNED_WINDOW_ACTIVITY", value: {}, fault });""",
        ),
        (
            """        const body = spawnedWindowUiByWindowId[windowId];""",
            """        const bodyStore = Object.hasOwn(spawnedWindowActivityByWindowId, windowId) ? builtNodeStoreCacheRef.current.storeOf(`spawned:${windowId}`) : null;""",
        ),
        (
            """          status: body?.activity,
          skeleton: <WindowBodySkeleton />,
          children: (
            <ChromeAwareWindowScrollSurface id={childElementId("framework.window", windowId)} className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={cursorFor(spawnedApp, windowId)}>""",
            """          status: spawnedWindowActivityByWindowId[windowId],
          skeleton: <WindowBodySkeleton />,
          children: (
            <ChromeAwareWindowScrollSurface id={childElementId("framework.window", windowId)} className="relative flex h-full min-h-0 min-w-0 flex-1 flex-col overflow-hidden" style={cursorFor(spawnedApp, windowId)}>""",
        ),
        (
            """                    {body ? <MediaTransportOwnerContext.Provider value={spawnedMediaOwner(windowId)}><InterpretedUiNode store={builtNodeStoreFor(`spawned:${windowId}`, body)} onAction={onActionStable} onIntent={onIntentStable} /></MediaTransportOwnerContext.Provider> : spawnedWindowFault""",
            """                    {bodyStore ? <MediaTransportOwnerContext.Provider value={spawnedMediaOwner(windowId)}><InterpretedUiNode store={bodyStore} onAction={onActionStable} onIntent={onIntentStable} /></MediaTransportOwnerContext.Provider> : spawnedWindowFault""",
        ),
        (
            """    spawnedWindowMeasures,
    spawnedWindowUiByWindowId,
    spawnedWindowFault,""",
            """    spawnedWindowMeasures,
    spawnedWindowActivityByWindowId,
    spawnedWindowFault,""",
        ),
        (
            """    pendingReloadKeys: () => [...pending.keys()],
  };
}
//#endregion 🔖️BuiltNodeStoreCache""",
            """    pendingReloadKeys: () => [...pending.keys()],
  };
}

/** 📤️ Loads bodies into their stores OUTSIDE render and reloads them at once. For a body no render owns — a spawned
 * program's window, `spawned:<windowId>` — the store is its only holder: the mounted `UiNodeView`s update in place and
 * only a window gaining or losing its body, or a changed activity, reaches the shell reducer. */
export function publishBuiltNodesV1(cache: BuiltNodeStoreCacheV1, entries: readonly (readonly [string, BuiltNode])[]): void {
  for (const [key, node] of entries) cache.storeFor(key, node);
  cache.flushPendingReloads();
}
//#endregion 🔖️BuiltNodeStoreCache""",
        ),
    ],
    CONTRACT: [
        (
            """  createBuiltNodeStoreCacheV1,
  type SpaceArtifactCreationCatalogAuthorityV1,""",
            """  createBuiltNodeStoreCacheV1,
  publishBuiltNodesV1,
  type SpaceArtifactCreationCatalogAuthorityV1,""",
        ),
        (
            """  it("forceReload queues loadSnapshot even when the node identity did not change", () => {""",
            """  it("publishes a body no render owns straight into its store: a new key loads, a changed body reloads and notifies at once, an unchanged one is a no-op", () => {
    const cache = createBuiltNodeStoreCacheV1();
    expect(cache.storeOf("spawned:puzzle-2::puzzle3d-main")).toBeNull();
    const first = contractNode("first");
    publishBuiltNodesV1(cache, [["spawned:puzzle-2::puzzle3d-main", first]]);
    const store = cache.storeOf("spawned:puzzle-2::puzzle3d-main");
    expect(store).not.toBeNull();
    const textOf = () => (store!.getNodeSnapshot(store!.getState().root ?? 0)?.component as { readonly value?: string } | undefined)?.value;
    expect(textOf()).toBe("first");
    let notifications = 0;
    const unsubscribe = store!.subscribeNode(store!.getState().root ?? 0)(() => {
      notifications += 1;
    });
    try {
      publishBuiltNodesV1(cache, [["spawned:puzzle-2::puzzle3d-main", first]]);
      expect(notifications).toBe(0);
      const revisionBefore = store!.getRevisionSnapshot();
      publishBuiltNodesV1(cache, [["spawned:puzzle-2::puzzle3d-main", contractNode("hovered")]]);
      expect(cache.storeOf("spawned:puzzle-2::puzzle3d-main")).toBe(store);
      expect(textOf()).toBe("hovered");
      expect(notifications).toBeGreaterThan(0);
      expect(store!.getRevisionSnapshot()).toBeGreaterThan(revisionBefore);
      expect(cache.pendingReloadKeys()).toEqual([]);
    } finally {
      unsubscribe();
    }
  });

  it("forceReload queues loadSnapshot even when the node identity did not change", () => {""",
        ),
        (
            """    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_UI", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_UI", value: (current) => current, fault: null })).toBe(state);""",
            """    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => current })).toBe(state);
    expect(shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => current, fault: null })).toBe(state);
    const opened = shellReducer(state, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: { "puzzle-2::puzzle3d-main": "idle" } });
    expect(opened.spawnedWindow.spawnedWindowActivityByWindowId).toEqual({ "puzzle-2::puzzle3d-main": "idle" });
    expect(shellReducer(opened, { type: "SET_SPAWNED_WINDOW_ACTIVITY", value: (current) => mergeRecordPreservingIdentity(current, [["puzzle-2::puzzle3d-main", "idle"]]) })).toBe(opened);""",
        ),
    ],
}


def main() -> int:
    dry = "--dry-run" in sys.argv
    failures = 0
    outputs: dict[str, tuple[str, str]] = {}
    for path, hunks in HUNKS.items():
        text = open(path, encoding="utf-8").read()
        changed = text
        for index, (old, new) in enumerate(hunks):
            name = f"{path.rsplit('/', 2)[-2]} #{index}"
            if changed.count(new) == 1 and changed.count(old) == 0:
                print(f"skip   {name} (applied)")
                continue
            if changed.count(old) != 1:
                print(f"FAIL   {name}: anchor found {changed.count(old)}x")
                failures += 1
                continue
            changed = changed.replace(old, new)
            print(f"apply  {name}")
        outputs[path] = (text, changed)
    if failures:
        print(f"{failures} hunk(s) failed — nothing written" if not dry else f"{failures} hunk(s) would fail")
        return 1
    if not dry:
        for path, (text, changed) in outputs.items():
            if changed != text:
                open(path, "w", encoding="utf-8").write(changed)
    print("dry run clean" if dry else "written")
    return 0


if __name__ == "__main__":
    sys.exit(main())
