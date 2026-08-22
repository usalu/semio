# P8za Global Payload Authority Corrective Repair

## Verdict target

Source/static audit-ready for the P8z P0/P1 findings. This is not a
compile/runtime-pass claim; Cargo, build, native/Wasm runtime and test execution
remain explicitly unrun.

## P0-01 — CAD preview freshness

- `CadConfig` now persists
  `engagement_preview_operation_json` and checked increment-only
  `engagement_preview_generation`.
- Production dispatch constructs `CadPreviewOperationIdentity` from the real
  `ArtifactView::operation()`: app instance, parent document, operation id,
  operation generation and canonical base revision.
- Every changed engagement checkpoint advances generation with
  `checked_add(1)` and persists that exact operation stamp. Overflow and a
  missing public-operation identity fail with conflict/invalid faults.
- `gesture_preview` returns an exact typed `CadPreviewStamp`; freshness
  requires identical operation identity and a greater generation. No hash is
  used.
- Source fixtures cover equal repeated reads, exact +1 ordering, A → B → A,
  forced equal-counter collision across two app identities, serialized config
  reopen, abort cleanup and two-app rejection.
- Rust, TypeScript, GraphQL, Proto and JSON config descriptors carry the two new
  fields; changed JSON descriptors parse.

## P0-02 — Puzzle3d isolated fill worker

- `FillWorkerState` replaces the three-integer worker input. It owns the
  `fill_job`, scene, bounded raw mesh sources, full `FillBuilder` checkpoint,
  fill-step cursor, fill revision/generation/preview sequence, observation and
  last emitted checkpoint.
- Admission serializes the complete state after binding the exact job,
  operation and generation. Checkpoints emitted after every incomplete bounded
  worker slice serialize the same complete state.
- Cold restore reconstructs collision meshes, scene JSON, `engine.fill`,
  operation/generation and every fill cursor before the first
  `drive_fill_job` call. Decode, mesh envelope, base checkpoint and
  operation/generation mismatches fail closed with distinct decode/restore/stale
  faults.
- Envelopes cap the complete checkpoint at 4 MiB, URLs at 4 KiB, meshes at 64,
  each positions/indices array at 196,608 values and checked aggregate mesh
  values at 393,216.
- Source fixtures restore a worker from only serialized admission bytes and
  prove its first tick does not stale-fault; a second fixture uses two distinct
  scenes/operations and rejects an ABA-swapped job/checkpoint.

## P0-03 — Sourcing pre-deserialization envelope

- The former `serde_json::Value` allocation and post-parse walk are removed.
- A byte/token scan runs before `serde_json::from_str::<CurateSnapshot>` and
  caps 256 KiB raw input, depth 32, 4 KiB string/key leaves and 4,096 aggregate
  containers/keys/scalars with checked counters.
- Source fixtures cover exact max and +1 for raw bytes, depth, string bytes and
  cardinality.

## Blocking-risk contribution repair

- Process and Sourcing scan the complete contribution JSON envelope before
  `parse_contributions`/topic decode and scan nested machines/typology/kinds
  JSON before their typed deserializers.
- Process applies the same 256 KiB/32-depth/4 KiB-string/4,096-item envelope;
  Sourcing uses the shared import envelope. Decoded vectors receive a final
  cardinality check.
- `ContributedMachineCatalog` and `ContributedSourcingModule` now own
  identifiers/labels/icons as `String`.
- `MachineCatalog` and `SourcingModule` return self-borrowed `&str`, so
  built-ins retain literal zero-allocation access while contributed values have
  bounded ordinary lifetimes.
- Exact scans over both plugin trees find no `Box::leak`,
  `into_boxed_str` or `leak_str`.
- Process source fixtures cover exact max/+1 contribution envelopes; Sourcing
  fixtures prove over-depth/string/cardinality envelopes install no module.

## P1 comments

Note and Layout comments now describe their snapshot-owned child records and no
longer promise scratch-cache lifecycle behavior.

## Static gates

- `rustfmt --edition 2021`: exit 0 for the corrective Rust files.
- Exact removed-registry/runtime-session scan across the seven plugin trees:
  clean.
- Exact rejection-pattern scans: no CAD preview `DefaultHasher`, impossible
  `seq_after_*` test, old incomplete Puzzle restore path, Sourcing
  `serde_json::Value`/post-parse validator, config-derived `Box::leak`, or
  stale Note/Layout scratch comment.
- All changed JSON descriptors in CAD/Process/Sourcing/Puzzle parse with
  `JSON.parse`.
- `git diff --check` across the seven plugin trees: exit 0.
- Added-line debug scan: clean.
- `bun ./📜️script.ts verify interactivity`: exit 0, deny-mode clean over its
  declared four UI roots.
- `bun ./📜️script.ts verify interactivity tool-jobs`: exit 1 with the
  repository-wide known residual ledger: 34 process-global candidates outside
  this repaired cohort, 12 framework-reserved routes pending factories and 875
  live registrations pending disposition. Summary at this scan: 9 admitted
  rows, 6 factories, 1 registration, 1 dispatch, 4 aliases and 10 self-tests.

## Unrun gates

Per instruction, no Cargo command, compilation/type/borrow/Send gate, build,
unit/integration test, actual worker launch, native/Wasm execution, generated
descriptor regeneration, cache deletion, git mutation or ticket metadata
mutation ran. The new fixtures therefore remain source evidence until the
independent native/release/Wasm and isolated-worker gates are authorized.

## P8zb final corrective pass

The final independent audit found two remaining source-contract failures. Both
are repaired in source:

- CAD preview generations now use one explicit nonnegative signed-32 domain,
  `0..=2_147_483_647`, across runtime Rust, persisted Rust config, generated
  Rust schema, Proto `int32`, GraphQL `Int`, JSON Schema integer min/max and a
  range-documented TypeScript `number`. JSON-backed config ingestion rejects
  negative or out-of-`i32` values, every changed-checkpoint increment uses
  `checked_add`, and exhaustion at the exact maximum returns the typed conflict
  without persisting a wrapped generation. Preview reads also fail closed for
  an invalid in-memory negative value.
- A source fixture round-trips the exact maximum through `CadConfig` JSON,
  rejects maximum + 1 before config ingestion, rejects an attempted increment
  at maximum, and pins the type/range declaration in every descriptor leaf.
- Note's durable `NoteTextChild.paragraphs` accessor, duplication mutation and
  fixtures no longer describe a working-scene cache, cache miss or staleness
  gap. The dead `WorkingScene` region name is replaced by `TextChildren`.
- Layout IO import/export comments, test names, locals and assertion text now
  describe `LayoutDrawingChild.content` as snapshot-owned durable content.

Final static rerun:

- `rustfmt --edition 2021` and `rustfmt --edition 2021 --check`: exit 0 on all
  seven final-corrective Rust files.
- CAD JSON config descriptor: parsed with Bun; generation is integer with
  minimum `0` and maximum `2147483647`.
- Exact CAD coherence scan: one `i32` field in runtime/config/generated Rust,
  Proto `int32`, GraphQL `Int`, TypeScript `number` with the same documented
  range, JSON min/max; no preview-generation `u64`/Proto `uint64` remains.
- Exact Note/Layout working-scene, scratch-cache, cache-miss, uncached and dead
  region/test-name scan: clean.
- `bun ./📜️script.ts verify interactivity`: exit 0, deny-mode clean.
- `bun ./📜️script.ts verify interactivity tool-jobs --format json`:
  expected exit 1 on the independent repository ledger: 34 process-global
  candidates, 12 framework-reserved routes and 875 live registrations. The
  report recorded 9 bounded rows, 6 factories, 1 registration, 1 dispatch, 4
  aliases and 12 self-tests.
- `git diff --check` over the seven repaired plugin cohorts: exit 0.
- Corrected PCRE2 added-line debug-output scan: clean.

No Cargo/build/runtime gate or ticket metadata operation ran in this final
corrective pass. Full details are in
`📓️p8zc-global-payload-descriptor-coherence-repair.md`.
