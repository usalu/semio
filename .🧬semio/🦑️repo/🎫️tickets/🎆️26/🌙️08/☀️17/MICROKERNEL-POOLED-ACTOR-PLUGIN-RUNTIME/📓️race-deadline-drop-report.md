# Drop redundant race_deadline wrapper

## Why

Packet terra-http-streaming wired internal deadline racing into `StorageTicket::await_result` (`StorageScheduler` in `semio-framework-os-services`). The external `race_deadline`/`Race<T>` helper in `plugin/host/⚡️effects` existed solely to cover that gap (its own doc said so). After the services fix, the outer race was double-racing dead complexity.

## Audit

See `📓️race-deadline-usage.md`. Only call site was `dispatch_storage`; HttpPool/ComputePool never used it.

## Change

File: `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/⚡️effects/🦀️component.rs`

- Removed `Race<T>`, `race_deadline`, and the `⏰️DeadlineTests` region (2 unit tests + `futures_lite_block`)
- Simplified `dispatch_storage` to `ticket.await_result().await`, matching:
  - `Ok(bytes)` → completed ok
  - `Err(StorageError::DeadlineExceeded)` → `deadline-exceeded` (same guest-facing code/message as the old `Race::TimedOut` branch)
  - other `StorageError` → `storage-error`
- Dropped unused `runtime` clone / external `sleep_until` race in that path
- Dropped unused `std::future::Future` / `std::task::{Context, Poll}` imports

Did **not** touch `semio-framework-os-services`.

## Verification

| Step | Result |
| --- | --- |
| `cargo check -p semio-framework-plugin-host --all-targets` before | ok (`terra-race-deadline-check-before.txt`) |
| `cargo test -p semio-framework-plugin-host` before | **119** passed, 1 ignored (`terra-race-deadline-test-before.txt`) |
| `cargo check -p semio-framework-plugin-host --all-targets` after | ok (`terra-race-deadline-check-after.txt`) |
| `cargo test -p semio-framework-plugin-host` after | **117** passed, 1 ignored (`terra-race-deadline-test-after.txt`) |

Delta = exactly the two removed `race_deadline` unit tests. Remaining suite green.

## Ticket / MCP

Repo MCP unavailable this session (`📌️mcp-unavailable-race-deadline.md`). Worked inside already-open `2026/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME` (goal `R26-02/RUNNING-SKETCHPAD`). Parent ticket left open — this is a packet cleanup, not the whole microkernel effort.
