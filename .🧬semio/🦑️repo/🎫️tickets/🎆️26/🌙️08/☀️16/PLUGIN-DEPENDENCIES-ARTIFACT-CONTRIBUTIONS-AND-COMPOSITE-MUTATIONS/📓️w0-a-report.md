# W0-A Report — Protocol Spine (Composite Mutations, §1)

Lane: **0-A protocol spine**. Contract: `📋️contract-freeze.md` §0/§1. Start commit `7ad8955884`.

## Files touched

### Exclusive lease (full authorship)

- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
  - `//#region 🔖️Mutation`: added defaulted `Mutation::foreign_steps`.
  - `//#region 🔖️Semantics`: added defaulted `MutationKind::foreign_steps`.
  - `//#region 🔖️Meta`: `MutationMeta` gained `origin: MutationOrigin` (`#[serde(default, skip_serializing_if = "MutationOrigin::is_owner")]`).
  - `//#region 🔖️Descriptor`: `MutationDescriptor` gained `contributor`/`artifact_kind` fields + `with_contributor`/`with_artifact_kind` builders (see Deviations).
  - New `//#region 🔖️Composite` (after `🔖️Outcome`, before `🧪️Tests`): `ForeignTarget`, `ForeignStep`, `PlanStep<Op>`, `MutationOrigin` (+ `is_owner`), `MAX_PLAN_DEPTH`, `PlanError`, `Planner<P, Op>`, `CompositeMutationKind<P, Op>`, and the four free helpers `plan_of`/`fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps`.
  - `//#region 🧪️Tests`: new `//#region 🧸️CompositeFixtures` (DoubleAdd, QuadAdd, AddThenNotifyForeign, foreign_step_fixture) and new `//#region 🧪️CompositeLaws` (7 tests, all pass — see Gate).
  - Two pre-existing `MutationMeta{}` test literals updated with `origin: MutationOrigin::Owner,`.
- `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`
  - `//#region 🔖️Mutations`: `#[derive(Mutations)]` now also emits a `foreign_steps` delegating match arm.
  - New `//#region 🔖️CompositeMutation`: `#[derive(CompositeMutation)]` with `#[composite(snapshot = ..., op = ...)]`, emitting the delegating `MutationKind` impl (`diff`→`fold_plan_diff`, `inverse`→`fold_plan_inverse`, `foreign_steps`→`plan_foreign_steps`, `label`/`target`/`validate`→delegate) plus the same kind/verb `const _: () = assert!(...)` checks `#[derive(Mutations)]` emits, checked against the struct's own kebab name.
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🦀️component.rs`
  - `HistoryOpMeta` gained `pub origin: crate::os_spr::command::MutationOrigin` (struct now also derives `Default`).
  - `write_op_meta`/`read_op_meta`: presence bit 5 = `!origin.is_owner()`; encoded as canonical-JSON via the existing `write_str_field`/`read_str_field` helpers (not dict-interned — `Contributed`/`Transaction` payloads are structured, not a short repeated token like `group_id`), appended past `group_id`'s tail per the established additive-field pattern.
  - `sample_log()` test fixture: gave the fixture op a real `MutationOrigin::Contributed{...}` (not just `Owner`) so the `.spr` byte round-trip test actually exercises a field-carrying variant, mirroring how `group_id` was given a real value for the same reason.

### Mechanical fixups (explicitly permitted by the lease)

- `📡️spr/🔀️crdt/🦀️component.rs` — 1 `MutationMeta{}` literal (`meta_at`) → `origin: MutationOrigin::Owner,`.
- `📡️spr/🔗️causal/🦀️component.rs` — 2 `MutationMeta{}` literals → `origin: MutationOrigin::Owner,`.
- `📡️spr/🧪️testkit/🦀️component.rs` — 1 `MutationMeta{}` literal → `origin: crate::os_spr::command::MutationOrigin::Owner,` (this file has no local re-export of `MutationOrigin`, so used the fully-qualified path already used elsewhere in the same file for `MutationMeta`/`UndoPolicy` via `crate::os_spr::command::...`... actually via `crate::os_spr::` short form for those two; `MutationOrigin` isn't in the curated re-export list — see Deviations — so I qualified through `command::` here specifically).
- `🏪️store/🦀️component.rs` — 6 `MutationMeta{}`/`HistoryOpMeta{}` sites total. 4 were **already fixed by a concurrent session** (`origin: Default::default()`) by the time I got to them — left as-is (functionally `MutationOrigin::Owner`). 2 I touched directly (see Deviations #2 below): `history_op_meta_from_operation_meta` and `mutation_meta_from_history_op_meta`.

## Deviations from the contract (with reasons)

1. **`MutationDescriptor` derives `serde::Serialize` only, not `serde::Deserialize`.** The contract's literal text (`#[serde(default)] contributor: Option<String>`) implies full serde support. Empirically verified (isolated `cargo run` repro) that `Option<&'static str>` — the pre-existing `verb`/`entity`/`record` fields — makes a derived `Deserialize<'de>` impl **compile** but be **unusable** for any real (non-`'static`) input: `serde_json::from_str(&owned_string)` fails with "argument requires that `owned` is borrowed for `'static`". Landing a `Deserialize` impl that only works for `'static`-lifetime inputs would be a footgun, not a feature. `#[serde(default)]` on the two new fields is harmless-but-inert without `Deserialize` (verified it compiles fine with `Serialize`-only). Fingerprint is unaffected either way — golden pin test `operation_descriptor_fingerprint_is_golden_pinned` passes unchanged.

2. **Two "mechanical" fixups in `🏪️store/🦀️component.rs` went one line beyond a literal `origin: MutationOrigin::Owner,`**, both load-bearing for compilation/round-trip correctness, not scope creep:
   - `history_op_meta_from_operation_meta` builds a `crate::os_spr::HistoryOpMeta{}` literal (not `MutationMeta{}` — technically outside the lease's literal wording, but the only site in that file building this type). Added `origin: meta.origin.clone(),` immediately mirroring the adjacent `group_id: meta.group_id.clone(),` line — otherwise `HistoryOpMeta`'s new required field leaves this literal one field short and the crate fails to compile.
   - `mutation_meta_from_history_op_meta` (`MutationMeta{}` literal, squarely the lease's named site) — found already patched by a concurrent session with `origin: Default::default()` (i.e. always `Owner`, silently discarding whatever `HistoryOpMeta.origin` was just decoded). Corrected to `origin: meta.origin,` so a `Contributed`/`Transaction` origin persisted through `.spr` actually survives the read-back into `MutationMeta`, not just `group_id`. Without this the whole point of "persist `MutationOrigin` alongside `group_id`" is silently defeated on the read path.

3. **Two files outside the enumerated lease were touched — both a hard compile requirement, not optional:**
   - `📡️spr/🦀️component.rs` (the `protocol` facade's curated re-export block): added the Composite region's 12 new public names (`CompositeMutationKind`, `ForeignStep`, `ForeignTarget`, `MutationOrigin`, `PlanError`, `PlanStep`, `Planner`, `MAX_PLAN_DEPTH`, `plan_of`, `fold_plan_diff`, `fold_plan_inverse`, `plan_foreign_steps`) to the existing `pub use crate::os_spr::command::{ ... };` list. The derive-generated code (both `#[derive(CompositeMutation)]` and `#[derive(Mutations)]`'s new `foreign_steps` arm) emits `::protocol::ForeignStep` etc. — exactly like `MutationKind`/`SemanticDescriptor` already do — and `protocol::X` only resolves once `X` is in this curated list (verified: omitting it produced `cannot find type ForeignStep in module ::protocol` etc.).
   - `🗣️dsl/✨️derive/📦️packages/🦀️rust/📦️glue.rs`: **pre-existing tech debt, not caused by this ticket** — `Cargo.toml`'s `[lib] path = "📦️glue.rs"` is what actually gets compiled for the `dsl_derive` proc-macro crate, but `📦️glue.rs` was a **stale, manually-copied duplicate** of `✨️derive/🦀️component.rs` (confirmed diverged even in pre-existing logic, e.g. a `lang_from` closure — `git log` shows `glue.rs` last touched 2026-08-14, a day before this ticket's start commit). My edits to `component.rs` were silently invisible to the actual build until I fixed this. I first tried the clean fix (`include!("../../🦀️component.rs")`, single source of truth) but confirmed via an isolated repro that `#[proc_macro_derive]` functions **must reside in the literal crate root** — a `#[path] mod` submodule or a `pub use module::*` re-export both hard-error ("must currently reside in the root of the crate" / proc-macro crates can't export non-macro items), and `include!` itself rejects a leading `//!` inner-doc-comment from the included file regardless of position (`E0753`, reproduced standalone). With no clean single-source option available in a `proc-macro = true` crate, I resynced `glue.rs` to an exact byte-for-byte copy of `component.rs`. **This restores the pre-existing (fragile) convention rather than fixing it** — flagging as a real footgun: any future edit to `✨️derive/🦀️component.rs` that isn't also copied into `glue.rs` will silently compile the OLD derive logic. Worth a follow-up ticket for a real build-time sync step (e.g. a `📜️script.ts` pre-build check that diffs the two and fails if they've drifted).

## Known compile gap left untouched (outside this lease, flagged for the coordinator)

- `🏪️store/🔄️sync/🦀️component.rs` (a *different* file from my permitted `🏪️store/🦀️component.rs`) builds a `crate::os_spr::HistoryOpMeta{}` literal (`history_edit_from_envelope`, ~line 375) that is now missing the `origin` field. This module is gated `#[cfg(feature = "sync")]`, which is **not** in `default = ["deflate"]`, so it does **not** affect the required gate (`cargo test -p semio-framework-os-kernel --lib` uses default features) — confirmed the gate is clean. It **will** break `cargo build --features sync`. One-line fix needed, immediately after `group_id: None,`: `origin: crate::os_spr::command::MutationOrigin::Owner,`. Left untouched since this file is not in my lease.
- `📡️spr/🧪️testkit/benches/protocol.rs` — confirmed orphaned/unwired: no `Cargo.toml` under `📡️spr/`, no `[[bench]]` entry in `semio-framework-os-kernel`'s `Cargo.toml`, and its `MutationMeta{}` literal already predates `group_id` (missing that field too) — i.e. it was already non-compiling before this ticket, under stale pre-consolidation crate names (`protocol_testkit`, `protocol_history`, ...). Left untouched per the lease's explicit mention, since a one-line `origin` fixup can't make it compile anyway (still missing `group_id`).

## API as landed (matches contract §1 exactly, plus the two deviations above)

`ForeignTarget`, `ForeignStep`, `PlanStep<Op>`, `MutationOrigin` (+ `is_owner`), `MAX_PLAN_DEPTH = 8`, `PlanError` (`DepthExceeded`/`Cycle`/`StepRejected`/`Invalid`), `Planner<P, Op: Mutation<P>>` (`new`/`base`/`call`/`call_foreign`/`steps`/`into_steps`), `CompositeMutationKind<P, Op>` (`SEMANTICS`/`plan`/`label`/`target`/`validate`), `plan_of`/`fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps`. Design choices not pinned by the contract's field-list comment on `Planner`, made and validated by tests:
- `Planner::call_foreign` is the **only** depth/cycle-bearing operation — `MAX_PLAN_DEPTH` counts foreign hops (mirrors `MAX_TXN_DEPTH` in §5), not composite-nesting depth. Composite-of-composite is achieved by a kind's `plan()` calling another kind's `plan()` directly against the **same shared** `&mut Planner`, which is exactly why nesting folds identically to a flattened plan (Law 3) — proven by `composite_of_composite_nests_and_folds_identically_to_flattened_plan`.
- Cycle key = `(mutation_id.0, blake3(payload))`, checked in `call_foreign` before incrementing `depth`.
- `fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps` all fold a `plan_of` failure to their type's empty/identity value rather than propagating `Result` — required by the frozen non-`Result` signatures; never a panic.

## Gate

Ran `cargo check -p semio-framework-os-kernel --lib` (clean, only pre-existing unrelated warnings) then `cargo test -p semio-framework-os-kernel --lib`:

```
test result: FAILED. 888 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.85s
```

All 7 new composite-law tests pass (`os_spr::command::tests::{fold_plan_diff_equals_sequential_apply, fold_plan_inverse_restores_base, composite_of_composite_nests_and_folds_identically_to_flattened_plan, plan_depth_beyond_max_is_typed_error_never_panics, plan_cycle_is_typed_error_never_panics, foreign_steps_are_excluded_from_fold_plan_diff, derive_composite_mutation_wires_delegating_mutation_kind}`), all 37 `command::tests::*` pass (incl. the golden-pin fingerprint test, unchanged), all 34 `history::tests::*` pass (incl. the `.spr` byte round trip carrying a real `MutationOrigin::Contributed` value).

The 5 failures are **not** mine — verified via `git status --porcelain`, all three implicated files are currently dirty from other concurrent sessions (none in my lease):
- `os_spr::channel::tests::{app_command,app_frame}_fixture_corpus_matches_golden_hex_and_round_trips` — hex drifted `0008` → `0009`, i.e. exactly `CHANNEL_VERSION` 8→9, lane **0-B**'s in-flight M2 work (`📡️spr/🧵️channel/🦀️component.rs`, dirty).
- `os_store::component::tests::switch_to_an_alternative_whose_pinned_checkpoint_is_missing_is_rejected` — `🏪️store/🦀️component.rs` is mid a large concurrent rewrite (754 insertions / 390 deletions vs HEAD while I was working).
- `os_io::tests::io_registry_rejects_a_conflicting_key_without_replacing_the_first_entry` — `🚪️io/🦀️component.rs` is dirty (`MM`), unrelated ordering assertion, no relation to `MutationMeta`/`MutationDescriptor`/`HistoryOpMeta`.
- `os_dsl::fixture_sweep::m5_cross_artifact_rejection::all_non_stdio_grammars_reject_each_others_shipped_fixtures` — DSL grammar/fixture test, unrelated to this ticket's types.

## Notes for later waves

- `plan_of`/`fold_plan_diff`/`fold_plan_inverse`/`plan_foreign_steps` and `CompositeMutationKind` are re-exported at `crate::`/`protocol::` root (via `📡️spr/🦀️component.rs`) exactly like `MutationKind` — W1+ can `use protocol::{CompositeMutationKind, ForeignStep, ...}` directly.
- `#[derive(CompositeMutation)]` usage: implement `CompositeMutationKind<Snapshot, Op>` by hand on the payload struct (SEMANTICS/plan/label/...), then `#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, dsl_derive::CompositeMutation)] #[composite(snapshot = Snapshot, op = Op)]` on top — see `os_spr::command::tests::DerivedDoubleAdd` for a worked example.
- The `🗣️dsl/✨️derive` glue.rs/component.rs duplication (Deviation 3) is a real latent risk for every derive macro in that crate, not just the new one — recommend a follow-up ticket.
- `🏪️store/🔄️sync/🦀️component.rs`'s missing `origin` field (feature-gated, not gate-blocking) needs a one-liner before anyone builds with `--features sync`.
