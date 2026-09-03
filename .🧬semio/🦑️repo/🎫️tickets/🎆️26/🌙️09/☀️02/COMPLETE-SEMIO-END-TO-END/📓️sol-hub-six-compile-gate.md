# Hub Six-Diagnostic Compile Gate

## Outcome

The six hub-owned Rust diagnostics exposed after the DB and replication repairs are reduced to zero. The production hub now compiles through the retained-page HTTP response, mutation-message rejection payload, timestamp construction, and associated response-type boundaries that previously stopped it.

This report is reconstructed by the coordinator from the executor's final evidence because the executor completed without leaving the announced report file. Counts below preserve the reported results and do not upgrade downstream failures to passes.

## Implementation

- Added `db_io_pages_into_http_bytes`, a bounded asynchronous lowering from `DbIoPages` to an Axum `Bytes` response. The function rejects content above the hub blob-response limit, copies only the retained fragments, and explicitly drives the page terminal to retirement while yielding between close steps on both success and oversize failure.
- Replaced the hub rejection-path `serde_json` encoding requirement on `MutationMessage` with its first-party `ToValue` representation and canonical `os_pack::json` encoding.
- Removed the invalid `.await` from the synchronous `HybridLogicalTimestamp::new(0, 0)` test-envelope constructor.
- Added the language-neutral fixture `🌎️hub/📦️packages/🦀️rust/🧪️fixtures/🧬️hub-boundaries/🔣️.json`. Its mutation-message projection is checked through the first-party encoder/decoder and a `serde_json::Value` oracle; its blob text is shared by the route test.

## Verification

| Probe | Current result |
| --- | --- |
| fresh-target `semio-hub --all-features` compile | six diagnostics to zero |
| hub quick discovery | 40 tests across two binaries |
| initial quick execution | 13 tests passed before fail-fast/cancellation exposed runtime failures |
| exact mutation-message fixture | 1/1 passed |
| focused share group with `RUST_MIN_STACK=16777216` | 3/4 passed; one test received a non-`Welcome` first frame |
| focused descriptor group with `RUST_MIN_STACK=16777216` | 2/3 passed; one test received a non-`Welcome` first frame |
| focused private-stream group with `RUST_MIN_STACK=16777216` | 0/1; test received a non-`Welcome` first frame |
| blob route with `RUST_MIN_STACK=16777216` | PUT completes; subsequent payload read fails before hub HTTP lowering |
| scoped diff check | passed |

Increasing the worker stack to 16 MiB removes the earlier stack overflow. It does not make the affected product assertions pass, so the larger-stack results are retained as partial evidence only.

## Newly Exposed Boundaries

1. The remaining share, descriptor, and privacy failures are handshake outcomes, not compile failures. Their first server frame is not `Welcome`; a read-only audit owns attribution between stale fixtures and a real authorization/descriptor regression.
2. Blob retrieval reaches `PayloadStorage::get` and fails there with `internal error: stale generation: expected GenerationId(35), got GenerationId(35)`. The equal printed generation values imply an unrendered lease/phase mismatch. This occurs before `db_io_pages_into_http_bytes`, so it is a DB runtime issue rather than evidence against the repaired lowering.
3. Three PostgreSQL tests require a Docker socket unavailable in the current environment. They are environment-blocked and are not counted as backend runtime passes.
4. The aggregate hub build is separately stopped before Rust by TypeScript parameter-property syntax in the actor-return path under strip-only transformation. That build blocker is outside this packet.

## Cleanup And Scope

The executor removed its private 1.9 GiB generated target recoverably from the ticket, preserved concurrent generated material, left no `[DEBUG]` logs, and reported a clean scoped diff check. No replication leaf serde derives were restored, and no DB runtime, actor-return TypeScript, artifact-bootstrap, WGPU, or goal/ticket lifecycle files were changed by this packet.
