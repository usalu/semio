# Terra Fourth Independent P2c Remediation Source-Static Audit — 2026-08-25

## Verdict

**RED — not accepted.** The fourth remediation materially closes the three preceding reports' live-authority qualification and retirement defects.  The final product ingress is still not atomic at `MAX+1` capacity: if its pre-reserved recovery slot cannot be allocated (or the fixed kind admission fails), the production bridge restores a raw dynamic `Effect::SpawnJob` to `ExchangeOutcome`, returns a string error with `?`, and immediately drops that outcome.  Its `String`/`Vec<u8>` owner consequently has neither a fixed refusal/recovery owner nor an observable handback.

This is a production-body counterexample, not a verifier-string inference.  It preserves all earlier reports:

- [`📓️terra-independent-p2c-live-fixed-replay-source-static-audit-2026-08-25.md`](./📓️terra-independent-p2c-live-fixed-replay-source-static-audit-2026-08-25.md)
- [`📓️terra-second-independent-p2c-remediation-source-static-audit-2026-08-25.md`](./📓️terra-second-independent-p2c-remediation-source-static-audit-2026-08-25.md)
- [`📓️terra-third-independent-p2c-remediation-source-static-audit-2026-08-25.md`](./📓️terra-third-independent-p2c-remediation-source-static-audit-2026-08-25.md)

## Scope and Limits

- Independently read the master plan, P2c contract, all three prior Terra REDs, updated Sol remediation report, accepted P2a1/P2d material, actor, shard, executor, plugin ActionBus/application, ProgramBridge, WGPU glue, and verifier on the exact current tree.
- Inspected the production caller and owner bodies directly.  No source was changed.
- No Cargo, Nx, Wasm, browser, build, or runtime/executable matrix was run.  The `bun` self-tests below are source/static mutation gates only.

## What Is Now Sound at the Source-Static Boundary

1. **The normal native caller is real and typed.** Both `ProgramBridge` action and command handlers complete `exchange`, then move one `Effect::SpawnJob` from that actual exchange outcome, decode the normal invocation result, mount the moved request, and request exactly one advance (`ProgramBridge/🧊️component.rs:183-198`, `:201-216`). A Rust-source census has no `mountedJobReplay` occurrence and no JSON replay-control parser.
2. **The admitted authority now validates every minted identity before replay work.** `MountedProductReplayAuthority::validate` compares fixed kind identity and request, placement, actor/job, operation/generation/seed, route, full checkpoint witness, full terminal header/prefix, physical process count/slot, logical profile count/slot, `begin`, and restore/start ordinal (`wgpu/📦️glue.rs:4185-4225`). The expected values come from the retained pinned replay/log state, not a test-only mirror. The qualified logical profiles are `1`, `2`, `4`, and process default, with deterministic process-slot modulo mapping, through the existing shared renderer pool.
3. **The pre-reserved authority lifecycle is substantially improved.** Request → claim → authority moves the one `MountedProductReplayRecoveryToken { index, generation }`; recovery publication verifies both before holding an exact owner (`wgpu/📦️glue.rs:3938-4028`, `:4076-4256`). Validation failure is reinserted then retired; mount refusal, abort, completion, app close, realm close, and ordinary populated `Drop` publish/retire an exact fixed owner one at a time. No product claim/authority body performs the former direct `= None` loss.
4. **The lower replay ACK rule remains accept-only.** Retained replay begin/restore packet and sequence remain unchanged on `Idle` or rejected submit; only `Backpressure::Accept` commits start/sequence, so a later identical/continuation retry cannot legally reach `JobStep` first. The actor/shard route retains the existing fuel-before-copy/ACK and incremental close structure.

Those repairs remove the prior identity and post-admission loss blockers. They do not make ingress capacity safe.

## Blocking Finding — Product Admission Drops the Exact `MAX+1` Owner

`MountedProductReplayRequest::try_from_effect` destructures a normal dynamic `Effect::SpawnJob { job, kind, input, placement }`. It returns that raw effect unchanged in two production failure cases:

```text
kind.len() > PRODUCT_REPLAY_KIND_BYTES
    -> Err(Effect::SpawnJob { job, kind, input, placement })

recovery_registry.reserve() == None
    -> Err(Effect::SpawnJob { job, kind, input, placement })
```

This is `wgpu/📦️glue.rs:4057-4064`. The latter is exactly the product ingress `MAX+1` condition: all `JOB_PROGRESS_ACTIVE_CAPACITY` recovery reservations are occupied before a new `SpawnJob` can transfer into its fixed request shell.

`ExchangeOutcome::take_product_replay_authority` removes the effect, calls that conversion, and reinserts the raw effect on `Err`, then returns only `Err("mounted product replay job kind exceeds its fixed authority")` (`wgpu/📦️glue.rs:4311-4319`). It does not distinguish oversized kind from a full recovery registry and does not construct a fixed typed refusal owner.

Each public native bridge calls this method through `?` before it decodes the invocation frames or obtains any recovery/retirement handle (`ProgramBridge/🧊️component.rs:187-189`, `:204-206`). Consequently the `Err` drops the whole local `ExchangeOutcome`. `ExchangeOutcome` holds `effects: Vec<Effect>` and has no `Drop` implementation or recovery publication path. The reinserted raw `SpawnJob` therefore falls out of scope with its dynamic `kind: String` and `input: Vec<u8>`:

```text
normal ActionBus/plugin exchange
  -> ExchangeOutcome.effects[spawn]
  -> reserve() == None (or kind too large)
  -> raw effect reinserted
  -> ProgramBridge ? returns string error
  -> ExchangeOutcome is dropped
  -> no exact product refusal/recovery owner exists
```

The original shard may separately retain its captured job source, but that cannot repair the moved *product ingress owner* which the bridge has explicitly selected for admission. This branch violates the plan's “user commands never silently dropped” rule and the P2c requirements for bounded `MAX+1`, pre-reserved recovery before transfer, no unadmitted dynamic authority, and an exact non-`Clone` owner on refusal. It also prevents acceptance of the claimed one-owner close graph: there is no owner to close or rediscover.

The 108-mutation static gate does not falsify this composition. Its recovery checks exercise the successful pre-reserved request/claim/authority paths, but no product-shaped law first saturates `MountedProductReplayRecoveryRegistry`, sends the `N+1` normal bridge effect, and proves a fixed exact handback instead of an early `ExchangeOutcome` drop.

## Static Gate Record

| Check | Result |
| --- | --- |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test` | PASS — `live-source clean; hostile-mutations=108` |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test` | PASS — `live-source clean; hostile-mutations=13` |
| Scoped `git diff --check HEAD --` across P2c actor/shard/executor/plugin/ProgramBridge/WGPU/verifier files | PASS |
| `rustfmt --edition 2021 --check ProgramBridge/🧊️component.rs` | PASS |
| `rustfmt --edition 2021 --check --config skip_children=true wgpu/📦️glue.rs` | FAIL — inherited module-wide formatting drift, including unrelated lines 91, 106, 115, 476, 569, 676, 727, 810, 2016, 2424-2443, 3004, 4375, 13309, and 13748. No audit source was changed. |

`git diff --check` being clean and both static mutation gates passing do not cover the lost-owner counterexample above. Runtime determinism/1-2-4-default routing, real GuestRuntime restoration/start, wall-clock bound, and fault scheduling remain deliberately unexecuted and therefore unaccepted.

## Required Closure

1. Make product `SpawnJob` admission atomic and one-owner at every failure. Pre-reserve a fixed refusal/recovery destination before removing the dynamic effect, or return an explicit non-string admission result that still owns the exact rejected `kind`, `input`, job, placement, and cause. A full registry and oversized kind need distinct exact refusal causes.
2. Compose that owner through both `ProgramBridge` handlers before any early `?` return. It must be retained/published to a bounded lane and drained one owner/control/page opportunity at a time; it must not rely on `ExchangeOutcome` destructor semantics.
3. Add production-shaped `MAX+1` and oversized-kind laws/mutations: fill every product recovery reservation, execute the normal action and command exchange, attempt one additional `SpawnJob`, and assert exact kind/input identity, no premature dynamic drop, no replay/`JobStep`, one-owner rediscovery, and bounded close. Cover refusal/abort/completion/app close/realm close/panic/ordinary drop from that state.
4. Then rerun a fresh independent source audit and the deferred executable gate: real normal ActionBus → plugin session → ProgramBridge → KernelClient → pinned shard → GuestRuntime checkpoint/restore/start/step at `1/2/4/default`, deterministic record/replay, wrong identity refusals, backpressure retry, capacity, fuel/deadline/cancel/stale/fault/panic/drop, and measured `<8 ms` opportunities.

P2c and Phase 2 remain **RED**.
