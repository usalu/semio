# P8l Independent Remodel Source Audit

## Verdict

**RE-AUDIT READY (SOURCE REPAIR)** — the rejection findings below were repaired in source on
2026-08-22. Production no longer exposes the arbitrary whole-mesh facades, shared durable chunk
admission is universally capped at 4 KiB raw, and authored public ActionBus/worker regressions now
cover real starts/continuations plus total process-state loss and typed-row replay. Rust execution
was expressly not run, so this status requests another independent source audit and does not close
the debug, release, Wasm, or runtime-timing gates.

## Original Audit Scope And Method

Read-only inspection on 2026-08-22 of the Phase 8 plan, prior P8 records, the registered Remodel
action/command bridge, reconstruction engine, image/SfM/dense/mesh stages, mutation diff path,
and source tests. No production source, manifests, ticket state, JSON, targets, caches, or Git
state was changed. No Cargo command ran.

Commands run (read-only):

```text
sed -n ... p8e-remodel-resumable-reconstruction.md
find/rg --files and rg -n across Remodel Rust sources
nl -ba .../run-reconstruction/.../component.rs
nl -ba .../engine/{reconstruction,images,sfm,dense,mesh}/.../component.rs
nl -ba .../artifacts/remodel/component.rs
nl -ba .../{create-asset,replace-mesh-result,delete-asset}/diff/component.rs
```

Not run: every Cargo/test/check/Clippy command, including debug, release, and `wasm32-wasip2`.
During the original rejected audit, the previously recorded static verifier was not rerun. The
repair pass reran its non-JSON form as recorded below; it remains catalog classification, not
runtime or timing evidence.

## Rejection Repair Record

- `mint_and_stash_mesh`, `remodel_mesh_workspace`, and the whole-mesh digest helper are now
  `#[cfg(test)]`. Production default, clear, reset, IO, inference, editor, viewer, and result-panel
  callers use stable empty/box constants or `resolve_bounded_remodel_mesh`; the latter accepts only
  fixed constants or committed replayable handles within a 512-vertex/512-triangle field envelope.
  `bounded_remodel_mesh_chunk_count` and `bounded_remodel_mesh_chunk` expose chunk-wise access.
- Shared asset and mesh staging both funnel through the same checked base64 admission and reject raw
  chunks above 4,096 bytes. Index/count conversions and `index + 1` relationships are checked.
  Raster PNG row emission is calculated against the same 4,096-byte framed limit.
- The authored public regression starts independently through `runReconstruction`, `runStage`, and
  `retryStage` via `ArtifactEditor::command_from_action` and `VcsArtifactApp::dispatch_typed`, then
  dispatches only the actual continuation effects. It observes ingest, decode/features, matches,
  SfM, dense, mesh, QC, and `Done`; records typed `OpText`; clears asset blob/content/staging,
  mesh blob/content/staging, session/admission/generation state; replays one typed row at a time
  from genesis; compares all four input images and the exact terminal mesh handle/value; clears all
  maps again; then checkpoint-pack restores and repeats the comparisons.
- A separate public two-document regression covers cancellation, stale continuation rejection, and
  generation/job ABA non-reuse. Admission tests cover exact 4 KiB, oversized, malformed base64, and
  overflowing indices for both asset and mesh staging. The two direct/manual tests cited by the
  rejection were removed; lower-level focused tests remain explicitly test-only.
- `rustfmt --edition 2021` completed on touched Rust sources. The non-JSON static verifier
  `bun ./📜️script.ts verify interactivity tool-jobs` exited 0 with 775/775 bounded production
  rows and zero batch-only, forbidden, or deleted rows. This is static classification only.

## Confirmed Source Shape

- `runReconstruction`, `runStage`, and `retryStage` enter the shared begin path; the latter two
  retain the requested stage ([run-reconstruction:631](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L631), [run-stage:18](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-stage/🦀️component.rs#L18), [retry-stage:18](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️retry-stage/🦀️component.rs#L18)).
- All four start/continuation actions are currently classified `Migrated`, contrary to the older
  P8 classification report ([editor:624](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs#L624)).
- The continuation carries generation, job, requested stage, pipeline/terminal phase, stream and
  frame cursors, terminal cursor, and tick; stale/cancelled documents are dropped before session
  take/work/publication ([run-reconstruction:523](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L523), [run-reconstruction:933](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L933)).
- The engine call is one scheduler unit (`RECONSTRUCTION_STEP_BUDGET = 1`); SfM seed,
  registration, and bundle calls use fuel `1`; mesh uses `new_bounded` and one pipeline step
  ([run-reconstruction:21](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L21), [reconstruction:1108](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️component.rs#L1108), [reconstruction:1406](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🏭️reconstruction/🦀️component.rs#L1406)).
- Production mesh chunk mutation routing is typed and one chunk/event at a time; staged chunks
  avoid document asset-map cloning and `ReplaceMeshResult` compacts the final handle
  ([run-reconstruction:833](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L833), [create-asset diff:40](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧷create-asset/🔺️diff/🦀️component.rs#L40), [replace-mesh-result diff:7](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧱replace-mesh-result/🔺️diff/🦀️component.rs#L7)).
- User cancellation, per-document session selection, and 32-generation admission exist
  ([run-reconstruction:400](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L400), [run-reconstruction:352](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L352)).

## Original Exit Blockers (Repaired In Source)

### P1 — Production Whole-Mesh Facades Remain — Repaired

`mint_and_stash_mesh` is public production code, accepts an arbitrary `MeshData`, serializes the
entire mesh with `serde_json::to_vec`, and inserts it wholesale into the process map. It is not
`#[cfg(test)]` ([artifact:609](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs#L609)). It is used by production reset/default/snapshot paths, and its public type permits
future large callers. `remodel_mesh_workspace` then clones the full committed mesh in production
([artifact:616](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs#L616)). This directly conflicts with the required absence of production whole facades and the
no eager whole-mesh encode/clone/allocation rule. The bounded reconstruction path does not cure a
parallel production escape hatch.

Required repair: remove or test-gate the arbitrary whole-mesh mint/resolve APIs; replace all
production placeholder/default callers with a small bounded durable construction or a typed
constant handle. Preserve only a bounded/chunked resolver for reconstructed mesh content.

### P1 — Restart Claim Is Narrower Than Claimed — Repaired

The cold-restart test manually seeds `MeshPreparation`, manually constructs every stage mutation,
and calls `handle_advance` only after inserting a terminal session directly into the process
registry ([run-reconstruction:1210](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L1210), [run-reconstruction:1273](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L1273)). The map-clear helper deletes only two mesh-map keys, not all process state
([artifact:620](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs#L620)); Remodel has separate asset-cache, asset-content, mesh-content, and session maps.

This validates a useful durable mesh replay subpath, but not cold process restart of the actual
public `runReconstruction` → worker job → continuations → event log sequence. It also cannot
establish that a restarted run can reacquire all inputs: ordinary image access remains backed by
`REMODEL_ASSET_BLOBS` ([artifact:257](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs#L257), [artifact:271](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🦀️component.rs#L271)).

Required repair: add one public ActionBus/worker test that starts via each public start action,
replays the emitted `OpText` rows from genesis after clearing every relevant process map, and
resolves the terminal handle. Keep each raw durable chunk at `<= 4,096` bytes and assert the
limit at the shared asset-stage admission boundary as well as mesh admission.

### P2 — Source Timing Tests Are Authored, Not Executed, And Bypass Part Of The Public Path — Public-Path Source Gap Repaired; Execution Still Unrun

The large-mesh source test times direct `handle_advance` calls and individual JSON/diff/apply
operations, but bypasses ActionBus dispatch, job submission, real start admission, image decode,
feature/match/SfM/dense work, and terminal-session creation ([run-reconstruction:1273](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🚀️run-reconstruction/🦀️component.rs#L1273)). The `64/32/8` SfM timing tests are likewise authored direct continuation tests
([sfm:3110](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📸️sfm/🦀️component.rs#L3110)). They are reasonable static coverage, but do not prove worker scheduling or timing. No debug,
release, or Wasm execution was performed for this audit.

Required gates, all currently **UNRUN**:

```text
cargo test -p semio-s-plugin-remodel --lib
cargo test --release -p semio-s-plugin-remodel --lib
cargo check -p semio-s-plugin-remodel --target wasm32-wasip2
cargo clippy -p semio-s-plugin-remodel --target wasm32-wasip2 -- -D warnings
<master Phase 8 ActionBus/tool-job quick suite>
```

## Additional Audit Notes

- The engine implementation itself has no `async fn` under the engine tree in the current source
  scan. Handler-level async use is tied to typed mutation construction/dispatch; no production
  thread spawn, private worker pool, or blocking executor was found in the scoped runtime path.
- Image, SfM, dense, TSDF, and bounded mesh code contain the advertised cursor/fixed-envelope
  mechanisms. `new_bounded` clamps mesh output to 512 vertices/triangles, texture to 64×64, and
  Taubin to four ([mesh:4051](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodel/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🥽️mesh/🦀️component.rs#L4051)). This is source support, not an executed proof.
- The original audit found an 8,192-byte shared asset admission ceiling. The repair now shares one
  checked decoder between asset and mesh staging, rejects decoded payloads above 4,096 bytes, and
  constrains terminal raster PNG emission to the same framed ceiling.

## Reconciliation Of Prior Findings

The older P8 bounded-reducer report lists Remodel start actions as `BatchOnlyPendingRewrite`.
Current actual registration explicitly marks them `Migrated`; this audit treats the source as the
authority and does not carry that former classification forward. The original P1 source blockers
are repaired and ready for independent re-audit. Runtime gates remain unrun and are not implied by
this source-repair status.
