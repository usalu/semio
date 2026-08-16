# W3-D 🏛️architect — MutationOutcome Fan-Out Report

## Inventory (full detail: `🧪️w3-d-inventory.txt`)
266 mutation-kind `🔺️diff` leaves, all hand-written `impl MutationKind<ProgramSnapshot, ProgramMutation>`
(no `#[derive(Mutations)]` here): create 64, delete 64, rename 67, replace 67, connect 2, disconnect 2.
Plus the schema-root `🔺️diff` dir (just the `ProgramDiff` struct — census's 267th "leaf" — not a
function, needs no change) and 2 hand-written `impl Mutation<..>` blocks outside `🧬️mutations/`
(`🎚️config`, `👥️presence`, whole-snapshot `Snapshot` ops) — 266+2 = 268 impl blocks, matches census.
0 `fn validate` overrides (matches census; one unrelated `validate_plugin` diagnostics fn left alone).

64 entities carry the full create/rename/replace/delete quad: 62 standard (Delta-struct, `base.<field>`)
+ 2 content-addressed-table (`benchmark-record`, `knowledge-record`, working-scene-cache + re-mint).
3 root-scoped singletons (`governance`, `meta`, `project`) have rename/replace only — always present,
no target-missing case.

## Verb-family mapping chosen (frozen table + my calls)
- **create** → Fatal `duplicate-id` on existing id. "Unknown container" Fatal N/A — flat
  single-snapshot collections, no nested containers.
- **delete** → Error `target-missing` on absent id.
- **replace** → mapped to **edit/replace**: Error `target-missing` / Warning `no-op` on unchanged
  whole value (via `existing == &payload.value`, all entity structs derive `PartialEq`).
- **rename** → Error `target-missing` / Warning `no-op` on unchanged name. Fatal `key-collision` N/A —
  `name` is a non-unique display field; `id` is the only key.
- **connect** (`connect-adjacency`, `connect-trace`) → mapped to **connect/bind**: Warning `no-op` on
  unchanged upserted value for both; `connect-adjacency` additionally gets Error `target-missing` on
  either endpoint element absent (checked against `base.elements`). `connect-trace`'s `from_id`/`to_id`
  are free-form cross-register references (any entity, any collection) — endpoint-existence checking
  is **not implemented** (no cheap generic membership helper exists across all ~65 collections); only
  the no-op check landed. Flagged as a gap, not silently dropped.
- **disconnect** → mapped to **disconnect/unbind**: Error `target-missing` on absent id.
- **governance/meta/project rename+replace** → root-scoped singletons (structurally always present,
  matches the contract's "root change-\<artifact\>-\<field\>… may legitimately return message-free
  outcomes" allowance) → Warning `no-op` only, no target-missing case.
- **config/presence `Snapshot` mutations** (`🎚️config`, `👥️presence` — whole-value replace, single
  variant) → mapped to **change/set/update**: Warning `no-op` when the new value equals the base
  value. Error/Fatal N/A — no target or domain-invariant concept for a local whole-snapshot config.

## Coverage (real counts, not rounded up)
- **Leaves compiled**: blocked on upstream, confirmed with a final clean single run (`EXIT:101`,
  `🧪️w3-d-cargo.txt`, 1006 lines). `cargo check -p semio-s-plugin-architect` transitively compiles
  `semio-framework-os-kernel` → `semio-framework-plugin` first; both are still red from concurrent W1
  lanes' in-flight C6–C9 work. Watched across 10 check attempts over ~10 minutes: error count moved
  6 → 3 (kernel-level `reconcile_with_last`/`resolve_conflict`/`HistoryLog.conflicts` landed and fixed
  by another lane mid-session) → `semio-framework-os-kernel` now compiles clean → `semio-framework-plugin`
  (next in the dependency chain) surfaced its own 3 errors: `error[E0063]` missing field `messages` /
  `report` on `AppFrame` initializers, `error[E0599]` no method `snapshot_with_conflicts` on
  `ArtifactStore` (C8/C9 + the C6 debt item 1 deletion, assigned to other lanes per `📓️w0-barrier.md`).
  **Zero of these 3 errors, across every attempt, ever referenced a path under
  `✏️s/🔌️plugins/🏛️architect/`** — verified programmatically (`content.count("architect") == 0`) on
  the final clean run, not just by eyeballing.
- **Leaves with real verb-family messages**: 266/266 mutation-kind leaves (100%) + 2/2 hand-written
  config/presence impl blocks. 0 leaves left as a bare `MutationOutcome::new(..)` wrap.
- **Leaves still bare**: 0.
- Self-verified (since full compile is blocked): brace/paren balance checked programmatically across
  all 248 auto-generated + 18 hand-edited diff.rs files + all 266 mutation.rs signature edits + the 2
  impl blocks — 0 mismatches. Manually re-derived and fixed every trait-method `.diff()` **call site**
  in the plugin (not just the leaf definitions) — 7 found via repo-wide grep (`round_trip` test helper,
  `assert_mutation_diff_absorb_law` inputs, `config_after`, one `apply_template` test) — all updated to
  thread `MutationOutcome` (`.diff()`/`.into_parts().0`), and fixed one now-invalid test fixture
  (`connect_and_disconnect_adjacency_round_trip` used two freshly-random element ids that don't exist
  in `sample_plugin()`'s `elements` — now correctly rejected as Error `target-missing` by the new
  endpoint check; test updated to use `snapshot.elements[0..1]`'s real ids instead).

## Pass 3 (facet tests)
The facet's `🧪️Tests` region lives in `🧬️mutations/🦀️component.rs` (`mod tests`, ~line 310). Landed
testkit helpers confirmed present: `assert_missing_target_is_error`, `assert_fatal_never_applies`
(`🧰️framework/…/📡️spr/🧪️testkit/🦀️component.rs`). **`assert_outcome_policy_matrix` is not landed yet**
(only the older, generic `assert_policy_matrix(3×4)` exists) — per the brief's instruction not to reach
into the testkit myself, I did not add it and report it pending on lane 1-D. Did not add new
`assert_missing_target_is_error`/`assert_fatal_never_applies` calls this pass given the upstream
compile blocker made it impossible to verify them against real output; existing `assert_mutation_inverse_law`
calls (already present, confirmed compatible with the new `MutationOutcome`-returning signature by
reading the testkit source) continue to assert "no Error/Fatal in the forward outcome" per the recipe.

## Blocked / handed to coordinator
- `assert_outcome_policy_matrix` (per verb-family-per-facet) not yet in testkit — cannot write these
  6 tests (create/delete/rename/replace/connect/disconnect) until lane 1-D lands it.
- Full `cargo check`/`cargo test` green confirmation blocked on lanes owning `semio-framework-plugin`'s
  C8/C9 work (`AppFrame.messages`/`.report`, `snapshot_with_conflicts`) — not in my lease, cannot fix.
- `connect-trace` endpoint-existence validation intentionally not implemented (see verb-family mapping
  above) — a real gap, not a silent omission.

Files touched: 266× `🧬️mutations/<kind>/🔺️diff/🦀️component.rs` + `🦠️mutation/🦀️component.rs`,
`🧬️mutations/🦀️component.rs` (facet test module), `🎛️apps/🏛️architect/🎚️config/🦀️component.rs`,
`🎛️apps/🏛️architect/👥️presence/🦀️component.rs`, `🎛️apps/🏛️architect/🦀️component.rs`.
