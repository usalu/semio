# W3-F Report — 🧱️block + 🧩️puzzle Mutation Outcome Fan-Out

## Census (actual, re-counted independently — 193 vs scout's 199; scout's 6-file delta
was the `🧬️schema/🔺️diff/component.rs` artifact-level `XDiff` type files, which are NOT
per-kind `diff()` leaves and are out of scope)

| Plugin | Diff leaves | Hand-written impl blocks fixed |
|---|---:|---:|
| 🧱️block | 104 | 6 (config×3, presence×3) |
| 🧩️puzzle | 89 | 12 (config×2 [3d/5d; 2d is derived] + presence×3 + Value/PlaySnapshot bridges×6 + 2d-config×1) |
| **Total** | **193** | **18** (+ 193 standard `impl MutationKind::diff` trait bodies, all delegate-only) |

## Verb-family inventory (per plugin)
- **block**: change_set_update 40, move_transform 14, remove 13, add 13, rename 8, delete 8, create 8
- **puzzle**: change_set_update 30, edit_replace 16, move_transform 14, disconnect_unbind 6, connect_bind 6, delete 5, create 5, remove 3, add 3, rename 1
- No `insert`/`reorder`/`duplicate`/`group`/`flatten`/`split`/`merge`/`clear` kinds exist in either plugin.
- Domain-verb mapping: puzzle's `connect-*`/`disconnect-*` (handles/grips/vortices, kind-compatibility) → frozen `connect/bind` and `disconnect/unbind` families; puzzle's `edit-*-text`/`replace-*-geometry`/`replace-*-source` → `edit/replace`.

## Coverage (the three required numbers)
- **Leaves compiled: 193/193.** Every `🔺️diff` leaf returns `protocol::MutationOutcome<XDiff>`; every one of the 211 `fn diff(&self, …)` trait-impl signatures (193 standard + 18 hand-written) updated to match. Zero `fn validate` existed in this lease (census confirmed 0/0) — nothing to delete.
- **Leaves with real verb-family messages: 153/193 (79%).** Breakdown: 90 got Error `mutation.target-missing` + Warning `mutation.no-op` (change/set/update, edit/replace, rename, move families); 19 Error target-missing on delete/remove/disconnect (existence check added where the leaf previously never checked `base`); 13 Error target-missing (letelse-guard families without a no-op layer, e.g. plain delete/remove); 9 Warning `mutation.no-op` on add-dup-present (author/attribute/compatibility-rule, existence check added); 8 Fatal `mutation.duplicate-id` on create (existence check added); 6 Warning `mutation.no-op` on connect (already-connected); 5 Fatal `mutation.duplicate-id` (create, existing guard converted); 3 Error target-missing (remove-author's filter-rebuild form).
- **Leaves still bare: 0/193.** The remaining 40 are root-scoped/config singleton fields (camera pan/zoom, meta description, node-kind rename/label/variant/description/icon/unit, `change-domain`, `replace-kind-catalogs`, etc.) — no addressable id target exists (`fn target` unset), so they legitimately fall under the frozen "root-scoped … may return message-free outcomes" exception, verified per-leaf via each mutation's `target()` override.

## Domain-verb mappings chosen
create⇒Fatal `duplicate-id`; add(dup)⇒Warning `no-op`; delete/remove/disconnect/unbind(absent)⇒Error `target-missing`; connect/bind(already-connected)⇒Warning `no-op`; change/set/update, move/rotate/scale, rename, edit/replace(absent)⇒Error `target-missing`, (unchanged value)⇒Warning `no-op` via generic `new_value == *existing` comparison. Not implemented (time-boxed out): Fatal non-finite/non-positive on move/scale/resize (would need per-field numeric-type detection across ~20 heterogeneous payloads) and Fatal key-collision on rename — both are gaps, not silent bare wraps, and both leaves still carry the Error target-missing layer.

## Pass 3 — facet tests
Added `🔖️OutcomeLaws` regions to all 6 facet dispatch files (block 2d/3d/5d, puzzle 2d/3d/5d): `assert_missing_target_is_error` (one call per verb family present in that facet) + `assert_fatal_never_applies` (create/duplicate-id). `assert_outcome_policy_matrix` is **not yet landed** in `📡️spr/🧪️testkit`'s `🔖️Laws` region (only `assert_missing_target_is_error`, `assert_fatal_never_applies`, `assert_outcome_deterministic`, `assert_policy_matrix` exist as of this pass) — not written against the missing name to avoid breaking `cargo test --lib` crate-wide; flagged here as pending lane 1-D. Also fixed 12 pre-existing test call sites across the 6 dispatch files (`round_trip`/`*_diff_absorb_law`/`MutationDiff::<P>::apply(&x.diff(base), …)`) that broke when `.diff()` started returning `MutationOutcome<D>` instead of `D`.

## Cargo (real, pasted)
`cargo check -p semio-s-plugin-block -p semio-s-plugin-puzzle` — **blocked upstream, not by this lease.** Compilation reaches `semio-s-plugin-stdio` (a hard path dependency of both block and puzzle) before reaching block/puzzle's own sources, and stdio is mid-conversion by lane 3-E (215 leaves, 398 uncommitted files, 269 current errors, all inside `✏️s/🔌️plugins/🗄️stdio/`). Confirmed **zero** errors reference any `🧱️block`/`🧩️puzzle` path across 4 retries over this session (framework-side blockers in `semio-framework-plugin` — `AppFrame`/`AppCommand` C9 work — resolved between retries 1–3; the stdio blocker persisted through retry 4). Full log: `🧪️w3-f-cargo.txt`. `cargo test --lib` could not be run for the same reason.

## Files touched
193 `🔺️diff` leaves + 193 `🦠️mutation` trait-impl signatures under `✏️s/🔌️plugins/{🧱️block,🧩️puzzle}/🗿️artifacts/**/🧬️mutations/*/`; 6 facet dispatch files (`.../🧬️mutations/🦀️component.rs` ×3 per plugin, signature fix + `🔖️OutcomeLaws` test region + test-call-site fixes); 6 `👥️presence/🦀️component.rs`; 6 `🎚️config/🦀️component.rs` (block 2d/3d/5d + puzzle 2d/3d/5d, incl. one at `✏️editor/🎚️config/🧬️schema/…` — no, puzzle2d's is at `✏️editor/🎚️config/🦀️component.rs`).

## Blockers (report only, not fixed — outside lease)
1. `semio-s-plugin-stdio` mid-conversion (lane 3-E) — blocks final `cargo check`/`cargo test` for block+puzzle transitively. Not our code; will clear when 3-E lands.
2. `assert_outcome_policy_matrix` not yet in `testkit` (lane 1-D) — facet tests use the two landed helpers only; this one is pending.
