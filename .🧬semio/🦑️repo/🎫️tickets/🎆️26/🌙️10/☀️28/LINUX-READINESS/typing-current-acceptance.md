# Current Typing Acceptance

Checkpoint: 2026-10-30, after the four-phase canonical check and all three mounted OS runtime tests passed. This report separates observed acceptance from remaining execution. Historical failures and unknown reboot outcomes remain in [typing-parent-acceptance-scope.md](typing-parent-acceptance-scope.md).

| Gate | Observed result | Durable evidence in 🗑️generated |
| --- | --- | --- |
| Complete current TypeScript conformance and node-graph laws | 37 passed, 0 failed; 25,517 assertions; status 0 at 01:06:30 UTC | typing-net-conformance-unit-coverage |
| Real Original edit unit inventory | 3 passed, 0 failed/ignored/filtered; status 0 at 00:50:35 UTC; no Nx cache hit | typing-net-edit-inventory-reboot11220615 |
| Canonical Rust check | All four phases passed; status 0 at 01:26:00 UTC; no Nx cache hit | typing-net-parent-check-companion-fixed-reboot11220615 |
| Tool-run, operations and render consumer checks | Three native check targets plus three declared prerequisites passed; status 0 at 22:19:28 UTC on October 29; no Nx cache hits | typing-net-os-consumers-restored |
| Previous combined parent runtime | TS prerequisites passed, then Cargo build exited 101 without delivered compiler diagnostics; frontend status 1 | typing-net-parent-full-inventory-reboot11220615 |
| Current combined parent runtime | Status 1 at 02:01:57 UTC: Cargo resource refusal, os error 11; TS prerequisites passed | typing-net-parent-runtime-four-checks-accepted |
| Mounted OS typing tests | 3 passed, 0 failed/ignored/filtered; status 0 at 02:14:31 UTC; no Nx cache hits | typing-net-os-commit-reboot11220615 |

Each evidence name has a .log and, after an observed child close, an .outcome.json. A missing outcome is never success. Startup metadata records boot ID, parent, process start ticks and argv. Current boot: 11220615-7932-4930-ae1a-7d7fd56ccf52.

## What Has Actually Been Proven

The edit unit log explicitly enumerates cancellation before publication, all neutral semantic outputs with real allocation/backing checks, and the funded resident growth test. The latter retains an issued reader over 8,197 strings while 65 earlier windows force real reserve and insertion work. Actual per-turn heap receipts, denied applicable grant currencies, stable net backing/key, and controlled source/resident cleanup passed.

The canonical compiler check now includes native production, wasm32-wasip2 production, production unit tests, and the real companion test harness. It exposed six stale test callers: four actor constructors still supplied String, one scrub state lacked its ActorId wrapper, and one exhaustive scrub caller passed an owned input rather than the current mutable Option. Those callers were corrected without changing production APIs or expected neutral outputs.

The neutral test-scope fixture and independent TypeScript AST/Ajv/TOML oracle first reproduced omission of production tests, then omission of production unit checking. Both owner recipes now include the real required scope. Existing native, wasm and companion check steps were preserved.

Checks type-check; they do not execute assertions or link every combined test binary. Separate check invocations also do not prove the runtime invocation with both packages and its unified feature selection. The remaining combined runtime must enumerate the actual production reader, transfer and resident growth tests, alongside all companion laws.

## Remaining Acceptance

The parent runtime selected both actual packages, all existing tests, current six caller corrections and the bounded shared diagnostic replay handler. It failed during Cargo build with EAGAIN before Rust assertions. The exact internal syscall/resource cause is not present in the public log. A subsequent run requires concrete diagnosis; no blind retry or capacity change was submitted. [Runtime Source and Failure Evidence](typing-runtime-source-evidence.md) records the precise stage, observed binary coordinate, and qualified current source identities.

The mounted OS gate executed all three authored Original typing methods, including the production reader/edit/commit integration that donates the same net vector and payload backings after paid reader return. This runtime acceptance is complete.

Atomic resident reserve and shift demands scale with descriptor count. Adequately funded progress and refusal without mutation are proven; arbitrary tiny-grant liveness is not claimed. No quota, guard, grant ceiling, Original authority, decoder, revision or writer fence was weakened.

The separate read-only schema API census is complete: [typing-schema-retrieval-context.md](typing-schema-retrieval-context.md). It identifies the existing source/input contract carrying real baseUri and source text, plus the caller boundaries that currently discard retrieval context. No schema or URI helper was edited for that research.

