# P8r Independent Remodel Final Audit

## Verdict

**REJECT.** The P8q disposition is not closed in the current source. This was a fresh read-only
source/static audit on 2026-08-22. No Cargo/build/test, generated-descriptor, cache/target,
ticket-status/JSON, or modifying Git command ran.

## Scope And Method

- Read the repository `AGENTS.md`, P8e Remodel reconstruction report, P8n independent re-audit,
  and the full P8q audit including its repair disposition.
- Inspected the current Remodel artifact/snapshot, command boundary, run reconstruction state
  machine, durable asset/mesh staging, terminal commit, PNG/JPEG rope decoder/instrumentation,
  public source tests, and read-only scoped working-tree diff.
- Conclusions below are source evidence only; no executable claim is made.

## Rejections

### P0 — Production Decoratively Async Action Parsing Has Direct Future/Value Mismatches

The non-trait `args_bridge` helpers are no-await `async fn`s:
`field` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs:208-210`,
`text`/`number`/`flag`/`vec3` at `:214-242`, and `unknown` at `:244-246`.
They are used as values without `.await`: `field(args, key)?` at `:215,224,232,240`,
the closure bodies at `:250-255`, direct argument reads at `:369-371,387,392-402`, and
`Err(unknown(action))` at `:406`.

Those expressions require `Option<_>`, `Fault`, or methods on their results, but the
declared functions return futures. This is a direct source type mismatch, not a merely stylistic
async concern. The outer `command_from_action` may remain async where the framework trait
requires it; these private pure parsing/building helpers cannot. The P8q requirement that live
no-await builders/callers be synchronous is therefore false.

### P0 — Legacy Production Staging Commits Still Exist Outside `cfg(test)`

The generic, production `CreateAsset` diff retains two legacy promotion paths:

- `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️component.rs:16-20`
  parses a magic commit key, calls `commit_staged_remodel_asset`, and returns an empty diff.
- The same non-test diff parses a content handle and calls that promotion at `:54-57`.
- The generic production `ReplaceMeshResult` diff promotes a staged mesh at
  `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🔺️diff/🦀️component.rs:9-14`.

The invoked helpers mutate private staging by removing an entry:
`commit_staged_remodel_asset_as` at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:463-469` and
`commit_staged_remodel_mesh` at `:751-757`. None of these production paths is
`#[cfg(test)]`. This contradicts the requested absence of hidden legacy commit insertion.
It also reintroduces a fallible/side-effecting path outside the one terminal
`CommitReconstruction` transaction.

By contrast, the intended terminal route does materialize artifacts before calling the
validate-all transaction at
`🏁commit-reconstruction/🔺diff/🦀️component.rs:38-64`, and
`commit_staged_remodel_reconstruction` validates both locked stores before removals at
`🗿artifacts/📸️remodel/🦀️component.rs:772-788`. That does not remove the reachable legacy
paths above.

### P0 — Reconstruction Reassembles The Entire Durable Compressed Input Before Constructing The Rope

`remodel_asset` reconstructs each persisted image by decoding every durable leaf into one
growing `Vec<u8>` and base64-encoding it as one complete `ImageAsset.data` string at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs:280-290`.
The active reconstruction path calls that function at
`🏅standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs:1026-1043`,
then feeds the complete string into the rope in 4-KiB base64 slices at `:1000-1013`.

Thus PNG/JPEG decoder internals do receive a rope, but the active durable-input path has already
joined the whole compressed payload before the rope exists. This fails the stated
“never joins whole compressed input” guarantee and means the decoder-local instrumentation cannot
observe the prior allocation.

### P1 — Rope Metrics Are Real but Their Tests Do Not Detect Duplicate Reads

The rope increments sequential counters in the actual `Read` implementation at
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs:298-321`
and random-byte reads at `:244-257`; PNG and JPEG correctly route through the reader/source at
`:364-369` and `:421-425`. So this repairs P8q's inert counter.

However the PNG regression only asserts `sequential_bytes >= encoded.len()` and one-read size
below input length at `:1288-1311`. Re-reading the entire rope multiple times still passes.
The JPEG regression only asserts `random_byte_reads > 0` and zero sequential bytes at
`:1375-1395`; arbitrarily duplicated random reads still pass. Neither source assertion bounds
total sequential or random accesses against an input-derived maximum, and neither exercises the
active snapshot-to-ingestion path above. The instrumentation exists but the authored tests would
not detect the required sequential/random duplication regressions.

## Static Evidence That Is Present But Insufficient To Accept

- The snapshot owns a serializable `durable_artifacts` store at
  `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️component.rs:31-66`;
  chunk decoding enforces at most 4 KiB at
  `🗿artifacts/📸️remodel/🦀️component.rs:546-553`.
- No current `REMODEL_ASSET_BLOBS` symbol was found. The retained global maps are named
  private asset/mesh staging at `🗿artifacts/📸️remodel/🦀️component.rs:392-396,603-607`;
  the whole-mesh test helper is correctly `#[cfg(test)]` at `:907-924`.
- Asset and mesh staging source does perform decoded-leaf, contiguous-index, aggregate,
  kind/order/field, 32-active, 512 vertex/triangle, and checked-math validation before inserting
  the candidate chunk at `:398-425,609-722`.
- Terminal source emits one mutation per active turn through `yield_terminal` at
  `run-reconstruction/🦀️component.rs:732-748`; terminal completion emits one
  `CommitReconstruction` at `:924-944`. The public test authoring checks one mutation per
  turn at `:1165-1179`, replay/process-clear/checkpoint recovery at `:1256-1329`, and
  two-document cancellation/stale/ABA behavior at `:1332-1361`.
- These tests are unexecuted and are currently blocked from proving behavior by the P0 action
  parser type mismatches. They cannot establish the repaired claims until source is repaired and
  executable gates run.

## Required Unrun Gates

All gates remain **UNRUN** under the disk-space prohibition:

1. Native debug: `cargo test -p semio-s-plugin-remodel --lib`.
2. Native release and timing: `cargo test --release -p semio-s-plugin-remodel --lib`.
3. Wasm compile: `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2`.
4. Wasm lint: `cargo clippy -p semio-s-plugin-remodel --target wasm32-wasip2 -- -D warnings`.
5. Runtime public ActionBus replay/cold-process-clear/checkpoint/two-document/cancel/stale/ABA,
   maximum, malformed, and capacity tests.
6. Runtime PNG/JPEG maximum-envelope, malformed-entropy, bounded-progress, and duplication-read
   instrumentation tests in debug and release.
7. Sanctioned descriptor regeneration/comparison for `cancelReconstruction` and
   `CommitReconstruction`.

The three P0 findings must be repaired before an acceptance decision or meaningful execution of
the authored public replay suite.

## Repair Disposition — 2026-08-22

All P8r rejection findings are repaired in current source:

- **P0 action parsing:** all pure `args_bridge` field/text/number/flag/vec3/unknown helpers and its command builder are synchronous. Only the framework trait boundary remains async, with no stale awaits or direct future/value use.
- **P0 legacy promotions:** `CreateAsset` no longer parses a magic commit key and rejects private reconstruction content handles; standalone production asset commit helpers were removed. `ReplaceMeshResult` rejects `mesh-stage:` handles. The direct mesh commit helper is explicitly `cfg(test)`, while terminal production promotion remains the existing atomic validate-all/apply `CommitReconstruction` path.
- **P0 durable ingestion:** `RemodelAssetChunkSource` creates independently owned decoded `Arc<[u8]>` leaves from snapshot durable state with checked aggregate accounting and a 4-KiB per-leaf ceiling. Active `FrameIngestion` receives those leaves through `CompressedChunkRope::from_leaves`, carrying identity and MIME separately, with no whole compressed `Vec`, base64 `ImageAsset`, private content registry, or `remodel_asset` call in reconstruction.
- **P1 access bounds:** PNG assertions cap sequential calls by input length, total sequential bytes by input length, random header probes by an input-derived maximum, and every sequential read at 4 KiB. JPEG assertions cap total random-byte reads below two input passes and its single access at one byte without brittle exact decoder counts. The snapshot-to-`FrameIngestion` production constructor and real PNG decoder are covered directly.

`rustfmt --edition 2021` completed. The interactivity static verifier exited 0 with 775 bounded production rows out of 775, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch path, and 773 unique rows. Scoped diff, debug, decorative-async, global whole-registry, legacy promotion, active reassembly, and join scans found no remaining P8r production rejection; diagnostic prints and unrelated nested byte vectors are confined to tests or the bounded video codec.

Native debug/release tests and timing suites, runtime ActionBus/replay/cold-restart/cancellation cases, Wasm check/clippy, descriptor regeneration/comparison, Cargo/build operations, and cache/target operations remain explicitly **UNRUN** under the disk-space prohibition. This disposition is source/static evidence and requests an independent final re-audit; it does not claim runtime acceptance.
