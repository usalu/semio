# Native Deadline Continuation and Typed Host Faults

## Current Qualification

The overall end-to-end goal remains active and incomplete. No real authenticated MCP-to-Hub-to-cold-Map inference journey has passed yet.

- Native reactor run `R35BWr/00` (session70505): build passed; first10 exact laws passed, including actual overlapping reconcile generations and exact issued feedback. Law10 failed inside native close terminal authority; laws11/12 did not run.
  Executable SHA256: `f5b7fa5913bbe82fa4b2312f24a8c2ae20972b4e5433a56eb59200b4c1cdc264`,125903040bytes.
- Diagnostic rerun `ge8MXp/00` (session53408): build passed and first10 laws passed again. Law10 failed with `plugin.internal.interactive-ceiling`: native close callback10439us versus strict8000us. It is not a passing lifecycle gate.
- Run65006/`2j5R8a/00`: terminal BUILD RED from the concurrently mounted cold-pair tests (wrong include path and header helper shadowing). No lifecycle laws ran. Home owns these corrections. The selector now contains20 exact laws including status round-trip, physical-close counting and submit-refusal coverage; none of the newly added laws has run yet.
- Independent actor deadline fixture oracle session37991:1 test passed,204 skipped. AJV structure and lodash elapsed comparison retained.
- Typed host fault source session97997: intentional RED before implementation; session73910: GREEN10 neutral cases. Native host session61794/`GFajeA/00` is building Wasmtime dependencies in the now-free root lifecycle cache. The two exact transport laws have not run yet.

## Native Close Repair

The callback had already mutated its retained cleanup pump before its wall-clock verdict. A late callback replaced the candidate Ready/Complete/ExternalWait with a permanent fault, leaving exact quarantine ownership but no rescheduler.

The implementation now retains a candidate status in the exact native worker state and publishes DeadlineYield on a non-fault overrun. A later foreground cleanup opportunity schedules a publication-only continuation. It reads the retained candidate without consuming it or entering the cleanup pump. Only a successful strict-clock verdict clears it. Repeated late publication retains the same candidate. Genuine pump faults dominate the deadline outcome. First late elapsed time remains separately retained after later short callbacks.

Clock absence/regression remains a real fault with retained quarantine; it is not a retry without time authority. The8ms limit is unchanged.

The new native law holds the pump mutex while retrying a late candidate, so Complete cannot pass by accidentally entering the pump. The existing clock fixture now explicitly distinguishes deadline-yield from permanent fault. Separate physical app-step counting and stopped-pool submit-refusal laws are now added; they are not yet run. The physical law injects the outer clock only after a real app close callback, then holds the exact retained outcome through publication-only retry. Missing/backward clocks still quarantine; deadline status encoding now has an explicit round-trip law.

## Typed Guest Fault Transport

Native host now has `TurnFault::Guest(Fault)`. Wasmtime poll, owned ABI decode and the async component PollTask preserve decoded code/retryability through a64KiB-bounded fault decoder. Poll's oneshot uses the typed result instead of String.

The host eligibility predicate independently refuses commands, ordinary events, mixed/multiple lifecycle events, wrong codes and nonretryable faults. Empty or one lifecycle-only event can qualify. This predicate is not yet a mounted scheduler continuation. Browser worker fault transport also remains string-only. No production retry completion claim is made.

New schema/fixture/tests live under `plugin/🖥️host/🔁️lifecycle`. The permanent `guest-fault-check [--native]` command and source/native launch seed entries are registered. Generated launch freshness must be checked after registry regeneration. Scoped format session66341 passed; scoped git diff --check passed.

## Parallel Lanes

The folder canonical-bootstrap mirror passed real Bun SQLite process checks plus the scoped worker race tests; see `📓️sol-folder-canonical-bootstrap-mirror.md`.

Home is implementing cold-pair ingress. Root review refused premature loader integration: the initial module still used whole-pair allocation/hash/wipe and detached load RAII ownership, and must be converted to incremental retained ownership before integration.

WGPU lane is implementing actual per-document GIS three-Store commit ownership and keeping production registration unavailable until close/recovery ownership is complete. Its prior storage/kernel tests do not qualify a real GIS factory.

FullHub session86949 remains building; no live Hub/materialization success. GIS native child65957 was a terminal SIGKILL build failure, not an observed browser/runtime result.
