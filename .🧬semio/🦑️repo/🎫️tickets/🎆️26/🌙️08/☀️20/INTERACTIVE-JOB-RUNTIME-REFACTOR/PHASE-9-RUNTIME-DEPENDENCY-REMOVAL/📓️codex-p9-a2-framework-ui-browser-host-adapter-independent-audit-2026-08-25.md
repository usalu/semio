# P9-A2 Framework UI Browser-Host Adapter Independent Audit

## Verdict

**RED — do not accept P9-A2.** The dependency-removal and owned-boundary claims are substantially true, and the supplied focused laws reproduce. The actual linear-memory browser port cannot receive a valid maximum-sized event: its fixed 1,024-byte receive buffer is smaller than the 1,051-byte encoded A1 event envelope. The JavaScript shim reports the required length and retains the event, while Rust converts that result to `LimitExceeded` without retrying at the required capacity. A valid event is therefore permanently undeliverable.

No production source, Cargo workspace, lockfile, Nx configuration, Wasm output, browser state, or script was modified by this audit. The two ticket-local test binaries are audit artifacts.

## Read Scope

- Root and UI `AGENTS.md`, the P9-A plan/scout, P9-A1 second GREEN audit, P9-A6 third GREEN audit, and the claimed P9-A2 report.
- Live UI-host Cargo manifest, Rust glue/event/window sources, schema, trace/limits fixtures, JavaScript shim/tests, tracked diff, and direct consumers.
- The mounted A1 ABI encoder/decoder, because the claimed linear-memory boundary depends on its full-envelope size.

## Release Blocker

The schema permits `eventBytes: 1024`. The shim's `event()` builds a complete A1 `AbiMessage::Event`; for a 1,024-byte event body its actual encoded message is 1,051 bytes (2-byte version/tag + 8 request id + 4 generation + 4 sequence + 2 event code + 3 status + 4 body length + 1,024 body).

`LinearMemoryBrowserHostPort::poll` allocates exactly `BROWSER_HOST_MAX_EVENT_BYTES` (1,024) at `🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust/🦀️window.rs:1354`. The shim returns the required length without dequeuing if the supplied capacity is too small (`🟨️browser-host.js:289-292`). Rust then treats every result greater than 1,024 as `AbiErrorCode::LimitExceeded` (`🦀️window.rs:1368-1370`) rather than resizing/retrying. Thus the retained valid message repeats the same failure forever.

I executed a fresh Bun host-shim probe with a valid `beforeinput` payload of 1,009 UTF-8 bytes. Its normalized event body is exactly 1,024 bytes; `poll(..., 1024)` returned `1051`, and a second `poll(..., 1051)` returned the retained `1051` bytes. This directly proves the interface incompatibility; the Rust receive capacity is 27 bytes short of the declared maximum message.

This violates the max-event law, linear-memory translation boundary, and the required loss-free per-byte/event-grant behavior. The supplied 32 Rust tests use `BrowserFixturePort`, not `LinearMemoryBrowserHostPort`, so they cannot observe it. The JavaScript mock test likewise polls with a 1,024-byte buffer only for small fixtures.

## Reproduced Gates

| Gate | Result |
| --- | --- |
| Focused dependency-free Rust wrapper, debug | GREEN — 32 passed, 0 failed |
| Focused dependency-free Rust wrapper, optimized | GREEN — 32 passed, 0 failed |
| Bun mock-browser suite | GREEN |
| Node syntax check for shim | GREEN |
| Bun schema/trace/limits parser | GREEN — version 1, 6 operations, 11 events, 10 trace rows, 10 limit rows |
| `rustfmt --check` on owned Rust | GREEN |
| Scoped `git diff --check` | GREEN |
| Shim maximum-event / linear-port compatibility probe | **RED** — 1,024-byte body requires and retains 1,051-byte envelope; Rust capacity is 1,024 |

The direct Rust commands used an explicit wasm cfg only to omit native `winit`/clipboard branches from the dependency-free wrapper; neither Cargo nor a Wasm target/browser run was invoked.

## Boundary And Census Findings

| Check | Result |
| --- | --- |
| Removed direct UI-host Wasm dependency rows | GREEN — exactly 4: `js-sys`, `wasm-bindgen`, `wasm-bindgen-futures`, `web-sys` |
| Current UI-host manifest direct rows for those four | GREEN — 0 |
| Rust browser-SDK deny pattern (`HtmlCanvasElement`, `ResizeObserver`, `JsValue`, `Promise`, `js_sys`, `wasm_bindgen`, `web_sys`, `Closure<`, `js_sys::Function`) | GREEN — 0 matches |
| Public browser boundary | GREEN — `BrowserHostPort` extends owned A1 `AbiPort`; browser identities, requests, replies, errors, progress, and frame envelopes are owned types |
| Shim dependencies | GREEN — no third-party imports; it is the sole owner of DOM, RAF, resize observer, clipboard promises, listeners, and Wasm memory |
| Direct non-comment external Rust consumers of P9-A2 browser symbols | GREEN — 0 |
| A3/render target changes in this packet's tracked diff | GREEN — 0 |
| Root `Cargo.toml` diff | GREEN — 0 |
| Root `📜️script.ts` diff | GREEN — 0 |
| Root `Cargo.lock` worktree diff | OUT OF SCOPE — present as concurrent `+1/-6`; not attributable to P9-A2 |

The JavaScript shim has bounded critical/latest/listener/clipboard identities, listener generations avoid reuse at `u32::MAX`, resize and pointer movement are latest-wins, RAF is deduplicated, accessibility requires a caller-provided/pre-existing label with no language default, and cancellation suppresses settled clipboard callbacks after cancel/close. The source also implements ordered internal close draining and detach-ACK gating. These positives do not repair the maximum-message port failure.

## Required Remediation

Define the linear-port receive capacity in terms of the complete A1 envelope (`maximum event body + ABI event framing`), then retry `poll` with the shim-reported required length only after validating a bounded complete-message ceiling. Retain the shim item on short capacity, preserve one-byte/grant inspection after decode, and add permanent debug/optimized laws that send an exactly-1,024-byte body through the actual `LinearMemoryBrowserHostPort` import seam. Add the matching max+1 rejection law so a malicious required length cannot force an unbounded allocation. Re-run the complete A2 matrix after that repair.

## Deferred Integration Gates

Cargo package/workspace checks, Nx, Wasm import linking, and real-browser execution remain unrun by packet constraint. They are not used as evidence for this RED verdict.
