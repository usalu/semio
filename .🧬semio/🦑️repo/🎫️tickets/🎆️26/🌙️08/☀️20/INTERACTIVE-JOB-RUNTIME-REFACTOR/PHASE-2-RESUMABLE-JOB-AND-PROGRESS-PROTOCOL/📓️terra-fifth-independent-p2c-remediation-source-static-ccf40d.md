# Terra Fifth Independent P2c Remediation Source-Static Audit — 2026-08-25

## Verdict

**RED — not accepted.** The fourth-RED refusal repair is a material improvement: it reserves an admission/refusal slot before a normal exchange, preserves a typed refusal for kind/input/recovery admission failure, and closes that refusal incrementally.  The normal **admitted** branch still destroys the selected raw `Effect::SpawnJob` immediately after borrowing its fields to make only a digest/fixed-kind request.  Its exact dynamic `kind`/`input` backing is not transferred to a fixed admitted owner, refusal/recovery owner, or close graph.

This is a production source finding. It preserves the four earlier Terra RED reports, including [`📓️terra-fourth-independent-p2c-remediation-source-static-audit-2026-08-25.md`](./📓️terra-fourth-independent-p2c-remediation-source-static-audit-2026-08-25.md).

## Scope and Limits

- Independently reread the P2c repair contract, the fourth Terra RED, updated Sol report, accepted P2a1/P2d material, ActionBus/plugin/session/shard/executor route, ProgramBridge, WGPU kernel/client/refusal code, and verifier on the exact current tree.
- No source was changed. No Cargo, Nx, Wasm, browser, build, or executable/runtime test was run.
- The `bun` commands below are static hostile-mutation gates, not runtime acceptance evidence.

## Fourth-RED Closure That Is Present

1. **Pre-exchange refusal capacity is real.** Both native `handle_action` and `handle_command` reserve `MountedProductReplayAdmissionPermit` before the normal `exchange` (`ProgramBridge/🧊️component.rs:187-189`, `:214-216`). A full refusal registry returns before an exchange can mint/transfer a product `SpawnJob`; an unused permit's `Drop` releases its exact generation.
2. **Preflight precedes the refusal move.** `ExchangeOutcome::take_product_replay_authority` borrows the selected spawn, checks fixed kind capacity, input-page byte capacity, and obtains the main recovery token before `effects.remove(index)` (`wgpu/📦️glue.rs:4633-4659`). It returns `None`, `Admitted`, or non-`Clone` `RefusedProductReplay`; there is no restored raw-effect insertion and no fallible extraction result.
3. **The refusal branch has a real exact handoff.** `RefusedProductReplay` retains token/generation, instance, cause, original selected index, selected raw spawn, and the untouched remaining `Vec<Effect>` backing. Both bridge handlers explicitly match it, publish it through `KernelClient::retire_product_replay_refusal` before any fallible invocation-frame decode, then return its diagnostic. Its `Drop` also publishes the typed refusal if the pending request/future is abandoned.
4. **Recovery-capacity retry and close are bounded.** A recovery-full refusal stays in `Retry`; after one product recovery reservation is released, one maintenance grant constructs and mounts one exact typed request. Otherwise close advances through spawn split, input backing, kind backing, one remaining effect per grant, remaining-Vec backing, and terminal slot release (`wgpu/📦️glue.rs:4132-4323`, `:6226-6240`). App and realm close drive the same exact refusal owner one opportunity at a time.
5. **Previously accepted post-admission checks remain structurally present.** The admitted request/claim/authority still uses the same generation-qualified product recovery token; authority validation compares kind/request/placement, route, operation/generation/seed, full checkpoint/terminal identity, physical and logical worker identity, begin, and restore ordinal. The shared-pool `1/2/4/default` modulo profiles and the lower accept-only replay-start transition remain in their production bodies.

## Blocking Finding — Successful Admission Still Raw-Drops the Product Spawn Owner

The success path does this in production:

```text
borrow SpawnJob.kind/input/placement
  -> preflight kind/input/recovery
  -> MountedProductReplayRequest::from_admitted_spawn(..., &kind, &input, ...)
  -> drop(self.effects.remove(index))
```

The final operation is explicit at `wgpu/📦️glue.rs:4650-4654`:

```rust
let request = MountedProductReplayRequest::from_admitted_spawn(instance, job, kind, input, placement, recovery);
drop(self.effects.remove(index));
permit.release();
return MountedProductReplayAdmission::Admitted(request);
```

`from_admitted_spawn` only copies fixed kind bytes and calculates `JobReplayRequest::from_spawn(kind, input)`; `MountedProductReplayRequest` stores `{ instance, job, fixed job_kind, request digest/schema identity, placement, recovery }` (`:4020-4046`). It retains neither the raw selected `Effect::SpawnJob` nor its original `String` and `Vec<u8>` backing. Its normal and `Drop` recovery variants likewise retain only the fixed kind/request/placement projection.

Therefore the selected raw spawn is dropped on the normal admission path, before `KernelClient::mount_product_replay` can accept/reject it. If mount later refuses for an unregistered/closed instance, duplicate job, malformed identity, or fixed claim capacity, the exact owner returned through `Err(MountedProductReplayRequest)` has already lost the selected raw input owner; `retire_product_replay` can publish only that reduced request. A separately existing shard/session input is not an ownership transfer from this selected product effect: the product authority only proves a digest, and no current body ties an exact raw allocation from this exchange to the later shard owner.

This falsifies the required no-production-raw-drop rule and the P2c contract's fixed/page exact-owner requirement. It also leaves a source-level gap in cancellation/fault/panic/close after *successful preflight but before accepted product mount*: all product recovery paths own the digest projection, not the selected exact raw spawn.

The 133-mutation gate verifies refusal retention but does not reject replacing the admitted raw drop with another discard. Its preflight predicate deliberately permits a post-preflight `self.effects.remove(index)` and has no law that proves the admitted request carries the raw spawn/input backing or that an admitted-then-mount-refused path returns that backing unchanged.

## Static Gate Record

| Check | Result |
| --- | --- |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test` | PASS — `live-source clean; hostile-mutations=133` |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test` | PASS — `live-source clean; hostile-mutations=13` |
| Scoped `git diff --check HEAD --` across P2c actor/shard/executor/plugin/ProgramBridge/WGPU/verifier files | PASS |
| `rustfmt --edition 2021 --check ProgramBridge/🧊️component.rs` | PASS |
| `rustfmt --edition 2021 --check --config skip_children=true wgpu/📦️glue.rs` | FAIL — inherited module-wide formatting drift outside the P2c change, including lines 91, 106, 115, 476, 569, 676, 727, 810, 2016, 2424-2443, 3004, 4714, 13864, and 14303. No audit source was changed. |

Runtime proof remains deferred: real ActionBus/session → ProgramBridge → KernelClient → pinned shard → GuestRuntime checkpoint/restore/start/step at `1/2/4/default`, deterministic replay, latency, and fault scheduling were not executed.

## Required Closure

1. Replace the admitted `drop(self.effects.remove(index))` with an exact, non-`Clone` raw-spawn transfer into an admitted fixed/page owner. Preserve its kind/input allocation until the corresponding fixed copy and mount acceptance are irrevocably committed; later mount refusal must return or publish the same owner, not only its digest projection.
2. Extend product recovery/request/claim/authority close states so every successful-preflight, mount-refused, cancel, stale, fault, panic, ordinary `Drop`, app-close, and realm-close branch owns and drains the exact admitted raw backing incrementally.
3. Add production-shaped laws and hostile mutations for: admitted raw-spawn retention; admitted → mount-refused exact handback; request/claim/authority drop after admission; and one-owner incremental input/kind/backing close. A mutation restoring `drop(self.effects.remove(index))` must fail.
4. After the source ownership repair, rerun this independent audit and the deferred executable P2c matrix.

P2c and Phase 2 remain **RED**.
