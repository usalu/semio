# 📓️ F2 — SDK body-wide node ledger, request priority, key uniqueness

Packet **F2** (SDK + framework Rust), 2026-09-17. Normative inputs: `📓️wave4-resume-brief.md`,
`📓️design-virtualised-tree.md` §3–§5/§7, `📓️p1-contract.md`, `📓️p3-sdk.md`, `📓️p5-wgpu-and-reconcile-law.md`,
`📓️w3-browser-verification.md` §6, and the coordinator's post-S3 decisions (`📓️s3-review-streaming-loop.md`
findings 1, 3, 4).

> ⏳️ **This file is being written while the verification runs finish.** The §6 run table is authoritative;
> anything not listed there was not run.

## 1. What the packet had to close

`📓️w3-browser-verification.md` §6.2 caught the fem3d **House** artifact panel surface faulting at
`1:framework.panel.artifact — nodes: 129 vs max_nodes: 128`. Per-container windowing bounds each
*container* at `UI_BUILT_CHILDREN_MAX`; nothing bounded the **body**. A previous agent had started a body
ledger inside `TreeWindows` and died before verifying it; S3 then found that ledger and the host's request
cap were two independently-authored numbers that do not compose (finding 1), that request order is the
host's alphabetical key order and not document order (finding 3), and that `node_key` collisions are
unenforced (finding 4).

## 2. Changes

### 2.1 One budget, defined once — `semio_framework_ui_contract`

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` (next to `TreeWindow`):

```rust
pub const TREE_WINDOW_FIXED_NODE_HEADROOM: usize = 16;
pub const TREE_WINDOW_BODY_NODE_BUDGET: usize = crate::UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM; // 111
```

The headroom **moved** out of the plugin SDK into the contract, so the derivation lives in one place and
both sides of the wire read the same number. The SDK now re-exports them
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, region `🔖️PanelWindowing`, plus the crate-root
`pub use app::{…, TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_DEFAULT_ROWS, TREE_WINDOW_FIXED_NODE_HEADROOM}`),
so `semio_framework_plugin::TREE_WINDOW_FIXED_NODE_HEADROOM` still resolves for every existing caller.

The headroom's value is justified on the constant itself: the framework's own `ui_history_panel` is the
widest un-windowed block in the fleet (an Actions section wrapping undo, redo and three filter rows, plus
its own section node ≈ 7 records) and the fem/cad/puzzle outliners add none; 16 doubles that.

### 2.2 Cost model — `1 + rows` per container, every node charged exactly once

`TreeWindows` (same region) is now:

```rust
pub struct TreeWindows<'a> {
    requests: Vec<(&'a TreeWindowRequest, Cell<usize>)>, // request + its outstanding reservation
    budget: Cell<u32>,                                   // shared first-paint viewport allowance
    nodes: Cell<isize>,                                  // the body ledger, may go negative
    reserved: Cell<usize>,                               // records held for requests not yet reached
    claimed: RefCell<Vec<String>>,                       // node keys this body has published
    nested: Cell<bool>,                                  // "we are inside a parent's row closure"
}
```

- `debit_container()` charges **1** for a container's own node — *unless* `nested` is raised, i.e. the
  container is being built as a row of an enclosing window, in which case its parent's row grant already
  paid for it. `tree_window_rows` raises and restores `nested` around each `row(entry)` call. This fixes the
  "a nested `tree_window_item` is charged twice" under-materialisation S3 quoted.
- `grant(want)` takes rows off the ledger; `debit(n)` is unconditional (a container node is not optional
  once the panel decided to build it), so an overdrawn ledger simply grants nothing afterwards and
  `nodes_remaining()` saturates at 0.
- Law `a_nested_container_is_charged_exactly_once` pins the equality
  `TREE_WINDOW_BODY_NODE_BUDGET - nodes_remaining() == body_nodes(built)`.

### 2.3 Request priority — order independence (S3 finding 3)

The host's request order is its node keys sorted **alphabetically** (wire byte-stability), not document
order, so document-order charging could starve a visible container behind an off-screen one that merely
sorts earlier. The ledger is therefore split at construction, before any container exists:

- `TreeWindows::seated` reserves `1 + min(rows, UI_BUILT_CHILDREN_MAX)` per request, **in request order**,
  until the budget is exhausted. A host that over-asks is clamped there — deterministically, tail last.
  `open: Some(false)` reserves nothing.
- An **unrequested** container (first paint) is served by `grant_unreserved` — out of
  `nodes_remaining() - reserved` only — and additionally clamped by the shared viewport `budget`.
- A **requested** container releases its own reservation when the render reaches it and is then served out
  of the whole remaining ledger, wherever it sits in the body.
- `rows: 0` with `open: true` (the host's "measured, open, off screen" spelling) is a valid request: the
  container node is built, `total`/`offset` are stamped, zero rows are materialised, and it never falls
  back to the first-paint default.
- A stale `offset` clamps to the **last** window (`total - rows`), not to `total - 1`, so a non-empty open
  container with `rows > 0` never comes back empty.

### 2.4 `node_key` uniqueness (S3 finding 4)

`TreeWindows::claim_window(node_key, entries)` refuses the second container in one body to claim a key,
with a distinct code `ui.tree-window.duplicate-key`.

⚠️ **Refinement, deliberate**: only a container that actually PUBLISHES a window claims. A container with
no entries that the host has never addressed stamps nothing and is not addressable, so two of them cannot
steer each other — which keeps trees whose *leaves* are built through `tree_window_item`
(`TreeWindowKit::render`, the stdio JSON/XML editors, the writer AST, procedural's structure window) out of
the uniqueness requirement entirely. It binds containers, not rows. §5 lists what still trips.

## 3. Files changed

| File | What |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` | `TREE_WINDOW_FIXED_NODE_HEADROOM`, `TREE_WINDOW_BODY_NODE_BUDGET` + the cost model on the docstring |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | region `🔖️PanelWindowing`: reservation ledger, exact-once charging, claim, offset clamp, re-exports |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs` | the laws in §4 |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | `house()` fixture + two House laws; `wide_view()` rows 128 → 8 (see §5.3) |

## 4. Laws

In `🔬️app-panel-kit` (`panel_kit_tests`), new this packet:

| Law | Pins |
| --- | --- |
| `the_body_node_budget_is_the_record_arena_less_the_root_and_the_fixed_reserve` | the derivation |
| `the_host_tree_element_declares_the_same_body_node_budget` | reads `🌳️Tree/🟦️.tsx` and asserts the literal `TREE_WINDOW_BODY_NODE_BUDGET = 111` |
| `a_nested_container_is_charged_exactly_once` | `spent == body_nodes(built)` |
| `an_open_request_for_zero_rows_materialises_nothing_and_keeps_its_offset` | the off-screen spelling |
| `a_stale_offset_clamps_to_the_last_window` | `offset + length == total`, and the shorter-than-window case |
| `a_duplicate_node_key_in_one_body_is_refused` | section↔section, section↔nested item, and the leaf exemption |
| `a_closed_container_costs_exactly_one_node` | closed cost |
| `a_host_request_outranks_a_first_paint_default_earlier_in_document_order` | the starvation case |
| `host_requests_beyond_the_ledger_are_clamped_in_request_order` | over-ask clamp, totals still stamped |
| `a_nested_tree_window_item_inside_a_window_section_stays_inside_the_arena` | fem3d's own shape |
| `a_row_error_that_is_not_fixed_capacity_still_propagates` | only `ui.fixed-capacity` shortens |

Pre-existing and kept green: `a_closed_container_materialises_no_rows_and_still_stamps_its_total`,
`a_host_request_materialises_exactly_its_slice_and_opens_a_closed_author_default`,
`a_first_paint_spends_one_shared_viewport_budget_in_document_order`,
`a_window_never_exceeds_one_built_children_page`, `a_refused_row_shortens_the_window_instead_of_faulting`,
`nine_windowed_sections_share_one_body_wide_node_ledger`,
`a_nested_group_item_charges_the_body_wide_node_ledger` (nested keys made unique),
`a_first_paint_never_outspends_the_node_ledger`, `ui_history_panel_stays_inside_the_body_node_ledger`,
`ui_history_panel_windows_its_commands_without_a_continuation_row`,
`interaction_domain_stamps_one_tree_level_activate_binding`.

fem3d (`📌️panels/🗿️artifact/🧪️tests/🔬️unit`):
`a_house_sized_first_paint_keeps_the_whole_body_inside_the_record_arena` and
`a_house_sized_body_honours_every_capped_host_window` — 63 nodes, 63 supports, 8 solids, four load cases of
twelve loads each, all open; the second drives it with a request set whose `Σ(1 + rows)` is inside
`TREE_WINDOW_BODY_NODE_BUDGET` exactly as the React host caps its own report, and asserts every requested
window is honoured **in full**.
