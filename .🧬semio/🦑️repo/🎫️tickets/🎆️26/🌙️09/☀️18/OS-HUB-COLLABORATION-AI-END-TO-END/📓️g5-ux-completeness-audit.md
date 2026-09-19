# G5 — os frontend UX-completeness audit (i18n, a11y, multi-device, progress/cancel, offline)

Read-only audit (no builds/servers/edits outside this file) against `AGENTS.md` lines 27-46:
accessible UIs; multi-language with no default language (English SHOULD be first, German second);
customizable UIs; multi-device (desktop, then mobile, then tablet); progress+cancellation for every
expensive operation; short connection loss must not freeze the app.

Scope: `🧰️framework/🔨️modules/🖱️ui` (element library, React + wgpu targets, contract layer),
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer` (`🧑‍🎨engine` — React `⚛️react` + wgpu `🧊️wgpu`
targets, `ShellHost`), shell chrome, plugin-facing UiNode schema pipeline (`✏️s/🔌️plugins`, 17
plugins sampled).

Cross-reference: `📓️g1-goal-gap-audit.md` §3 already flagged "no mobile layout system" and
"i18n drift between wgpu/React"; `📓️k1-hygiene.md` §4 and `📓️au2-os-sign-in-and-spaces-ui.md` were
launched to dig further but both worker sessions died leaving every section a `(pending)`/`(in
progress)` placeholder with zero measured content — this report supersedes them for its scope with
direct file:line evidence. G1's "no mobile layout" claim is **corrected** below (§3): a real,
byte-parity mobile mode exists in both renderers, just not where G1's shallow grep looked.

---

## 1. i18n

### 1.1 Mechanism

Two independent, both real, translation mechanisms exist:

- **Chrome/product TS bundles** — `UiTranslationSchema` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx:90-670`,
  ~402 leaf `UiLabelValue` keys, counted via `grep -c "UiLabelValue;"`). The domain-neutral chrome
  bundle `uiChromeTranslationBundles` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2389-4071`)
  is typed `satisfies Record<UiLocale, { readonly translation: UiTranslationSchema }>` (line 4071) —
  a **compile-time completeness check**: TypeScript refuses to build if `en` or `de` is missing a
  key the other has. This is a materially stronger guarantee than typical i18next setups (which
  silently fall back). Products register additional bundles through `registerUiTranslationBundles<S>`
  (`🎯️targets/⚛️react/🟦️.tsx:4100`), whose generic signature `{ readonly [L in UiLocale]: {
  readonly translation: S } }` requires the identical shape `S` for every locale, giving the same
  parity guarantee to product-owned bundles.
- **Plugin manifest labels (Rust)** — `LocalizedLabel` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs:96-131`)
  is a locale×terminology matrix; `LocalizedLabel::native(en, de)` (line 123) does an **exhaustive
  match on `Locale`** (no catch-all arm), so adding a locale without translating every call site is
  a compile error. Used for manifest-declared commands/modes/actions across plugins.

### 1.2 Measured en/de key coverage — 0 missing in every bundle checked

I did not just trust the type system; I counted leaf `label: {` occurrences inside each locale's
block by line range, for every registered bundle in the renderer tree:

| bundle | file | en | de | missing |
|---|---|---|---|---|
| chrome (`ui.*`, `settings.*`, `tooltip.*`, `introduction.*`, `tutorial.*`) | `🎯️targets/⚛️react/🟦️.tsx:2389-4071` | ~402 (schema-derived) | ~402 | **0** (compile-enforced) |
| `os.taskManager.*` | `📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🟦️.tsx:98-184` | 27 | 27 | **0** |
| `os.spaceAdministration.*` | `📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx:51-153` | 43 | 43 | **0** |
| `os.agent.*` (AgentChatPanel/AgentBridge) | `📺️renderer/🧑‍🎨engine/🧱️elements/🔗️AgentBridge/🟦️.tsx:31-133` | 37 | 37 | **0** |

`LocalizedLabel::native("X", "X")` (a duplicate-text stub that would defeat translation while still
type-checking) has **zero** occurrences across all 2853 `.native(` call sites in `✏️s/🔌️plugins`
(`grep -c` check on the exact-duplicate regex). Spot-checked German text is real, not placeholder
(e.g. `writer/…/✏️editor/🦀️.rs:1374-1375`: `"Format Document"/"Dokument formatieren"`,
`"Lint Document"/"Dokument prüfen"`).

**Conclusion: the key-based i18n system itself has no missing-key problem.** The real i18n gaps are
strings that bypass the system entirely (below).

### 1.3 Hard-coded strings that bypass i18n — the actual gap

1. **`MutationKind::label()` — 2690 hard-coded English strings feeding the undo/history UI, across
   every plugin, with zero localization.** The trait (`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️.rs:230-231`,
   doc: *"Human undo/history label, e.g. `Rename piece "a" to "b"`"*) returns a plain `String`, e.g.
   `✏️s/🔌️plugins/✒️writer/…/✏️edit-text/🦀️.rs:37-39`: `fn label(&self) -> String { "Edit document
   text".to_string() }`. `🕰️HistoryTable` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🕰️HistoryTable/🟦️.tsx:156-159`)
   renders these `label: string` values verbatim in the history/undo panel — the entire mutation
   history is English-only regardless of the active shell locale. Counted per-plugin (17 of 34
   plugins sampled, `grep -c 'fn label(&self) -> String {'`):

   | plugin | count | plugin | count | plugin | count |
   |---|---|---|---|---|---|
   | 📕️norm | **393** | 🏛️architect | **268** | 🧩️puzzle | 106 |
   | 🧱️block | 105 | 🀄️wfc | 76 | 🏗️fem | 59 |
   | 🎥️shooting | 39 | 🌍️gis | 21 | 📐️cad | 25 |
   | 🕸️dag | 17 | ➗️mathematical | 16 | 🎞️animate | 11 |
   | 🌊️flow | 10 | ✒️writer | 9 | 🎬️sequence | 8 |
   | 🌿️vcs | 6 | 🎪️demonstrator | 1 | **total (all 34 plugins)** | **2690** |

   Worst offenders: `📕️norm` (393), `🏛️architect` (268), `🧩️puzzle` (106), `🧱️block` (105).

2. **`ShellSync/🟦️.tsx:40-47` (`syncStatusLabel`) — the connection-status text shown for a document's
   backbone sync (the exact UI relevant to §5 offline/reconnect) is entirely hard-coded English**:
   `"live · N peers"`, `"connecting…"`, `"reconnecting…"`, `"offline"`, `"saved"`/`"unsaved"`,
   `"N pending"` — none routed through `useLabel`/`UiTranslationSchema`, unlike every other string
   in the same file (`attachLabel`/`detachLabel`/`browseLabel` on lines 51-53 are correctly
   localized). A German-locale user sees this status line in English.

3. **`ShellHost/🟦️.tsx:8172-8177` (`showMutationRejectedNotice`)** — the rejected-dispatch toast body
   is `${localized code label} — ${cause.message}`, where `cause.message` is the Rust fault's raw
   English prose (documented as intentional at lines 8168-8171: *"UI localizes by code, never by
   parsing the English message prose"*). This is a **known, designed** partial gap, not an
   oversight, but it does mean German (and any non-English) users see a mixed-language toast on
   every rejected mutation with a populated `causes` array.

4. `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/👥️presence-scope/🌐️browser/🟦️.tsx:126,136,137` —
   hard-coded English JSX text (`"Scoped presence browser shell"`, `"Close A"`, `"Heartbeat B"`).
   Low priority: this is a headless Playwright test-harness page (`ScopedPresenceBrowserConfigV1`),
   not a real user-facing surface, but it lives inside the same `🏛️ShellHost` element tree.

Across a systematic grep of the two main element trees (`🧱️elements` under `🖱️ui` and under
`📺️renderer/🧑‍🎨engine`) for `aria-label="[A-Za-z]`, `title="[A-Za-z]{3,}`, `placeholder="[A-Za-z]{3,}`
and JSX text children `>[A-Z][a-zA-Z ]+<`, excluding tests/stories/fixtures, **only the presence-scope
test harness above matched** — meaning outside the two exceptions above (mutation labels, sync
status), the disciplined `useLabel`/`t()` convention is followed consistently.

### 1.4 Implicit default language

`createShellI18nInstance`/`initializeUiI18n` (`🎯️targets/⚛️react/🟦️.tsx:4137,4249`) both set
`fallbackLng: "en"`, and `normalizeUiLocale`/`detectShellLocale` (lines 4197-4204) resolve anything
that isn't `de*` to `"en"`. AGENTS.md's exact wording is "You MUST support multiple languages with
no default language. You SHOULD use English as first, then German as second." The `fallbackLng`/
normalize-to-en behavior is defensible as satisfying the SHOULD (English first), but it is
technically an implicit default: if a key is ever missing in `de` (structurally impossible for the
compile-checked bundles, but not for the free-text gaps in §1.3), the system silently falls back to
English rather than surfacing a gap. `initUiLocaleSync` (line 4292) explicitly exists to prevent an
English flash for German-locked brands, which shows the team is already aware of and mitigating this
tension — worth a docs-only note rather than a code fix.

---

## 2. Accessibility

### 2.1 Per-element role/keyboard/focus-ring survey

Grepped every element under `🧰️framework/🔨️modules/🖱️ui/🧱️elements/*/🟦️.tsx` for `role=`/`aria-`
(role), `onKeyDown`/`onKeyUp`/`useHotkeys`/`tabIndex` (kb), `focus-visible`/`focus-within`/`:focus`/
`ring-` (ring):

- **Strong**: `🌳️Tree` (role=23), `🖱️ContextMenu` (role=20, kb=3), `🔽️Select` (role=19, kb=7),
  `💬️Dialog` (role=16, kb=1), `⌨️Command` (role=15, kb=3), `🎚️Slider` (role=11, kb=5, ring=1),
  `📑️Tabs` (role=9, kb=5, ring=1), `🎨️Canvas` (role=8, kb=13).
- **Zero role + zero keyboard-handler hits**: `🕸️Diagram` (1842 lines, built on `@xyflow/react`
  `ReactFlow`), `🔌️Ports`, `🎬️Scene`, `🌈️Surface`, `🔘️Button` (delegates to `🔳️ButtonGroup`'s native
  `<button>`, so this is fine — not a real gap), `🔚️Footer`, `🔝️Navbar`, `🔲️WindowSilhouette`,
  `🚗️UiDriver`, `🧾️Form`, `🎗️UiLabel`, `📝️Field`.
  - `🕸️Diagram/🟦️.tsx` is the genuinely concerning one: it is the node-graph editor surface (used by
    flow/procedural-style plugins) and adds **no** role/aria/keyboard layer on top of ReactFlow's own
    (limited) defaults — node-graph editing is effectively mouse-only. This matches the design note
    already written into the shared a11y contract itself
    (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:108-112`): *"the node-graph and
    World3d canvases carry their whole interaction surface inside a texture no assistive technology
    can walk... a canvas nobody can Tab into is a canvas nobody can drive without a mouse"* — an
    honest, self-documented limitation rather than an unnoticed gap, but still unresolved.
  - `Footer`/`Navbar`/`WindowSilhouette`/`Surface`/`Scene`/`Form`/`Field`/`UiLabel` are
    layout/decorative/composition wrappers whose interactive children carry their own roles
    elsewhere — not real gaps on inspection.

### 2.2 Shared accessibility contract (both renderers answer the same fixture)

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs` defines `AccessibilitySpec` (label,
description, `Liveness` off/polite/assertive, keyboard shortcut, hidden — carried by every
`UiNodeRecord`) and derives `AccessibilityProjectionNode` (role, depth, focusable, actionable,
focused, rect, `value_min/max/now/text`, `busy`) from it (lines 30-235). `accessibility_role`
(line 66) and `accessibility_is_focusable` (line 113) are the ONE place role/focusability is derived
from `Component`, shared by every target so React (which gets roles for free from real DOM elements)
and wgpu (which has none) can never disagree. Note: the file's own header (line 3) says
`"⚠️ SCAFFOLD — owned by packet contract-layout. Replace this placeholder wholesale"` — despite being
fairly complete and tested, it is still explicitly marked provisional.

### 2.3 wgpu accessibility story — real DOM-mirror ARIA tree (answers "AccessKit-like tree?")

**Yes, concretely**: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs` builds a
flat, pre-order `Vec<AccessibilityProjectionNode>` per window from the retained document
(`accessibility_projection`, lines 48-76; ceiling `UI_ACCESSIBILITY_PROJECTION_DEPTH = 64`), stamping
live `focused`/`rect` from the arena. The browser host consumes this over the wire and **mirrors it
into a real, offscreen DOM subtree beside the `<canvas>`**:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🚀️browser-boot/🟦️.ts:112-236`
(`WGPU_ACCESSIBILITY_MIRROR_ID = "semio-wgpu-accessibility"`, `accessibilityMirror()`) — it walks the
projection and sets real `aria-label`/`aria-live`/`aria-keyshortcuts`/`aria-hidden`/`aria-disabled`/
`aria-valuemin/max/now/text`/`aria-busy`/`aria-describedby` on synthetic DOM nodes, refreshed on
every UI turn (line 468). It is even **locale-aware**: line 86 sets the canvas's own
`aria-label` to `"Semio Arbeitsfläche"` (de) / `"Semio workspace"` (en), and line 172 does the same
for the mirror root. This is a real, working DOM-mirror accessibility bridge for a GPU-only
renderer — not a stub — and it is exercised by `🧪️tests/🔬️targets-wgpu-accessibility-projection`.
No native (non-browser) AccessKit integration was found for the native wgpu winit build
(`🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs`) — the mirror only exists for the browser host; a native
desktop build of the wgpu target has no assistive-technology story at all as far as this audit found.

### 2.4 Colour contrast / theming tokens

`🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🟦️.ts:676-696` implements real WCAG relative luminance
(`relativeLuminance`) and `readableForegroundHex`, which auto-picks a readable foreground for any
background token. `🎨️styling/🧪️tests/🧪️levels-oklabmix/🟦️.ts:70-75` asserts the **generated default
palette** stays ≥4.5:1 (WCAG AA) against its chrome foreground, at every level, both light/dark
appearances. This only covers the system-generated default palette; the theme editor
(`settings.theme.*` — colors/import/export/save keys exist in the I18n schema, §1.1) lets a user
pick **arbitrary custom colors**, and no live contrast-ratio warning in the editor itself was found
(`grep` for `contrastRatio`/`4.5` inside the styling module only hits the test suite, not the editor
UI) — a customizable-but-unchecked gap: a user can save a theme with sub-AA foreground/background
pairs with no warning.

### 2.5 Reduced motion

Real support on both fronts: CSS (`🎨️styling/🖌️ui/🎨️.css` — 5+ `@media (prefers-reduced-motion:
reduce)` blocks) and React (`🎯️targets/⚛️react/🟦️.tsx:5377-5621` — a demonstrator/tutorial overlay
checks `matchMedia("(prefers-reduced-motion: reduce)")` and renders `null` entirely under reduce,
lines 5377/5621), plus wgpu paint (`🎯️targets/🧊️wgpu/🖌️paint/🦀️.rs`) and per-surface tool-run traces
(`Canvas2dHost/⏯️tool-run-trace`, `World3dHost/⏯️tool-run-trace`) all reference the same preference.

---

## 3. Multi-device

**G1's audit claim ("no plugin or shell surface... discusses a phone/tablet layout... incidental
grep hits, not a deliberate responsive-layout system") is contradicted by direct evidence at the
shell-chrome level** — G1 evidently did not grep deep enough into `📺️renderer/🧑‍🎨engine`. A real,
byte-parity mobile mode exists in both renderer targets:

- **Breakpoint**: `UI_MOBILE_MEDIA_QUERY = "(max-width: 767px)"` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:1649`).
  React: `useUiMobile()` (lines 4397-4402) wraps `useMediaQuery` + a `UiMobileContext` override.
  wgpu: `ShellDock::mobile_panel_active` reads `self.screen_w <= crate::dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX`
  (`📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:7855-7859`, set at line 20559) —
  same 767px value, cross-checked by the wgpu file's own comment ("the same
  `dock::MODE_DOCK_MOBILE_MAX_WIDTH_PX` the window dock reads").
- **Layout collapse**: below the breakpoint, all eight dock anchors collapse into ONE flat mobile
  panel/tab-stack in both targets — `ShellHost/🟦️.tsx` wires `const mobile = useMediaQuery(...)`
  (line 1984) through `mobilePanelVisible`/`mobilePanelPath` state (`🧑‍🎨engine/🧱️elements/🐚️Shell/🟦️.tsx:518-520,924-926`)
  into panel selection, tool panes, and a synthetic `"framework.mobile.app"` menu tab (line 9564);
  the wgpu Rust side (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:145-146,2991-2994,7837-7859,11784-11788`)
  implements the identical flattened-tab-list/`mobile_panel_reconcile_path` behavior, with its own
  doc comment at line 145 stating it is *"byte-identical to React's synthetic mobile 'App' tab"*.
  This is a deliberately dual-implemented, tested responsive system, not incidental.
- **Settings surface** also exposes an explicit, user-selectable `ElementsSurfaceDevice = "desktop" |
  "tablet" | "mobile"` (`🎯️targets/⚛️react/🟦️.tsx:1646`) and `settings.layout.desktop/tablet/mobile`
  translation keys (`📚️I18n/🟦️.tsx:590-594`) — but **automatic** detection (`useUiMobile`) is binary
  (mobile vs. not), so "tablet" only exists as a manual override, never auto-detected from viewport
  width. This partially satisfies AGENTS.md's "desktop, then mobile, then tablet" priority order —
  desktop and mobile are both real; tablet is manual-only.

### Touch gestures in 2D/3D viewports — real single-pointer, no multi-touch gestures

`🌐️World3dHost/🟦️.tsx` uses the unified `PointerEvent` model throughout (`onPointerDown`, lines
2913/3113/3144/6832/6948-7003, `setPointerCapture`/`hasPointerCapture`) rather than mouse-only
events, so a single-finger tap/drag (select, orbit-via-drag, marquee) works on a touchscreen for
free. However, `grep` for `pointerId`-keyed multi-pointer tracking, `touches[1]`, or any pinch/rotate
gesture recognizer across `World3dHost` and `Board2dHost` found **none** — there is no two-finger
pinch-to-zoom or two-finger pan/rotate anywhere in either the 2D or 3D viewport. Zoom in
`World3dHost` has no `onWheel` handler at all (only `Board2dHost`/`Paint2dHost`/`InkCanvasHost`/
`EngineCanvas` (wgpu) register `onWheel`) — meaning on a touch-only device with no trackpad/wheel,
**3D viewport zoom has no input path at all** (orbit/pan work via drag, but not zoom). The fixture
`🌐️World3dHost/🧫️fixtures/🖱️pointer-gestures.json` models pick/hover/orbit/marquee, all
mouse-semantics — no touch-specific case.

### Phone-width shell chrome — what's desktop-only

Dock anchors, the eight-panel layout, and drag-based window rearrange (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx`
per the wgpu file's own comment at `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:11007-11008`: *"`startTabDrag:
mobile ? noopDrag : startTabDrag`... because a mobile mode is ONE flat stack with nothing to drop
into"*) are explicitly desktop/tablet-only and intentionally disabled under the mobile breakpoint
rather than left broken — a deliberate, documented design choice, not an oversight.

---

## 4. Progress / cancellation

AGENTS.md: "You MUST support progress and cancellation for all expensive operations."

| operation | progress shown? | cancel available? | evidence |
|---|---|---|---|
| Actor-level tasks (kernel shards) | yes, real table (`🧵️TaskManager`) | yes — `suspend`/`resume`/`cancel` route through `ActivationRegistry` to a real `ShardClient` | **BUT the panel is not mounted as a real window anywhere yet** — `🧵️TaskManager/🟦️.tsx:22-24` own header: *"Mounting this as a real window (`host.open_window(...)`)... needs a window-kind registration and a `ShellHost` mount — both registrar-only"*. The one central progress+cancel surface for actor-level work exists in code but is **not wired into the running shell** for a user to ever see. |
| Plugin install | status text only (`ui.plugins.status.installing`, `📌️ChromePanels/🟦️.tsx:1235-1239`) | **no** — `📌️ChromePanels/🟦️.tsx:1275-1286` wires `install`/`reload`/`uninstall` buttons but no cancel-in-flight action | Plugin-load liveness (`🔌️PluginRuntime/🫀️load-progress/🟦️.ts`) is an internal idle/ceiling deadline clock, not a user-facing progress bar or cancel control. |
| Agent tool calls / inference | yes — `running`/`succeeded`/`failed` state per entry (`💬️AgentChatPanel/🟦️.tsx:46-53`) | **no** — no cancel button anywhere in `AgentChatPanel/🟦️.tsx` (137 lines, read in full) for an in-flight tool call, despite `mcp__semio__inference_cancel`/`job_cancel`/`action_cancel` existing as real MCP-layer tools per M2's report | Backend capability exists (job-registry-backed cancel per `📓️m2-agent-surface-and-inference.md`); **the end-user chat UI never exposes it**. |
| Document/window load | `ui.common.loadingSurface`/skeleton states exist | not found | Loading state is a skeleton/spinner; no cancel affordance located for a stuck document open. |
| Export (segmented download) | queue/chunk mechanism exists (`📤️SegmentedDownload/🟦️.ts`) | not confirmed | Logic-only `.ts` module (no companion `.tsx`); could not confirm a user-visible progress/cancel control from this file alone within audit time budget — flag for a deeper pass. |
| Hub bootstrap/rebootstrap | connection-status badge (`ShellSync`, see §5) | n/a (not a cancellable user action) | — |
| Mutation dispatch / undo-redo | immediate, not "expensive" in the progress sense | n/a | — |

**Section conclusion**: the codebase has the right primitives (job registry, `ActivationRegistry`
suspend/resume/cancel, MCP `*_cancel` tools, a liveness/deadline clock for plugin loads) but the
**last-mile UI wiring is the gap** — `TaskManager` unmounted, plugin install missing a cancel
button, and the agent chat panel not surfacing the cancel capability the backend already has.

---

## 5. Offline / reconnect UX

- **What the user sees during hub disconnect/reconnect**: `ShellSync/🟦️.tsx`'s `SyncAttachCard`
  footer popover (`syncStatusLabel`, lines 40-47) is the concrete surface — it renders
  `live · N peers` / `connecting…` / `reconnecting…` / `offline`, plus `saved`/`unsaved` and a
  pending-mutation count. This is real, wired state (`ArtifactSyncStatus`), not a stub — but (a) it
  is hidden inside a popover the user must open (not a persistent banner), and (b) as noted in §1.3,
  every one of those status words is hard-coded English, unlocalized.
- **Short connection loss not freezing the app**: `🔌️PluginRuntime`'s bounded-retry-with-backoff
  fix (`📓️c1-collaboration-e2e.md` §1, `PLUGIN_INSTANCE_BUSY_BACKOFF_MS = 25`,
  `isPluginInstanceBusyFaultV1`, `readHistoryWithBoundedRetryV1`) is the concrete mechanism keeping a
  transient hub hiccup from freezing the UI — confirmed real and tested per that report's §1.4
  renderer test. This audit did not re-run it (read-only, no builds), so it is corroborating, not
  independently re-verified.
- **Rejected commands**: `ShellHost/🟦️.tsx:8172-8180` (`showMutationRejectedNotice`) shows a
  transient toast with severity, localized mutation-code label, and (per §1.3 finding 3) an
  unlocalized English detail appended for the specific cause.
  `ui.mutation.rejected.title`/`.body` and 7 `ui.mutation.code.*` keys are all present with full
  en/de coverage (part of the compile-checked chrome bundle, §1.2).
  `ui.conflict.*` (panel/accept/discard/quarantined/degraded) similarly covers the
  first-class-conflict quarantine UI (`ShellSync/🟦️.tsx:86-90` renders `quarantinedLabel` count).
- **Fatal loss (worker died, cannot rebuild)**: `ui.common.workerLost` exists specifically for this
  ("the app instance died with its worker and the shell could not rebuild it — the user has to
  reload... never shown for a loss the shell recovered from on its own", `📚️I18n/🟦️.tsx:327-329`) —
  fully localized, and a `WindowFault` vocabulary (`ui.windowFault.*`, lines 310-318: abiMismatch,
  interactiveCeiling, clock, pluginInternal, installFailed, unknown) gives the shell a real,
  discriminated-cause, localized vocabulary for a window that faulted rather than a generic blank
  surface — this is a solid, thoughtful piece of offline/fault UX design already in place.
- **Gap**: no persistent, always-visible "hub connection" indicator was found outside the
  per-document `ShellSync` popover — a user with several documents open has no single place to see
  "the hub is unreachable" independent of opening each document's own sync card.

---

## Ranked fix list

**P0 — user-visible, backend capability already exists, only UI wiring is missing**
1. Mount `🧵️TaskManager` as a real window (`host.open_window("os.task-manager", …)` + `ShellHost`
   registration, per its own header's punch-list, `🧵️TaskManager/🟦️.tsx:22-24`) — today the only
   central progress+cancel surface for actor-level work is invisible to every user.
2. Add a cancel affordance to `AgentChatPanel/🟦️.tsx` for an in-flight `toolCall`/inference — the
   MCP-layer `inference_cancel`/`job_cancel`/`action_cancel` tools already exist (M2); wire one to a
   button next to the `running` state (line ~53-59).
3. Route `MutationKind::label()` (2690 call sites, worst offenders `📕️norm` 393, `🏛️architect` 268,
   `🧩️puzzle` 106, `🧱️block` 105) through `LocalizedLabel`/the TS translation system instead of a
   bare `String`, so the undo/history panel (`🕰️HistoryTable`) is not permanently English-only. This
   is a trait-signature change (`🎮️command/🦀️.rs:231`) with a very large blast radius — needs its own
   scoped ticket, not a quick patch.

**P1 — real, localized, but incomplete or partially wired**
4. Localize `ShellSync/🟦️.tsx:40-47`'s `syncStatusLabel` (`live`/`connecting…`/`reconnecting…`/
   `offline`/`saved`/`unsaved`/`pending`) through `useLabel` — small, contained fix, high visibility
   (this is the literal offline/reconnect status text).
5. Add a cancel button for an in-flight plugin install in `📌️ChromePanels/🟦️.tsx:1275` (status
   already distinguishes `installing`; only the cancel action is missing).
6. Add multi-touch pinch-zoom/two-finger orbit to `🌐️World3dHost`/`Board2dHost` (currently
   single-pointer only via `PointerEvent`, no `pointerId`-keyed multi-touch tracking anywhere) —
   and give `World3dHost` a wheel-equivalent zoom path for touch devices with no trackpad (currently
   no `onWheel` handler at all in that file).
7. Auto-detect "tablet" as a distinct breakpoint (currently `useUiMobile()` is binary; `tablet` is
   settings-only, never auto-selected) if a real tablet layout is wanted, per AGENTS.md's explicit
   desktop→mobile→tablet priority.
8. Add a persistent, always-visible hub-connection indicator independent of the per-document
   `ShellSync` popover (today a user must open a specific document's sync card to learn the hub is
   unreachable).

**P2 — smaller/lower-risk cleanups**
9. Add a live contrast-ratio check/warning to the theme editor (`settings.theme.*`) for
   user-customized colors — the generated default palette is tested ≥4.5:1 (`🎨️styling/🧪️tests/🧪️levels-oklabmix/🟦️.ts:70-75`)
   but arbitrary custom themes are not checked at save time.
10. Add basic keyboard/ARIA affordances to `🕸️Diagram` (node-graph editor, 0 role/kb hits in 1842
    lines) — already self-documented as a known gap in the shared a11y contract
    (`♿️accessibility/🦀️.rs:108-112`), but still unresolved.
11. Give the native (non-browser) wgpu winit build (`🎯️targets/🧊️wgpu/🪟️winit-app/🦀️.rs`) an
    accessibility story — the DOM-mirror only exists for the browser host.
12. Replace the presence-scope test harness's hard-coded English JSX
    (`🏛️ShellHost/👥️presence-scope/🌐️browser/🟦️.tsx:126,136,137`) — cosmetic, test-only, not
    user-facing.
13. Fix or document the `cause.message` English-prose leak in `ShellHost/🟦️.tsx:8176`'s rejected-
    mutation toast (currently an intentional, documented tradeoff — worth a decision either way once
    the guest-side `causes` wiring referenced in the same comment lands).
