# r11-store-inf: infinite crate migration to the grant/receipt close API

Crate `semio-framework-os-infinite` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite`). Gate runs: `cargo check --lib`, `cargo check --lib --tests`, `cargo test --lib` (logs were under `🗑️generated/r11-store-inf`, deleted after this report).

## Result

- `cargo check -p semio-framework-os-infinite --lib`: 0 errors.
- `cargo check -p semio-framework-os-infinite --lib --tests`: 0 errors.
- `cargo test -p semio-framework-os-infinite --lib -- --test-threads=1`: 516 passed, 3 failed (519 total). The new law test and the three source-scan fill tests pass.
- `cargo test ... --lib` with default parallel threads: 463 passed, 56 failed. Every failure there is a world test hitting `Busy` (`mesh3d_begin` claim) or `Capacity` faults: the process-global mesh claim cannot run in parallel, and the same tests pass with `--test-threads=1`. Not caused by this port; the crate needs serial test execution (as nx scripts presumably set).

## What was migrated

### `🎲️board/🔌️ports/➡️directed/➕️normal/🦀️.rs` (the BIM-critical fill machinery)

All three old `close_step(maximum_items, maximum_bytes)` implementations with their ~51 `Pending{released_items,released_bytes}` sites were rewritten:

- `BoardFillSnapshotCapture::close_step(grant)` and `BoardFillSnapshotIngress::close_step(grant)` (plain inherent methods).
- `impl InteractiveJob for BoardFillJob`: `close_step(grant) -> InteractiveJobCloseStep` plus the four demand methods.

Shared building blocks (module level, before `BoardFillPage`):

- `board_fill_close_admits(grant, release_bytes, terminal_is_empty)`: one admitted item, `maximum_release_bytes >= release`, `maximum_depth >= 1` unless terminal-empty.
- `board_fill_close_idle()` (default `Pending`), `board_fill_close_receipt(copied_items, released_bytes)`, `board_fill_close_complete(copied_items)`.
- `board_fill_close_first_page!(owners...)` macro replacing the three duplicated `retire_*_page!` macros.
- On `BoardFillFixedPages`: `last()`, `next_close_release_bytes() -> Option<usize>` (`Some(0)` while items remain, else the last page backing), `close_page_step()`, `close_turn()` (pop one item, else retire the last empty page).
- On `BoardFillSnapshot`: `next_close_kind_handles_bytes`, `next_close_release_bytes`, `close_kind_handles_turn` (shared by capture and ingress).
- Mirrors: `BoardFillSnapshotCapture/Ingress::next_close_release_bytes()` (pub), `BoardFillJobState::next_close_release_bytes()`, `BoardFillJob::close_release_bytes()`.

Old to new currency: the old `released_items: 1` handoff becomes `copied_items: 1`; page retirement is `copied_items: 1, released_bytes: page backing`; the terminal turn that drops the emptied snapshot/state is `Complete{copied_items: 1}` (`Complete{default}` when nothing was left); `JobPayloadCloseStep` of the commit writer (job crate still exposes that old shape) is mapped to `copied_items = released_items`, `released_bytes`.

### Demand declarations (BoardFillJob)

- copy = 0 and capacity = 0: no turn copies or allocates.
- release = exactly the physical page backing the next close turn frees. `close_release_bytes()` mirrors the decision cascade of `close_step` in the same order: checkpoint adoption (0), commit encoder (0), commit writer (`RetainedJobPayloadWriter::next_close_byte_demand()`, the 16 KiB payload page), preview/fault (0), pending placement handles (pop 0, else the page), the per-field takes (0), last snapshot kind handles (pop 0, else the page), any remaining poppable item (0), then the first retained page in the cascade order (virtual_handles, virtual_nodes, candidates, sources, snapshot rules, handles, nodes, kinds), else 0.
- depth = 1 for these flat owners while not terminal-empty, 0 once `terminal_is_empty` (so the trait default's terminal-empty acceptance stays consistent).
- A grant below any axis (items 0, depth 0 while non-terminal, release below the quote) returns `Pending{progress: default}` with no mutation. The only side effect of a yielded turn is the idempotent `closing = true`, as before. Receipts always fit the unchanged grant.

### Test files

- `🌍️world/🧪️tests/🔬️unit/🦀️.rs`: added `close_quoted_job` (drives any `InteractiveJob` with exactly its quoted demands); migrated the `InteractiveJob::close_step(job, 1, PAGE)` loop, the `PreparedRenderJob::close_step` loop, `packet.retire_step()` (now `next_close_demands(0)` + `close_step(grant)` until `Complete`, then asserts `retirement_is_empty`), `PreparedRenderInput::close_abandoned_step()` (now `next_abandoned_close_demands(0)` + `close_abandoned_step(grant)`), the `semio_framework_value::retained_clone::` import, and the broken `include_str!` of the inline-source fixture (path was `../../🧫️fixtures`, the fixture lives in `🧪️tests/🧫️fixtures`, so `../🧫️fixtures`).
- `🎲️board/➕️normal/↔️undirected/🧪️tests/🔬️unit/🦀️.rs`: macro used the unlinked `semio_framework_os_infinite::` path; now `crate::infinite::` (as the dag leaf tests do).
- `🎲️board/🔌️ports/➡️directed/🕸️dag/🧪️tests/🔬️unit/🦀️.rs`: removed `set_selection_domains_json` use; now `set_selection_domains(&DagSelectionDomains{..})`.
- New law test `fill_close_turns_quote_exact_demands_and_fit_their_unchanged_grants` in `🎲️board/🔌️ports/➡️directed/➕️normal/🧪️tests/🔬️board-host-standalone/🦀️.rs`: for the ingress (open node + 9 published nodes over 3 pages) and for a `BoardFillJob`, each turn tries the three under-grants (items 0, depth 0, release - 1) and asserts a default `Pending` plus an unchanged quote; then grants exactly the quote and asserts `fits`, that the summed `released_bytes` equals the sum of page backings, and terminal-empty at the end. (Ran: passes. No third-party cross-check library exists for this internal page allocator; the oracle is the independently summed `backing_bytes` of the pages.)

## Files outside the brief that needed no change

`🌍️world/🦀️.rs`, `🕸️dag/🦀️.rs` (lib) and `📦️packages/🦀️rust/🦀️.rs` compile after r11-store's fixes. The dag leaf's internal `DagRetirementStep{released_items,..}` is its own type and unaffected. `🗿️artifacts/🕸️dag/**` belongs to another crate and was not touched.

## Remaining issues (not caused by this port)

3 world tests still fail in a serial run:

- `world::tests::world_native_overlapping_wires_pick_original_closest_depth`: `WorldComponentPickCursor` returns `Stale` for wire-only meshes (`positions: []`, `edgePositions`/`edgeIds`).
- `world::tests::world_native_clipped_wire_pick_preserves_original_visible_depth`: closest-depth mismatch for the same wire-only mesh family.
- `world::tests::on_a_real_gpu_a_section_removes_the_far_half_and_the_cap_closes_the_cut_without_leaking_stencil`: pixel mismatch (`[47,57,60,255]` vs `[0,17,23,255]`) on this machine's adapter.

These do not touch the code changed here (no board-fill path, and the helpers I introduced run after the failing assertions). Likely candidates for the pick pair: the staleness comparison `admitted.edges != if schema.edge_ids == schema.edges {..}` in the world pick/marquee cursors (owner's 677 world edit). Not root-caused.
