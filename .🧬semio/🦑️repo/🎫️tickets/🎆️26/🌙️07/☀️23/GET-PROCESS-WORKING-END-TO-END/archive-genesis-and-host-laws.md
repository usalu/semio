# Archive Genesis Roster and Host Retained-Laws

## 1. `history_edits_end_to_end` closure `Incomplete`

Decision: the closure stays strict (`members == reachable`). The roster is what must be exact. A member is part of the head closure only if the head snapshot names it by slot, child id **and artifact id**. A superseded content-addressed child (same slot and child id, new artifact id) is therefore retired from the roster, with every member owned only through it.

Root cause: `complete_document_archive_genesis` pruned archived members by `(slot, child_id)` and skipped minting by `(slot, child_id)`. A content-addressed child whose artifact id changed kept its old roster entry, and the minted/archived new one made 7 members against 6 reachable.

Fix (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🚪️ingress/🦀️.rs`):
- new pure `retain_head_reachable_members(members, root_id, projected)` keyed by `(slot, child_id, artifact_id)`, used by the genesis completion;
- the skip test for minting is by `artifact_id` (unique across a closure).

Law (language-agnostic): `…/🚪️ingress/🧪️tests/head-reachability.json` (3 cases: superseded child with unchanged child id dropped with its subtree, unchanged head keeps all, orphan under unreachable parent dropped). Rust law `🧪️tests/🦀️.rs` (`roster_keeps_exactly_the_head_reachable_members`, `serde_json` dev-dependency added to `semio-plugin-instance-docs`); TS twin `🧪️tests/🟦️.ts` validates the corpus with `graphlib` (`alg.preorder`). The twin logic was run with node: 3/3 ok (bun is not installed here). The Rust law compiled and ran under the root workspace (`seed-cargo.sh`, `-p semio-plugin-instance-docs --lib roster_keeps`, log `seed-g6.txt`).

Verification: `process3d_panels_artifact` `history_edits_end_to_end` no longer fails closure validation. It now loads, edits and reaches the final close, and fails there (`replay-g4.txt`): `registered fixture did not reach its exact terminal-empty witness after 5021 turns`, quote `copy_bytes 64` against remaining `copy 35` on rung `RetainedFields/cache_retirement`, source total copy exhausted. That is the stock/close-drain family (cause 3 of `editor-tests-pass2.md`), not the archive roster. `conflict_history_edits_end_to_end` passed in the same run (the other agent's release-drain work). The other 7 targets were not run: `process3d_workpiece_window` currently fails to compile because of another agent's change (`render(...)` gained a `Locale` argument).

## 2. Host `retained-laws`: `fixture initializer must converge`

Root cause (measured with phase prints in the replay initializer): the initializer stalled forever in `BindGenesis`. `advance_registry_birth` opens a quoted receiving scope with body `{copy 2368, capacity 2384}`; the body was handed `actual = {copy 2368, capacity 0, release 0}`, so `SnapshotReadRegistryBirth::advance` refused (`retained_clone_grant_admits`) and returned empty progress every turn.

Cause of the zero capacity: `OriginalReturnScopePlan::quote` (`🧵️job/📬️outcome/🪙️budget/📦️stock/🎟️scope/🦀️.rs`), loaned-scope branch. When a scope is already held (the receiving scope now survives across Source turns, see `seed-rebind.md`), it reused the existing scope whenever items and copy fit and silently clamped the quoted capacity and release to what the held scope still had (0). Fix: reuse the held scope only when items, copy, **capacity and release** fit; otherwise fall through to the existing top-up path (which draws on the fresh stock). The now redundant clamp was removed. Result: the initializer converges (all phases through `Complete`, `replay-h6.txt`). This is the same family as the host contribution stall (quote refused or under-granted in a held scope), so the host-contribution owner should re-test after this change.

Result: the two tests now fail one step later, and both are not about the initializer. After convergence the store close loop (`close_store`, and the displaced-store loop at the end of `every_store_replacement_phase_unit_fits_the_interactive_step_budget`) fails with `ticket close changed its selected body` (`FactoryTicketClose::next_demand`). Measured (temporary prints, removed): the first close turn that reaches the store's factory tickets begins an invocation whose pinned body and grant are `copy 40, depth 1` (the effect grant at that level is already narrowed), while the caller quotes with 65,536 B (`PLUGIN_SELF_FUNDED_BODY_BYTES`; the test originally quoted `PROCESS3D_OWNER_BYTES`). Two experiments did not help and were reverted:
- quote and grant with one tracked body (still a mismatch, then the loop never reaches terminal-empty);
- letting `next_demand`/`next_work` answer from the pinned invocation without comparing the body (the quote passes, but `close_step_cold` builds a fresh `AuthoringWallet` per call, so a ticket invocation left pending cannot be resumed and the store never reaches terminal-empty: `the displaced store must reach terminal-empty ownership within its bounded close`).

So the open question for the store/ticket owner is why the first ticket step leaves the child reply pending under a cold 64-work wallet (child close needs more than one effect call), or whether the cold helper must hold one effect across turns. This only showed up now because the initializer previously never converged, so these two tests never reached their close loops (earlier "6/6" ran against the earlier initializer that did not hold a scope across turns).

STATUS: host retained-laws 4/6 (unchanged count), initializer convergence fixed, close-ticket step open.

## Files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📥️staging/🚪️ingress/🦀️.rs`, `🚪️ingress/🧪️tests/{🦀️.rs,🟦️.ts,head-reachability.json}`, `📄️docs/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🧵️job/📬️outcome/🪙️budget/📦️stock/🎟️scope/🦀️.rs`
- Touched and restored to HEAD: `🌱️value/♻️retirement/🎟️tickets/🔒️close/🦀️.rs`, host `retained-laws/🦀️.rs`.
- All `[DEBUG]` prints I added in the store replay initializer and registry birth are removed.
