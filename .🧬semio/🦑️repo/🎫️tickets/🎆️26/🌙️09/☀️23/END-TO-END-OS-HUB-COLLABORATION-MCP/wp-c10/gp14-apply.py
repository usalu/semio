"""C10 G-P1-4: actor-rendered panels — applies every edit in one pass so the tree is never half-changed.

Each edit is an exact, count-asserted string replacement; nothing is written unless every edit matches."""
import os
import shutil
import sys

ROOT = "/Users/ueli/Documents/semio"
OS = f"{ROOT}/🧰️framework/🛍️products/💻️os"
MOD = f"{OS}/🔨️modules"
STAGE = f"{ROOT}/.tmp-ticket/wp-c10/generated/gp14"
ELEMENTS = f"{MOD}/📺️renderer/🧑‍🎨engine/🧱️elements"

FILES = {
    "handoff": f"{MOD}/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts",
    "handoffTest": f"{MOD}/🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🧪️tests/🧪️browser-actor-patch-handoff-validates-the-neutral-schema-and-exact-owner/🟦️.ts",
    "worker": f"{MOD}/🏪️store/👷️worker/🟦️.ts",
    "store": f"{ELEMENTS}/📃️UiDocumentStore/🟦️.tsx",
    "intent": f"{MOD}/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/🧭️intent/🟦️.ts",
    "mailbox": f"{MOD}/🔌️plugin/🌐️browser-bundle/🎯️action-handoff/📮️requests/🟦️.ts",
    "helpers": f"{ELEMENTS}/🛠️ShellHelpers/🟦️.tsx",
    "host": f"{ELEMENTS}/🏛️ShellHost/🟦️.tsx",
    "ownerTest": f"{OS}/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts",
}
text = {key: open(path, encoding="utf-8").read() for key, path in FILES.items()}


def rep(key, old, new, count=1):
    found = text[key].count(old)
    if found != count:
        sys.exit(f"{key}: expected {count} match(es), found {found}: {old[:120]!r}")
    text[key] = text[key].replace(old, new)


def cut(key, start, end, new):
    source = text[key]
    if source.count(start) != 1 or source.count(end) != 1:
        sys.exit(f"{key}: region markers not unique: {start[:80]!r} / {end[:80]!r}")
    a = source.index(start)
    b = source.index(end)
    if b <= a:
        sys.exit(f"{key}: region order")
    text[key] = source[:a] + new + source[b:]


def region(key, start, end):
    source = text[key]
    a = source.index(start)
    b = source.index(end, a) + len(end)
    return source[a:b]


# ── 🩹️ patch handoff: staged new module, schema, fixture; test follows the multi-surface corpus
text["handoff"] = open(f"{STAGE}/patch-handoff.new.ts", encoding="utf-8").read()
rep("handoffTest", '''    expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(fixture.rejected))).toBe(true);
    for (const hostile of fixture.hostileResults) expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(hostile))).toBe(false);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, baseRevision: "0" } })).toThrow(/baseRevision/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, ops: [{ type: "remove", id: 1, extra: true }] } })).toThrow(/invalid fields/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patch: { ...fixture.offer.patch, ops: [{ type: "unknown", id: 1 }] } })).toThrow(/type: invalid/u);''', '''    expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(fixture.rejected))).toBe(true);
    expect(offer.patches.map((patch) => patch.surface)).toEqual(result.verdicts.map((verdict) => verdict.surface));
    for (const hostile of fixture.hostileResults) expect(browserActorUiPatchOwnerMatchesV1(offer, parseBrowserActorUiPatchResultV1(hostile))).toBe(false);
    for (const hostile of fixture.hostileOffers) expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: hostile.patches }), hostile.name).toThrow(/patches/u);
    const [windowPatch] = fixture.offer.patches;
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: [{ ...windowPatch, baseRevision: "0" }] })).toThrow(/baseRevision/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: [{ ...windowPatch, ops: [{ type: "remove", id: 1, extra: true }] }] })).toThrow(/invalid fields/u);
    expect(() => parseBrowserActorUiPatchOfferV1({ ...fixture.offer, patches: [{ ...windowPatch, ops: [{ type: "unknown", id: 1 }] }] })).toThrow(/type: invalid/u);
    expect(() => parseBrowserActorUiPatchResultV1({ ...fixture.rejected, verdicts: [{ surface: "map", outcome: "rejected", revision: 0 }] })).toThrow(/invalid fields/u);''')

# ── 🗄️ UiDocumentStore: the receiver state a rejection leaves behind
rep("store", '''  private notifyDiff(previous: UiDocumentState, next: UiDocumentState): void {''', '''  /** 🧹️ Returns the store to the empty document at revision 0 — the receiver state a `patch-rejected` leaves
   * behind, which the producer's full resend (a fresh reconciler, base revision 0) assumes. Subscribers see the
   * removal like any other change. */
  reset(): void {
    const previous = this.state;
    this.state = emptyUiDocumentState(previous.surface);
    this.notifyDiff(previous, this.state);
  }

  private notifyDiff(previous: UiDocumentState, next: UiDocumentState): void {''')

# ── 🧭️ intent request: any rendered surface of the actor, not only its window
rep("intent", '''export function createBrowserActorUiIntentRequestV1(owner: BrowserActorActionOwnerV1, windowKindId: string, intent: UiIntent): BrowserActorActionRequestV1 {
  if (windowKindId.length === 0 || intent.surface !== windowKindId || intent.revision !== owner.surfaceRevision) throw new Error("browser-actor-intent: stale surface");''', '''export function createBrowserActorUiIntentRequestV1(owner: BrowserActorActionOwnerV1, surfaceKey: string, intent: UiIntent): BrowserActorActionRequestV1 {
  if (surfaceKey.length === 0 || intent.surface !== surfaceKey || intent.revision !== owner.surfaceRevision) throw new Error("browser-actor-intent: stale surface");''')
rep("intent", "    surface: `${owner.instanceId}:${windowKindId}`,", "    surface: `${owner.instanceId}:${surfaceKey}`,")
rep("mailbox", '''  /** 🎯️ Captures an intent before handing its immutable bytes to the worker transport. */
  async dispatchIntent(owner: Omit<BrowserActorActionOwnerV1, "actionSequence">, windowKindId: string, intent: UiIntent): Promise<BrowserActorActionResultV1> {
    return await this.dispatchRequest(actionSequence => createBrowserActorUiIntentRequestV1({ ...owner, actionSequence }, windowKindId, intent));''', '''  /** 🎯️ Captures an intent made on one rendered surface (the window or a panel body) before handing its immutable
   * bytes to the worker transport. */
  async dispatchIntent(owner: Omit<BrowserActorActionOwnerV1, "actionSequence">, surfaceKey: string, intent: UiIntent): Promise<BrowserActorActionResultV1> {
    return await this.dispatchRequest(actionSequence => createBrowserActorUiIntentRequestV1({ ...owner, actionSequence }, surfaceKey, intent));''')

# ── 👷️ worker: verified panel surfaces, multi-surface offers, per-surface painted revisions
rep("worker", 'import { browserActorUiPatchOwnerMatchesV1, captureBrowserActorUiPatchV1, type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1 } from "../../🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";',
    'import { BROWSER_ACTOR_UI_PATCH_SURFACE_MAXIMUM, browserActorUiPatchOwnerMatchesV1, captureBrowserActorUiPatchV1, type BrowserActorUiPatchOfferV1, type BrowserActorUiPatchResultV1 } from "../../🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";')
rep("worker", 'import { windowViewContext, type ResolvedPluginViewState } from "../../../../../🔨️modules/🛂️manifest/🟦️.ts";',
    'import { panelTabKindId, panelViewContext, windowViewContext, type PanelTabKind, type ResolvedPluginViewState } from "../../../../../🔨️modules/🛂️manifest/🟦️.ts";')
rep("worker", "  #surfaceBodyKey: string | null = null;\n", "  #renderSurfaces: DocumentRenderSurfacesV1 | null = null;\n")
rep("worker", '''  renderBodyKey(): string {
    if (!this.#live || !this.#descriptor) throw new Error("document browser actor: dropped render descriptor");
    this.#surfaceBodyKey ??= parseVerifiedPackageDescriptorV1(this.#descriptor, this.#fields);
    return this.#surfaceBodyKey;
  }''', '''  /** 🪟️ The verified window and panel bodies this target's actor child renders (see {@link DocumentRenderSurfacesV1}). */
  renderSurfaces(): DocumentRenderSurfacesV1 {
    if (!this.#live || !this.#descriptor) throw new Error("document browser actor: dropped render descriptor");
    this.#renderSurfaces ??= parseVerifiedPackageDescriptorV1(this.#descriptor, this.#fields);
    return this.#renderSurfaces;
  }''')
rep("worker", '''/** 📜️ Strictly admits the raw descriptor bytes: canonical re-encoding equality first, then exact
 * equality between the decoded package descriptor and the verified lease fields. A sibling JSON
 * manifest or a caller-selected URL is never descriptor authority. */
function parseVerifiedPackageDescriptorV1(bytes: Uint8Array, fields: DocumentExecutionTargetLeaseFieldsV1): string {''', '''/** 🪟️ One body a verified actor child renders, keyed as the shell keys its store and the guest its surface ref. */
type DocumentRenderSurfaceV1 = Readonly<{ key: string; bodyKey: string }>;

/** 🪟️ The bodies one verified actor child renders: the lease's window and every panel-tab leaf of the verified app,
 * each panel keyed exactly as the shell keys its tab (`panelTabKindId`). The shell's own manifest never selects them. */
type DocumentRenderSurfacesV1 = Readonly<{ window: DocumentRenderSurfaceV1; panels: readonly DocumentRenderSurfaceV1[] }>;

function renderSurfaceTextV1(value: PackValue | undefined): value is string {
  return typeof value === "string" && value.length > 0 && new TextEncoder().encode(value).byteLength <= 256 && !/[\\u0000-\\u001f\\u007f]/u.test(value);
}

/** 🗂️ Every panel-tab leaf that carries a body, depth-first like the shell's `flattenPanelTabLeaves`. A malformed tab,
 * a key that repeats (or names the window) and more surfaces than one patch offer carries fail the descriptor. */
function verifiedPanelSurfacesV1(tabs: PackValue | undefined, windowKey: string): readonly DocumentRenderSurfaceV1[] {
  const panels: DocumentRenderSurfaceV1[] = [],
    keys = new Set([windowKey]);
  const visit = (value: PackValue | undefined, depth: number): void => {
    if (value === undefined) return;
    if (!Array.isArray(value) || depth > 8) throw new Error("document execution target: descriptor mismatch");
    for (const raw of value as readonly PackValue[]) {
      if (raw === null || typeof raw !== "object" || Array.isArray(raw) || raw instanceof Uint8Array || isPackInteger(raw)) throw new Error("document execution target: descriptor mismatch");
      const tab = raw as Readonly<Record<string, PackValue>>;
      if (Array.isArray(tab.children) && tab.children.length > 0) {
        visit(tab.children, depth + 1);
        continue;
      }
      if (tab.bodyKey === undefined) continue;
      const kind = tab.kind;
      const kindTag = kind !== null && typeof kind === "object" && !Array.isArray(kind) ? (kind as Readonly<Record<string, PackValue>>).kind : undefined;
      const key = typeof kindTag !== "string" ? undefined : kindTag === "app" ? (kind as Readonly<Record<string, PackValue>>).id : (panelTabKindId({ kind: kindTag } as unknown as PanelTabKind) as string | undefined);
      if (!renderSurfaceTextV1(key) || !renderSurfaceTextV1(tab.bodyKey) || keys.has(key) || keys.size === BROWSER_ACTOR_UI_PATCH_SURFACE_MAXIMUM) throw new Error("document execution target: descriptor mismatch");
      keys.add(key);
      panels.push(Object.freeze({ key, bodyKey: tab.bodyKey }));
    }
  };
  visit(tabs, 0);
  return Object.freeze(panels);
}

/** 📜️ Strictly admits the raw descriptor bytes: canonical re-encoding equality first, then exact
 * equality between the decoded package descriptor and the verified lease fields. A sibling JSON
 * manifest or a caller-selected URL is never descriptor authority, and the render surfaces it
 * answers come from those verified bytes alone. */
function parseVerifiedPackageDescriptorV1(bytes: Uint8Array, fields: DocumentExecutionTargetLeaseFieldsV1): DocumentRenderSurfacesV1 {''')
rep("worker", '''    throw new Error("document execution target: descriptor mismatch");
  return window.bodyKey as string;
}''', '''    throw new Error("document execution target: descriptor mismatch");
  return Object.freeze({ window: Object.freeze({ key: fields.surface.windowKindId, bodyKey: window.bodyKey as string }), panels: verifiedPanelSurfacesV1(app.panelTabs, fields.surface.windowKindId) });
}''')
rep("worker", '''function browserActorUiIntentBytes(request: BrowserActorActionRequestV1, windowKindId: string): Uint8Array {''', '''/** 🧭️ Admits one canonical UI intent and names the rendered surface (the window or a panel body) it was made on;
 * whether that surface was painted at the intent's revision is the reservation's check. */
function browserActorUiIntentV1(request: BrowserActorActionRequestV1): Readonly<{ surfaceKey: string; bytes: Uint8Array }> {''')
rep("worker", '''  if (
    value.surface !== `${request.instanceId}:${windowKindId}` ||
    encoder.encode(value.surface).byteLength > 512 ||''', '''  const prefix = `${request.instanceId}:`,
    surface = value.surface;
  if (
    typeof surface !== "string" ||
    !surface.startsWith(prefix) ||
    surface.length === prefix.length ||
    encoder.encode(surface).byteLength > 512 ||''')
rep("worker", '''  if (canonical.byteLength !== bytes.byteLength || canonical.some((byte, index) => byte !== bytes[index])) throw new Error("document browser actor: noncanonical intent");
  return bytes;
}''', '''  if (canonical.byteLength !== bytes.byteLength || canonical.some((byte, index) => byte !== bytes[index])) throw new Error("document browser actor: noncanonical intent");
  return { surfaceKey: surface.slice(prefix.length), bytes };
}''')
rep("worker", '''  private readonly windowKindId: string;
  private readonly abort = new AbortController();''', '''  private readonly windowKindId: string;
  /** 🪟️ The window and panel bodies this child renders, by the key the shell's stores and the guest's surface refs share. */
  private readonly surfaceKeys: ReadonlySet<string>;
  private readonly abort = new AbortController();''')
rep("worker", '''  private renderedUiPatch = false;
  private renderedUiRevision = 0;
  private acknowledgedUiRevision = 0;''', '''  private renderedUiPatch = false;
  /** 🩹️ The last revision the shell acknowledged per rendered surface; an intent is admitted against its own surface's. */
  private readonly renderedSurfaceRevisions = new Map<string, number>();
  private acknowledgedUiRevision = 0;''')
rep("worker", '''  private retirement: Promise<"retired" | "unconfirmed"> | null = null;
  private closed = false;
''', '''  private retirement: Promise<"retired" | "unconfirmed"> | null = null;
  private closed = false;

  private get renderedUiRevision(): number {
    return this.renderedSurfaceRevisions.get(this.windowKindId) ?? 0;
  }
''')
rep("worker", '''    this.windowKindId = fields.surface.windowKindId;
    this.generation = ++documentBrowserActorGeneration;''', '''    this.windowKindId = fields.surface.windowKindId;
    this.surfaceKeys = new Set([this.windowKindId, ...lease.renderSurfaces().panels.map(({ key }) => key)]);
    this.generation = ++documentBrowserActorGeneration;''')
rep("worker", '''        // 🪞️ The mailbox sends one action at a time and stamps each at issue, so a queued click carries the revision
        // on screen when it was made, which later acknowledged patches have since passed. Any revision this lifetime
        // painted (1 ..= `renderedUiRevision`, monotonic per activation generation) is the user's; the guest judges
        // an intent's geometry staleness itself (`DEFAULT_REVISION_TOLERANCE`).
        const painted = request.surfaceRevision >= 1 && request.surfaceRevision <= this.renderedUiRevision;''', '''        // 🪞️ The mailbox sends one action at a time and stamps each at issue, so a queued click carries the revision
        // on screen when it was made, which later acknowledged patches have since passed. Any revision this lifetime
        // painted on the action's own surface (1 ..= its last acknowledged revision: the window's for an app command,
        // the intent's window or panel body for a UI intent) is the user's; the guest judges an intent's geometry
        // staleness itself (`DEFAULT_REVISION_TOLERANCE`).
        const intent = request.payload.kind === "ui-intent" ? browserActorUiIntentV1(request) : null;
        const painted = request.surfaceRevision >= 1 && request.surfaceRevision <= (this.renderedSurfaceRevisions.get(intent?.surfaceKey ?? this.windowKindId) ?? 0);''')
rep("worker", '''        if (request.payload.kind === "ui-intent") {
          const intent = browserActorUiIntentBytes(request, fields.surface.windowKindId);
          invoked = true;
          const result = await this.invokePoll(child, [{ tag: "ui-intent", val: { instance: 0, intent } }], null, () => this.assertDocumentOwnerCurrent());''', '''        if (intent !== null) {
          invoked = true;
          const result = await this.invokePoll(child, [{ tag: "ui-intent", val: { instance: 0, intent: intent.bytes } }], null, () => this.assertDocumentOwnerCurrent());''')
rep("worker", "    this.renderedUiRevision = 0;\n", "    this.renderedSurfaceRevisions.clear();\n", 2)
rep("worker", '''  private async renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> {''', '''  /** 🖼️ Makes the window and every verified panel body visible to the child (window context for the window, panel
   * context for the panels, exactly as the local refresh binds them) and reconciles the turn's patches until the
   * render settles. Rendering the panels here is what keeps an actor-bound document's inspector live (G-P1-4). */
  private async renderSurface(child: DocumentBrowserActorChild, assertCurrent: () => void): Promise<void> {''')
rep("worker", '''    const surface = { instance: lifetime.instanceId, surface: windowId };
    const bodyKey = this.lease.renderBodyKey();
    for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
      assertCurrent();
      let result: BrowserActorChildValue | null = await this.invokePoll(child, [turn === 0 ? { tag: "surface-visible", val: { surface, bodyKey, viewState: encodePackValue(viewState) } } : { tag: "wake" }], null, assertCurrent);''', '''    const { window: windowSurface, panels } = this.lease.renderSurfaces();
    const panelView = encodePackValue(panelViewContext(hostView));
    const visible: BrowserActorChildValue[] = [
      { tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface: windowId }, bodyKey: windowSurface.bodyKey, viewState: encodePackValue(viewState) } },
      ...panels.map((panel) => ({ tag: "surface-visible", val: { surface: { instance: lifetime.instanceId, surface: panel.key }, bodyKey: panel.bodyKey, viewState: panelView } })),
    ];
    for (let turn = 0; turn < DOCUMENT_BROWSER_ACTOR_RENDER_TURN_LIMIT; turn += 1) {
      assertCurrent();
      let result: BrowserActorChildValue | null = await this.invokePoll(child, turn === 0 ? visible : [{ tag: "wake" }], null, assertCurrent);''')
rep("worker", "lifetime, this.windowKindId, { decodePack: decodePackWire, natural: packWireNatural });", "lifetime, this.surfaceKeys, { decodePack: decodePackWire, natural: packWireNatural });")
rep("worker", '''          patch: captured.patch,
          receipt: Array.from(encodeActorUiPatchReceipt(captured.receipt)),
        };
        wipeBrowserActorValue(value);
        value = null;
        const result = await this.awaitUiPatchResult(offer);
        assertCurrent();
        if (result.outcome === "acknowledged" && result.revision !== captured.patch.revision) throw new Error("document browser actor: acknowledged revision mismatch");
        if (result.outcome === "acknowledged") {
          this.renderedUiPatch = true;
          this.renderedUiRevision = result.revision;
        }
        const feedback = {
          tag: result.outcome === "acknowledged" ? "patch-ack" : "patch-rejected",
          val: {
            receipt: { lifetime: { ...captured.receipt.lifetime }, patchSequence: captured.receipt.patchSequence },
            surface: { instance: captured.instanceId, surface: captured.patch.surface },
            revision: BigInt(result.revision),
            ...(result.outcome === "rejected" ? { reason: result.reason } : {}),
          },
        };
        value = await this.invokePoll(child, [feedback, ...acknowledgements.splice(0)], null, assertCurrent);
        assertCurrent();
        if (browserActorColdStatus(value).kind !== "idle") throw new Error("document browser actor: cold ingress after patch feedback");
        if (result.outcome === "acknowledged") this.acknowledgedUiRevision = result.revision;
        else lastRejection = result.reason ?? "<unnamed>";
''', '''          patches: captured.patches,
          receipt: Array.from(encodeActorUiPatchReceipt(captured.receipt)),
        };
        wipeBrowserActorValue(value);
        value = null;
        const result = await this.awaitUiPatchResult(offer);
        assertCurrent();
        const feedback = result.verdicts.map((verdict, index) => {
          if (verdict.outcome === "acknowledged" && verdict.revision !== captured.patches[index]!.revision) throw new Error("document browser actor: acknowledged revision mismatch");
          if (verdict.outcome === "acknowledged") this.renderedSurfaceRevisions.set(verdict.surface, verdict.revision);
          else lastRejection = verdict.reason ?? "<unnamed>";
          return {
            tag: verdict.outcome === "acknowledged" ? "patch-ack" : "patch-rejected",
            val: {
              receipt: { lifetime: { ...captured.receipt.lifetime }, patchSequence: captured.receipt.patchSequence },
              surface: { instance: captured.instanceId, surface: verdict.surface },
              revision: BigInt(verdict.revision),
              ...(verdict.outcome === "rejected" ? { reason: verdict.reason } : {}),
            },
          };
        });
        const windowVerdict = result.verdicts.find((verdict) => verdict.surface === this.windowKindId);
        if (windowVerdict?.outcome === "acknowledged") this.renderedUiPatch = true;
        value = await this.invokePoll(child, [...feedback, ...acknowledgements.splice(0)], null, assertCurrent);
        assertCurrent();
        if (browserActorColdStatus(value).kind !== "idle") throw new Error("document browser actor: cold ingress after patch feedback");
        if (windowVerdict?.outcome === "acknowledged") this.acknowledgedUiRevision = windowVerdict.revision;
''')

# ── 🛠️ ShellHelpers: panel keys, per-surface apply, actor-hosted panel trees
rep("helpers", "    type UiIntent,\n    unresolvedActionArgs,", "    type UiIntent,\n    type UiPatch,\n    unresolvedActionArgs,")
rep("helpers", 'import { builtNodeToSnapshot, UiDocumentStore } from "../📃️UiDocumentStore/🟦️.tsx";',
    'import { builtNodeToSnapshot, UiDocumentStore } from "../📃️UiDocumentStore/🟦️.tsx";\nimport type { BrowserActorUiPatchVerdictV1 } from "../../../../🔌️plugin/🌐️browser-bundle/🩹️patch-handoff/🟦️.ts";')
rep("helpers", '''  treeWindows: TreeWindowHostV1 | null = null,
  cache?: PanelTreeConfigCacheV1,
): PanelTabNode {''', '''  treeWindows: TreeWindowHostV1 | null = null,
  cache?: PanelTreeConfigCacheV1,
  actorPanels: BrowserActorPanelHostV1 | null = null,
): PanelTabNode {''')
rep("helpers", "      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay, terminology, locale, treeWindows, cache)),",
    "      children: tab.children.map((child, childOrder) => panelTabDefinitionToNode(child, group, panelUiByKey, onAction, childOrder, appLabelsOverlay, terminology, locale, treeWindows, cache, actorPanels)),")
rep("helpers", "    tree: staticTreePanelDefinition(cachedTreePanelConfigV1(cache, tabId, panelUiByKey[tabId] ?? pendingPanelUiNodeV1(), tab.bodyKey ?? tabId, onAction, treeWindows)),",
    "    tree: staticTreePanelDefinition(\n      actorPanels === null\n        ? cachedTreePanelConfigV1(cache, tabId, panelUiByKey[tabId] ?? pendingPanelUiNodeV1(), tab.bodyKey ?? tabId, onAction, treeWindows)\n        : cachedActorTreePanelConfigV1(cache, tabId, actorPanels, tab.bodyKey ?? tabId, treeWindows),\n    ),")
rep("helpers", '''export function resolveCanvasBodyKey(app: AppDefinition): string {''', '''/** 🎭️ The panels of an actor-bound (hub) document, rendered by the document's verified browser actor rather than
 * the shell's local instance, which never sees the live document (ticket 26/09/23 C10, audit G-P1-4). `stores`
 * holds one retained store per panel-tab id, patched in place by the actor's offers; `onIntent` routes that
 * panel's gestures back to the same actor; `onAction` refuses a bare descriptor exactly as the actor's window does.
 * A tab without a store yet shows the pending body — never the local instance's stale one. */
export type BrowserActorPanelHostV1 = Readonly<{
  stores: ReadonlyMap<string, UiDocumentStore>;
  onIntent: (tabId: string, intent: UiIntent) => void;
  onAction: (action: ActionDescriptor) => void;
}>;

/** 🗂️ The panel-tab ids whose bodies an actor may render for `app`: its panel-tab leaves with a body, exactly the
 * panels the local refresh asks a guest for (`buildUiRefreshRequest`). */
export function browserActorPanelKeysV1(app: Pick<AppDefinition, "panelTabs">): ReadonlySet<string> {
  return new Set(flattenPanelTabLeaves(app.panelTabs).flatMap((tab) => (tab.bodyKey ? [panelTabKindId(tab.kind)] : [])));
}

export type BrowserActorUiStoresV1 = Readonly<{ window: UiDocumentStore; panels: Map<string, UiDocumentStore> }>;

/** 🩹️ Applies one browser-actor patch offer surface by surface and answers one verdict per patch, in offer order
 * — the guest acknowledges and resends per surface (`patch-ack` / `patch-rejected`), so one stale panel never
 * costs the window its frame. A patch for a surface this app does not render is refused `unknown-surface`; a
 * patch that does not apply resets its store to the empty document (see `UiDocumentStore.reset`), which is what
 * the guest's full resend assumes. The FIRST offer of an opening must paint the window, or nothing is retained
 * and every surface is refused `window-surface-unpainted` so the guest resends them all. */
export function applyBrowserActorUiPatchesV1(
  patches: readonly UiPatch[],
  windowKey: string,
  panelKeys: ReadonlySet<string>,
  retained: BrowserActorUiStoresV1 | null,
): Readonly<{ verdicts: readonly BrowserActorUiPatchVerdictV1[]; stores: BrowserActorUiStoresV1 | null; panelsAdded: boolean }> {
  const window = retained?.window ?? new UiDocumentStore(windowKey),
    panels = retained?.panels ?? new Map<string, UiDocumentStore>();
  let panelsAdded = false;
  const verdicts = patches.map((patch): BrowserActorUiPatchVerdictV1 => {
    if (patch.surface !== windowKey && !panelKeys.has(patch.surface)) return { surface: patch.surface, outcome: "rejected", revision: 0, reason: "unknown-surface" };
    let store = patch.surface === windowKey ? window : panels.get(patch.surface);
    if (store === undefined) {
      store = new UiDocumentStore(patch.surface);
      panels.set(patch.surface, store);
      panelsAdded = true;
    }
    const applied = store.applyPatch(patch);
    if (applied.ok) return { surface: patch.surface, outcome: "acknowledged", revision: store.getRevisionSnapshot() };
    store.reset();
    return { surface: patch.surface, outcome: "rejected", revision: 0, reason: applied.rejection.type };
  });
  if (retained !== null || verdicts.some((verdict) => verdict.surface === windowKey && verdict.outcome === "acknowledged")) return { verdicts, stores: { window, panels }, panelsAdded };
  return {
    verdicts: verdicts.map((verdict): BrowserActorUiPatchVerdictV1 => (verdict.outcome === "acknowledged" ? { surface: verdict.surface, outcome: "rejected", revision: 0, reason: "window-surface-unpainted" } : verdict)),
    stores: null,
    panelsAdded: false,
  };
}

export function resolveCanvasBodyKey(app: AppDefinition): string {''')
rep("helpers", '''export type PanelTreeConfigCacheV1 = Map<string, { readonly node: BuiltNode; readonly onAction: unknown; readonly bodyKey: string; readonly treeWindows: TreeWindowHostV1 | null; readonly openSignature: string; readonly config: TreePanelConfig }>;

function cachedTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, node: BuiltNode, bodyKey: string, onAction: (action: ActionDescriptor) => void, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  const openSignature = treeWindows ? treeWindowOpenSignatureV1(treeWindows.openStatesFor(bodyKey)) : "";
  const entry = cache?.get(tabId);
  if (entry && entry.node === node && entry.onAction === onAction && entry.bodyKey === bodyKey && entry.treeWindows === treeWindows && entry.openSignature === openSignature) return entry.config;
  const config = uiNodeToTreePanelConfig(node, onAction, bodyKey, treeWindows);
  cache?.set(tabId, { node, onAction, bodyKey, treeWindows, openSignature, config });
  return config;
}''', '''export type PanelTreeConfigCacheV1 = Map<string, { readonly source: BuiltNode | UiDocumentStore; readonly onAction: unknown; readonly bodyKey: string; readonly treeWindows: TreeWindowHostV1 | null; readonly openSignature: string; readonly config: TreePanelConfig }>;

function cachedTreePanelConfigV1(cache: PanelTreeConfigCacheV1 | undefined, tabId: string, node: BuiltNode, bodyKey: string, onAction: (action: ActionDescriptor) => void, treeWindows: TreeWindowHostV1 | null): TreePanelConfig {
  const openSignature = treeWindows ? treeWindowOpenSignatureV1(treeWindows.openStatesFor(bodyKey)) : "";
  const entry = cache?.get(tabId);
  if (entry && entry.source === node && entry.onAction === onAction && entry.bodyKey === bodyKey && entry.treeWindows === treeWindows && entry.openSignature === openSignature) return entry.config;
  const config = uiNodeToTreePanelConfig(node, onAction, bodyKey, treeWindows);
  cache?.set(tabId, { source: node, onAction, bodyKey, treeWindows, openSignature, config });
  return config;
}

/** 🎭️ {@link cachedTreePanelConfigV1} for an actor-rendered panel: the tree hosts the actor's retained store itself,
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
}''')
old_panel_host = region("helpers", "export function uiNodeToTreePanelConfig(", "    sortableSections: false,\n  };\n}\n")
if old_panel_host.count("const store = new UiDocumentStore(`panel:${node.key}`);") != 1 or old_panel_host.count("boundaryId={`panel-${node.key}`}") != 1 or old_panel_host.count("<InterpretedUiNode store={store} onAction={onAction} onIntent={(intent) => onAction(uiIntentToActionDescriptor(intent))} />") != 1:
    sys.exit("helpers: uiNodeToTreePanelConfig shape changed")
head, body = old_panel_host.split("  const store = new UiDocumentStore(`panel:${node.key}`);\n  store.loadSnapshot(builtNodeToSnapshot(`panel:${node.key}`, node));\n", 1)
new_panel_host = (
    head
    + "  const store = new UiDocumentStore(`panel:${node.key}`);\n  store.loadSnapshot(builtNodeToSnapshot(`panel:${node.key}`, node));\n"
    + "  return interpretedTreePanelConfigV1(store, node.key, onAction, (intent) => onAction(uiIntentToActionDescriptor(intent)), bodyKey, treeWindows);\n}\n\n"
    + "/** 🌲️ Hosts one retained panel store full-width in a tree leaf, with its tree-window context — shared by the\n"
    + " * authored-body path ({@link uiNodeToTreePanelConfig}) and the actor-rendered path ({@link BrowserActorPanelHostV1}). */\n"
    + "function interpretedTreePanelConfigV1(store: UiDocumentStore, boundaryKey: string, onAction: (action: ActionDescriptor) => void, onIntent: (intent: UiIntent) => void | Promise<void>, bodyKey: string, treeWindows?: TreeWindowHostV1 | null): TreePanelConfig {\n"
    + body.replace("boundaryId={`panel-${node.key}`}", "boundaryId={`panel-${boundaryKey}`}").replace(
        "<InterpretedUiNode store={store} onAction={onAction} onIntent={(intent) => onAction(uiIntentToActionDescriptor(intent))} />",
        "<InterpretedUiNode store={store} onAction={onAction} onIntent={onIntent} />",
    )
)
rep("helpers", old_panel_host, new_panel_host)

# ── 🏛️ ShellHost: per-surface stores, panel host, panel intents
rep("host", "  type PanelTreeConfigCacheV1,\n", "  type PanelTreeConfigCacheV1,\n  applyBrowserActorUiPatchesV1,\n  browserActorPanelKeysV1,\n  type BrowserActorPanelHostV1,\n")
rep("host", "readonly windowKindId: string; readonly store: UiDocumentStore; readonly identity: BrowserActorUiMountedV1 | null; readonly actions: BrowserActorActionMailboxV1 };",
    "readonly windowKindId: string; readonly store: UiDocumentStore; readonly panels: Map<string, UiDocumentStore>; readonly identity: BrowserActorUiMountedV1 | null; readonly actions: BrowserActorActionMailboxV1 };")
cut("host", '      if (message.kind === "browser-actor-ui-patch") {\n', '      if (message.kind === "browser-actor-ui-mounted") {\n', '''      if (message.kind === "browser-actor-ui-patch") {
        const runtimeKey = documentRuntimeKeyV1({ kind: "hub", dataClass: "persistedShared", spaceId: message.scope.spaceId, documentId: message.scope.documentId });
        const entry = openDocumentSessionsRef.current.get(runtimeKey);
        const expectedSurfaceId = entry?.scope !== undefined && entry.session.app.dialect ? canonicalSurfaceId(entry.session.app.dialect, entry.session.app.role) : null;
        // 🩻️ An offer this shell cannot apply is REFUSED by name, never dropped. Dropping it left the
        // worker's 15 s patch-result deadline to close the child with no reason anywhere, so a guest
        // whose surface the session disagreed about looked exactly like a guest that had stopped.
        // Only an offer addressed to no live client of this shell stays silent: there is no session
        // left to answer for.
        if (entry === undefined || entry.clientInstanceId !== message.clientInstanceId) return;
        const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
        const windowKind = entry.session.app.windowKinds.find((candidate) => (retained === undefined ? message.patches.some((patch) => patch.surface === candidate.id) : candidate.id === retained.windowKindId));
        const refusal =
          entry.scope === undefined || entry.scope.spaceId !== message.scope.spaceId || entry.scope.documentId !== message.scope.documentId
            ? "scope-mismatch"
            : expectedSurfaceId !== message.verifiedSurfaceId
              ? `verified-surface-mismatch: session ${expectedSurfaceId ?? "<none>"} offer ${message.verifiedSurfaceId}`
              : windowKind === undefined
                ? `unknown-window-kind: ${message.patches.map((patch) => patch.surface).join(",")}`
                : message.instanceId !== 0
                  ? `instance-mismatch: ${message.instanceId}`
                  : retained !== undefined && (retained.clientInstanceId !== message.clientInstanceId || retained.activationGeneration !== message.activationGeneration || retained.verifiedSurfaceId !== message.verifiedSurfaceId || retained.sessionInstanceId !== entry.session.instanceId)
                    ? "retained-owner-mismatch"
                    : null;
        const answer = (verdicts: readonly { readonly surface: string; readonly outcome: "acknowledged" | "rejected"; readonly revision: number; readonly reason?: string }[]) =>
          worker.postMessage({ wire: encodeBackboneWorkerRequest({ kind: "browser-actor-ui-patch-result", clientInstanceId: entry.clientInstanceId, scope: message.scope, verifiedSurfaceId: message.verifiedSurfaceId, activationGeneration: message.activationGeneration, instanceId: message.instanceId, receipt: message.receipt, verdicts }) });
        if (refusal !== null || windowKind === undefined) {
          answer(message.patches.map((patch) => ({ surface: patch.surface, outcome: "rejected" as const, revision: 0, reason: refusal ?? "unknown-window-kind" })));
          return;
        }
        const applied = applyBrowserActorUiPatchesV1(message.patches, windowKind.id, browserActorPanelKeysV1(entry.session.app), retained === undefined ? null : { window: retained.store, panels: retained.panels });
        if (applied.stores !== null && retained === undefined) {
          const actions = new BrowserActorActionMailboxV1((request) => worker.postMessage({ wire: encodeBackboneWorkerRequest({ ...request, clientInstanceId: entry.clientInstanceId }) }));
          browserActorUiByRuntimeKeyRef.current.set(runtimeKey, { clientInstanceId: message.clientInstanceId, activationGeneration: message.activationGeneration, verifiedSurfaceId: message.verifiedSurfaceId, scope: { ...message.scope }, sessionInstanceId: entry.session.instanceId, windowKindId: windowKind.id, store: applied.stores.window, panels: applied.stores.panels, identity: null, actions });
        }
        if (applied.stores !== null && (retained === undefined || applied.panelsAdded)) setBrowserActorUiVersion((current) => current + 1);
        answer(applied.verdicts);
        return;
      }
''')
rep("host", '''  const onBrowserActorIntent = useCallback((runtimeKey: string, captured: RetainedBrowserActorUiV1, intent: Parameters<typeof uiIntentToActionDescriptor>[0]) => {
    const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    if (retained?.actions !== captured.actions''', '''  const onBrowserActorIntent = useCallback((runtimeKey: string, captured: RetainedBrowserActorUiV1, surfaceKey: string, intent: Parameters<typeof uiIntentToActionDescriptor>[0]) => {
    const retained = browserActorUiByRuntimeKeyRef.current.get(runtimeKey);
    const entry = openDocumentSessionsRef.current.get(runtimeKey);
    const store = surfaceKey === captured.windowKindId ? retained?.store : retained?.panels.get(surfaceKey);
    if (store === undefined || retained?.actions !== captured.actions''')
rep("host", '''      surfaceRevision: retained.store.getRevisionSnapshot(),
    }, retained.windowKindId, intent).then((result) => {''', '''      surfaceRevision: store.getRevisionSnapshot(),
    }, surfaceKey, intent).then((result) => {''')
rep("host", "onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, intent);", "onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, currentBrowserActorUi.windowKindId, intent);")
lookup = '''  /** 📌️ Reverse-lookup: `openDocumentSessionsRef` is keyed by exact runtime identity, never
   * the other way around (no `ActiveSession.documentId` field exists — see `📓️w3-a-report.md`'s
   * "Design decisions"). This tiny scan intentionally runs on every render because `openDocument`
   * fills the ref while retaining the same visible session identity. */
  const currentDocumentRuntimeKey = (() => {
    if (!session) return null;
    for (const [runtimeKey, entry] of openDocumentSessionsRef.current) {
      if (entry.session.pluginId === session.pluginId && entry.session.instanceId === session.instanceId) return runtimeKey;
    }
    return null;
  })();
'''
actor_ui = '''  const currentBrowserActorUi = useMemo(
    () => currentDocumentRuntimeKey === null ? undefined : browserActorUiByRuntimeKeyRef.current.get(currentDocumentRuntimeKey),
    [browserActorUiVersion, currentDocumentRuntimeKey],
  );
'''
rep("host", lookup, "")
rep("host", actor_ui, "")
rep("host", "  const activeRightPanelTab = session?.app.panelTabs.find(", lookup + actor_ui + '''  /** 🎭️ The panel route of an actor-bound document ({@link BrowserActorPanelHostV1}): its intents go to the verified
   * actor under the panel's own surface; `browserActorUiVersion` moves when a panel store first arrives. */
  const browserActorPanelIntent = useMemo(
    () => currentDocumentRuntimeKey === null || currentBrowserActorUi === undefined ? null : (tabId: string, intent: Parameters<typeof uiIntentToActionDescriptor>[0]) => onBrowserActorIntent(currentDocumentRuntimeKey, currentBrowserActorUi, tabId, intent),
    [currentBrowserActorUi, currentDocumentRuntimeKey, onBrowserActorIntent],
  );
  const browserActorPanels = useMemo(
    (): BrowserActorPanelHostV1 | null => !session || browserActorPanelIntent === null || currentBrowserActorUi?.sessionInstanceId !== session.instanceId ? null : { stores: new Map(currentBrowserActorUi.panels), onIntent: browserActorPanelIntent, onAction: refuseBrowserActorActionDescriptor },
    [browserActorPanelIntent, browserActorUiVersion, currentBrowserActorUi, refuseBrowserActorActionDescriptor, session],
  );

  const activeRightPanelTab = session?.app.panelTabs.find(''')
rep("host", "panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current)",
    "panelTabDefinitionToNode(tab, tab.group, panelUiByKey, onAction, order, appLabelsOverlay, uiTerminology, uiLocale, treeWindowHost, panelTreeConfigCacheRef.current, browserActorPanels)", 3)
rep("host", "[appLabelsOverlay, onAction, panelUiByKey, session, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration]",
    "[appLabelsOverlay, browserActorPanels, onAction, panelUiByKey, session, uiTerminology, uiLocale, treeWindowHost, treeWindowGeneration]", 2)
rep("host", "  }, [appLabelsOverlay, onAction, panel?.spawnedApps.length, panelUiByKey, session, hostMode, uiLocale, uiTerminology, hostAppId, openWithEntries, openWithFocusRole, openArtifactWithAppRef, dispatchSetDefaultApp, dispatchClearDefaultApp, treeWindowHost, treeWindowGeneration]);",
    "  }, [appLabelsOverlay, browserActorPanels, onAction, panel?.spawnedApps.length, panelUiByKey, session, hostMode, uiLocale, uiTerminology, hostAppId, openWithEntries, openWithFocusRole, openArtifactWithAppRef, dispatchSetDefaultApp, dispatchClearDefaultApp, treeWindowHost, treeWindowGeneration]);")

# ── 🧪️ worker owner test: the fake shell answers per-surface verdicts
rep("ownerTest", '''        patchOffers.push(message);
        if (message.patch.baseRevision === 1 && message.patch.revision === 2) {
          heldActionPatch = message;
          return;
        }
        const before = uiStore.getState();
        const applied = uiStore.applyPatch(message.patch);''', '''        patchOffers.push(message);
        expect(message.patches).toHaveLength(1);
        const offered = message.patches[0]!;
        if (offered.baseRevision === 1 && offered.revision === 2) {
          heldActionPatch = message;
          return;
        }
        const before = uiStore.getState();
        const applied = uiStore.applyPatch(offered);''')
rep("ownerTest", '''            receipt: message.receipt,
            outcome: "acknowledged",
            revision: uiStore.getRevisionSnapshot(),
          });
          return;
        }
        expect(uiStore.getState()).toBe(before);''', '''            receipt: message.receipt,
            verdicts: [{ surface: offered.surface, outcome: "acknowledged", revision: uiStore.getRevisionSnapshot() }],
          });
          return;
        }
        expect(uiStore.getState()).toBe(before);''')
rep("ownerTest", '''          receipt: message.receipt,
          outcome: "rejected",
          revision: uiStore.getRevisionSnapshot(),
          reason: applied.rejection.type,
        });''', '''          receipt: message.receipt,
          verdicts: [{ surface: offered.surface, outcome: "rejected", revision: uiStore.getRevisionSnapshot(), reason: applied.rejection.type }],
        });''')
rep("ownerTest", '''        const actionApplied = uiStore.applyPatch(actionPatch.patch);''', '''        const actionApplied = uiStore.applyPatch(actionPatch.patches[0]!);''')
rep("ownerTest", '''          receipt: actionPatch.receipt,
          outcome: "acknowledged",
          revision: uiStore.getRevisionSnapshot(),
        });''', '''          receipt: actionPatch.receipt,
          verdicts: [{ surface: actionPatch.patches[0]!.surface, outcome: "acknowledged", revision: uiStore.getRevisionSnapshot() }],
        });''')

for key, path in FILES.items():
    shutil.copyfile(path, f"{STAGE}/{key}.before")
for key, path in FILES.items():
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text[key])
handoff_dir = os.path.dirname(FILES["handoff"])
shutil.copyfile(f"{STAGE}/patch-handoff-fixture.new.json", f"{handoff_dir}/🧫️fixtures/🔣️.json")
shutil.copyfile(f"{STAGE}/patch-handoff-schema.new.json", f"{handoff_dir}/🧬️schema/🔣️.json")
print("applied", ", ".join(FILES))
