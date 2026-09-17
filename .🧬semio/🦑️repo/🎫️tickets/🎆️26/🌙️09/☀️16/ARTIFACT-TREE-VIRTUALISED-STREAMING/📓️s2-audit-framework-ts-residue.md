# 📓️ S2 Audit — Framework/TS residue and design conformance (virtualised tree)

Scope audited: `🧰️framework/**`, all TS/TSX repo-wide (incl. plugin TS sides and `os/dev`), generated
schemas (`🧬️schema/*`, `🤖️generated/*`), JSON fixtures/manifests, probes/tests, docs. Excluded:
`✏️s/🔌️plugins/*/` Rust app code, `node_modules`, `dist`, build caches, `🗑️generated`, ticket folders.

## Verdict summary

**Wave-1 framework mechanism (design §3–§7) is fully implemented and matches the design doc closely.**
No residue of the old paging mechanism (`setPanelPage`, `panel_pages`, `paged_panel_section`,
`panel_continuation_row`, `panel_page_rows`, `PanelRowBudget`, `PANEL_RECONCILE_NODE_BUDGET` as a live
constant, `HISTORY_COMMAND_ROWS`, `history_page`, `.more` keys, `+N` labels) was found anywhere in scope.
The only two `setPanelPage` hits found in the whole repo are doc-comments inside excluded
`✏️s/🔌️plugins/` Rust app test files, describing the *old*, now-fixed behaviour — correctly out of this
audit's scope and not counted against the framework verdict.

Every §3–§7 normative item checked (contract fields, `ViewModel`, SDK `TreeWindows`/`tree_window_*`,
React `Tree.tsx` spacers, `Interpreter` observer + pick synthesis, `ShellHost`/`ShellHelpers` plumbing and
`WINDOW_CONFIG_RAIL_ACTION_IDS` cleanup, wgpu paint + reconcile read, and the 128/128/1 budgets +
reconcile size law) is **implemented**, with only cosmetic naming differences from the design's pseudocode
(noted below) — none of them functional gaps.

One adjacent finding for item 3: `🛂️SpaceAdministration/🟦️.tsx` (a framework OS panel, not a plugin)
still pages its members/invites lists with a cursor + a `+`/"more" button — a flat list outside the new
`TreeWindow` mechanism. It is not a `UiTree` and so is arguably outside this ticket's normative scope, but
it is exactly the residual pagination pattern item 3 asks about and is flagged for a scoping decision.

No other tree-like list UI in the framework (history panel — migrated; `UtilityTree`, `ShellSearch`,
`TaskManager`, `EventFeedHost`, `NodeGraph`, `BlockListHost` — spot-checked) still truncates or pages
outside the window mechanism.

---

## 1. Residue of the old mechanism

Searched repo-wide (outside plugin app code, node_modules, dist, `🗑️generated`, ticket folders) for:
`setPanelPage`/`SetPanelPage`, `panel_pages`/`panelPages`, `paged_panel_section`,
`panel_continuation_row`, `panel_page_rows`, `PanelRowBudget`, `PANEL_RECONCILE_NODE_BUDGET`,
`HISTORY_COMMAND_ROWS`, `history_page`, `.more` row keys, `+N` label patterns (`"+" +`, `` `+${ ``,
`format!("+{`), "continuation row", and any TS Tree "more" affordance.

| Term | Hits outside excluded scope | Classification |
|---|---|---|
| `setPanelPage` / `SetPanelPage` | 0 (2 hits, both in-comment, inside excluded plugin app tests — see below) | n/a |
| `panel_pages` / `panelPages` | 0 | — |
| `paged_panel_section` | 0 | — |
| `panel_continuation_row` | 0 | — |
| `panel_page_rows` | 0 | — |
| `PanelRowBudget` | 0 | — |
| `PANEL_RECONCILE_NODE_BUDGET` | 1, doc-comment only (see below) | test, historical |
| `HISTORY_COMMAND_ROWS` | 0 | — |
| `history_page` | 0 | — |
| `.more` continuation-row key | 0 (all "`more`"/"`.more`" hits are unrelated: context-menu overflow bucket, space-administration "more" button — see §3, i18n strings, keyboard-chord joins) | — |
| `+N` label patterns | 0 genuine hits (all `+${...}` hits are template concatenations unrelated to row counts: tool-run ids, keybinding chords, diff-line prefixes, plugin-size deltas) | — |
| "continuation row" | 0 | — |

Excluded-scope residue found (listed for completeness only, NOT counted against the framework verdict —
both are inside `✏️s/🔌️plugins/*/` Rust app code, out of this audit's mandate):
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs:198` — doc-comment: "closed with a `+N` whose `setPanelPage` cursor died with the command" (test, plugin app code, describes the pre-fix defect)
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs:121` — same pattern (test, plugin app code)

In-scope, non-residue hit:
- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️reconcile-unit/🦀️.rs:861` — doc-comment inside the
  **new** `🪟️TreeWindowReconcileSizeLaw` region: "the puzzle3d artifact panel whose stale
  `PANEL_RECONCILE_NODE_BUDGET = 16` this law replaces" (test, doc-comment describing the deletion, not
  a live constant — classification: test/historical reference, not residue)

Confirms design §8.4(d) is enforced by two live test assertions:
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs:4809` — `assert!(!json.contains(".more"), "no continuation row key survives: {json}");` (test)
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs:310` — identical assertion (test)

---

## 2. Design conformance, §3–§7

### §3 Contract (`semio_framework_ui_contract`)
**Implemented.** `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs:435` defines `TreeWindow { total, offset }`;
`:454` `TreeSectionProps.window: Option<TreeWindow>`; `:485` `TreeItemProps.window: Option<TreeWindow>`;
`:490` `TreeItemProps.granularity: Option<UiText>`. Constants confirmed exact:
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🏗️builder/🦀️.rs:51` — `UI_BUILT_CHILDREN_MAX: usize = crate::UI_DOCUMENT_NODES`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📃️document/🦀️.rs:95` — `UI_DOCUMENT_NODES: usize = 128`
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:55` — `UI_VALUE_PAGE_ROWS: usize = crate::UI_BUILT_CHILDREN_MAX` (= 128)
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🎬️action/🦀️.rs:66` — `UI_VALUE_LIVE_PAGES: usize = 1` (unchanged, per design)

TS mirror regenerated: `🧬️schema/🦀️.rs` (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧬️schema/🦀️.rs:857`) registers
`TreeWindow` in `TYPES`; typegen count test confirmed exact:
`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧪️tests/🧬️typegen-export/🦀️.rs:9` —
`assert_eq!(schema_metadata::TYPES.len(), 81);`
Generated TS (`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/📜️ui-contract/🟦️.ts:537`) —
`export type TreeWindow = { total: number, offset: number, };`, plus `window`/`granularity` on the row/section
types (lines ~499–529). **Implemented, matches design exactly.**

Note: conformance fixtures under `🧫️fixtures/🧪️conformance/…/🌳️tree*`
(`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧫️fixtures/🧪️conformance/🧩️component/🌳️tree/🎯️expect.json` and
`…/🖥️composite/🌳️tree-nested-sections/🎯️expect.json`) contain no literal "window" — these are structural
shape/accessibility fixtures with the new fields left `None` (skip-serialized), which is expected, not a
gap. Could not verify a literal "`page_arity`" budget-pin fixture referenced in
`audit-surface-budgets.md §6`; the string does not appear anywhere under `🧰️framework` — low confidence,
worth a follow-up grep with that audit doc's exact vocabulary in hand.

### §4 `ViewModel` (host → guest request)
**Implemented, matches exactly.** `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:4596` `TreeWindowRequest { body_key,
node_key, open: Option<bool>, offset, rows }`; `:4581` `ViewModel.tree_windows: Vec<TreeWindowRequest>`;
`:4587` `ViewModel.tree_viewport_rows: Option<u32>`. `for_panel()`/`for_window_instance()`
(`🦀️.rs:4640-4654`) both use `..self.clone()`, so both new fields survive projection, exactly as
specified. TS mirror (`🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts:977` `TreeWindowRequest`,
`:1453` `treeWindows: Array<TreeWindowRequest>`, `:1458` `treeViewportRows?: number`) present.

Checked for a second host builder that might drop the fields: the only other production `ViewModel`
constructor found is `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:4364`
`live_view_state`, which clones `session.view_state` and overrides unrelated fields — it does not
reconstruct `ViewModel` field-by-field, so `tree_windows`/`tree_viewport_rows` are not dropped (the wgpu
shell simply never populates them from its own input, which is the documented §6.4 gap, not a §4 bug). No
TS site constructs a literal `ViewModel` object either — the only production write path is the flattened
`treeWindows`/`treeViewportRows` fields in `🛠️ShellHelpers/🟦️.tsx` (see §6.3 below). `os/dev` builds no
`ViewModel` at all. **No second/blind host found.**

### §5 SDK (`semio_framework_plugin`, region `🔖️PanelKit`)
**Implemented, matches design almost verbatim.** In
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`:
- Region markers `//#region 🔖️PanelKit` (`:5680`) … `//#endregion 🔖️PanelKit` (`:6210`); no `🔖️PanelPaging`
  sub-region exists anywhere (search returned 0 hits).
- `TREE_WINDOW_DEFAULT_ROWS: u32 = 48` (`:5933`); `TreeWindows<'a>` (`:6002`), `for_body`/`unhosted` (`:6009-6018`);
  `tree_window_section`/`tree_window_section_or_placeholder`/`tree_window_item` (`:6144`, `:6158`, `:6184`);
  `ui_node_list` (`:6199`); `PanelTreeBuilder::window_section`/`window_section_or_placeholder`/`interaction_domain`
  (`:5839`, `:5849`, `:5888`).
- The old re-export at the design's "~39485" is replaced by
  `🦀️.rs:39621` — `pub use app::{tree_window_item, tree_window_section, tree_window_section_or_placeholder, ui_node_list, TreeSlice, TreeWindows, TREE_WINDOW_DEFAULT_ROWS, TREE_WINDOW_FIXED_NODE_HEADROOM};`
  (`TREE_WINDOW_FIXED_NODE_HEADROOM` is an implementation addition beyond the design's literal listing —
  a headroom constant used by the reconcile-cost accounting mentioned in the surrounding doc-comments;
  not a spec deviation, just an added implementation detail).
- `ui_history_panel` migrated off `HISTORY_COMMAND_ROWS`/`history_page`: `🦀️.rs:10336-10374` builds
  `TreeWindows::for_body(view, FRAMEWORK_HISTORY_BODY_KEY)` and calls `tree_window_section` for the
  Commands section, exactly as §5 specifies.
- `ui_tree_domain_topology` doc'd as seeing only materialised rows, and the "no app declares
  `HierarchyProvider::UiTree`" note is present verbatim at `🦀️.rs:655-657`.

### §6.1 `🌳️Tree/🟦️.tsx`
**Implemented, matches design.** `TreeDataItem.window?`/`TreeDataSection.window?` present (lines
~975-1002); `treeWindowDomAttributes()` (`:916`) stamps `data-tree-window-key/-total/-offset/-length`;
`treeWindowSpacer()` (~`:936`) renders `data-slot="tree-window-spacer"` divs at `rows × treeRowHeightPx`;
`treeWindowRequestsForViewport` pure helper present (~`:826`) with `TREE_WINDOW_OVERSCAN_ROWS`; loading-ring
state for an announced-but-unstreamed window (`isWindowPending`, lines ~3232, ~3329). Implementation adds
extra clamps beyond the design text (`capTreeWindowRequests`, `TREE_WINDOW_BODY_ROWS_MAX` — see §6.2), which
are hardening, not deviations.

### §6.2 `🗣️Interpreter/🟦️.tsx`
**Implemented.** `TreeWindowContext`/`useTreeWindowContext` (`:1468-1479`); `useTreeWindowObserver`
(`:1600`) wires `treeWindowRequestsForViewport` + `capTreeWindowRequests`/`TREE_WINDOW_BODY_ROWS_MAX` +
`reportWindows` (`:1623-1628`), reports `viewportRows` (`:1624`). Pick synthesis present:
`treePickTargetsV1`/`treePickIntentInputV1`/`dispatchTreePick` (`:1681-1712`), gated on `granularity` +
no own `activate` binding + tree-level pick binding (`:1753-1755`), using `interactionMergeFromModifiers`
exactly as specified. A dedicated test suite exists:
`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🧪️tests/🪟️tree-windows/🟦️.tsx`.

### §6.3 `🏛️ShellHost/🟦️.tsx` + `🛠️ShellHelpers/🟦️.tsx`
**Implemented, differs cosmetically from the design's pseudocode.** Design describes
`treeWindowsRef: Map<bodyKey, {...}>` living directly in `ShellHost`; the actual implementation factors
this into a `TreeWindowHostV1` abstraction (`🛠️ShellHelpers/🟦️.tsx:2087-2231`, `uiNodeToTreePanelConfig`
at `:2250`, `panelTreeConfigCacheRef` caching keyed on an `openSignature`) consumed from `ShellHost`
(`🏛️ShellHost/🟦️.tsx:8809-8885`, threading `treeWindowHost`/`treeWindowGeneration` into the per-tab memos).
Functionally equivalent to the spec: `setOpen`/`reportWindows` per body key, `viewStateFields()` flattening
into `treeWindows`/`treeViewportRows` (`ShellHelpers/🟦️.tsx:2184`). `WINDOW_CONFIG_RAIL_ACTION_IDS`
(`ShellHelpers/🟦️.tsx:4853-4875`) contains no `setPanelPage` entry — confirmed clean. A dedicated test
suite exists: `🛠️ShellHelpers/🧪️tests/🪟️tree-windows/🟦️.tsx`. Did not independently verify the specific
40 ms debounce constant or the "narrow the `panelUiByKey` `useMemo`s to per-tab" line numbers the design
cites (~8727/8779/8801); the memos at `ShellHost/🟦️.tsx:8857/8862/8885` do appear per-tab-scoped and
depend on `treeWindowHost`/`treeWindowGeneration`, consistent with the intent — low-confidence on the
exact debounce value only.

### §6.4 wgpu
**Implemented, including the documented gap.** `🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs`: `tree_window_pitch`
(`:98`) paints leading/trailing spacer pitch at `TREE_ROW_HEIGHT`; `tree_item_expandable` (`:105`) treats
`window.total > 0` as expandable; doc-comment at `:93-96` states verbatim: "the wgpu shell has no
equivalent [viewport observer], so a wgpu tree shows only whatever first-paint window its guest chose and
scrolling into a spacer band [...]" — matches design's "No host-side request wiring for wgpu this ticket
(documented gap; no `+N` fallback)" exactly. Reconcile
(`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs:479-518,794`) reads `TreeItemProps.window`/
`TreeSectionProps.window` and projects to `UiTreeWindow` for the painted node.

### §7 Budgets
**Implemented, exact values confirmed** (see §3 table above): `UI_BUILT_CHILDREN_MAX` 128,
`UI_VALUE_PAGE_ROWS` 128, `UI_VALUE_LIVE_PAGES` 1. `PANEL_RECONCILE_NODE_BUDGET` fully deleted (0 live
hits). New reconcile size law present and named as designed:
`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️reconcile-unit/🦀️.rs`, region `//#region 🪟️TreeWindowReconcileSizeLaw`
(~`:858`), with `TREE_WINDOW_LAW_SECTIONS = 4`, `TREE_WINDOW_LAW_SECTION_TOTAL = UI_BUILT_CHILDREN_MAX`,
and a `TREE_WINDOW_LAW_PAGE_ROWS` computation that walks down until admission accepts the page — matches
the ¶"asserts a 128-row window × 4 sections reconciles under `SURFACE_RECONCILE_SURFACE_BYTES`" intent.

---

## 3. Other tree-like list UI still truncating/paging outside the window mechanism

- **`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛂️SpaceAdministration/🟦️.tsx`**
  (production, framework OS panel — not a plugin, not excluded):
  - `:407-413` — members list: `{members.nextCursor === undefined ? null : (<Button ... onClick={() => onIntent({ kind: "page", cursor: members.nextCursor as string })}>{labels.more}</Button>)}`
  - `:449-454` — identical pattern for the invites list.
  - This is a flat `<ul>` list with a manual "load next page" button and a host-round-trip `page` intent
    carrying a `nextCursor` — structurally the same "+N continuation" shape the ticket eliminated for
    trees, just never converted to (or covered by) the `TreeWindow` mechanism, and not called out in the
    design's wave-2 migration list. It is not a `UiTree`, so it is arguably legitimately out of this
    ticket's scope, but it is the one concrete match for audit item 3's "other tree-like list UI ... that
    still truncates or pages." Flagging for a scoping decision rather than calling it a design gap.
- **History panel** (`ui_history_panel`): migrated onto `tree_window_section`, confirmed in §2 above — not
  a gap.
- Spot-checked and found clean (no row-count truncation/paging, only CSS text-truncate or unrelated
  `.slice()`/`.take()` usages for strings/labels): `🎛️UtilityTree/🟦️.tsx`, `🔎️ShellSearch/🟦️.tsx`,
  `🧵️TaskManager/🟦️.tsx`, `📡️EventFeedHost/🟦️.tsx`, `🕸️NodeGraph/🟦️.tsx`, `🧩️BlockListHost/🟦️.tsx`,
  `💬️AgentChatPanel/🟦️.tsx`. No dedicated Layers/Outliner/File-list/Command-palette panel component
  exists in the framework itself (those live in plugin app code, out of this audit's scope).

## Low-confidence / not fully verified (time-boxed out)
- Exact `🛠️ShellHelpers` 40 ms debounce constant and the precise `~8727/8779/8801` line numbers the design
  cites for the `panelUiByKey` memo narrowing (functionally present, exact original line numbers not
  re-verified against the design's now-stale references).
- Literal `page_arity` fixture-pin string from `audit-surface-budgets.md §6` — not found verbatim under
  `🧰️framework`; the 128-value constants are independently confirmed correct, but the specific named
  fixture/test this refers to was not located in the time available.
