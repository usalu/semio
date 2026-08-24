# os-kernel `🏪️store` Blocker Clearance — 2026-08-24

## TL;DR

Applied the 62 mechanical fixes plus the `drain_applied_envelopes` → `take_next_applied()`
migration identified in `📓️os-kernel-store-refactor-assessment-2026-08-23.md`.
`cargo check -p semio-framework-os-kernel --lib` went from **63 errors → 6 errors**. The 6
remaining are **not mechanical**: 1 is the pre-flagged `ArtifactEnvelope: Clone` DESIGN
question (left untouched, per instruction), and 5 are freshly-surfaced fallout from a
**separate, concurrent, in-flight refactor** (`semio-framework-job`'s
`INTERACTIVE-JOB-RUNTIME-REFACTOR` / `RetainedJobPayload`) that landed on disk partway through
this session — not something I should guess-fix. Full stop-and-report on both, per the
assignment's own instructions.

## What was done

### `drain_applied_envelopes` migration (5 call sites → `take_next_applied()`)
- `📡️spr/🦀️component.rs:24-28` — added `MutationDagAppliedStep` to the `os_spr` facade's
  `pub use crate::os_spr::causal::{...}` re-export list (row A, 3× E0433).
- `📡️spr/🧪️testkit/🦀️component.rs` — added a private `take_all_applied(dag) -> Vec<MutationEnvelope>`
  helper mirroring the peer's own tested loop pattern
  (`🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs:791-800`), used at the two live
  call sites (`assert_op_dag_convergence`, `assert_merge_convergence`); updated one stale doc
  comment that referenced the old name.
- `📡️spr/🧪️testkit/benches/protocol.rs:122` — inlined the same loop (benches don't share the
  testkit module).
- The assessment's "5 call sites" count included this doc comment; there were 3 real call sites,
  now all migrated. Semantics are unambiguous — `take_next_applied()`'s docstring and the peer's
  own test helper make the drain/consume/order contract explicit, so no call site needed a stop.

### The 62 mechanical fixes (rows A–N from the assessment)
All applied in batches of 1-10, `cargo check` between each, per the instruction to avoid the
previous attempt's mistake of losing the thread. Notable ones beyond straightforward
rustc-suggested edits:

- **Row C (`Sync` bound) cascaded much further than the assessment's "3 sites" estimate.**
  `SnapshotReadLeaseRegistry::try_issue`/`try_take_one_returned` need `T: Sync`; adding `P: Sync`
  as a **method-scoped** `where` clause (not the shared `impl<P, Mutation> ArtifactStore<P,
  Mutation>` header, confirmed by direct experiment — see below) had to propagate through every
  caller in the chain: `snapshot_read` → `take_returned_snapshot_read_retirement` →
  `retire_resolution_candidate` → `resolve_conflict`/`dispatch_inner` → `dispatch` →
  `dispatch_text`/`dispatch_binary`, plus the `ArtifactStoreCloseView`/`ArtifactStoreCursorDisposer`
  wrapper impls, `apply_ops_binary_impl`, `ArtifactCodec::of`, and 4 testkit law fns
  (`assert_store_roundtrip`, `assert_document_text_round_trip`, `assert_document_pack_round_trip`,
  `assert_live_equals_replay`, `assert_subset_roundtrip`). ~18 sites total, not 3.
  **I tested the alternative** (adding `Sync` to the shared impl header) as a direct experiment:
  it made the error count *worse* (42 → 54), confirming the assessment's instinct not to touch it.
- **The `Drop`-impl trap (flagged in the assessment) recurred twice more**, beyond
  `ArtifactOwnedSprEditAuthority`: `ArtifactEnvelopeFreshVcsAuthority` and
  `ArtifactEnvelopeFreshFieldDecoder` both needed `Send [+ 'static]` added to their **struct
  definitions** (not just their `Drop` impls) before the `Drop` impl could add the same bound —
  Rust requires exact bound parity between a type and its `Drop` impl.
- **The `+ 'static` trap (flagged in the assessment) recurred beyond the two testkit law fns**:
  `ArtifactEnvelopeFreshVcsAuthority`'s and `ArtifactEnvelopeFreshFieldDecoder`'s own inherent
  impls and struct definitions needed `Send + 'static` (not just `Send`) once their `Send`-only
  version compiled cleanly but exposed a live `'static` requirement one layer down
  (`ArtifactStoreVcsRetirement`/`ArtifactStoreDecodedEditRetirement` construction).
- **Two more borrow-checker (E0502) instances of the same `ManuallyDrop`-mediated-index pattern**
  surfaced in `PresencePeersPublication::adopt` (lines with `self.created.entries[self.created.len]
  = Some(...)`) only *after* fixing the row-E `Arc::new` reorder — they'd been masked by the
  original type error suppressing borrowck for that function. Same fix shape (hoist the index into
  a local) applied.
- Similarly, several new `self.diagnostic(...)`-vs-`ManuallyDrop`-mutable-borrow E0502s surfaced in
  `ArtifactOwnedSprMutationArrayAuthority` only once its own `Send` bound cascade resolved. Same
  fix shape (hoist `path` into a local, construct `OwnedSchemaDecodeDiagnostic` inline instead of
  calling `self.diagnostic`) applied at each.

None of this contradicts the assessment's classification — every fix was still a single,
rustc-confirmed, unambiguous edit — but the true fix count for rows C and F–M was closer to 35-40
individual edits across ~6 more compile/patch iterations than the assessment's own scoped
estimate, exactly matching its own warning that this cluster "needs 2-3 iterative passes, not
one."

### DESIGN item — left untouched, exactly as instructed

`🏪️store/🦀️component.rs:8836-8837` (`ParsedDocumentText<P, Mutation>` deriving `Clone` over
`ArtifactEnvelope<P, Mutation>`, which no longer implements `Clone`) is unchanged. Still 1×
E0277. The peer's call, not mine — see the assessment doc for the full option analysis.

## What's newly blocking — NOT part of this assignment's scope

5× E0308 in `🏪️store/🦀️component.rs` (lines 6637, 6714×2, 6757×2), all `expected
RetainedJobPayload, found Vec<u8>`/`Vec<_>`. `RetainedJobPayload`
(`🧰️framework/🔨️modules/🧵️job/🦀️component.rs:342-423`) is a **new, non-trivial, page-pool-backed
retained-buffer type** — no `From<Vec<u8>>`, no simple constructor; building one requires a
`RetainedJobPayloadWriter` (pool allocation + seal/finish), which needs context
(`JobPayloadStream`, a ledger) not available at the 3 call sites in `store/component.rs`
(`terminal_fault`, `step` — both just want to stuff a small diagnostic/empty payload into a
`semio_framework_job::JobFault`/`CommitCandidate`).

This is fallout from a **separate, concurrent, in-flight refactor**: `semio-framework-job`
(`🧰️framework/🔨️modules/🧵️job/🦀️component.rs`) was being actively edited by another session for
essentially this entire task (confirmed via `git status --porcelain`/`git diff --stat`, not
mtime — the working tree went from clean, to a small 8-line diff, to 1500+ inserted lines, to
finally stabilizing at a compilable state partway through my work; the ticket folder itself shows
this is the `INTERACTIVE-JOB-RUNTIME-REFACTOR` / `EVERY-TOOL-INTERACTIVE-JOB-MIGRATION` effort's
own doing). It only became visible once the job crate itself started compiling again — before
that, `cargo check -p semio-framework-os-kernel --lib` failed upstream at the job crate and never
reached these 3 call sites.

**I did not touch these.** They need the `RetainedJobPayload` migration's own author: what should
`terminal_fault`'s `JobFault.detail` and the two empty `CommitCandidate`s actually construct
(`RetainedJobPayload::empty(stream)` needs a `JobPayloadStream` value I have no basis to pick
correctly for these 3 unrelated call sites), and that's a real design/integration decision, not a
rustc-suggested one-liner.

## Final verification state

- `cargo check -p semio-framework-os-kernel --lib` → **6 errors** (down from 63), all previous
  62+drain-migration errors cleared. Remaining: 1× DESIGN (untouched, as instructed) + 5×
  out-of-scope `RetainedJobPayload` fallout from the concurrent job-crate refactor.
- `cargo check -p semio-framework --lib` → same 6 errors (depends on os-kernel).
- `cargo check -p semio-s-plugin-stdio --lib` → same 6 errors (depends on os-kernel).
- **Did not reach** the `bun ./📜️script.ts subject exhaustive`/`parity exhaustive` step — blocked
  by the above; stdio's crate doesn't compile yet.

## Files touched
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧪️testkit/benches/protocol.rs`

No other files were modified. No git history commands were run.
