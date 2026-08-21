# Packet P2b: Actor Job Bridge and Owned Publication Wire

**Date:** 2026-08-21

## Outcome

The actor layer now exposes a clean, breaking resumable-job protocol built directly on
`semio-framework-job`:

- `JobOperation` carries operation id, base document revision, generation, preview sequence, and
  deterministic seed.
- `JobCheckpoint` carries both opaque state and authoritative `applied_progress`.
- `JobStepOutcome` losslessly mirrors every universal `StepOutcome`: yield, preview, checkpoint,
  complete candidate, cancelled, and fault.
- `JobTurn`, `JobPublication`, and `JobReplayLog` provide explicit step identity and deterministic
  publication order.
- `JobTurnBridge::step` invokes exactly one `InteractiveJob::step` through `drive_step`, rejects
  mismatched operation/revision/generation/seed/preview/step identity before work, rejects invalid
  preview advancement, and makes terminal publication final.
- `Payload::Suspend`, `Payload::Resume`, and `Payload::JobStep` use the new records directly. There is
  no compatibility variant or legacy decoder.
- Actor `TurnStatus` now carries real checkpoint state/progress and losslessly represents preview,
  commit candidate, cancellation, and fault states.

All actor job records have owned pack encoders/decoders. The actor typegen export includes every new
record and regenerated the ignored local TypeScript mirror successfully.

## Plugin-host shard bridge

The shard bridge now requires an explicit `JobTurn` before a guest job can step. It maintains the
active identity/cursor per `(actor, job)`, rejects stale or non-deterministic turns, and schedules at
most one bounded job step per actor per pump. Guest-runtime results map as follows:

| Guest result | Actor publication |
|---|---|
| `Running { progress: None }` | `Yield` |
| `Running { progress: Some(bytes) }` | ordered `PreviewReady` |
| `Done { output }` | `Complete` with runtime checkpoint state and output |
| `Failed { error }` | `Fault` |
| actor cancellation | terminal `Cancelled` / cancelled shard outcome |

Suspend checkpoints publish the runtime's actual state bytes with the incoming operation identity
and applied-progress boundary; resume restores those exact bytes. Terminal job state removes the
active turn cursor.

`ShardOutcome` was converted from serde JSON to an owned tagged pack codec, including turn, job,
fault, checkpoint, resume, and cancellation variants. The executor and process-transport consumer
decode that codec directly. The pre-existing kernel `Event` input and opaque turn UI/effect bridge
still use their existing JSON representation inside opaque byte fields; no actor/job identity,
checkpoint, outcome, or shard-outcome record uses serde on its wire.

## Conformance coverage

Actor-focused tests prove:

- exactly one job implementation call per actor turn;
- checkpoint state and `applied_progress` propagate unchanged;
- cancellation skips job work and is terminal;
- stale revision/generation is rejected before work or publication;
- replayed preview identity is rejected before work;
- previews must advance the sequence exactly once;
- identical identity and outcomes produce byte-identical replay logs;
- every new job record round-trips through its owned pack codec.

Shard-focused tests were migrated to explicit turns and owned outcome decoding. They cover explicit
multi-step progression, checkpoint/resume byte identity, cancellation, one-step-per-actor placement,
preview order, and all six `ShardOutcome` pack variants. Their mounted compile/test is pending the
upstream blocker below.

## Files changed

- `🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🔨️modules/🎭️actor/🦀️component.rs`
- `🧰️framework/🔨️modules/🧵️job/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs`
- this report

The framework kernel/WIT/reactor/root-host checkpoint chain and renderer constructors were updated
by the coordinated Phase 3 owner; they are not attributed to this packet.

## Verification run

| Command | Result |
|---|---|
| `bun nx run @semio-tech/framework-actor-rs:test-quick` | 88/88 passed, debug |
| `bun nx run @semio-tech/framework-actor-rs:test-long -- --release` | 88/88 passed, release |
| `bun nx run @semio-tech/framework-job-rs:test-quick` | 16/16 passed, debug |
| `bun nx run @semio-tech/framework-job-rs:test-long -- --release` | 16/16 passed, release |
| `bun nx run @semio-tech/framework-actor-rs:typegen` | export test 1/1 passed; mirror refreshed |
| `cargo clippy -p semio-framework-job --all-targets -- -D warnings` | clean |
| `cargo clippy -p semio-framework-actor --all-targets -- -D warnings` | clean |
| `cargo check -p semio-framework-job --target wasm32-unknown-unknown` | clean |
| `cargo check -p semio-framework-job --target wasm32-wasip2` | clean |
| `bun ./📜️script.ts verify dependencies` | 238 baseline, 238 current; clean |
| `cargo fmt -p semio-framework-actor -p semio-framework-job -p semio-framework-plugin-host` | completed |

Direct cargo was used only for clippy and the two target checks because those checks have no Nx
target, matching P2a's established verification surface. Plugin-host has no Nx project target; its
native mounted check used direct cargo for the same reason.

## Exact remaining blockers

### Actor wasm glue

Both of these were attempted and failed before reaching this packet's actor component:

- `cargo check -p semio-framework-actor --target wasm32-unknown-unknown --message-format=short`
- `cargo check -p semio-framework-actor --target wasm32-wasip2 --message-format=short`

The failures are in the pre-existing
`🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust/📦️glue.rs`: missing `pack::read_opt`, async actor
methods used without awaiting (`Kernel::new`, decode/encode, activate, submit, tick, complete,
metrics), and missing `wasm_bindgen_futures`. This packet did not edit glue because its repair is a
separate async/reachability ownership surface.

### Plugin-host mounted native/test/clippy gate

`cargo check -p semio-framework-plugin-host --all-targets --message-format=short` was attempted. The
latest retry stopped in the unrelated mesh-engine dependency before compiling plugin-host:

- `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs:844:38` — `String` is not a future.
- `🧰️framework/🔨️modules/🔺️mesh-engine/📦️packages/🦀️rust/📦️glue.rs:895:30` —
  `Result<MeshData, String>` is not a future.

An earlier attempt stopped in concurrent config diff code with E0728; that blocker moved during the
shared-worktree run. Because the latest upstream dependency failure prevents plugin-host itself from
being compiled, no plugin-host test, release, clippy, or wasm success is claimed. The shard changes
need one focused retry after the Phase 1/1.5 async/glue repairs stabilize.

## Handoff

No ticket lifecycle or git-modifying command was used. No new scripts were created. Concurrent
shared-worktree edits were preserved. The next owner should rerun the plugin-host all-target native
check first, fix only errors originating in the three shard files above, then run its focused shard
tests/release/clippy and supported target checks.
