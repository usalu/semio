# terra — packet `kernel-fanout-store`

## Scope
Owned path: `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/**` (single file in play: `🦀️component.rs`, 12,704 → 12,770 lines; `👷️worker/🦀️component.rs` and `🔄️sync/🦀️component.rs` were untouched — zero errors attributed to either).

## Headline numbers (measured, not estimated)
| stage | store errors (`cargo check -p semio-framework-os-kernel --lib`) |
|---|---:|
| start (sol's precomputed list) | **233** |
| after asyncify/deasyncify/insert-await tool passes + hand work, checkpoint 1 | 162 |
| checkpoint 2 | 103 |
| checkpoint 3 | 78 |
| checkpoint 4 | 46 |
| **final, this report** | **42 — all attributable to two named cross-module blockers, zero unexplained residue** |

Every number above is a pasted `grep -c "error\["` count against a fresh `cargo check --message-format=short` run, `CARGO_TARGET_DIR` pointed at the shared scratchpad `target-fanout`. Six full crate-check runs total (more than the "2-3" guidance; each was cheap once the crate started compiling past its own syntax errors, and each surfaced either a live sibling fix or a regression of my own that needed correcting — see log below).

## Method actually followed
1. **Fixed a pre-existing syntax error first** (line 5011, a mangled struct-literal field `started_at.await,` — malformed by an earlier bad tool run, not by me) — the file would not even parse before this.
2. `asyncify-universal.py --scan` on my path → **0 to convert, 960 already async, 1 tagged-exempt** — this module was NOT mid-revert; no signature conversion needed.
3. `deasyncify-external-impls.py --scan` → **0 reverted** — no E1 damage present.
4. `insert-await.py --apply --scope '🏪️store'` — ran to fixpoint every time I invoked it; it never found more than a handful of unambiguous edits per pass because the bulk of the residue here is the R10 "no tool can fix this" shapes, not plain missing-`.await`. Its real value was the two runs where it told me definitively "0 unambiguous edits, 97 errors" — confirming residue was hand-work, not a missed pass.
5. **The rest — the overwhelming majority — was hand work**, driven by the exact rustc diagnostics from repeated `cargo check` runs (not from re-deriving positions off the stale `sol-fanout-store.txt` list once the file had moved under edits).

## R10 residue shapes hit (all four, repeatedly)
- **Shape 1 (async fn in a sync closure)**: `Vec::sort_by_key`, `Option::map`/`unwrap_or_else`, `Iterator::filter`/`flat_map`/`find_map`/`position`, `is_some_and` — every one of these was called with a bare async fn item or a closure that itself called an async fn without `.await`. Fixed by hoisting into explicit `for`/`match` loops, in ~25 distinct spots (`history_composition_from_envelope`, `fold_compensation_error`, `last_local_edit_timestamp`/`last_undone_local_edit_timestamp`, `dispatch_inner`'s lane-search arms, `ingest_remote`/`merge_remote_snapshot`'s HLC sort+merge-insert (×2, full binary-search-preserving rewrite), the quarantine/degraded conflict blocks (×4), `dispatch_relation_group`'s fingerprint/child-id loops, `assert_subset_roundtrip`, `sync_member`, and more).
- **Shape 2 (one future, awaited more than once, or borrowed-then-moved)**: `back.await.reverse()` mutating a throwaway temporary instead of `back` itself (3 occurrences: `replay_mutations`, `replay_suffix`, and one already-correct sibling that confirmed the fix pattern); `pending`/`timestamp`/`candidate`/`alt_id`/`checkpoint_id`/`invocation_id`/`next_id` bound unawaited then `.await`ed 2-7× across `dispatch_inner`'s `CommitCheckpoint` arm, `reconcile_alternative`, `resolve_conflict`'s `Quarantined`+`Accept` arm (7 separate `.await`s on one `candidate` — the worst single instance), `set_checkpoint_composition_pins`, `CreateAlternative`, `dispatch_relation_group`.
- **Shape 3 (self/mutual recursion)**: `has_main_only_descendant` (direct self-recursion) and the `dispatch`↔`dispatch_inner` mutual cycle (3 call edges: `Undo`, `CreateAlternative`, and `dispatch_inner`'s own `SemanticUndo` arm calling itself) — all four boxed with `Box::pin(...)`.
- **Shape 4 (futures stored/chained)**: `edit_id.await` where `edit_id` was already a resolved `String` from an earlier `.await`; `next_id`/`alt_id` similarly double-consumed.

## R9 (pure-fn de-asyncification) applied, with evidence, never as a blanket sweep
Every one of these was verified I/O-free by reading the body, and verified against every call site before flipping:
- `json_value_to_dsl` / `dsl_value_to_json` / `renormalize_whole_number_floats` / `json_values_equal` / `renormalize_json_wire_value` (pack_rt module) — a fleet plugin (`puzzle/2d` `Puzzle2dPlaySnapshot`'s `impl PartialEq { async fn eq }`) already called `json_values_equal` with **no** `.await`, which is the smoking gun that confirmed the sync direction was already assumed elsewhere.
- `history_message_from_mutation_message` / `mutation_message_from_history_message` / `history_op_meta_from_operation_meta` / `mutation_meta_from_history_op_meta` / `protocol_undo_policy_ordinal` / `protocol_undo_policy_from_ordinal` — every call site is a bare fn-item argument to `Iterator::map`/`?`, none awaited.
- `same_operation_identity_and_payload` — 2 of 5 call sites need it inside `is_some_and`/`Iterator::all`; made sync, the other 3 `.await` call sites de-awaited.
- `PresenceStore::new` / `TransientStore::new` / `InteractionStore::new` (+ their `Default` impls, E1) — pure struct construction; the `🔌️plugin/🦀️component.rs` caller (off-limits file, read-only checked) already calls them **without** `.await`, so this was a compatibility fix for that file too, not just mine.
- `HybridLogicalTimestamp { actor: 0, physical_ms: 0, logical: 0 }` replacing 5 call sites' `HybridLogicalTimestamp::new(0, 0)` fallback default — `::new` is async (📡️replication, out of scope) but the fields are `pub`, so the trivial-zero case doesn't need the constructor at all.

## Correctness bugs found and fixed that were **not** compile errors (silent dropped-future bugs)
The universal-async codemod left several call sites where a now-`async` fn was called as a bare statement with no `.await` and no use of its return value — these compile cleanly (the `Future` is just dropped) but silently no-op at runtime. Found by manual audit while fixing adjacent compile errors, fixed regardless of whether rustc flagged them:
- `ArtifactCommand::encode_op` — **14 of 17** `crate::os_pack::write_varint_u64`/`write_command_str` calls were missing `.await`; the binary command encoder would have written almost no bytes for every variant except `Undo`/`Redo`/`PruneDrafts`.
- `self.clock.merge(...)` / `remote_clock.merge(...)` / `candidate_clock.merge(...)` / `candidate_clock.tick(...)` — 4 sites where the hybrid-logical-clock update was silently a no-op.
- `self.graph.insert_link(...)` — link-graph edge insert silently dropped.
- `build_apply_command_bytes`'s own callers, `write_command_str`, `write_command_ops` helper fns — same missing-await pattern inside the helpers themselves.

These are flagged with inline `// 🌀️` comments explaining the fix at each site (not `[DEBUG]`, they're permanent).

## What remains — 42 errors, exactly two cross-module blockers, nothing else
Both blockers are **outside my owned paths** (`🏪️store/**` only); I cannot fix either without a lease.

### Blocker A — 41 errors — `crate::os_dsl::{FieldSpec::new, FieldSpec::positional, FieldSpec::optional, RecordSpec::new}` (🗣️dsl/🧬️schema/🦀️component.rs) must go sync
- **5 direct errors** (lines 642, 689, 771, 788, 818): my file's hand-crafted P6-idiom `DslField` impls (`artifact_child_spec`, `owner_ref_spec`, `link_pin_spec`, `artifact_link_spec`) store their own fn items into `crate::os_dsl::Shape::Record(fn() -> RecordSpec)` — a genuine E4 fn-pointer slot (confirmed: `Shape::Record`'s own doc comment explains it's deliberately a lazy zero-capture `fn` pointer to avoid infinite recursion on self-referential grammars). These four spec functions in my file MUST be sync (E4), which I cannot do while their bodies call `FieldSpec::new(...).await`/`RecordSpec::new(...).await` — those callees are still `pub async fn` in `os_dsl`, a module I don't own.
- **36 errors** (lines 2226, 2255, 3488, 6364 — the `#[derive(DslRecord)]`/`#[derive(DslOps)]` attribute sites on `OpsAuthor`/`OpsHeaderLine`/`CommandHeaderLine`/`BackboneMessage`): all four are macro-expansion-attributed (rustc points at the derive attribute, column 35 = the derive name's own position on the line — confirmed by inspecting the actual struct/enum text). The derive-generated code is invisible from my file; I cannot add `.await` to code I cannot see. Its generated builders internally call the SAME `FieldSpec::new`/`RecordSpec::new` chain, so **this is the same root cause as Blocker A's direct errors, not a second problem** — fixing `crate::os_dsl::schema`'s four methods resolves both the 5 direct errors and all 36 derive errors in one change.

`crate::os_dsl::FieldSpec::new/positional/optional` and `RecordSpec::new` are pure struct-field-set/push operations (verified by reading every body — zero I/O, zero `.await` inside them today). They are textbook R9: pure, and consumed by a hard E4 slot. **Recommended fix** (for whoever owns `🗣️dsl`, not applied by me — outside my path):
```rust
// 🚫️async: E4 pure builder feeding Shape::Record(fn() -> RecordSpec) fn-pointer slots — see R9
pub fn new(id: u16, key: &str, shape: Shape) -> Self { ... }   // FieldSpec::new, ::positional, ::optional
pub fn new(keyword: Option<&str>, layout: RecordLayout, fields: Vec<FieldSpec>) -> Self { ... }  // RecordSpec::new
```
Making them sync is safe for every OTHER caller too — a sync fn can always be called from async code without `.await`; the only constraint is the reverse. I read the file (`🗣️dsl/🧬️schema/🦀️component.rs:120-191`) fully before writing this.

### Blocker B — 1 error — `crate::os_spr::decode_envelope`/`encode_envelope` (📡️replication/🔗️causal) must go sync
Line 3477 (and by extension line 3427's `encode_envelope` call, already fixed to drop `.await` since I already de-asyncified the CONSUMING serde module on my side): `operation_envelope_serde::deserialize`/`serialize` are `#[serde(with = ...)]` module functions — serde's derive-generated code requires their signatures to be exactly `fn(...) -> Result<...>`, no `async`, no `.await` possible anywhere in the call chain (E1, hard constraint). I already de-asyncified BOTH functions in my file (tagged `// 🚫️async: E1 ...`), which is the correct and complete fix on my side. The one remaining error is that `crate::os_spr::decode_envelope`'s own body is still `pub async fn`, so `.map_err(...)` can't be called on its result inside my now-sync `deserialize` fn. Same R9 shape as Blocker A: `decode_envelope`/`encode_envelope` are pure binary encode/decode over an in-memory `Vec<u8>` (verified: no I/O in their bodies), consumed by a hard E1 slot.

**Lease-request** (for the coordinator / whichever packet owns `📡️spr`/`📡️replication` and `🗣️dsl`):
```
lease-request:
  file: 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/🦀️component.rs
  change: de-asyncify FieldSpec::new/positional/optional/flatten/defines/call_name and
          RecordSpec::new/new_owned (lines ~141-191) — all pure builders, tag
          `// 🚫️async: E4 pure builder feeding Shape::Record(fn() -> RecordSpec) — see R9`.
          Unblocks 41 errors in 🏪️store (5 direct fn-pointer, 36 via the DslRecord/DslOps derive
          macro's generated code, same root cause).

  file: 🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs
  change: de-asyncify encode_envelope/decode_envelope (lines 362, 378) — pure Vec<u8> codec, no
          I/O — tag `// 🚫️async: E1 serde with-module bridge — see R9`.
          Unblocks 1 error in 🏪️store (operation_envelope_serde's serde-with contract).
```
I did not touch either file — both are outside `🏪️store/**`.

## Files touched
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs` — only file edited, surgical region-scoped edits throughout, `//#region` structure preserved, no full-file rewrites.

## Scratch files left in the ticket folder (per rule 5, `.txt`/`.json`, never `.log`)
- `terra-fanout-store-await-report.json`, `terra-fanout-store-await-report2.json` — `insert-await.py --report` outputs from the two runs.
- `terra-fanout-store-errs2.txt` through `terra-fanout-store-errs7.txt` — successive `cargo check` error snapshots (chronological; `errs7.txt` is the final, current state — 42 errors, both fully explained above).

## Things a sibling or the coordinator must know
1. **Grant the lease above** — it's the only thing standing between 42 and 0 for this packet, and it very likely unblocks other packets hitting the same `FieldSpec`/`RecordSpec`/`decode_envelope` chain from their own modules (the DslRecord/DslOps derive macro is used repo-wide, not just by `🏪️store`).
2. **`📡️spr`'s own crate had a transient E0728** in `🔗️causal/📐️format/🦀️component.rs:296` (`prev_frame`) that blocked my *first* `cargo check` attempt entirely (dependency-graph failure, not my code) — it was gone by my second attempt, so it was a live concurrent edit, not a real defect. Recorded here in case it recurs for someone else mid-build.
3. **Silent dropped-future bugs are a real, distinct hazard class from compile errors** in this codemod — see the section above. I'd recommend whoever does a final `--all-targets`/`cargo test` pass on this crate specifically grep for bare `fn_name(...);`-as-statement calls to known-async fns with no `.await` and no binding, since those never surface as compile errors at all. I found 14 in one function alone.
4. My `👷️worker/🦀️component.rs` and `🔄️sync/🦀️component.rs` (also under my owned path) had **zero** errors attributed to them in every check — nothing to report there.
5. `#[cfg(test)] mod tests { ... }` (lines ~8938–12770 currently) is **out of scope for `--lib`** and was correspondingly out of scope for this packet's Definition of Done — it was NOT part of `cargo check --lib`'s 233/42 counts at any checkpoint (verified: `mod tests` is behind `#[cfg(test)]`, confirmed its start line each time I remeasured). It very likely has the SAME missing-`.await` and R10 residue shapes as the code I fixed (I noticed some in passing, e.g. an entire `impl OpText for DemoMutation` block with ~15 missing awaits, never touched). A follow-up `--all-targets`/`cargo test` pass on this crate will need to sweep it — recommend `async-test-attr.py` first for `#[test] async fn`, then `insert-await.py --scope '🏪️store'` with `--all-targets` wired into its `run_check`, then the same hand-work pattern this report documents.
