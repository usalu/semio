# WGPU Ordinary Deferred Owner Recovery

## Scope

The native renderer previously moved `AppInteractionState` and `FrameDeferredCursor` directly into the ordinary deferred future. Dropping that future lost the remaining cursor and left the runtime checkout without an owner able to return the interaction state. The maintenance lane already had a recoverable owner registry, but its 8 ms deadline is specific to synchronous maintenance and cannot govern legitimate async action, sync-pump, tutorial, or settle work.

## Implementation

- `FrameExecutionOwnerCell<T>` and `FrameExecutionRegistry<T>` now provide the common exact-owner registry.
- `FrameDeferredExecutionOwner` owns the interaction state, remaining cursor, and current `FrameDeferredWork`.
- The native ordinary deferred envelope and guard publish one empty `ResumeFrameDeferred` wake when a queued or running future is dropped.
- A queued drop leaves the current work in the registry owner; recovery cancels its action receipt once.
- A running drop has already moved the current work into the future; `FrameActionReceiptOwner` cancels its receipt once, and registry recovery sees no current work to settle twice.
- `start_frame_deferred`, presenter-only restoration, and `close_input_step` recover abandoned owners before treating the checkout as ownerless or terminal.
- Recovered remaining actions stay in `FrameDeferredCursor` and retirement closes one action per step.
- Maintenance keeps its independent deadline and authority.
- Table stepper semantic cells now follow the same staged/sealed/acknowledged frame boundary as VFS controls; stale or unpresented geometry is no longer synthesized from an unlaid-out retained tree.

## Neutral law

- Schema: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/📮️wgpu-frame-deferred-ownership/🧬️schema/🔣️.json`
- Fixture: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/📮️wgpu-frame-deferred-ownership/🔣️.json`
- TypeScript oracle: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/📮️wgpu-frame-deferred-ownership/🟦️.ts`
- Native law: `dropped_ordinary_frame_deferred_jobs_recover_the_exact_pair_and_close_remaining_actions_one_per_step`

The two cases cover a drop while queued and a drop after execution takes the current action. Both require one current-receipt settlement, two recoverable remaining actions, three bounded close steps, and zero ready or in-flight reservation leaks.

## Verification

- JSON schema and fixture parse: passed.
- Rust parser over renderer, Scenes, Interpreter, and async-boundary law: passed through `rustfmt --check`; formatting differences remain elsewhere in shared files and therefore the command exits 1.
- Focused TypeScript command attempted:
  `bun nx exec -- bun vitest run 🧪️tests/📮️wgpu-frame-deferred-ownership/🟦️.ts --config 🎯️targets/🧊️wgpu/🧪️tests/🎚️config/🟦️.ts`
  It did not execute tests because the shared Nx project graph currently has the cycle `framework-actor-rs -> framework-replication-rs -> value-derive-rs -> framework-os-kernel -> value-derive-rs`. Retrying with `--nxIgnoreCycles` reported the same cycle and did not produce a test receipt.
- Root native161 executed the new forced-drop law successfully: `dropped_ordinary_frame_deferred_jobs_recover_the_exact_pair_and_close_remaining_actions_one_per_step` passed.
- Root's full WGPU TypeScript run passed 419 tests across 42 files, including the new directory deadline oracle. The earlier isolated command remained an honest non-execution because its Nx graph snapshot had a cycle; the later full run is the authoritative receipt.
- The retained checkout diagnostic now records the exact deferred work category and a fixed safe host-I/O request class for the next live reproduction.
