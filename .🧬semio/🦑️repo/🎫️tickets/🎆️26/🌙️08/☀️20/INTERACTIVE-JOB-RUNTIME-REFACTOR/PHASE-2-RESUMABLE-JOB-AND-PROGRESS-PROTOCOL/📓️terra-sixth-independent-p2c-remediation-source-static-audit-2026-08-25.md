# Terra Sixth Independent P2c Remediation Source-Static Audit — 2026-08-25

## Verdict

**GREEN — source/static re-audit accepted.** The fifth-RED admitted-owner gap is closed on the exact inspected tree. The selected normal `Effect::SpawnJob` is moved once into a non-`Clone` `RawSpawnJobOwner`; its original `String` allocation, `Vec<u8>` allocation, placement, job, and selected effect index stay owned through request, claim, authority, recovery, and incremental close. Fixed kind/request fields are derived witnesses, not replacements for the raw owner.

This is not final P2c/Phase-2 acceptance. The required compiled and executable native/Wasm/browser/real-pool timing and determinism matrices remain deferred and therefore Phase 2 remains RED at the final acceptance boundary.

This report preserves all prior Terra reports, including [`📓️terra-fifth-independent-p2c-remediation-source-static-audit-2026-08-25.md`](./📓️terra-fifth-independent-p2c-remediation-source-static-audit-2026-08-25.md).

## Scope and Limits

- Independently reread the P2c contract, fifth Terra RED, updated Sol report, accepted P2a1/P2d material, ActionBus/plugin/session/shard/executor route, ProgramBridge, WGPU product ownership/recovery/refusal code, and root verifier on the exact current tree.
- Inspected production ownership and caller bodies directly; did not rely on verifier strings or test names for the verdict.
- No source was changed. No Cargo, Nx, Wasm, browser, build, or runtime/executable test was run.

## Accepted Source Evidence

1. **Exact raw move, no clone/drop.** Preflight still borrows kind/input and reserves recovery before `effects.remove(index)`. The admitted branch moves that one effect into `MountedProductReplayRequest::from_admitted_effect`, which consumes it into `RawSpawnJobOwner`; no `drop(self.effects.remove(index))`, projection-only constructor, raw reinsertion, or raw clone remains (`wgpu/📦️glue.rs:4219-4275`, `:4821-4852`). Pointer identity is preserved by Rust moves and the raw owner stores the original `String`, `Vec<u8>`, job, placement, and `selected_index`.
2. **Raw is checked at each authoritative boundary.** Mount recomputes job, kind, placement, and request identity from the raw backing before it acknowledges mount or claims a slot (`:6030-6061`). Full authority validation repeats raw job/kind/request/placement checks alongside the complete route/operation/generation/seed/checkpoint/terminal/physical-slot/logical-profile/begin/restore identity before replay work (`:4697-4729`). Request → claim → authority and all retained recovery variants move the same raw owner using `take`, with no `Clone` implementation.
3. **No early accepted retirement.** The raw owner records fixed-witness, accepted-mount, and qualification acknowledgements. Its terminal acknowledgement requires those three plus an accepted replay ordinal; the authority requires all `1/2/4/default` profiles terminal and none in flight before recording terminal ordinal/prefix and changing disposition to `Accepted` (`:3939-4007`, `:4732-4739`, `:6083-6086`). Normal stale/cancel/fault/abort paths leave it `Rejected`, not falsely accepted.
4. **All error and close paths retain exact raw backing.** Mount refusal, request/claim/authority `Drop`, abort, app close, realm close, and queued-client abandonment reject/publish the same generation-qualified owner. Recovery drains raw input then raw kind on separate calls before removing/releasing its slot (`:4065-4207`, `:4254-4275`, `:4618-4640`, `:4741-4778`). An accepted authority remains accepted through its final recovery close; `reject()` only changes pending owners.
5. **The fourth-RED pre-admission refusal remains sound.** Action and command reserve their refusal permit before normal exchange, match `None`/`Admitted`/`Refused` explicitly, and publish a typed `RefusedProductReplay` before returning a diagnostic. Oversized kind/input and full recovery preserve the exact raw spawn, original index, cause, and remaining exchange effects; recovery retry takes that original effect into the admitted raw owner rather than minting a digest-only replacement. Refusal close remains one input/kind/effect/backing opportunity at a time.
6. **Previously accepted replay properties remain present.** The native route is normal ActionBus/plugin exchange → typed product request → `KernelClient` → pinned kernel/shard route. No `mountedJobReplay` JSON control or WGPU-private worker pool exists. Product profiles remain `1/2/4/default` with deterministic modulo slot mapping; lower restore/start state changes only under `Backpressure::Accept`, retaining the exact packet/sequence on non-accept.

## Static Gate Record

| Check | Result |
| --- | --- |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2c-only --self-test` | PASS — `live-source clean; hostile-mutations=159` |
| `bun --no-cache ./📜️script.ts verify interactivity tool-jobs --p2a1-only --self-test` | PASS — `live-source clean; hostile-mutations=13` |
| Scoped `git diff --check HEAD --` across P2c actor/shard/executor/plugin/ProgramBridge/WGPU/verifier files | PASS |
| `rustfmt --edition 2021 --check ProgramBridge/🧊️component.rs` | PASS |
| `rustfmt --edition 2021 --check` actor/shard/executor/plugin scope | Actor reports only inherited formatting at lines 320/331; shard/executor/plugin parse clean |
| `rustfmt --edition 2021 --check --config skip_children=true wgpu/📦️glue.rs` | FAIL — inherited module-wide formatting drift outside P2c, including lines 91, 106, 115, 476, 569, 676, 727, 810, 2016, 2424-2443, 3004, 14167, and 14606. No audit source was changed. |

## Deferred Final Gates

This GREEN establishes only the requested source/static ownership boundary. Before final P2c acceptance, run the real normal ActionBus/session → ProgramBridge → KernelClient → pinned shard → GuestRuntime checkpoint/restore/start/step matrix at `1/2/4/default`, deterministic record/replay and wrong-identity refusal, capacity/backpressure retry, fuel/deadline/cancel/stale/fault/panic/drop/app/realm-close stress, native plus both Wasm builds, browser-mounted P2d observation, and measured `<8 ms`/p99 timing gates.
