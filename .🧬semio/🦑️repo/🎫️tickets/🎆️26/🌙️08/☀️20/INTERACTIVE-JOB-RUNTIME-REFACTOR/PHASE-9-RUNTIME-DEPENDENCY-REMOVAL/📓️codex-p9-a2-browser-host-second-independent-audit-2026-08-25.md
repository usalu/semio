# P9-A2 Browser Host Second Independent Audit

Date: 2026-08-25  
Verdict: **GREEN under the authorized dependency-free A2 gates.**

## Scope

This is a fresh read-only audit after the first independent audit's maximum-envelope RED finding. I read the root and UI instructions, the Phase 9 plan and A2 implementation report, the first RED audit, the mounted A1 ABI source, the actual staged Rust/schema/fixture/JavaScript diff, and the direct symbol references. No production source, manifest, lockfile, Nx configuration, Wasm artifact, or browser state was modified. The two newly built test binaries are ticket-local audit artifacts.

## Remediated Release Blocker

The contract is now exact and consistent across schema, Rust, JavaScript, and the A1 encoder:

| Value | Bytes |
| --- | ---: |
| Maximum event body | 1,024 |
| Event envelope overhead | 27 |
| Maximum encoded event | 1,051 |
| Page envelope overhead | 18 |
| Maximum page body | 1,033 |
| Initial metadata probe | 1,024 |

`LinearMemoryBrowserHostPort::poll` first probes with 1,024 bytes, rejects a reported required length above 1,051 before resizing, then retries once at the exact required capacity. The focused Wasm-import seam proves the previous failure path is repaired:

- a 0-byte event body encodes to 27 bytes and copies once;
- a 1,024-byte event body encodes to 1,051 bytes, produces capacities `[1024, 1051]`, copies once, and is removed;
- a 1,025-byte event body and a 1,034-byte page require 1,052 bytes and are rejected before allocation/copy;
- a retained exact-size event is dropped without copy when cancellation occurs after the short probe, and is closed without copy when close occurs after that probe;
- the complete 1,033-byte page envelope uses the same `[1024, 1051]` path.

The JavaScript host independently reports the required length while retaining its item and source/key metadata, copies only after an exact-capacity retry, moves the delivered event identity into its bounded ACK ledger, accepts the first ACK, and rejects the duplicate. Its Bun mock run also exercised cancellation and close between probes, max+1 rejection before copy, pointer latest-wins, frame deduplication, callback close cleanup, and accessibility label handling.

## Rechecked A2 Properties

| Property | Result |
| --- | --- |
| Direct UI-host Wasm rows | GREEN — 4 removed, 0 remain (`js-sys`, `wasm-bindgen`, `wasm-bindgen-futures`, `web-sys`) |
| Owned Rust browser-SDK tokens | GREEN — 0 matches for the A2 deny census |
| Owned boundary and identities | GREEN — `BrowserHostPort: AbiPort`, `CanvasId`, generation-bearing `ListenerId`, and owned A1 messages only |
| Callback/ABA lifecycle | GREEN — guarded listener generation checks; exhausted generations are not reused; stale/unknown/ABA laws pass |
| Coalescing | GREEN — metrics and pointer move are latest-wins; RAF is deduplicated |
| Accessibility | GREEN — focusability plus configurable role; caller/pre-existing localized label required, with no default language |
| Detach/close | GREEN — pending owned work drains, detach request is tracked, and terminal-empty requires the detach reply acknowledgement |
| Progress and acknowledgement | GREEN — CanvasHost inspects one body byte per grant; exact event and page laws produce one event reply/page acknowledgement respectively |
| Non-comment direct external A2 consumers | GREEN — 0; the only out-of-directory textual match is a documentation reference in `deadlines.rs` |

The staged A2 packet is 10 files, 2,180 additions and 218 deletions. Its Rust portion is `Cargo.toml -4`, `glue.rs +3/-1`, `event.rs +243/-3`, and `window.rs +1216/-210`; the JavaScript shim/test are `+409/+209`.

## Fresh Gates

| Gate | Result |
| --- | --- |
| Focused Rust wrapper, debug | GREEN — 34 passed, 0 failed |
| Focused Rust wrapper, `-C opt-level=3` | GREEN — 34 passed, 0 failed |
| Actual linear-memory import seam | GREEN — 0/exact/+1, bounded preflight, exact retry, one copy, cancel/close paths, page ceiling |
| Bun mock browser suite | GREEN — emitted `encodedMax:1051`, `retainedRetry:"exact"`, `acknowledgement:"once"`, and max+1-before-copy success markers |
| Node syntax check | GREEN |
| Bun schema/fixture parser | GREEN — version 1; 6 operations; 11 events; 10 trace, 14 limit, and 8 framing rows |
| `rustfmt --check` on owned Rust | GREEN |
| Scoped staged `git diff --check` | GREEN |

The focused host wrapper is compiled with the explicit test-only Wasm cfg seam; this is not a Wasm build. Rust emits two `unsafe_op_in_unsafe_fn` warnings inside that test-only import forwarding shim, but compilation and all 34 runtime tests pass in both configurations. They are not a regression in the repaired envelope behavior.

## Deferred Gates / Blockers

There is **no blocker within the authorized A2 acceptance envelope**. Cargo workspace/package checks, Nx, a real Wasm target build and generated-import link, and real browser DOM/RAF/DPR/ResizeObserver/clipboard-permission behavior were intentionally not run. They remain integration gates, not evidence against this GREEN result.
