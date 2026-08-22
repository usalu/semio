# P8u Independent Remodel JPEG Final Audit

## Verdict

**PASS — source/static acceptance.** This is an independent read-only audit on 2026-08-22 of
the current P8s repair. It accepts the requested JPEG metric and obsolete-dedup removal at source
level, and confirms the P8r closures remain present. It does **not** claim that any test or build
passed.

## Method And Boundaries

- Read the repository `AGENTS.md`; the complete P8e and P8r reports; and the complete P8s report,
  including each repair disposition.
- Inspected current Remodel image/rope, run-reconstruction, artifact/staging, snapshot, mutation,
  ActionBridge, and owned JPG decoder sources; the relevant authored tests; and the read-only scoped
  working-tree diff.
- Ran no Cargo command, build, test, formatter, descriptor generation, cache/target operation,
  ticket metadata operation, or modifying Git command. `git diff --check` was read-only and emitted
  no whitespace diagnostics for the scoped Remodel/JPG diff.

## JPEG Evidence — Accepted

`jpeg_access_evidence_accepts` in
`✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🖼️images/🦀️component.rs:874-876`
requires all of: nonzero random-byte reads, `largest_random_read == 1`, zero sequential bytes, and
`random_byte_reads < input_len.checked_mul(2)`. Therefore it is strictly below a checked second
full-pass boundary; zero input has a zero ceiling and cannot satisfy the nonzero/strict predicate,
and multiplication overflow returns `None` and rejects.

The negative fixture at `:1430-1441` statically covers zero input, the one-byte accept/reject
boundary, `usize::MAX` multiplication overflow, exactly `2 * input_len` simulated accesses, and a
multi-byte unit. The simulated second complete pass is passed through the same predicate and must
fail; it is not merely a separate arithmetic assertion.

The metric is on the real source boundary: `CompressedChunkRope::byte` increments the cumulative
atomic at `:256-269`, and the rope is the `JpgByteSource` implementation at `:289-297`. The
production JPEG state invokes `decode_jpg_source(&rope)` at `:436-440`. That decoder obtains source
bytes only through `JpgByteSource::byte` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/✳️any/🚪️io/🦀️component.rs:795-832,843-1076`), including its entropy `BitReader` (`:261-340`). No metric reset or alternate JPEG byte
path was found between admission/SOF0 probing and decode. Its maximum unit is structurally one byte;
the test-only metric exposes that as exactly `1` whenever an access occurred (`images:277-285`).

The positive 64×64 encoded checkerboard fixture (`images:1404-1428`) rejects restart markers,
uses the real rope/decoder path, then enforces both the generic strict predicate and a tighter
fixture ceiling. `jpeg_fixture_access_ceiling` (`:878-883`) is `input_len + (2 *
(SOF0-marker-position + 1) + 4)`, all checked. This is defensible from the owned decoder's
monotone no-restart byte walk (header ranges and entropy reader advance forward) plus the bounded
SOF0 probe, whose worst case is two byte reads per scanned position and four dimensional reads
(`images:407-426`). The fixture separately proves this ceiling is strictly below `2 * input_len`
before comparing the cumulative recorded accesses. Runtime execution remains required to prove the
fixture's observed count satisfies that authored bound.

## Dedup Removal And Durable Mesh Evidence — Accepted

Scoped production searches found no remaining Remodel dedup verification parser/key/effect/helper,
process `content_exists` function, verification cursor, `verified_against` state, or committed
staging state. The only staging validation retained is the terminal transaction, not dedup: it
validates all private bounded asset/mesh entries under both locks and only then removes them
(`🗿️artifacts/📸️remodel/🦀️component.rs:462-472,682-709`). `CreateAsset` stages only bounded
private chunks or persists a normal asset directly to the snapshot; it has no verification-key
branch (`🧷create-asset/🔺️diff/🦀️component.rs:15-47`). `DeleteAsset` only discards private staging
for staging keys (`🗞️delete-asset/🔺️diff/🦀️component.rs:9-36`).

Authoritative leaves are document-owned: `RemodelSnapshot.durable_artifacts` is persisted at
`🧬️schema/📸️snapshot/🦀️component.rs:31-65`; terminal `CommitReconstruction` materializes staged
asset/mesh leaves into that store before the single locked validate-all/apply cleanup transaction
(`🏁commit-reconstruction/🔺️diff/🦀️component.rs:36-64`). Mesh resolution takes an explicit
`RemodelDurableArtifactStore` rather than a process committed registry
(`🗿️artifacts/📸️remodel/🦀️component.rs:771-813`). The cross-thread authored mesh fixture creates
that durable store, inserts the bounded materialization, drops private staging, and resolves from
the durable store (`🚀️run-reconstruction/🦀️component.rs:1418-1441`). Arbitrary mesh minting remains
`#[cfg(test)]` (`🗿️artifacts/📸️remodel/🦀️component.rs:816-829`).

## P8r Closures Still Preserved

- The pure ActionBridge helpers and private command builder are synchronous
  (`✏️editor/🦀️component.rs:205-270`); only the framework-required trait method remains async and
  directly returns the bridge value (`:520-529`).
- Active reconstruction reads `remodel_asset_chunk_source`, creates a rope directly from the
  independently decoded durable leaves, and retains identity/MIME separately
  (`🚀️run-reconstruction/🦀️component.rs:271-287`). The whole-value `remodel_asset` façade is not
  used by this production path.
- Every normal terminal turn constructs exactly one mutation through `yield_terminal`
  (`:709-725`); final completion emits exactly one `CommitReconstruction` mutation (`:879-895`).
  The public ActionBus test statically asserts one durable mutation for every active turn
  (`:1203-1230`).
- Generic `ReplaceMeshResult` rejects private `mesh-stage:` handles
  (`🧱replace-mesh-result/🔺️diff/🦀️component.rs:7-18`); `CreateAsset` rejects private
  `remodel-content:` handles outside `CommitReconstruction`
  (`🧷create-asset/🔺️diff/🦀️component.rs:36-47`).

## Unrun Gates

All gates below are **UNRUN**:

1. `cargo test -p semio-s-plugin-remodel --lib`.
2. `cargo test --release -p semio-s-plugin-remodel --lib`, including all `<8 ms` timing and JPEG
   access assertions.
3. `cargo check -p semio-s-plugin-remodel --target wasm32-wasip2`.
4. `cargo clippy -p semio-s-plugin-remodel --target wasm32-wasip2 -- -D warnings`.
5. Runtime ActionBus replay, process-clear/checkpoint restore, two-document, cancellation,
   stale/ABA, capacity, malformed, and exact-cap paths.
6. Runtime PNG/JPEG maximum-envelope, malformed-entropy, bounded-progress, and duplicate-read
   instrumentation in native debug and release.
7. Sanctioned descriptor regeneration/comparison for `cancelReconstruction` and
   `CommitReconstruction`.

The result is a source/static PASS only; these gates remain necessary for executable acceptance.
