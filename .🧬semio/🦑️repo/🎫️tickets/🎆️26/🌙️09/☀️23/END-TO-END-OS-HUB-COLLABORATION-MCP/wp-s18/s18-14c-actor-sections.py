"""🧩️ S18 §14c (C13 P1): an actor-bound hub document's engagements, measures, tool measures and catalogue come from the
browser actor's reserved section surfaces — worker visible set (`🪟️visible-surfaces`), shared section decoder
(PluginRuntime), section stores + request strip (ShellHelpers), ShellHost dispatch — applied as one set.
usage: python3 s18-14c-actor-sections.py [--dry-run]"""
import sys
from pathlib import Path

OS = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os")
ELEMENTS = OS / "🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements"
WORKER = OS / "🔨️modules/🏪️store/👷️worker/🟦️.ts"
RUNTIME = ELEMENTS / "🔌️PluginRuntime/🟦️.tsx"
HELPERS = ELEMENTS / "🛠️ShellHelpers/🟦️.tsx"
HOST = ELEMENTS / "🏛️ShellHost/🟦️.tsx"
CONFIG = OS / "🧪️tests/🎚️config/🟦️.ts"
DRY = "--dry-run" in sys.argv

edits: dict[Path, str] = {}


def swap(path: Path, old: str, new: str) -> None:
    text = edits.get(path) or path.read_text(encoding="utf-8")
    if text.count(old) != 1:
        raise SystemExit(f"anchor count {text.count(old)} in {path.name}: {old[:90]!r}")
    edits[path] = text.replace(old, new)


swap(WORKER, 'import { panelTabKindId, panelViewContext, windowViewContext, type PanelTabKind, type ResolvedPluginViewState } from "../../../../../🔨️modules/🛂️manifest/🟦️.ts";',
     'import { panelTabKindId, panelViewContext, sectionViewContext, windowViewContext, type PanelTabKind, type ResolvedPluginViewState } from "../../../../../🔨️modules/🛂️manifest/🟦️.ts";\nimport { BROWSER_ACTOR_VISIBLE_SURFACES_V1, browserActorVisibleSurfacesV1, type BrowserActorVisibleTurnV1 } from "./🪟️visible-surfaces/🟦️.ts";')
swap(WORKER, "    this.surfaceKeys = new Set([...this.windowKeys, ...lease.renderSurfaces().panels.map(({ key }) => key)]);",
     "    this.surfaceKeys = new Set([...this.windowKeys, ...lease.renderSurfaces().panels.map(({ key }) => key), ...BROWSER_ACTOR_VISIBLE_SURFACES_V1.sections.map(({ bodyKey }) => bodyKey)]);")
swap(WORKER, '''  /** 🖼️ Makes every verified window (each in its own window context) and every verified panel body (panel context)
   * visible to the child, exactly as the local refresh binds them, and reconciles the turn's patches until the render
   * settles. Rendering the panels here is what keeps an actor-bound document's inspector live (G-P1-4); rendering every
   * window is what lets a second window of the document show and command the live document. */''',
     '''  /** 🖼️ Makes every verified window (each in its own window context), every verified panel body (panel context) and every
   * reserved refresh section (section context; the static catalogue on the lifetime's first paint only) visible to the
   * child, exactly as the local refresh binds them, and reconciles the turn's patches until the render settles. Rendering
   * the panels here is what keeps an actor-bound document's inspector live (G-P1-4); rendering every window is what lets a
   * second window of the document show and command the live document; rendering the sections is what keeps its window
   * chrome and tool measures live (C13 P1). */''')
swap(WORKER, "    const visible: BrowserActorChildValue[] = [...this.windowVisibleEvents(lifetime, hostView), ...this.panelVisibleEvents(lifetime, hostView)];",
     '    const visible = this.visibleEvents(lifetime, hostView, this.renderedViewState === null ? "mount" : "repaint");')
swap(WORKER, '''  /** 🪟️ One `surface-visible` per verified window, each in its own window context of `hostView`. */
  private windowVisibleEvents(lifetime: ActorInstanceLifetime, hostView: ResolvedPluginViewState): BrowserActorChildValue[] {
    return this.lease.renderSurfaces().windows.map((window): BrowserActorChildValue => {
      const viewState = windowViewContext(hostView, window.key);
      if (!viewState || viewState.activeWindowKindId !== window.key) throw new Error("document browser actor: unknown host window instance");
      return { tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface: window.key }, bodyKey: window.bodyKey, viewState: encodePackValue(viewState) } };
    });
  }

  /** 🗂️ One `surface-visible` per verified panel body, each in the panel context of `hostView` (a fresh encoding per
   * event: one buffer shared by two events is a `value alias` the child boundary refuses). */
  private panelVisibleEvents(lifetime: ActorInstanceLifetime, hostView: ResolvedPluginViewState): BrowserActorChildValue[] {
    const panelView = panelViewContext(hostView);
    return this.lease.renderSurfaces().panels.map((panel) => ({ tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface: panel.key }, bodyKey: panel.bodyKey, viewState: encodePackValue(panelView) } }));
  }
''', '''  /** 🪟️ One `surface-visible` per surface `turn` announces (`🪟️visible-surfaces`): a window in its own window context of
   * `hostView`, a panel in its panel context, a reserved refresh section in its section context — a fresh encoding per
   * event (one buffer shared by two events is a `value alias` the child boundary refuses). */
  private visibleEvents(lifetime: ActorInstanceLifetime, hostView: ResolvedPluginViewState, turn: BrowserActorVisibleTurnV1): BrowserActorChildValue[] {
    return browserActorVisibleSurfacesV1(this.lease.renderSurfaces(), turn).map(({ surface, bodyKey, context }): BrowserActorChildValue => {
      if (context === "window") {
        const viewState = windowViewContext(hostView, surface);
        if (!viewState || viewState.activeWindowKindId !== surface) throw new Error("document browser actor: unknown host window instance");
        return { tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface }, bodyKey, viewState: encodePackValue(viewState) } };
      }
      return { tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface }, bodyKey, viewState: encodePackValue(context === "panel" ? panelViewContext(hostView) : sectionViewContext(hostView)) } };
    });
  }
''')
swap(WORKER, '''  /** 🗂️ Re-projects the document's rendered surfaces after a turn that may have changed the document — the actor lane's
   * twin of the Shell's refresh, which re-takes every window and panel after each local action. Every verified panel
   * body always (on a document change the guest re-renders only its mounted WINDOW: without this an open inspector
   * stayed at its last projection, ticket 26/09/23 C10 run `c10gp14e`); every verified window too when `windows` —
   * after an app command or a mutating action, whose turn does not re-render the author's own window: the author saw
   * its own edit only on its next action (~20 s later, C11 `c11self4`) while the peer saw it at once from the relay.
   * Unchanged surfaces answer no patch. */''', '''  /** 🗂️ Re-projects the document's rendered surfaces after a turn that may have changed the document — the actor lane's
   * twin of the Shell's refresh, which re-takes every window, panel and section after each local action. Every verified
   * panel body and every live section always (on a document change the guest re-renders only its mounted WINDOW: without
   * this an open inspector stayed at its last projection, ticket 26/09/23 C10 run `c10gp14e`, and draw's "N layers"
   * engagement stayed at the opening's count, C13 P1); every verified window too when `windows` — after an app command or
   * a mutating action, whose turn does not re-render the author's own window: the author saw its own edit only on its next
   * action (~20 s later, C11 `c11self4`) while the peer saw it at once from the relay. Unchanged surfaces answer no patch. */''')
swap(WORKER, "    const events = [...(windows ? this.windowVisibleEvents(lifetime, hostView) : []), ...this.panelVisibleEvents(lifetime, hostView)];\n    if (events.length === 0) return;",
     '    const events = this.visibleEvents(lifetime, hostView, windows ? "repaint" : "refresh");')

swap(RUNTIME, '''      retryable: false,
    });
  }
}

/** 🪟️ The ONE host→guest view-context gate''', '''      retryable: false,
    });
  }
}

/** 🧩️ One reserved section's value from its retained document (`undefined` while it has no root) — the one decoder the
 * local refresh and an actor-bound document's retained section stores share (C13 P1). */
export function retainedSectionValueV1(bodyKey: string, surface: RetainedSurface, producer: string): unknown {
  const built = retainedSurfaceToBuiltNode(surface);
  return built === null ? undefined : sectionValueFromBuiltNode(bodyKey, built, producer);
}

/** 🪟️ The ONE host→guest view-context gate''')
swap(RUNTIME, '''    const built = retainedSurfaceToBuiltNode(surface);
    if (built) sections[section.key] = { key: section.key, hash, value: sectionValueFromBuiltNode(section.bodyKey, built, `instance ${instanceId}`) };''',
     '''    const value = retainedSectionValueV1(section.bodyKey, surface, `instance ${instanceId}`);
    if (value !== undefined) sections[section.key] = { key: section.key, hash, value };''')

swap(HELPERS, "    type PluginUiRefreshSectionResponse,\n", "    type PluginUiRefreshSectionResponse,\n    type UiRefreshSectionKey,\n    UI_REFRESH_SECTIONS,\n")
swap(HELPERS, 'import { loadPluginModule, pluginLoadProgressAt, pluginLoadRemainingMs, PLUGIN_LOAD_IDLE_TIMEOUT_MS, type PluginWasmHandle } from "../🔌️PluginRuntime/🟦️.tsx";',
     'import { loadPluginModule, pluginLoadProgressAt, pluginLoadRemainingMs, PLUGIN_LOAD_IDLE_TIMEOUT_MS, retainedSectionValueV1, type PluginWasmHandle } from "../🔌️PluginRuntime/🟦️.tsx";')
swap(HELPERS, '''export type BrowserActorUiStoresV1 = Readonly<{ windows: Map<string, UiDocumentStore>; panels: Map<string, UiDocumentStore> }>;

/** 🩹️ Applies one browser-actor patch offer surface by surface and answers one verdict per patch, in offer order
 * — the guest acknowledges and resends per surface (`patch-ack` / `patch-rejected`), so one stale panel never
 * costs a window its frame. Every window kind of the app and every bodied panel is a surface: an actor-bound document
 * renders all of its windows from its one actor. A patch for a surface this app does not render is refused
 * `unknown-surface`; a patch that does not apply resets its store to the empty document (see `UiDocumentStore.reset`),
 * which is what the guest's full resend assumes. The FIRST offer of an opening must paint a window, or nothing is
 * retained and every surface is refused `window-surface-unpainted` so the guest resends them all. */
export function applyBrowserActorUiPatchesV1(
  patches: readonly UiPatch[],
  windowKeys: ReadonlySet<string>,
  panelKeys: ReadonlySet<string>,
  retained: BrowserActorUiStoresV1 | null,
): Readonly<{ verdicts: readonly BrowserActorUiPatchVerdictV1[]; stores: BrowserActorUiStoresV1 | null; surfacesAdded: boolean }> {
  const windows = retained?.windows ?? new Map<string, UiDocumentStore>(),
    panels = retained?.panels ?? new Map<string, UiDocumentStore>();
  let surfacesAdded = false;
  const verdicts = patches.map((patch): BrowserActorUiPatchVerdictV1 => {
    const owner = windowKeys.has(patch.surface) ? windows : panelKeys.has(patch.surface) ? panels : null;''', '''export type BrowserActorUiStoresV1 = Readonly<{ windows: Map<string, UiDocumentStore>; panels: Map<string, UiDocumentStore>; sections: Map<string, UiDocumentStore> }>;

/** 🧩️ The reserved refresh-section surfaces an actor renders beside its windows and panels, each keyed by its reserved body
 * key (`framework.section.*`) — the local refresh's own section binding (`UI_REFRESH_SECTIONS`). */
export const BROWSER_ACTOR_SECTION_KEYS: ReadonlySet<string> = new Set(UI_REFRESH_SECTIONS.map(({ bodyKey }) => bodyKey));

/** 🩹️ Applies one browser-actor patch offer surface by surface and answers one verdict per patch, in offer order
 * — the guest acknowledges and resends per surface (`patch-ack` / `patch-rejected`), so one stale panel never
 * costs a window its frame. Every window kind of the app, every bodied panel and every reserved refresh section is a
 * surface: an actor-bound document renders all of its windows and their chrome from its one actor. A patch for a
 * surface this app does not render is refused `unknown-surface`; a patch that does not apply resets its store to the
 * empty document (see `UiDocumentStore.reset`), which is what the guest's full resend assumes. The FIRST offer of an
 * opening must paint a window, or nothing is retained and every surface is refused `window-surface-unpainted` so the
 * guest resends them all. `sectionsChanged` says an acknowledged patch moved a section the shell must re-dispatch. */
export function applyBrowserActorUiPatchesV1(
  patches: readonly UiPatch[],
  windowKeys: ReadonlySet<string>,
  panelKeys: ReadonlySet<string>,
  retained: BrowserActorUiStoresV1 | null,
): Readonly<{ verdicts: readonly BrowserActorUiPatchVerdictV1[]; stores: BrowserActorUiStoresV1 | null; surfacesAdded: boolean; sectionsChanged: boolean }> {
  const windows = retained?.windows ?? new Map<string, UiDocumentStore>(),
    panels = retained?.panels ?? new Map<string, UiDocumentStore>(),
    sections = retained?.sections ?? new Map<string, UiDocumentStore>();
  let surfacesAdded = false;
  const verdicts = patches.map((patch): BrowserActorUiPatchVerdictV1 => {
    const owner = windowKeys.has(patch.surface) ? windows : panelKeys.has(patch.surface) ? panels : BROWSER_ACTOR_SECTION_KEYS.has(patch.surface) ? sections : null;''')
swap(HELPERS, '''  if (retained !== null || verdicts.some((verdict) => windowKeys.has(verdict.surface) && verdict.outcome === "acknowledged")) return { verdicts, stores: { windows, panels }, surfacesAdded };
  return {
    verdicts: verdicts.map((verdict): BrowserActorUiPatchVerdictV1 => (verdict.outcome === "acknowledged" ? { surface: verdict.surface, outcome: "rejected", revision: 0, reason: "window-surface-unpainted" } : verdict)),
    stores: null,
    surfacesAdded: false,
  };
}
''', '''  if (retained !== null || verdicts.some((verdict) => windowKeys.has(verdict.surface) && verdict.outcome === "acknowledged"))
    return { verdicts, stores: { windows, panels, sections }, surfacesAdded, sectionsChanged: verdicts.some((verdict) => BROWSER_ACTOR_SECTION_KEYS.has(verdict.surface) && verdict.outcome === "acknowledged") };
  return {
    verdicts: verdicts.map((verdict): BrowserActorUiPatchVerdictV1 => (verdict.outcome === "acknowledged" ? { surface: verdict.surface, outcome: "rejected", revision: 0, reason: "window-surface-unpainted" } : verdict)),
    stores: null,
    surfacesAdded: false,
    sectionsChanged: false,
  };
}

/** 🧩️ The reserved refresh sections an actor-bound document's section stores hold now, keyed like the local refresh's
 * cache (`engagements`/`measures`/`tools`/`catalogue`), each with the store revision it was read at; a section whose store
 * has no root is absent. Decoded by the local refresh's own `retainedSectionValueV1`, so both paths read one carrier. */
export function browserActorSectionValuesV1(sections: ReadonlyMap<string, UiDocumentStore>, producer: string): ReadonlyMap<UiRefreshSectionKey, Readonly<{ revision: number; value: unknown }>> {
  const values = new Map<UiRefreshSectionKey, Readonly<{ revision: number; value: unknown }>>();
  for (const section of UI_REFRESH_SECTIONS) {
    const store = sections.get(section.bodyKey);
    if (store === undefined) continue;
    const value = retainedSectionValueV1(section.bodyKey, store.getState(), producer);
    if (value !== undefined) values.set(section.key, { revision: store.getRevisionSnapshot(), value });
  }
  return values;
}

/** 🧩️ `request` without its reserved sections — for a session an actor serves, whose engagements, measures, tool measures
 * and catalogue come from the actor's section surfaces (its local instance never sees the hub document); `null` when
 * nothing is left to fetch. */
export function withoutUiRefreshSectionsV1(request: PluginUiRefreshRequest | null): PluginUiRefreshRequest | null {
  if (request === null) return null;
  const stripped: PluginUiRefreshRequest = { ...request, engagements: undefined, measures: undefined, tools: undefined, catalogue: undefined };
  return (stripped.windows ?? []).length === 0 && (stripped.panels ?? []).length === 0 && stripped.labels === undefined ? null : stripped;
}
''')

swap(HOST, "  applyBrowserActorUiPatchesV1,\n  browserActorPanelKeysV1,\n", "  applyBrowserActorUiPatchesV1,\n  browserActorPanelKeysV1,\n  browserActorSectionValuesV1,\n  withoutUiRefreshSectionsV1,\n")
swap(HOST, "readonly sessionInstanceId: number; readonly windows: Map<string, UiDocumentStore>; readonly panels: Map<string, UiDocumentStore>; readonly identity: BrowserActorUiMountedV1 | null;",
     "readonly sessionInstanceId: number; readonly windows: Map<string, UiDocumentStore>; readonly panels: Map<string, UiDocumentStore>; readonly sections: Map<string, UiDocumentStore>; readonly identity: BrowserActorUiMountedV1 | null;")
swap(HOST, '''  const [browserActorUiVersion, setBrowserActorUiVersion] = useState(0);
''', '''  const [browserActorUiVersion, setBrowserActorUiVersion] = useState(0);
  /** 🎭️ The painted actor serving `session`'s document, if any: its reserved section stores are then the ONLY source of the
   * session's engagements, measures, tool measures and catalogue — the local instance never sees a hub document, so its
   * sections froze at the opening state (C13 P1: draw's "N layers" stayed 1 after committed `addLayer`s). */
  const browserActorForSessionV1 = (session: Pick<ActiveSession, "pluginId" | "instanceId">): RetainedBrowserActorUiV1 | null => {
    for (const [runtimeKey, retained] of browserActorUiByRuntimeKeyRef.current) {
      const entry = openDocumentSessionsRef.current.get(runtimeKey);
      if (entry?.clientInstanceId === retained.clientInstanceId && retained.sessionInstanceId === session.instanceId && entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId) return retained;
    }
    return null;
  };
  /** 🧩️ Dispatches `cache`'s reserved sections — after taking them from `retained`'s section stores when an actor serves the
   * session — exactly as a local refresh does, so window chrome and tool measures follow the live document in every open
   * window. The app-static catalogue is kept by identity, so a scene host subscribing through `AppCatalogueContext` never
   * re-renders on an unchanged one. */
  const publishUiRefreshSectionsV1 = (cache: UiRefreshCache, retained: RetainedBrowserActorUiV1 | null): void => {
    if (retained !== null) for (const [key, { revision, value }] of browserActorSectionValuesV1(retained.sections, `actor ${retained.verifiedSurfaceId}`)) cache.set(key, { hash: `actor:${revision}`, value });
    const engagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {},
      measures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {},
      toolMeasures = (cache.get("tools")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {},
      catalogue = (cache.get("catalogue")?.value as AppCatalogue | undefined) ?? EMPTY_APP_CATALOGUE;
    dispatch({ type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(engagements)) });
    dispatch({ type: "SET_WINDOW_MEASURES_BY_WINDOW_ID", value: (current) => mergeRecordPreservingIdentity(current, Object.entries(measures)) });
    dispatch({ type: "SET_TOOL_MEASURES_BY_TOOL_ID", value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, null, spawnedIdsRef.current, Object.entries(toolMeasures))) });
    dispatch({ type: "SET_APP_CATALOGUE", value: (current) => preserveJsonIdentity(current, catalogue) });
  };
''')
swap(HOST, "sessionInstanceId: entry.session.instanceId, windows: applied.stores.windows, panels: applied.stores.panels, identity: null, actions });",
     "sessionInstanceId: entry.session.instanceId, windows: applied.stores.windows, panels: applied.stores.panels, sections: applied.stores.sections, identity: null, actions });")
swap(HOST, "retained === undefined ? null : { windows: retained.windows, panels: retained.panels });",
     "retained === undefined ? null : { windows: retained.windows, panels: retained.panels, sections: retained.sections });")
swap(HOST, '''        if (applied.stores !== null && (retained === undefined || applied.surfacesAdded)) setBrowserActorUiVersion((current) => current + 1);
        answer(applied.verdicts);
        return;''', '''        if (applied.stores !== null && (retained === undefined || applied.surfacesAdded)) setBrowserActorUiVersion((current) => current + 1);
        const live = browserActorUiByRuntimeKeyRef.current.get(runtimeKey),
          shown = shellStateRef.current.pluginRuntime.session;
        if (applied.sectionsChanged && live !== undefined && shown?.pluginId === entry.session.pluginId && shown.instanceId === entry.session.instanceId) publishUiRefreshSectionsV1(uiRefreshCacheRef.current, live);
        answer(applied.verdicts);
        return;''')
swap(HOST, "      const request = buildUiRefreshRequest(scope, fetchWindowInstances, panelTabLeaves, viewState, cache);\n",
     "      const built = buildUiRefreshRequest(scope, fetchWindowInstances, panelTabLeaves, viewState, cache);\n      const request = browserActorForSessionV1(nextSession) === null ? built : withoutUiRefreshSectionsV1(built);\n")
swap(HOST, '''      const dynamicEngagements = (cache.get("engagements")?.value as Readonly<Record<string, WindowEngagement>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_ENGAGEMENTS_BY_WINDOW_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicEngagements)),
      });
      const dynamicMeasures = (cache.get("measures")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({
        type: "SET_WINDOW_MEASURES_BY_WINDOW_ID",
        value: (current) => mergeRecordPreservingIdentity(current, Object.entries(dynamicMeasures)),
      });
      const dynamicToolMeasures = (cache.get("tools")?.value as Readonly<Record<string, readonly WindowMeasure[]>> | undefined) ?? {};
      dispatch({
        type: "SET_TOOL_MEASURES_BY_TOOL_ID",
        value: (current) => mergeRecordPreservingIdentity(current, withProgramEntriesV1(current, null, spawnedIdsRef.current, Object.entries(dynamicToolMeasures))),
      });
      const freshAppLabelsOverlay = normalizeAppLabelsOverlay(cache.get("labels")?.value as Partial<PluginAppLabelsOverlay> | undefined);
      dispatch({ type: "SET_APP_LABELS_OVERLAY", value: (current) => preserveJsonIdentity(current, freshAppLabelsOverlay) });
      // 🛍️ App-static: fetched once per app instance and kept by identity, so a scene host subscribing
      // through `AppCatalogueContext` never re-renders on an unchanged catalogue.
      const freshAppCatalogue = (cache.get("catalogue")?.value as AppCatalogue | undefined) ?? EMPTY_APP_CATALOGUE;
      dispatch({ type: "SET_APP_CATALOGUE", value: (current) => preserveJsonIdentity(current, freshAppCatalogue) });
''', '''      publishUiRefreshSectionsV1(cache, browserActorForSessionV1(nextSession));
      const freshAppLabelsOverlay = normalizeAppLabelsOverlay(cache.get("labels")?.value as Partial<PluginAppLabelsOverlay> | undefined);
      dispatch({ type: "SET_APP_LABELS_OVERLAY", value: (current) => preserveJsonIdentity(current, freshAppLabelsOverlay) });
''')

swap(CONFIG, '"../../🧪️tests/🚑️actor-recovery/🟦️.ts", ', '"../../🧪️tests/🚑️actor-recovery/🟦️.ts", "../../🧪️tests/🪟️visible-surfaces/🟦️.ts", ')

for path, text in edits.items():
    if DRY:
        print("dry", path.parent.name, path.name, len(text))
    else:
        path.write_text(text, encoding="utf-8")
        print("ok", path)
