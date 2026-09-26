# Dock Accessibility, Label Lifecycle, and Surface Census Audit

Read-only source audit on 2026-09-26. No browser journey, native run, Cargo invocation, or Nx target ran. Findings describe the inspected source only; they are not runtime-acceptance claims.

## Scope and ownership

This packet audits the WGPU Dock's accessible semantics, drag acceptance, the accepted-frame control-name lifecycle, and the current `dumpChrome` surface census. It excludes the Dock name implementation and role metadata work owned by the integration owner, the Tree/Dock inline toolbar owned by SolTree, and Marketplace work owned by SolSelect.

## React contracts and current WGPU facts

### Mode-Dock window tabs

React's mode dock has a real tab relationship:

* `Canvas/🟦️.tsx:1046-1053` renders the selectable window label as `button[role="tab"]`, with `aria-selected`, `aria-controls`, and roving `tabIndex`.
* `Canvas/🟦️.tsx:1109` owns those controls under `role="tablist"`; `Canvas/🟦️.tsx:1207-1210` publishes the paired `tabpanel`.
* The current React component test requires each mode-dock label to be a tab, and its Focus and Close chips to be separately named buttons (`Window/🧪️tests/🧩️component/🟦️.tsx:80-87`).

The WGPU Dock painter registers the matching selectable label as `dock.tab.<path>.<windowId>`, but assigns it `HitKind::Window` (`Dock/🎯️targets/🧊️wgpu/🦀️.rs:1736-1740`). The generic chrome projection maps only `HitKind::PanelTab` to `tab`; `Window` falls through to `button` (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:30927-30939`). It presently computes no selected state for the Dock window-label ID (`:20132-20140`, `:31063-31088`). This is an accessible role-and-state mismatch, separate from the corrected names.

The owned metadata slice needs to preserve React's two structures: the selectable window label is a selected tab, and the focus/close/drag chips are independent controls. The selected value must come from the Dock stack's active window, and the control relationship must remain scoped to the accepted frame. This audit does not prescribe the owner's representation.

### Panel-anchor tabs are not mode-dock tabs

React's `PanelTabBar` deliberately uses a different pattern. It is a `button` with `aria-pressed={isActive}` because pressing the active item folds the panel; the component documents the button-pattern reason (`PanelTabBar/🟦️.tsx:391-395,422-436`). WGPU's extra projected rows for open panel anchors currently publish `role: "tab"` and `selected` (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:31102-31138`). The projection contract exposes `checked` and `selected`, but no `pressed` value (`ui/🧬️contract/♿️accessibility/🦀️.rs:158-205`); the browser mirror correspondingly supports `aria-checked` and `aria-selected` only (`wgpu/♿️accessibility-mirror/🟦️.ts:80-90`).

This proves a second semantics mismatch. It must not be "fixed" by making all Dock controls tabs: a panel-anchor control needs the React button/toggle contract, while the mode-dock window label needs the tab contract above.

### Drag and drop acceptance

The window-tab drag behaviour is already source-aligned. React starts it from `DragHandle` only (`Canvas/🟦️.tsx:1099-1103`), while a click on the label selects. WGPU's pointer path recognises only `dock.tab.<path>.<windowId>.drag` as the pending-drag source and explicitly leaves the label as selection on release (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:13664-13696`). The drag promotes only after more than 5px movement and clears on cancellation (`:13841-13850`, `:16431-16434`). The current native input test covers arm, promotion, non-mutating ghost state, and cancellation (`Shell/🧪️tests/🔬️wgpu-shell-input/🦀️.rs:1206-1228`).

No remaining source-only Dock drag acceptance gap was found. A metadata change must retain this test and add a painted Dock-path test only if the owner changes hit kinds or control IDs.

## Accepted-frame label map leaks its capacity

`CHROME_CONTROL_NAMES` is a single process/thread-local `BTreeMap`, with a 512 entry guard (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:30870-30908`). `note_chrome_control_name` admits a new record only while the map is below capacity; an existing key can overwrite its old value. Production code has no clear operation. The existing tests explicitly call `names.clear()` before their fixtures (`Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:481-487,774-816`), so they do not cover the production lifecycle.

One chrome walk already has a correct staging boundary: `FrameSetup` step zero clears the staged hit/owner maps and begins the accessibility-visible-document candidate (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:23976-23982`). The walk records labels, `seal_presented_input_candidate` binds the candidate to its witness (`:14039-14051`), and only the accepted witness derives chrome accessibility from the current hits and publishes it (`:14065-14093`). Presentation is serial: the presenter verifies the candidate, waits for it to be ready, then acknowledges it while holding the runtime (`wgpu/🧊️renderer/🦀️.rs:16093-16139`). Rejected work uses the matching discard door (`Shell/🎯️targets/🧊️wgpu/🦀️.rs:14097-14105`).

Consequences of the missing reset are bounded but user-visible after enough churn:

1. A discarded or now-occluded ID is not independently projected because the projection iterates current accepted hits. It therefore does not create a stale hidden accessibility element by itself.
2. Its map entry survives. After 512 distinct previous IDs, any new currently painted ID is refused and falls back to `humanize_control_id`, rather than reporting the painted/localized label.
3. A current ID that already exists does overwrite, so this is capacity starvation for new controls rather than a persistent same-ID label error.

The correct lifecycle boundary is the start of the staging walk, alongside the other staging-map clears, before the first current-frame naming call. Clearing at acknowledgement would destroy candidate labels before they are projected. The accepted projection already persists independently in `presented_chrome_accessibility`, so it does not need this mutable staging map after acknowledgement. Clearing on the matched discard is optional resource hygiene; the next walk's mandatory reset is the correctness boundary.

Required tests for the owner:

1. Fill a candidate with 512 unique labels, discard it, start a successor walk, and prove a new successor label appears rather than its humanized ID.
2. Prove an ID that exists only in the discarded candidate is absent from the next accepted projection.
3. Accept frame A, stage and discard B, then prove A's stored accessibility projection remains unchanged until C is accepted.
4. Use the real Dock paint route to assert a window-label accessible row has React's tab role and selected value, while its focus, close, and drag rows remain separate labelled controls.
5. Use the React component test as the browser oracle and the WGPU mirror/projection as the native projection oracle. Test both English and German action labels from the existing localization fixture.

## Current surface census

The current census is deliberately a level census, not a generic overlay enumeration. After an accepted frame it records:

* each entry in `dock_window_plan` as `window`;
* each open anchor's active tab as `panel`, using React's `panelTabElementId`; and
* the tour, Search, and Find overlays as `dialog`.

That implementation is `Shell/🎯️targets/🧊️wgpu/🦀️.rs:14119-14148`; publication occurs only after the same accepted input witness (`:14065-14093`). The current focused test proves pane/window rows and open-anchor panel rows with React's element IDs (`Shell/🧪️tests/🔬️wgpu-panel-anchor-model/🦀️.rs:431-450`). React has corresponding `data-level` declarations for Canvas window surfaces (`Canvas/🟦️.tsx:1230-1248`), Panels (`Panel/🟦️.tsx:566-570`), and Dialog (`Dialog/🟦️.tsx:409-414,541-564`).

This audit found no current source-only missing `window`, `panel`, or `dialog` row under that stated census contract. Dropdowns, context menus, tooltips, and drag previews are not part of the three-level census; a future requirement to inventory every transient overlay needs a new schema and React oracle, rather than quietly changing the existing metric. No live `dumpChrome` result was collected.

## Executable priority

1. **Accepted-frame names:** clear the mutable map at staging-walk start and add the discard/capacity regression tests. This protects every future chrome label, including dynamically created controls.
2. **Dock semantics:** the owner can extend its metadata slice using the mode-dock versus panel-anchor facts above. Add role/state parity tests before coding, preserving the existing drag path.
3. **Census:** retain the current source-backed three-level test. Do not schedule a surface-producer port from this audit.
