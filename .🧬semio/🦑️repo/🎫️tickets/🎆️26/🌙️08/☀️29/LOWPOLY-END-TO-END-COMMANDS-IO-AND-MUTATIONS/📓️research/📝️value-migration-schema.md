# value_derive migration — 🧬️schema scope

## Primary root cause
`#[derive(dsl::Mutations)]` on `LowpolyMutation` (`🧬️mutations/🦀️component.rs:45`) failed its own
precondition, not a value_derive issue: `error: Mutations source authority failed: aggregate source is
not the taxonomy canonical mutation primary`. The derive (`dsl_derive::mutation_aggregate_source_authority`
in `🧰️framework/.../🗣️dsl/✨️derive/🦀️component.rs`) requires the file the enum is defined in to be
named exactly `{emoji}{extension}` from the taxonomy's `rust-source` fileKind — i.e. `🦀️.rs`, not
`🦀️component.rs`. stdio's `ply` sibling (`.../☁️ply/.../🧬️schema/🧬️mutations/🦀️.rs`) already follows this
convention; lowpoly's aggregate file did not. This single filename mismatch was the sole cause of the
`LowpolyMutation: Mutation<LowpolySnapshot>` cascade (~75 sites, all outside 🧬️schema too).

## Changes made (all under `$A/🧬️schema/`, my exclusive scope)
- Renamed `🧬️mutations/🦀️component.rs` → `🧬️mutations/🦀️.rs`.
- `🧬️schema/🦀️component.rs`: updated its own `include_str!("🧬️mutations/🦀️component.rs")` →
  `"🧬️mutations/🦀️.rs"`.
- Fixed pre-existing (not value_derive-related, only surfaced under `--tests`) stale
  `<leaf>::mutation::Type` path references — a dead nested-module shape from before the mutation-leaf
  split; leaves now export their payload type directly (`create_object::CreateObject`, no `mutation`
  submodule). Fixed in `🧬️schema/🦀️component.rs`, `🧬️mutations/💾️binary/🦀️component.rs`,
  `🧬️mutations/📝️text/🦀️component.rs` (20 call sites total).
- `🧬️mutations/💾️binary/🦀️component.rs`: two test call sites used `store::ArtifactStore::new(...)` and
  `.dispatch(...)` without `.await` — both are `async fn` now (pre-existing async-convention debt, not
  value_derive). Added `.await`.
- `LowpolyDiff`/`LowpolySnapshot`/`LowpolyInference` already had `ToValue`/`FromValue` from the earlier
  mechanical pass — no gap found. No `E0046 DESCRIPTORS/descriptor` gap found inside 🧬️schema (that
  error class exists only in `✏️editor/`, out of scope).

## Verification
`cargo check -p semio-s-plugin-lowpoly --lib` then `--tests`, `--message-format short`, filtered to
`🧬️schema` paths. Before: every leaf + the aggregate + `🦀️component.rs` erroring
(`Mutation<LowpolySnapshot>` not satisfied, cascade). After: **0 errors under `🧬️schema/`** in both
`--lib` and `--tests`.

Caveat: verifying `--tests` required the crate to link past `📦️glue.rs`'s `#[path]` mount, which still
points at the deleted `🧬️mutations/🦀️component.rs` (packages is not my scope). I temporarily edited that
one line locally to compile/verify, then reverted `📦️glue.rs` byte-for-byte to its original content
before finishing — it is unchanged in the working tree right now.

## Handoffs (files I do not own, need this exact update)
1. **`✏️s/🔌️plugins/💠️lowpoly/📦️packages/🦀️rust/📦️glue.rs`** (packages owner) — line 83:
   `#[path = "…/🧬️schema/🧬️mutations/🦀️component.rs"]` → `#[path = "…/🧬️schema/🧬️mutations/🦀️.rs"]`.
   **Blocking**: the whole crate currently fails to compile (`couldn't read …/🦀️component.rs: No such
   file`) until this one-line change lands — the rename is the confirmed fix, not optional.
2. `✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🧪️tests/mutate-lowpoly-1/🦀️.rs:48` — doc-comment only,
   references the old `.../🧬️mutations/🦀️component.rs` path; cosmetic, update when convenient.
3. `✏️editor/` still has real, unrelated errors (E0046 DESCRIPTORS/descriptor on 2+ impls,
   `LowpolyTransient: ToValue/FromValue` missing, stale `mutations::<leaf>::mutation::Type` imports
   matching the same dead-submodule shape I fixed in schema, `Label` API mismatches, a few `.await`
   gaps) — all outside my scope, editor agent's territory, unaffected by my changes.

## Error counts, 🧬️schema/ path only
- Start of this session (after prior mechanical pass): all schema mutation leaves + aggregate +
  `🦀️component.rs` cascading off `Mutation<LowpolySnapshot>` (~20 distinct schema-path error lines).
- End: 0 (`--lib` and `--tests`, both confirmed by real `cargo check` runs, not assumed).
