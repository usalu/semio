# Wave 0 — Mechanism report

Serial, additive-only framework changes enabling the semantic-mutations overhaul. Full plan:
`/Users/ueli/.claude/plans/the-mutations-are-extremely-compiled-pumpkin.md`.

## Scope cut from the original plan (de-risked for a shared, concurrently-edited live tree)

- **No `Edit.inverse` → `backwards` rename.** Blast-radius check found the golden-schema-matching
  rename would touch ~12 real construction sites across `store`/`causal`/`crdt`/`testkit` plus every
  serde consumer; the cosmetic naming alignment wasn't worth the repo-wide compile break risk.
  `Edit.inverse: Vec<Op>` stays as-is.
- **No `AppliedMutation`/`modifications()` restructuring of `ArtifactStore::replay_mutations`.**
  Checked: `replay_mutations` already calls `vcs::apply_mutation`, which is `operation.diff(base).apply(base)`
  — i.e. it is ALREADY diff-first. The only change made there is deleting the stale
  `TODO(OPERATIONS-TO-MUTATIONS Wave 1)` comment referencing a `crate::os_engine::ArtifactEngine`
  trait that was confirmed dead (grepped: no live `trait ArtifactEngine` anywhere in the codebase).
- **No `Emit.reset_document` / new `HostEffect` variant.** The "no whole-document mutation" decision
  doesn't require new plumbing: `store.reset(...)` (used today by `load_document_text`/
  `load_document_pack`) already is the sanctioned non-history whole-doc-replace path, outside
  `ArtifactApp::handle`'s `Emit`. Per-artifact "replace current content" app gestures are a
  vocabulary/wiring decision for the fan-out wave, not a mechanism change.
- **No `KernelMutation.semantic_kind`/`label` wiring.** `MutationMeta` gained the fields (see below)
  but populating them requires a `Mutation: SemanticMutation<P>` bound the store doesn't have yet
  (no facet implements it until fan-out). Wiring is deferred to the final ratchet wave, once real
  artifacts implement `SemanticMutation`.

## What actually landed (all compiled + tested)

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
- New region `🔖️Semantics`: `APPROVED_VERBS` (34 verb pairs), `str_eq`/`is_approved_verb` (const-fn,
  usable inside compile-time assertions), `SemanticDescriptor { verb, entity, kind, record }`,
  `MutationKind<P, Op>` trait, `SemanticMutation<P>: Mutation<P>` trait.
- New region `🔖️DiffKit`: `NamedTripleDiff<K,V,Patch>` + `named_apply`, `IndexedTripleDiff<V,Patch>`
  + `indexed_apply` — the shared struct shapes that collapse the repo's ~10 hand-copied
  `NamedTripleDiff`/`IndexedTripleDiff` definitions into one. `absorb`/`inverse`/`between` stay
  handcrafted per artifact (documented why: generic composition semantics depend on each artifact's
  own `Patch` type in ways this crate can't know).
  `apply` is what's shared and testable.
- `MutationDescriptor` gained `verb`/`entity`/`record: Option<&'static str>` fields (NOT part of the
  fingerprint — golden pin test untouched) + `with_semantics(&SemanticDescriptor) -> Self` builder.
  `MutationDescriptor::new(...)`'s signature is UNCHANGED (zero call-site breakage).
- `MutationMeta` gained `semantic_kind: Option<SchemaId>` / `label: Option<String>` (`#[serde(default,
  skip_serializing_if)]`) — required updating the 12 real `MutationMeta { .. }` struct-literal
  construction sites across `command`, `testkit`, `crdt`, `causal`, `store` (grepped exhaustively;
  none live in `✏️s`, all framework-internal).
- Full test coverage added: DiffKit apply laws, semantics/verb-table tests, a
  `MutationDescriptor::with_semantics` test, and (region `🧪️MutationsDeriveLaws`) a complete
  synthetic `MiniDoc`/`MiniDiff`/`RenameMini`/`MiniMutation` fixture proving the WHOLE pipeline —
  `#[derive(dsl_derive::Mutations)]` → real `Mutation`/`SemanticMutation` impls → descriptor
  registration — end to end. This fixture is the reference pattern fan-out agents should mirror.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs` (the `protocol` facade)
Re-exports `DiffAlgebra`, `MutationKind`, `SemanticMutation`, `SemanticDescriptor`, `APPROVED_VERBS`,
`str_eq`, `is_approved_verb`, `NamedTripleDiff`, `IndexedTripleDiff`, `named_apply`, `indexed_apply`
— all now reachable as `protocol::X` from any plugin crate (every plugin's `📦️glue.rs` already
declares `extern crate semio_framework_os_kernel as protocol;`).

### `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
Doc-only change: `CollectionMutation` and its three helper fns are re-documented as an INTERNAL
diff/inverse engine for triad leaves to call from inside a handcrafted `🔺️diff`/`↩️inverse` — never
to appear in a public `pub enum *Mutation` again. No code change, no behavior change.

### `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs` (+ its `📦️glue.rs` mirror)
New region `🔖️Mutations`: `#[proc_macro_derive(Mutations, attributes(mutations))]`. Takes
`#[mutations(snapshot = X, diff = Y, schema = "x.doc")]` on a dispatch enum whose every variant must
be a single-field tuple wrapping a `MutationKind` payload (compile error otherwise, with a clear
message naming the offending variant). Generates: `impl Mutation<Snapshot>` (match-delegates
diff/inverse to each payload's `MutationKind` impl), `impl SemanticMutation<Snapshot>`
(kinds/semantics/label/target), a `register_<enum-kebab>_descriptors()` fn, and per-variant
`const _: () = assert!(...)` compile-time checks that `SEMANTICS.kind` matches the variant's own
kebab form and that `SEMANTICS.verb` is approved. Re-exported as `crate::os_dsl::Mutations` /
`dsl::Mutations` alongside the existing `DslOps`/`DslRecord`/etc.

**Gotcha found and fixed**: this crate's Cargo entry point is `📦️glue.rs`, a file the tooling keeps
as a byte-for-byte MANUAL MIRROR of `🦀️component.rs` (no `#[path]` include) — editing only
`component.rs` silently doesn't compile. Fixed via `mcp__repo__file_integrate`, then a direct
overwrite once that tool's merge produced a duplicate. **Any future edit to this specific derive
crate must land in both files** (or re-run file_integrate) — flagging this since it cost real time
to discover and every other framework crate I touched uses real `#[path]` inclusion instead.

### `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
Four new `pub fn assert_*` law helpers matching the file's existing one-law-per-fn style:
`assert_mutation_diff_absorb_law`, `assert_mutation_inverse_law`, `assert_diff_algebra_between_law`,
`assert_diff_algebra_inverse_law`. Each has its own self-test using the file's existing `AddOp`/
`AddDiff` toy fixture (gained a `DiffAlgebra<i64>` impl for the between/inverse tests). These are
what every fan-out facet's `🧪️Tests` region should call per mutation kind.

### `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
Cosmetic only: replaced the stale `TODO(OPERATIONS-TO-MUTATIONS Wave 1)` comment (referencing the
dead `ArtifactEngine` trait) with an accurate one; fixed the 7 `MutationMeta` literal sites in this
file for the new fields.

## Verification

- `cargo check -p semio-framework-os-kernel` — clean (only pre-existing warnings).
- `cargo test -p semio-framework-os-kernel --lib -- command::` — 23/23 pass, including the full
  derive-macro end-to-end fixture test.
- `cargo check -p semio-framework-os-kernel-dsl-derive` — clean.
- Full-crate `cargo test -p semio-framework-os-kernel --lib` shows 3 unrelated failures from a
  DIFFERENT, concurrent session's in-flight work: a new `🔖️Inference` region/test added to the same
  `command/component.rs` file mid-session (ticket
  `INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING`, not this one), and two
  `os_dsl::fixture_sweep::m5_*` grammar-coverage failures for fem/norm/dag/stdio-dwg fixtures — none
  of these touch any file this ticket edited. Not fixed here per house policy on concurrent
  workspace churn (poll/verify scope, don't chase another session's WIP).
- Policy rules (`📜️script.ts`) handled as a separate sub-task — see `📓️wave0-policy-report.md` (or
  check ticket status if not yet written).

## What downstream waves (1–4) get from this

- A working `MutationKind`/`SemanticMutation`/`#[derive(Mutations)]` pipeline, proven correct by a
  real compiling+passing fixture (`command/component.rs`'s `MiniMutation` test) — fan-out agents
  copy that shape verbatim, substituting their artifact's real snapshot/diff/payload types.
- `assert_mutation_inverse_law`/`assert_mutation_diff_absorb_law`/`assert_diff_algebra_*_law` to
  drop straight into each facet's existing `🧪️Tests` region.
- `NamedTripleDiff`/`IndexedTripleDiff`/`named_apply`/`indexed_apply` to stop copy-pasting the same
  struct shapes into every artifact's diff module.
- The taxonomy (`📓️taxonomy.md`) and derivation recipe (`📓️derivation-rules.md`) in this same ticket
  folder.
