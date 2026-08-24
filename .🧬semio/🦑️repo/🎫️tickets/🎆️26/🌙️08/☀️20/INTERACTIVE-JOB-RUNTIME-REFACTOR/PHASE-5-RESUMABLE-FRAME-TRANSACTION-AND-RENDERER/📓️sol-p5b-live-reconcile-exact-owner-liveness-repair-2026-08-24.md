# Sol P5b Live Reconcile Exact-owner and Liveness Repair

Date: 2026-08-24  
Status: source-audit-ready; runtime acceptance not claimed

## Boundary

This repair is limited to P5b live reconciliation:

- `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs`;
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🩹️patches/🦀️component.rs`;
- the previously mounted one-opportunity scheduling seam in reactor `🦀️component.rs`, inspected but not changed by this repair;
- the distinct P5b predicate and self-test mutation corpus in root `📜️script.ts`;
- this report.

P5a, P5c, P5d, and P5e were not started. P1q/P2a1 regions and accepted adjacent packets were preserved.

## Production repair

### B1 — bounded semantic census

- Removed production `tree_node_semantic_usage` and the recursive `include_ui_value` descent.
- Added `SurfaceSemanticCensusCursor` with explicit field, container, entry, binding, action, data-attribute, string-byte, and depth cursors.
- Nested `UiValue` traversal uses a fixed 64-frame stack. The exact node stays boxed and unmodified until census admission completes.
- Every job grant checks complete generation, cancellation, fuel, and deadline before advancing one retained cursor opportunity. Zero fuel and an expired deadline leave the node and cursor unchanged.
- Removed eager `max_nodes`/`max_items` cursor and retirement-forest reservations. Semantic capacity is charged from actual `String`/`Vec`/`HashMap` capacities before the first candidate key or record clone.

### B2 — persistent exact owner credit

- Added `SurfaceReconcileReservation`; mounted render reserves aggregate credit before `plugin_render` can materialize the tree.
- The reservation follows the exact unadmitted owner and original generation into `SurfaceReconcileJob::try_new_reserved`.
- `SurfaceReconciler` carries `persistent_credit`. `take_ready` transfers the job credit into the candidate rather than releasing it.
- A later generation owns both the prior reconciler credit and the new operation credit until incremental previous-owner retirement returns the former.
- Ready storage is a fixed `[Option<ReadySlot>; 64]`. ACK revision gates the next generation so a published patch cannot outlive its charged last-valid reconciler owner.
- The O(1) `revision()` accessor remains the mounted scalar path; no snapshot clone is used.

### B3 — saturation-safe close

- When no terminal target is free, instance close first advances one matching capacity-producing terminal.
- Only a later grant converts the matching unadmitted, rejected, or surface owner.
- Ready, deferred, unadmitted, rejected, surface, terminal, close, and credit roots remain represented by terminal emptiness.

### B4 — permanent generation exhaustion

- Replaced every `checked_add(1).unwrap_or(u64::MAX)` tracker assignment with `issue_generation`.
- `u64::MAX` is issued once after a legitimate `u64::MAX - 1`; `generation_exhausted` permanently refuses all later begin, unadmitted, and mounted reservation requests without mutating their exact owners.
- Admission-credit epochs also use checked addition rather than wrapping reuse.

### B5 — lossless public handback

- Replaced the best-effort fixed terminal insertion with an owner-intrusive handback chain.
- Job, rejected, and terminal states are already boxed retained owners. Drop links the complete state into the registry; it never destroys the cap-plus-one owner when a fixed array is full.
- Generation-keyed retrieval detaches exactly one matching owner, after which one-owner close grants return candidate/current/source/patch/credit backing to terminal-empty.

## Hostile fixtures

Runtime reconcile fixtures:

- `identifier_cap_plus_one_returns_the_exact_tree_owner_before_cursor_mutation`;
- `dynamic_semantic_page_plus_one_faults_before_key_or_record_clone`;
- `semantic_census_zero_fuel_and_expired_deadline_leave_every_cursor_and_owner_unchanged`;
- `semantic_census_low_fuel_wide_container_and_deep_value_advance_one_unit_without_recursion`;
- `persistent_credit_transfers_through_ready_and_returns_only_after_incremental_retirement`;
- `public_drop_handback_is_lossless_at_terminal_cap_and_plus_one`;
- `stale_cancel_and_drop_handoff_preserve_public_terminal_ownership`.

Mounted tracker fixtures retain the accepted FIFO/revision/close corpus and add:

- `terminal_full_plus_matching_unadmitted_advances_capacity_before_conversion`;
- `terminal_full_plus_matching_rejected_advances_capacity_before_conversion`;
- `terminal_full_plus_matching_surface_advances_capacity_before_conversion`;
- `generation_max_is_issued_once_and_repeated_exhaustion_returns_exact_owners_without_mutation`.

The earlier mounted one-opportunity, cap/+1 exact handback, stale generation, resize coalescing, FIFO effects, ABA resume, pre-materialization reservation, all-class close, and local terminal-saturation fixtures remain required by the permanent predicate.

## Permanent verifier

The P5b predicate reads live reconcile, tracker, and mounted reactor sources. Its 33 restore-and-kill mutations cover:

- slot/page/byte-cap drift and cursor-before-reservation;
- missing generation, cancellation, fuel/deadline, double-step, and terminal Drop;
- dynamic tracker or ready storage, effect reorder, instance-close erasure, tree clone, missing mounted drive/close, and the old diff seam;
- recursive whole-node semantic census, dynamic value stack, whole-list traversal, and missing hostile cap fixture;
- credit release in `take_ready`, missing pre-materialization reservation, exact unadmitted clone/drop, and retry generation reminting;
- unmounted render, blocked-before-producing-terminal ordering, saturating maximum generation, lossy public handback, and close-ready omission;
- missing terminal-saturation, generation-exhaustion, and public-Drop cap/+1 fixtures.

Every mutation was killed by `bun 📜️script.ts verify interactivity --self-test`, and the unmutated P5b predicate was clean.

## Scoped gates

- `rustfmt --edition 2021 --config skip_children=true --check` on UI-runtime `reconcile.rs` and tracker `patches/component.rs`: PASS.
- `bun 📜️script.ts verify interactivity --self-test`: PASS; DENY clean, one existing allowlisted blocking bridge.
- scoped `git diff --check` over reconcile, tracker, mounted reactor, and root verifier: PASS.
- production scan: recursive semantic helper, saturating generation, eager maximum cursor reservations, mounted `.reconcile`, and `snapshot().revision` absent. The remaining `.reconcile` calls are inside the test-only parity oracle.

The shared index already contained the main P5b source/verifier wave while this repair was in progress. It was not modified, reset, or restaged. The root verifier also has a working-tree predicate/mutation delta over that peer-owned index; both current source and the combined current verifier were used for the scoped gates.

## Deferred evidence and blockers

No Cargo, Nx, Wasm, browser, network, broad build, or runtime command was run. Therefore this report makes no compile, executable timing, allocation-runtime, browser, or runtime-acceptance claim. No source/static blocker was found by the permitted gates. P5b still requires independent source audit and the later serialized runtime matrix; Phase 5 remains RED because P5a/P5c/P5d/P5e are separate.
