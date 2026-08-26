# Process-Global Payload Owner Inventory

## Official boundary

The fresh official run is `📊️sol-fixed-operation-language-neutral-draw-honesty-2026-08-26.json`: 157 failures, 724 remaining command rows, 413 verifier self-tests after the fixed-scheduler language-neutral and Draw persistence hostile laws, and 41 live process-global constructs containing 44 owners. The exact machine-readable owner/file/line/type/kind projection is `🔬️sol-global-payload-owner-inventory-owner-local-1-2026-08-26.txt`.

The original 47-construct census changed only for proved reasons:

- three nested `#[cfg(test)]` hostile registries are now removed by balanced production-item stripping;
- Flow's mutable `ENTROPY_SEED` was deleted and unseeded calls now consume call-local entropy;
- Layout's rejection/session retirement registry moved from TLS into the retained `LayoutSession`/`LayoutExportOperation` `Rc` owner;
- the latest count is therefore 41, with no live mutable payload exempted as immutable.

## Classification

| Kind | Owners | Disposition |
| --- | ---: | --- |
| child-content scratch | 22 | RED; must move under child-content/store roots; no exemption |
| resizable operation registry | 8 | RED; Forms/Draw migration cohort |
| fixed operation registry | 11 | RED until each instance uses the proved typed scheduler authority and has bounded terminal-empty close |
| ABI bridge retention | 2 | RED; Sequence bridge owner migration |
| mutable payload | 1 | RED; Remodel reconstruction owner migration |
| immutable static | 0 | no exemption exists |

## Exact owners

### Resizable operation registries

- Forms: `TRY_VALUE_SESSIONS`, `ACTIVE_TRY_VALUE_GENERATIONS`, `BULK_SESSIONS`, and `ACTIVE_BULK_GENERATIONS`.
- Draw: `DRAW_SESSIONS`, `ACTIVE_DRAW_SESSIONS`, `TRACE_POINTER_JOBS`, and `ACTIVE_TRACE_POINTER_JOBS`.

### Fixed operation registries awaiting instance proof

- `DRAW_MUTATION_ARENA_POOL`
- `FEM3D_BACKING_RECOVERY`
- `FORMS_INPUT_REGISTRY`
- Procedural2d, Procedural3d, and Process3d `LEASES`
- FEM2d and FEM3d `MOUNTED`
- Energy `RECOVERY` and `REGISTRY`
- Puzzle3d `REGISTRY`

### ABI and mutable operation payloads

- Sequence `BRIDGE` and `RETAINED`
- Remodel `RECONSTRUCTION_SESSIONS`

### Child-content scratch owners

- `DAG_SCRATCH`
- `DIN18599_CLIMATE_SCRATCH`
- `EN1990_QK_SCRATCH`
- `FLOW_SCRATCH`
- `FORMS_SCRATCH`
- `IMPERATIVE_FLOW_SCRATCH`
- `IMPERATIVE_SEED_SCRATCH`
- `JACK_SCRATCH`
- `LOWPOLY_SCRATCH`
- `MATH_SCRATCH`
- `PLAYBOOK_SCRATCH`
- `PROGRAM_BENCHMARKS_SCRATCH`
- `PROGRAM_KNOWLEDGE_SCRATCH`
- `RASTER_SCRATCH`
- `REMODEL_PRIVATE_ASSET_STAGING`
- `REMODEL_PRIVATE_MESH_STAGING`
- `SEQUENCE_SCRATCH`
- `SHOOTING_EMBLEM_SCRATCH`
- `TRY_VALUES_BATCHES`
- `TRY_VALUE_BLOBS`
- `WIRES_SCRATCH`
- `WRITER_SCRATCH`

Every exact Rust type and source line is recorded in the TSV and JSON evidence named above; duplicate names such as `LEASES`, `MOUNTED`, and `REGISTRY` remain disambiguated by full file path and line.

## Fixed scheduler foundation

`semio_framework_job::FixedOperationRegistry<T, CAPACITY>` is a typed scheduler authority rather than an arbitrary payload map. It admits exact `(OperationId, Generation)` keys into a fixed allocation and byte credit, returns the same typed owner on rejection, refuses collisions/saturation, prevents stale/ABA take, cancels and begins close without detaching, advances one slot and one owner close unit per call, and asserts terminal-empty on registry Drop. Its hostile Rust laws cover maximum/plus-one, slot saturation, exact rejected handback, stale generation, interrupted close, ABA, repeated close, and accepted owner handback. The root verifier has 18 structural hostile mutations for these anchors. No process-global owner is cleared merely because this type exists; each live owner must move into an instance-retained scheduler using the type and its close contract.

The scheduler now additionally executes the schema-first eight-case law stream in production Rust from a verifier-generated fixture, matches a ticket-only third-party ArrayVec oracle byte-for-byte, and uses a four-worker median construction timing law. The rejected Draw Config-checkpoint shortcut is not credited: both Draw globals remain in the 41-construct census and a dedicated root failure prevents any persisted Config/Draft/document gesture-session replacement.


## Draw Instance-Retained Gesture Owner

Draw's former `DRAW_SESSIONS` pair moved into one concrete `DrawInstanceOperationOwner` constructed by each `VcsArtifactApp`. A cloneable, object-safe instance capability crosses only into the exact registered Draw jobs and renderer; it is not exported as an application payload store. This is not yet accepted as the complete Draw operation owner because the older trace-pointer subsystem remains process-global and the worker still wraps monolithic decode/reducer paths.

- `FixedOperationRegistry<DrawGestureOperationOwner, 64>` owns exact operation/generation keys, byte credit, the live `DrawSession`, cancellation, bounded close, and terminal-empty authority.
- Six Draw gesture actions use one exact `DrawGestureOperationJobFactory`. Retained fixed pages cross admission unchanged, are copied/scanned one bounded unit per worker turn, and decode only inside the mounted job before the reducer receives the same typed command.
- Active owner reuse is accepted only for the same exact key and canonical base revision. A new key/generation or revision cancels the displaced owner. Renderer publication reads the same app-instance handle, validates the current canonical revision, cancels stale preview state, and renders the live gesture snapshot.
- Config, Draft, Snapshot, document schema, and persisted checkpoint reconstruction remain forbidden. The removed `gesture_checkpoint_json`, `SetGestureCheckpoint`, `checkpoint_from_config`, `DRAW_SESSIONS`, and `ACTIVE_DRAW_SESSIONS` census is zero.
- Rust laws cover exact capacity maximum/+1 handback, stale generation/ABA, interrupted and repeated close, and stale-preview cancellation. The renderer now consumes a fixed-capacity `DrawGesturePreview` projection capped at 256 points rather than cloning the FSM/session working state.
- The gate additionally rejects whole `serde_json::from_slice`, generic `command.dispatch`, whole session cloning, and the `TRACE_POINTER_JOBS`/`ACTIVE_TRACE_POINTER_JOBS` process-global system. Those genuine source violations remain, so the Draw blocker is deliberately RED.

Evidence:

- `📊️sol-draw-semantic-honesty-final-2026-08-26.json`: `failureCount=157`, `remaining=724`, `selfTests=419`, raw global candidates 39, and the single Draw semantic blocker remains present.
- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: 419 clean, including whole-parse, generic-dispatch, preview-clone, trace-global, wrong-owner, stale-generation, preview-consumer, and persisted-lane hostile mutations.
- `🧪️sol-draw-instance-owner-rustfmt-check-2026-08-26.txt`: all touched Rust parses; it records shared formatting differences, so no broad formatting rewrite was applied.
- Focused Draw Cargo validation is recorded separately below because concurrent workspace checks held the shared Cargo lock during the initial run.
