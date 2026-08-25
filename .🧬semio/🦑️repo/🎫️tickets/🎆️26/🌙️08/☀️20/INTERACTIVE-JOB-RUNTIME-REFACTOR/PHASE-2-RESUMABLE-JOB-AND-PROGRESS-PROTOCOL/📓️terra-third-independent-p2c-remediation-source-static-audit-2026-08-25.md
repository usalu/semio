# Terra Third Independent P2c Remediation Source-Static Audit — 2026-08-25

## Verdict

**RED — not accepted.** The second remediation removes the untyped JSON escape hatch, adds a real typed `Effect::SpawnJob` extraction after native exchange, and retains the accepted-submit retry rule. It still fails the required complete-identity and exact-owner laws: critical authority fields are stored but never validated, and a stale/missing identity is immediately discarded by kernel idle maintenance.

This is additive and preserves both prior RED reports:

- [`📓️terra-independent-p2c-live-fixed-replay-source-static-audit-2026-08-25.md`](./📓️terra-independent-p2c-live-fixed-replay-source-static-audit-2026-08-25.md)
- [`📓️terra-second-independent-p2c-remediation-source-static-audit-2026-08-25.md`](./📓️terra-second-independent-p2c-remediation-source-static-audit-2026-08-25.md)

## Method and Limits

- Independently read the P2c contract, both prior Terra REDs, the updated Sol report, root verifier, ProgramBridge, WGPU client/kernel, plugin ActionBus/application, actor, and shard path on the exact current tree.
- No source was edited. No Cargo, Nx, Wasm, browser, build, or executable/runtime test was run.
- Findings below are from production bodies and caller/field census, not from verifier predicates or test names.

## What the Current Source Fixes

1. **No live JSON replay control remains.** A Rust-source census has zero `mountedJobReplay` occurrences. The remaining occurrences are only verifier deny/mutation strings in `📜️script.ts`.
2. **The native bridge has a causal typed entry.** `ProgramBridge` completes `exchange`, moves one `Effect::SpawnJob` from the resulting mutable `ExchangeOutcome` with `take_product_replay_authority`, decodes the invocation result, mounts the moved owner through `KernelClient`, and requests one advance. There is no arbitrary argument parser or `replay_job_turn` caller in this route.
3. **The lower shared-pool path is now structurally selected.** `KernelPoolState::new` uses `renderer_worker_pool`; the replay request admits logical `1`, `2`, `4`, and process-default profiles; the physical pinned shard slot is deterministically reduced modulo that profile. It does not mint a private/headless pool or retain the former equality-to-singleton rejection.
4. **The retry/ACK transition remains correct in the lower mounted replay entry.** A rejected replay submit retains its sequence/restore packet and returns `Idle`; only `Backpressure::Accept` calls `accept_replay_submission`, after which a later opportunity can be `JobStep`.

These are meaningful improvements, but they do not close the following production failures.

## Blocking Finding 1 — “Full” Product Authority Does Not Validate Most of Its Required Identity

`MountedProductReplayAuthority` stores kind/placement, checkpoint ordinal/digest/pages/progress, worker count/slot, `begin`, and `restore_start_ordinal` (`wgpu/📦️glue.rs:3906-3931`). Its sole qualification function, however, is `validate` at `:3934-3952`. That predicate checks actor/job, operation/generation/seed, request, route, and terminal fields only.

It does **not** check:

- fixed job kind or placement;
- checkpoint ordinal, digest, page count, or applied progress;
- physical pinned slot or logical worker profile;
- `begin` state; or
- accepted restore/start ordinal.

The focused production-field census confirms the gap: `authority.checkpoint` is read only by a test assertion; the production body only overwrites `worker_count`, `worker_slot`, `begin`, and `restore_start_ordinal` at `:5228-5249`. Those values are never compared against retained state before replay. In particular, `request_job_replay` recomputes the slot/profile and then the authority overwrites its old values; it never rejects a mismatching stored authority.

Thus the code **carries** the requested fields but does not qualify them. A wrong/missing checkpoint witness, logical profile, physical slot, or restore/start identity cannot cause the required exact-owner refusal. This directly fails the requested complete-identity contract.

## Blocking Finding 2 — Validation Fault Drops the Exact Owner

`advance_product_replay` briefly handles a validation mismatch correctly: it puts `Err(authority)` back into its fixed authority slot and returns an error (`wgpu/📦️glue.rs:5217-5220`). The actual idle-maintenance caller then defeats that retention:

```text
run_kernel_pool
  state.advance_product_replay(instance).await -> Err
  state.abort_product_replay_one(instance)

abort_product_replay_one
  product_replay_authorities[index] = None
  or product_replay_claims[index] = None
```

The relevant production lines are `:5188-5193` and `:6745-6749`. There is no refusal/recovery lane for `MountedProductReplayRequest` or `MountedProductReplayAuthority`, and no `Drop` implementation analogous to `MountedJobReplay`'s pre-reserved recovery owner. `destroy_app_step` likewise clears a claim/authority directly before the established incremental replay close graph.

Consequently stale, missing-log, invalid-profile/slot, and other `advance_product_replay` errors discard the one typed authority rather than returning it, retaining it for retry, or retiring it through an exact bounded owner path. This falsifies the requested wrong/missing-identity and cancel/deadline/stale/fault/panic/Drop/close acceptance law for the newly added production authority.

The same issue reaches the bridge boundary for mount refusal: `ProgramBridge` receives `Err(authority)` but converts it to `Err(authority.rejection_reason())`, dropping the owner after producing a string. The kernel-only `Result<(), MountedProductReplayRequest>` is therefore not an end-to-end exact-owner return path.

## ACK, Worker Profiles, and Existing Lower Lifecycle

The lower `MountedJobReplay` still source-validates its retry cursor before start and changes `replay_started` only under the accepted `runtime.submit` branch (`wgpu/📦️glue.rs:4611-4629`, `:6185-6240`). Existing actor/shard replay records, recovery registry, and incremental close logic remain present. They cannot repair the loss of the added `MountedProductReplayClaim`/`Authority`, which has become the live gate controlling whether those lower owners are reached.

The 1/2/4/default logic is a real production maintenance route rather than the old mock-only matrix, but this audit cannot accept it as deterministic end-to-end evidence while the authority that advances it neither validates nor survives the profile/checkpoint/restore identities. Runtime verification of all profiles remains deferred.

## Static Gate Record

| Check | Result |
| --- | --- |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test` | PASS — `hostile-mutations=80` |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test` | PASS — `hostile-mutations=13` |
| Scoped `git diff --check HEAD --` across P2c actor/shard/executor/plugin/ProgramBridge/WGPU/verifier files | PASS |
| `rustfmt --edition 2021 --check ProgramBridge/🧊️component.rs` | PASS |
| `rustfmt --edition 2021 --check --config skip_children=true wgpu/📦️glue.rs` | FAIL — inherited tree-wide WGPU formatting drift, including lines 91, 106, 115, 476, 569, 676, 727, 810, 2016, 2424-2443, 3004, 4053, 12812, and 13251; no source was changed by this audit. |

The 80-mutation verifier does not model the fatal caller composition above. Its authority law calls `validate` directly and proves only a wrong seed returns `Err(self)`; it neither mutates/tests checkpoint/slot/profile/begin/restore fields nor runs that error through `run_kernel_pool`'s unconditional `abort_product_replay_one`.

## Required Closure

1. Extend `MountedProductReplayAuthority::validate` (or replace it with a one-owner qualification transition) to compare fixed kind/placement, checkpoint ordinal/digest/pages/progress, process slot, logical profile, begin state, and accepted restore/start ordinal against actual retained log/seed/runtime facts before each profile transition.
2. Make every failed product qualification return or retain an exact typed owner. Remove the unconditional `abort_product_replay_one` loss path; introduce a fixed refusal/recovery/close lane that drains one owner/control/page per opportunity and composes with cancel, deadline, stale, fault, panic, ordinary `Drop`, app destroy, and realm close.
3. Preserve the moved `MountedProductReplayRequest` through public mount refusal rather than converting it to a string-only error at ProgramBridge, or prove a bounded, observable exact handback at that boundary.
4. Add source mutations and production-shaped laws for each currently unchecked field and for the `advance_product_replay`-error → idle-maintenance path.
5. After these source repairs, run the deferred executable matrix: real normal ActionBus session through ProgramBridge/KernelClient/shared renderer pool/pinned shard/GuestRuntime at 1/2/4/default; identity refusal; record/replay determinism; pre-accept backpressure retry; MAX+1; and all cancellation/terminal/close modes.

P2c and Phase 2 remain **RED**.
