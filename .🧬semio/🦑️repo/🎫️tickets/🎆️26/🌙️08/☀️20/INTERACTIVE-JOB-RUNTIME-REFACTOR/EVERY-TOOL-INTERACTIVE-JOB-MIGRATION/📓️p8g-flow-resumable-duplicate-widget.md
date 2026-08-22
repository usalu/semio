# Flow Resumable Duplicate Widget

## Outcome

Flow's `duplicateWidget` action now owns a real resumable collision-search job. The public action captures an owned `FlowWorkingScene` before dispatch, creates a fresh generation for the current document, cancels older sessions for that document, and dispatches the hidden `duplicateWidgetStep` continuation. Each continuation performs at most 64 row comparisons in exactly one of the source, widget-id, or synapse-id search phases before yielding.

The owned graph travels inside the process-owned generation session. Worker continuations never call `flow_working_scene` and therefore never consult the originating thread's `FLOW_SCRATCH`; only the initiating action uses that UI-thread bridge to take the snapshot before scheduling worker work.

The continuation payload carries `generation`, `phase`, `scanIndex`, `suffix`, and `candidateId`. Those fields provide progress and candidate preview while also forming an exact checkpoint: forged, replayed, superseded, or reordered payloads cannot advance a live session. A changed composed-flow `content.child_id` cancels the generation before graph work or mutation emission. Process-owned live state is capped at 32 sessions.

Terminal completion still emits the existing composite `FlowMutation::DuplicateWidget`. Its source id, collision-free widget id, collision-free synapse id, and empty port selectors are unchanged, so the existing composite plan remains the sole owner of widget/value cloning and graph connection semantics. The prior naming sequence is preserved: `<source>-copy`, `<source>-copy-2`, and `<source>-to-<copy>` with numeric suffixes after collisions.

## Proof Added

- A dense 10,000-id fixture proves a probe examines no more than 64 rows.
- A superseded-generation fixture proves an old continuation emits neither preview nor commit.
- A changed-content fixture proves document-generation freshness is enforced before graph work.
- A spawned-worker fixture moves an owned graph session across a real thread boundary and drives the production step core to a terminal `FlowMutation::DuplicateWidget`, without a thread-local cache lookup.
- A terminal fixture pins the existing composite clone/connection payload.
- The command-surface fixture now covers all 37 rows, including two pre-existing omissions (`duplicateWidget` and `setContributions`) and the appended `duplicateWidgetStep` row.
- The opaque-reducer ledger removed only `duplicateWidget` after the source proof.

## Focused Static Validation

`bun ./📜️script.ts verify interactivity tool-jobs --format json` passed:

- command rows: 774
- bounded rows: 774
- batch-only rows: 0
- forbidden rows: 0
- deleted rows: 0
- failures: 0

No Cargo command was run because P4 exclusively owns Cargo validation during this packet.

The two command files were normalized with targeted `rustfmt --edition 2021` after the worker-portability repair.

## Deferred Cargo Gates

- `cargo test -p semio-s-plugin-flow duplicate_widget --lib`
- `cargo check -p semio-s-plugin-flow --target wasm32-wasip2`
- `cargo clippy -p semio-s-plugin-flow --target wasm32-wasip2 -- -D warnings`
- the master ActionBus quick suite and its demonstrator build gate

## P8i Authoritative-Child and Terminal Repair

This section supersedes the pre-audit `FLOW_SCRATCH`, 32-session cap, and terminal-composite description above.

The final source audit findings are repaired in source:

- `ChildContentView` now retains each live child store's authoritative typed snapshot behind `Arc<dyn Any + Send + Sync>` alongside shared pack bytes. Flow's initiating worker calls `typed_arc::<SemioFlowSnapshot>` and only downcasts/clones the `Arc`; it performs no pack decode and no full graph clone in the initiating worker step.
- The process-wide duplicate session owns that immutable `Arc<SemioFlowSnapshot>`. Continuations never read `FLOW_SCRATCH` or rebuild `FlowWorkingScene`.
- Source lookup, widget-id collision search, and edge-id collision search each examine at most 64 rows. The source node is cloned once when its bounded source chunk finds it; retained cursors prepare the new node/edge plan incrementally.
- Terminal completion emits one atomic `ChildEmit` containing the stdio child's own `InsertNode` and `InsertEdge` mutations. It no longer calls the parent `DuplicateWidget` composite, no longer rescans/clones the full graph at terminal, and preserves node params/position plus the prior empty-port connection semantics.
- `FlowConfig::duplicate_widget_progress_json` publishes the exact generation/phase/cursor/suffix/candidate checkpoint. Continuations require the authoritative config checkpoint before taking a session, providing observable progress/preview and latest-wins rejection.
- Focused tests cover a real authoritative child store through the production initial handler on one OS thread and continuation handler on another, exact child-op decoding, 10,000-id bounded probes, changed-content and superseded checkpoints, and a 100,000-edge high-degree graph whose final bounded scan plus child-op encoding remains under 8 ms and emits exactly two compact ops.
- The duplicate composite plan's async precondition and working-scene calls are awaited, while the pure `widget_with_id` helper is synchronous. The framework Flow wasm VCS wrapper now follows the working DAG wrapper shape: async constructor/exports, guarded `try_borrow*`, and awaited `ArtifactStore` calls.

The exact interactivity verifier remains green at 774/774 bounded rows and zero failures. Source-only `rustfmt --check` parsed the touched Rust and reported formatting differences only. Cargo, Clippy, and wasm execution remain deferred to P4.
