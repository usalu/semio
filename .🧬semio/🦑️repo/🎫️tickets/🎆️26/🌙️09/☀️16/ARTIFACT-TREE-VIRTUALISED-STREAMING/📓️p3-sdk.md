# 📓️ P3 — Plugin SDK tree windows (design §5), final API

Packet P3 of ARTIFACT-TREE-VIRTUALISED-STREAMING. Everything below is landed in
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, region `🔖️PanelWindowing` (inside `🔖️PanelKit`),
which REPLACES the deleted region `🔖️PanelPaging`. Wave-2 app packets call exactly these names.

## 1. Public API (verbatim signatures)

```rust
pub const TREE_WINDOW_DEFAULT_ROWS: u32 = 48;

pub struct TreeSlice {
    pub open: bool,
    pub offset: usize,
    pub len: usize,
    pub total: usize,
}

pub struct TreeWindows<'a> { /* requests: Vec<&'a TreeWindowRequest>, budget: Cell<u32> */ }

impl<'a> TreeWindows<'a> {
    pub fn for_body(view: &'a ViewModel, body_key: &str) -> Self;
    pub fn unhosted() -> Self;
    pub fn is_open(&self, node_key: &str, default_open: bool) -> bool;
    pub fn slice(&self, node_key: &str, default_open: bool, total: usize) -> TreeSlice;
    pub fn window(slice: &TreeSlice) -> TreeWindow;
}

pub fn tree_window_section<T>(windows: &TreeWindows<'_>, id: &str, label: Label, default_open: bool, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;

pub fn tree_window_section_or_placeholder<T, L: TryInto<Label>>(
    windows: &TreeWindows<'_>,
    id: &str,
    label: Label,
    default_open: bool,
    entries: &[T],
    row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>,
    placeholder_label: L,
) -> UiAssemblyResult<BuiltNode>;

pub fn tree_window_item<T>(windows: &TreeWindows<'_>, item: TreeItemBuilder, id: &str, default_open: bool, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;

pub fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<UiFixedList<BuiltNode>>;

impl PanelTreeBuilder {
    pub fn window_section<T>(mut self, windows: &TreeWindows<'_>, id: &str, label: Option<Label>, default_open: bool, entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<Self>;

    pub fn window_section_or_placeholder<T, L: TryInto<Label>>(
        mut self,
        windows: &TreeWindows<'_>,
        id: &str,
        label: Option<Label>,
        default_open: bool,
        entries: &[T],
        row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>,
        placeholder_label: L,
    ) -> UiAssemblyResult<Self>;

    pub fn interaction_domain(mut self, controller_id: &'static str, domain: impl TryInto<UiText>) -> UiAssemblyResult<Self>;
}

pub async fn ui_history_panel(history: &HistoryView, controller_id: &str, is_de: bool, read_only: bool, view: &ViewModel) -> UiAssemblyResult<BuiltNode>;
```

Crate-root re-export (replaces `pub use app::{paged_panel_section, panel_continuation_row, panel_page_rows};`):

```rust
pub use app::{tree_window_item, tree_window_section, tree_window_section_or_placeholder, ui_node_list, TreeSlice, TreeWindows, TREE_WINDOW_DEFAULT_ROWS};
```

`PanelTreeBuilder` stays exported as before; `PanelRowBudget` was removed from the big `pub use app::{…}` type list.

### Notes for callers
- `controller_id` on `interaction_domain` is `&'static str` (that is what `ActionFactory::new` takes). Every
  plugin's `CONTROLLER_ID` const already satisfies it.
- `tree_window_section*` take `label: Label` (not `Option<Label>`); the `PanelTreeBuilder` methods take
  `Option<Label>` to match the existing `.section()`/`.section_or_placeholder()` spelling.
- `placeholder_label` is the LAST parameter, after `row` (design §5 ordering).
- `tree_window_item` takes an already-configured `TreeItemBuilder` (its id/label/icon) plus the `id` used as
  the window's node key; it applies `.default_open(default_open)` itself.

## 2. Semantics

`slice(node_key, default_open, total)`:
- `open = request.open` if the host filed one for this node key, else `default_open` (host wins).
- Closed → `TreeSlice { open: false, offset: 0, len: 0, total }`.
- Open **with** a request → `offset = min(req.offset, total.saturating_sub(1))`,
  `len = min(req.rows, UI_BUILT_CHILDREN_MAX, total - offset)`.
- Open **without** a request (first paint, or a container the host has not seen) → `offset = 0`,
  `len = min(total, UI_BUILT_CHILDREN_MAX, budget_remaining)` and the shared first-paint budget is
  decremented by `len`. The budget is `ViewModel::tree_viewport_rows.unwrap_or(TREE_WINDOW_DEFAULT_ROWS)`,
  held in a `Cell` on the `TreeWindows` value, so it is spent in document order across every container built
  from the SAME `TreeWindows` — nested containers included. Build one `TreeWindows` per body per render and
  thread it everywhere.

Row materialisation:
- Rows are pushed straight into the section/item builder's children via `try_child` — there is NO intermediate
  `UiFixedList`, so the old 32-item intermediate ceiling is gone.
- A `row` refused with code `ui.fixed-capacity` (or a full `BuiltChildren`) ENDS the window early with a
  shorter materialised run and never faults. Any other error propagates.
- `window` is stamped whenever `total > 0` OR a request exists for the node key, so the host always learns the
  full extent even when zero rows are materialised (closed container, or budget exhausted).
- `tree_window_section_or_placeholder` with empty `entries` emits one `{id}.empty` placeholder row and stamps
  NO window (an empty container has no extent to publish).

`interaction_domain(controller_id, domain)` now does two things: it sets `Component::Tree.interaction_domain`
as before, AND stamps exactly ONE tree-level `Trigger::Activate` binding
`ActionFactory::new(controller_id).action(INTERACTION_SELECT_ACTION_ID, Some({ "domainId": domain }))` on the
tree root. Pick rows therefore carry `TreeItemBuilder::granularity(..)` and NO binding and NO argument map —
they cost zero `UiValue` arena, which is what makes a 128-row window affordable.

## 3. Removed symbols

| Removed | Replacement |
| --- | --- |
| region `🔖️PanelPaging` | region `🔖️PanelWindowing` |
| `panel_page_rows()` | — (the window is clamped by `UI_BUILT_CHILDREN_MAX`; `ui_value_headroom` stays the runtime valve via the `ui.fixed-capacity` early end) |
| `PanelRowBudget` (`new`/`spend`/`remaining`/`nested`) | `TreeWindows`' shared first-paint budget |
| `panel_continuation_row()` | — (no `+N` anywhere) |
| `paged_panel_section()` | `tree_window_section` / `tree_window_section_or_placeholder` |
| `pub use app::{paged_panel_section, panel_continuation_row, panel_page_rows};` | the new re-export above |
| `PanelRowBudget` entry in the crate-root `pub use app::{…}` list | — |
| `HISTORY_COMMAND_ROWS` | `TREE_WINDOW_DEFAULT_ROWS` via the window |
| `page_history_command_nodes()` | — (a window is at most one `BuiltChildren` page, so no `N`-ary page tree) |
| `ui_history_panel(.., command_page: u32)` | `ui_history_panel(.., view: &ViewModel)` |
| `VcsArtifactApp::history_page` field (decl, init, dispatch writes) | host-owned `ViewModel::tree_windows` |
| `setHistoryCommandFilter`'s `page` argument handling | — (the action definition only ever declared `value`; the filter itself is unchanged) |
| the history panel's `+N` "More" row and its `framework.history.commands.more` key | — |
| fixture `🧫️fixtures/history-panel-command-pages/🔣️.json` | `🧫️fixtures/history-panel-command-window/🔣️.json` |
| law `ui_history_panel_pages_command_rows_from_the_live_count` | `ui_history_panel_windows_command_rows_over_the_live_count` |
| law `ui_history_panel_bounds_revert_row_actions_to_the_arena_page` | `ui_history_panel_keeps_every_materialised_revert_inside_the_arena_page` |

`history_panel_icon_id` was de-`async`ed (pure total match) so it can be called from the synchronous row
closure; it is private to the module and had exactly one caller.

## 4. Docstring notes added (design §5 last paragraph)

`ui_tree_domain_topology` and `VcsArtifactApp::stamp_and_cache_interaction_ui` both now state that a windowed
tree feeds only its MATERIALISED rows into the derived `DomainTopology` — harmless because no app in the fleet
declares `HierarchyProvider::UiTree`; a domain that needs the complete hierarchy declares
`HierarchyProvider::Topology`.

## 5. Tests

New/updated laws in `🧪️tests/🔬️app-panel-kit/🦀️.rs` (`panel_kit_tests`):
- `a_closed_container_materialises_no_rows_and_still_stamps_its_total`
- `a_host_request_materialises_exactly_its_slice_and_opens_a_closed_author_default`
- `a_first_paint_spends_one_shared_viewport_budget_in_document_order`
- `a_window_never_exceeds_one_built_children_page`
- `a_refused_row_shortens_the_window_instead_of_faulting`
- `interaction_domain_stamps_one_tree_level_activate_binding`
- `ui_history_panel_windows_its_commands_without_a_continuation_row`
- `panel_tree_builder_produces_a_namespaced_tree_with_placeholder` (updated to the two-arg `interaction_domain`)

In `🧪️tests/🔬️plugin-runtime-plugin-builder-contract/🦀️.rs`:
- `ui_history_panel_windows_command_rows_over_the_live_count` (replaces the pager law; reads the new fixture)
- `ui_history_panel_keeps_every_materialised_revert_inside_the_arena_page` (replaces the `UI_VALUE_PAGE_ROWS` pager law)
- every `ui_history_panel(..)` call site now passes `&ViewModel::default()` / a `ViewModel` carrying `tree_windows`
- `TreeItemProps`/`TreeSectionProps` literals filled with P1's new `window`/`granularity` fields

⚠️ Two deliberate robustness decisions, learned from red runs:
1. `BuiltChildren` refuses a direct `serde` walk ("requires retained page transport"), so the "no `.more` /
   no `+`" laws project the tree node by node (`panel_body_json` / `history_body_json`) instead of
   `serde_json::to_string(&panel)`.
2. The `UiValue` argument arena is process-global and the test binary runs tests in PARALLEL, so a law that
   pins an exact row count uses NON-revertible rows (zero argument arena). Only the dedicated revert law
   builds argument maps, and it asserts `actions == rows` / `actions <= UI_VALUE_PAGE_ROWS` rather than a
   fixed count. Likewise the shared-budget law uses 30-entry sections so it does not entangle the separate
   `UI_BUILT_CHILDREN_MAX` ceiling, which has its own dedicated law.

## 6. Verification (run 2026-09-17, foreground, shared build dir, no `CARGO_TARGET_DIR`)

All commands prefixed `DEVELOPER_DIR=/Library/Developer/CommandLineTools`.

| Command | Result |
| --- | --- |
| `cargo check -p semio-framework-plugin` | ✅ `Finished dev profile in 25.41s`, 0 errors |
| `cargo check -p semio-framework-plugin --target wasm32-wasip2` | ✅ `Finished dev profile in 9.23s`, 0 errors (wasm32-wasip2 debug artifacts verified newer than the last source edit) |
| `cargo test -p semio-framework-plugin --lib panel_kit_tests` | ✅ **11 passed; 0 failed** (773 filtered out) |
| `<test binary> ui_history_panel` | ✅ **6 passed; 0 failed** (778 filtered out) — the 2 new window laws, the 3 pre-existing clip/filter laws, and the panel-kit history law |

The 11 panel-kit laws cover the packet's required cases (a)–(f):
(a) `a_closed_container_materialises_no_rows_and_still_stamps_its_total`;
(b) `a_host_request_materialises_exactly_its_slice_and_opens_a_closed_author_default`;
(c) `a_first_paint_spends_one_shared_viewport_budget_in_document_order` + `a_window_never_exceeds_one_built_children_page`;
(d) `a_refused_row_shortens_the_window_instead_of_faulting`;
(e) `interaction_domain_stamps_one_tree_level_activate_binding`;
(f) `ui_history_panel_windows_its_commands_without_a_continuation_row`.

### ⚠️ The FULL `cargo test -p semio-framework-plugin` does not complete — for reasons outside P3

Running the whole binary aborts (SIGABRT, exit 134) after ~105 tests with:

```
thread 'component::app::mutation_fixture::dummy::assert_two_instances_converge_on_disjoint_edits' has overflowed its stack
fatal runtime error: stack overflow, aborting
```

and 8 tests fail before that, all with the same peer-owned cause — the interactive-job classification gate
refusing a `resize` command declared in `🧪️tests/🔬️app-app-builder/🦀️.rs`:

```
app-definition.interactive-job-classification: unclassified interactive command 'main:resize';
unclassified interactive command 's.test.app-builder.args-app@1/*#editor:resize'
```

Failing names: `app_builder_tests::{action_args_attaches_declared_arguments,
build_definition_accepts_app_and_mode_scope_commands, build_definition_accepts_tutorial_with_declared_action_utility_and_gesture_targets,
build_definition_accepts_introduction_with_declared_window_utility_and_action_targets,
build_definition_derives_command_owner_from_structural_containment, declaring_dialog_appends_to_definition,
operation_view_and_shell_actions_are_declared_with_their_kind}` and
`child_member_registry_tests::owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally`.

None of these touch panel trees, `PanelTreeBuilder`, `ui_history_panel`, or any symbol this packet added or
removed; P3 declares no commands and no window kinds. They are the concurrent classification/convergence work,
not this packet, and they were failing before P3's laws were added.

### ⚠️ A peer currently holds the contract constants at a bisect value

At the time of this run `🧰️framework/🔨️modules/🖱️ui/🧬️contract/` reads:

```rust
pub const UI_BUILT_CHILDREN_MAX: usize = 32;  // [DEBUG] temporary bisect
pub const UI_VALUE_PAGE_ROWS: usize = 31;     // [DEBUG] temporary bisect
```

i.e. someone has temporarily reverted design §7's `128`/`= UI_BUILT_CHILDREN_MAX`. P3 does NOT touch those
constants (they belong to P1/§7). Every P3 law was written to derive from the LIVE constants
(`TREE_WINDOW_DEFAULT_ROWS.min(UI_BUILT_CHILDREN_MAX)`, a dedicated `a_window_never_exceeds_one_built_children_page`
law, 30-entry budget sections) so they stay green under both `32` and the design's `128`. Nothing needs
revisiting here once the bisect is reverted.

## 7. Outstanding

- App crates under `✏️s` do not compile until wave 2 migrates them (expected; P3 touched no app crate, and the
  removed `paged_panel_section`/`panel_page_rows`/`PanelRowBudget` re-exports are what they will fail on).
- The full plugin-crate suite cannot be reported green until the two peer-owned faults above are fixed (the
  `resize` classification gate and the `mutation_fixture::dummy` stack overflow). P3's own laws and both
  `cargo check`s are green.
- `ui_node_list` now exists once in the SDK; the ~35 per-app copies are deleted by wave 2, not here.
