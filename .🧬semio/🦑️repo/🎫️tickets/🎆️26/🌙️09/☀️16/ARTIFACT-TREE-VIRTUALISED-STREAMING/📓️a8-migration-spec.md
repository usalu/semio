# 📓️ A8 — the exact migration spec every plugin in this packet follows

Normative names come from `📓️design-virtualised-tree.md` §5 (SDK) and §8 (migration rule) and `📓️wave2-app-brief.md`.
Wave-1 (P1 contract / P3 SDK) is landing concurrently — **write code against these names, do not run cargo** until both
`📓️p1-contract.md` and `📓️p3-sdk.md` exist in this ticket folder. The A8 owner runs every build.

## SDK names you call (all re-exported from `semio_framework_plugin`)

```rust
pub const TREE_WINDOW_DEFAULT_ROWS: u32 = 48;
pub struct TreeSlice { pub open: bool, pub offset: usize, pub len: usize, pub total: usize }
pub struct TreeWindows<'a> { … }
impl<'a> TreeWindows<'a> {
    pub fn for_body(view: &'a ViewModel, body_key: &str) -> Self;
    pub fn unhosted() -> Self;
    pub fn is_open(&self, node_key: &str, default_open: bool) -> bool;
    pub fn slice(&self, node_key: &str, default_open: bool, total: usize) -> TreeSlice;
    pub fn window(slice: &TreeSlice) -> TreeWindow;
}
pub fn tree_window_section<T>(windows, id: &str, label: Label, default_open: bool, entries: &[T],
    row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;
pub fn tree_window_section_or_placeholder<T>(…, placeholder_label: Label) -> UiAssemblyResult<BuiltNode>;
pub fn tree_window_item<T>(windows, item: TreeItemBuilder, id: &str, default_open: bool, entries: &[T],
    row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<BuiltNode>;
impl PanelTreeBuilder {
    pub fn window_section<T>(self, windows: &TreeWindows<'_>, id: &str, label: Option<Label>, default_open: bool,
        entries: &[T], row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<Self>;
    pub fn window_section_or_placeholder<T>(…, placeholder_label: impl TryInto<Label>) -> UiAssemblyResult<Self>;
    pub fn interaction_domain(self, controller_id: &str, domain: impl TryInto<UiText>) -> UiAssemblyResult<Self>;
}
pub fn ui_node_list(values: impl IntoIterator<Item = UiAssemblyResult<BuiltNode>>) -> UiAssemblyResult<UiFixedList<BuiltNode>>;
```
`TreeItemBuilder::granularity(UiText)` and `::window(TreeWindow)` live on the UI contract builder.
`ViewModel` gains `pub tree_windows: Vec<TreeWindowRequest>` and `pub tree_viewport_rows: Option<u32>`;
`TreeWindowRequest { body_key: String, node_key: String, open: Option<bool>, offset: u32, rows: u32 }`.

## What you change, per panel

1. **Sections.** Every `PanelTreeBuilder::section(id, label, default_open, items)` whose `items` come from a
   document/config collection becomes `.window_section(windows, id, label, default_open, &entries, |entry| …)`;
   every `.section_or_placeholder(...)` becomes `.window_section_or_placeholder(...)`. Fixed-roster sections
   (catalogues over a compile-time array) migrate too — build a `&[Descriptor]` slice and window it. Sections whose
   children are a handful of hand-written heterogeneous rows keep `.section(...)` with `ui_node_list([...])` (there is
   no entry slice to window); that is the ONLY case that may stay unwindowed, and only when it is ≤ 8 static rows.
2. **Nested group rows.** Any row that nests children (`.try_children(...)`, `try_with_children`, `tree_group`,
   a recursive `Group`/`Boolean`/object›part walk) becomes `tree_window_item(windows, item_builder, node_key,
   default_open, &children, row)` — **at every level**, recursion included. `node_key` is the row's own authored id.
3. **Pick rows.** Where a tree already declared `.interaction_domain(DOMAIN)` and rows dispatched a per-row
   `interactionSelect` argument map (or a per-row select action carrying `targets`/`domainId`), delete the per-row
   binding and instead: `.interaction_domain(CONTROLLER_ID, DOMAIN)` on the builder + `.granularity(g)` on each row,
   row key = the RAW target id. `CONTROLLER_ID` is the id the app's own action factory uses (the same string the app
   passes to `ActionFactory::new(...)`, usually a `*_CONTROLLER_ID`/app id constant already in the crate — grep the
   crate's `*_action(...)` helper for it). `g` is the granularity string from the app's `InteractionDefinition`
   granularities (grep `InteractionDefinition` in the crate).
   Rows with their OWN app action (add/install/checkout/toggle/select-with-args) keep their binding untouched.
   Row actions (`RowAction`, hide/lock/delete) stay as they are. Draggable rows keep `draggable`/`drag_data`.
   Trees that deliberately bind NO domain (mixed id namespaces, catalogues) stay unbound — do not add one.
4. **Delete** the crate-local `fn ui_node_list` copy and import `semio_framework_plugin::ui_node_list` at every call
   site; delete every `.take(N)` list truncation, every static `+N`/`…more` row, every `*_ROWS`/`LIST_ROWS_MAX`/
   `SECTION_ROWS` constant, and any app-local `.chunks(UI_FIXED_LIST_ITEMS)`-into-sections idiom (architect) —
   replaced by ONE windowed section over the whole roster.
5. **Thread the windows.** In the app's `Present`/`ArtifactEditor` `fn render(body_key, doc, cfg, view_state)`
   dispatch, build `&semio_framework_plugin::TreeWindows::for_body(view_state, <THAT BODY KEY>)` per arm and pass it
   into the panel. Panels take `windows: &TreeWindows<'_>` as their last parameter.

## Worked example (already migrated — copy this shape)

`✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🗿️artifact/🦀️.rs`
and its `🧪️tests/🔬️unit/🦀️.rs` (window laws), plus the `SHOOTING_PLAY_BODY_*` arms in that editor's `🦀️.rs`.

## Tests — the four window laws

Add them to each **artifact / layers / document** panel's `🧪️tests/🔬️unit/🦀️.rs` (create the module and the
`#[cfg(test)] #[path = "🧪️tests/🔬️unit/🦀️.rs"] mod tests;` block in a `//#region 🧪️Tests` the way sibling panels in
the same crate do it, when the panel has none). Drive the panel's `render` DIRECTLY — no app fixture:

```rust
let view = ViewModel { tree_windows: requests, ..Default::default() };
let node = render(&snapshot, …, &TreeWindows::for_body(&view, BODY_KEY)).expect("render");
let json = semio_framework_plugin::artifact_app_laws::project_and_retire_fixture_tree(
    semio_framework_plugin::built_to_component_tree(node)).expect("project");
```

- **(a)** oversized document (a few hundred entries, built by a local `oversized_*` fixture fn): every container
  stamps `"total":<entries.len()>`, the body contains no `.more` key and no `"+` label, and the materialised row
  count is ≤ `TREE_WINDOW_DEFAULT_ROWS`.
- **(b)** `TreeWindowRequest{ open: Some(false) }` (or an authored `default_open: false` container): `total` stamped,
  zero children materialised.
- **(c)** `TreeWindowRequest{ offset: k, rows: n }`: exactly entries `[k, k+n)`, keyed by the raw entry id.
- **(d)** domain-bound trees: rows carry `granularity` and NO per-row activate binding, while the tree root carries
  exactly one `interactionSelect`. For a deliberately unbound tree, assert instead that it declares no domain, stamps
  no `granularity`, and that rows keep their own action.

## House rules

- `cd` into `/Users/ueli/Documents/semio` explicitly in every Bash call; quote every emoji path.
- Never `git stash/commit/checkout/reset`, never worktrees, never sweep any `🗑️generated` folder, never open/close a
  ticket, never edit `AGENTS.md`.
- No comments inside definitions; docstrings start with an emoji; concise code.
- Other agents are editing the framework and other plugins concurrently — ignore unrelated churn.
