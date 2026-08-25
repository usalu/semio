# P9-A1 Owned Byte/Message ABI Kernel

## Verdict

**GREEN and audit-ready after independent-audit remediation.** A schema-first, dependency-free, domain-neutral ABI/data-transfer kernel now exists at `🧰️framework/🔨️modules/🌉️abi`. It is mounted and re-exported by the existing first-party framework package without a Cargo workspace/member/dependency change. Browser/product owners, generated JavaScript, manifests, locks, shared scripts, launch configuration, and ticket metadata remain outside this packet.

The independent RED findings in `📓️codex-p9-a1-owned-abi-kernel-independent-audit-2026-08-25.md` are closed: retained cancellation now blocks all further writer progress and returns the exact admitted page/credits; generation exhaustion quarantines slots without wrapping; handle/reply lookup has no linear scan; and reader allocation/state mutation occurs only after successful work admission.

## Owned Surface

- `AbiRequest { operation, request_id, generation, bytes }`
- `AbiReply { request_id, generation, status, bytes }`
- ordered `AbiEvent { request_id, generation, sequence, event, status, bytes }`
- bounded `AbiPage { handle, index, bytes }`
- `AbiControl::{Cancel, Close, Acknowledge}`
- stable `AbiStatusCode` and `AbiErrorCode`, with optional bounded owned UTF-8 diagnostic bytes
- `AbiHandle { slot, generation }`, constructible only with non-zero slot and generation
- primitive/owned-only `AbiPort`; a future generated JS shim implements this seam and remains outside A1

No public or internal A1 type names `JsValue`, `Promise`, `Function`, `web_sys`, `js_sys`, `wasm_bindgen`, serde, or another external ABI/runtime type.

## Structural Ledger

The canonical record is version byte `1`, envelope tag, then declaration-order fields. Every integer is fixed-width little-endian; every byte vector has a checked `u32` length. Envelope tags are request `1`, reply `2`, event `3`, page `4`, and control `5`; control subtags are cancel `1`, close `2`, and acknowledge `3`. Decoding rejects an unknown version/tag/status/error code, invalid operation/event code, zero handle identity, missing field/optional marker, malformed/trailing length, invalid UTF-8 diagnostic, and every configured capacity overflow.

The language-neutral sources of truth are:

- `🧬️schema/🔣️component.json`: JSON Schema plus explicit binary version/order metadata;
- `🧪️fixtures/📒️ledger.tsv`: eight exact hexadecimal empty/single request/reply/event/page/control ledgers;
- `🧪️fixtures/📐️limits.tsv`: ten exact maximum/maximum-plus-one laws, including non-wrapping handle-generation exhaustion.

The Rust tests consume the schema and both fixtures directly. A Bun standard-library parser independently parsed the JSON/TSV, reconstructed every hex record, checked version/tag order and the little-endian operation field, and checked every `max + 1` row.

## Fixed Limits

| Resource | Maximum | Maximum + 1 |
| --- | ---: | ---: |
| operation code | 4,095 | 4,096 |
| event code | 4,095 | 4,096 |
| request/reply/event body | 1,048,576 bytes | 1,048,577 |
| page | 65,536 bytes | 65,537 |
| diagnostic message | 1,024 UTF-8 bytes | 1,025 |
| pages per transfer | 256 | 257 |
| transfer | 16,777,216 bytes | 16,777,217 |
| in-flight handles | 64 | 65 |
| in-flight requests | 256 | 257 |
| handle generation | 4,294,967,295 | 4,294,967,296 — exhausted/quarantined |

Bound constructors and retained admission failures return the caller's exact allocation/page components; the `max + 1` path does not consume them.

## Retained Lifecycle

`AbiPageWriter` admits one exact page, retains its page/cursor, and copies only the current byte credit. Cancelling after admission atomically removes and returns that full original page in `AbiCancelOutcome`, reports its admitted and already-copied byte credits, and makes every later write step return `Cancelled`; only incremental retirement of already-copied bytes remains legal. `AbiPageReader` retains source/staging/cursor plus exactly one outstanding page until the matching handle-generation-index ACK. It runs cancellation/deadline/interruption/credit admission before setting a target, reserving, or copying, and reserves only the admitted byte count. Both close incrementally under an explicit work budget and expose an idempotent terminal-empty witness.

`AbiHandleTable` uses a direct free-slot stack and direct indexed access. It increments generations with `checked_add`; a slot closed at `u32::MAX` is permanently quarantined, and an all-quarantined table returns `GenerationExhausted` together with the exact rejected value. It can therefore never reissue an old identity. `AbiReplyLedger` uses one fixed direct slot selected by the request id modulo 256, computed in `u64` identically on native and Wasm; an active collision returns `Busy`, while late, lost-generation, and duplicate replies remain rejected. No lookup performs an unbudgeted table scan.

## Canonical Laws

The same-module Rust suite executes these laws:

1. schema and language-neutral fixture completeness;
2. empty, single, maximum, and exact maximum-plus-one handback;
3. deterministic request/reply/event/page/control ordering and native/Wasm-safe little-endian ledger bytes;
4. malformed tag, malformed/trailing length, invalid UTF-8, and missing optional marker;
5. credit/deadline/interruption non-advancement;
6. unknown, future-stale, old-generation ABA, and duplicate ACK behavior;
7. fixed page-count/transfer limits and exact rejected allocations;
8. cancellation before/after seal and after partial admitted-page copy, with exact page/credit handback and deterministic close;
9. near-maximum generation transition, permanent max-generation quarantine, full-table `GenerationExhausted`, and no alias;
10. fixed direct reply-slot collision, loss, replacement, late reply, cross-generation reply, and duplicate reply;
11. reader zero-credit/cancel/interruption/deadline rejection before allocation, target mutation, or copy;
12. interrupted owned-port callback with exact message return;
13. interrupted incremental close;
14. idempotent terminal-empty close for reader and writer.

## Executed Gates

| Gate | Result |
| --- | --- |
| focused `rustfmt --check` on A1 component | GREEN |
| direct dependency-free `rustc -D warnings --crate-type lib` | GREEN |
| direct native `rustc -D warnings --test` | GREEN — 14 passed, 0 failed |
| optimized native `rustc -D warnings -O --test` | GREEN — 14 passed, 0 failed |
| independent audit cancel-after-admission harness | GREEN — 15 passed, 0 failed (14 embedded plus independent hostile law) |
| Bun schema/fixture parser | GREEN — 15 schema definitions, 8 ledger records, 10 limit laws |
| scan/wrap/pre-admission source-static deny-list | GREEN — zero slot scans, wrapping increments, or pre-admission full-page reserves |
| A1 module external ABI/runtime deny-list | GREEN — zero matches |
| framework-glue added-lines external ABI/runtime deny-list | GREEN — zero matches |
| public-signature external type deny-list | GREEN — zero matches |
| scoped `git diff --check` | GREEN |

A deliberately over-broad scan of the whole pre-existing framework glue sees unrelated historical serde documentation. The authoritative boundary scan is therefore the complete new A1 module plus the exact added glue lines; both are clean.

No Cargo workspace, Nx, Wasm, browser, product, manifest, lock, shared script, or launch command was run or changed.

## Changed Paths

- `🧰️framework/🔨️modules/🌉️abi/🦀️component.rs`
- `🧰️framework/🔨️modules/🌉️abi/🧬️schema/🔣️component.json`
- `🧰️framework/🔨️modules/🌉️abi/🧪️fixtures/📒️ledger.tsv`
- `🧰️framework/🔨️modules/🌉️abi/🧪️fixtures/📐️limits.tsv`
- `🧰️framework/📦️packages/🦀️rust/📦️glue.rs`
- this report
