# H3 — db authority lane: finish w2-e (db + hub)

Read `📋️contract-freeze.md` §C9/§C10 and `📓️w2-e-report.md` (a stub — verification never finished)
first. Logs: `🧪️h3-test-initial.txt` (baseline, 417 passed/7 failed), `🧪️h3-testkit-detail.txt`
(per-test `--nocapture` detail), `🧪️h3-test-after-fix.txt` (final, 423 passed/1 failed),
`🧪️h3-hub-check.txt` (`cargo check -p semio-hub`, 0 errors), `🧪️h3-mutation-outcome-law.txt`
(gate, 0 breaches).

## The 7 failures — verdict per test

All 6 of the first group were **the same bug**: commit `e648c495c2` (this ticket's own w2-e lane)
added `db_artifact::encode_pathmap_json` and switched every write-side test helper
(`WorkloadGen::disjoint_batch`, `schema_erased_envelope`, `db_engine`'s `envelope()`, the facade
test's `envelope()`) to it — correctly, since the pathmap wire format is `store::pack_rt`-encoded
`DslValue` bytes, never raw JSON text. But the matching **read-side** assertions were left calling
`serde_json::from_slice` directly on those same pack_rt bytes, which always fails with `json:
Error("expected value", line: 1, column: 1)` (pack_rt bytes never start with valid JSON). Verdict:
**code bug, not stale-CRDT test** — fixed by adding a decode counterpart and pointing every
read-side assertion at it.

1. `db_testkit::tests::law_inverse_undo_roundtrip` — fixed.
2. `db_testkit::tests::law_preview_never_durable` — fixed.
3. `db_testkit::tests::workload_gen_disjoint_batch_is_deterministic_and_covers_distinct_paths` — fixed.
4. `db_engine::tests::a_document_survives_a_full_database_shutdown_and_reopen_at_the_same_root` — fixed.
5. `db_engine::tests::full_submit_durable_query_round_trip_over_a_real_document_authority` — fixed.
6. `db_facade::tests::full_round_trip_reachable_purely_through_facade_reexports` — fixed.
7. `db_artifact::tests::bridge::envelope_from_operation_uses_operation_and_diff_traits` —
   **NOT fixed, out of lease.** Panics `invalid type: integer 15, expected tuple struct Counter`
   deserializing a 1-field tuple struct back out of a `DslValue::Number`. Root cause traced into
   `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧬️schema/dsl_value_serde.rs`: `ValueDeserializer`'s
   `deserialize_newtype_struct` is blanket-forwarded to `deserialize_any` via
   `forward_to_deserialize_any!`, so a scalar payload calls `visitor.visit_u64(15)` directly instead
   of `visitor.visit_newtype_struct(self)` — the derived `Deserialize` for a newtype/1-field tuple
   struct only implements `visit_newtype_struct`/`visit_seq`, not `visit_u64`, hence the "invalid
   type" error. This is a real codec bug, but it lives in `🗣️dsl`, outside this lane's lease
   (`🛢️db/**` + hub `📦️bin.rs`), and the test itself is correct/current (asserts real
   `Mutation`/`MutationDiff` trait usage, untouched by this ticket beyond the `MutationOutcome`
   return-type change). Per the worker-brief rule ("if you must touch a file outside it, STOP and
   report"), left as-is and reported here for the coordinator/`🗣️dsl`'s owning lane.

## Fix applied (all inside `🛢️db/**`)

- `📄️artifact/🦀️component.rs`: added `pub fn decode_pathmap_json(bytes) -> Result<serde_json::Value,
  DbError>` right after `encode_pathmap_json` (its exact inverse — `decode_pathmap` +
  `dsl::from_dsl_value`); simplified the file's own private test helper `stored_json` to delegate to
  it instead of duplicating the two-line decode.
- `🧪️testkit/🦀️component.rs`: `assert_inverse_undo_roundtrip` (2 sites), `assert_preview_never_durable`
  (2 sites), `workload_gen_disjoint_batch_is_deterministic_and_covers_distinct_paths` (1 site) —
  every `serde_json::from_slice(...)` on pack_rt bytes replaced with `db_artifact::decode_pathmap_json(...)`.
- `⚙️engine/🦀️component.rs`: 2 sites (both `full_submit_durable_query_round_trip...` and
  `a_document_survives_a_full_database_shutdown_and_reopen...`), same fix.
- `🦀️component.rs` (db facade root): 1 site in `full_round_trip_reachable_purely_through_facade_reexports`,
  using the facade's own `document::decode_pathmap_json` re-export (keeps the test's whole point —
  "everything reachable purely through facade re-exports" — intact).

## Hub verification (§C9)

`🌎️hub/📦️packages/🦀️rust/📦️bin.rs` already carries the policy correctly (landed by the prior w2-e
pass, just never verified): `merge_policy_from_env` → `HubState.merge_policy` → `submit_commands`
passes it as `SubmitOptions.policy`; a `DbError::Rejected` maps to `ApplyOutcome::Rejected{reason,
messages}` via `messages_for_error`/`encode_messages`; the accepted path's gap (accepted-but-degraded
`receipt.messages` has no `ApplyOutcome::Accepted`/`ServerFrame::Commands` field to carry it on the
currently-landed `📡️wire` shape) is already correctly identified and documented in-file as lane
1-C's wire-widening call, not something this lease can fix unilaterally. Nothing changed here — just
confirmed by reading + `cargo check`.

Ran `cargo test -p semio-hub --lib` as an extra sanity check (not in the required VERIFY list): 7
pre-existing `directory::tests::*` failures (`FOREIGN KEY constraint failed`, sqlite backend) — file
`🌎️hub/📇️directory/🦀️component.rs`, outside this lease, last touched by commit `23d0db6833`
(unrelated, pre-ticket). Not investigated further — out of scope for this lane.

## VERIFY (real numbers)

- `cargo test -p semio-framework-os-kernel-db --lib` → **423 passed; 1 failed** (was 417/7). The 1
  remaining is the out-of-lease `🗣️dsl` bug above.
- `cargo check -p semio-hub` → **0 errors** (62 pre-existing warnings, all unrelated
  `unused_qualifications`/`unexpected_cfgs` lint noise, not from this lane's edits).
- `bun ./📜️script.ts verify mutation-outcome-law` → **0 breaches**, including the no-CRDT-vocabulary
  check; also independently grepped `🛢️db/**` for `merge_strategy|MergeStrategyKind|
  merge_concurrent_diffs|ConflictRule|ResolutionPlan|combine_conflict_rules|conflict_rule|
  parse_merge_strategy|describe_merge_strategy` — zero hits.

## Files touched

- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📄️artifact/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🧪️testkit/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🦀️component.rs`
- `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🦀️component.rs`
- (read-only, verified, unchanged) `/Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust/📦️bin.rs`

Ticket left open (not closed by this lane per the worker brief).
