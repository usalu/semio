# P1k Nonblocking Process Transport

## Outcome

The last two production `thread::Builder` sites outside the process-wide `WorkerPool` were removed from the plugin shard process transport. Parent child-stdout and child stdin are now nonblocking incremental byte streams polled during bounded transport turns.

## Architecture

- Native Unix pipes are switched to `O_NONBLOCK` through the platform ABI.
- Native Windows pipes are observed with `PeekNamedPipe` before a read, without adding a dependency.
- Each poll reads at most 64 KiB and decodes at most 32 frames.
- The persistent decoder preserves fragmented frames, concatenated frames, and queued data across heartbeat/liveness polls.
- Frame payloads are capped at 16 MiB before allocation.
- Parent heartbeat/EOF state is updated during bounded polls; child heartbeats remain finite `Lane::Timer` work.
- `Drop` terminates and reaps the child directly and contains no executor bridge.

## Verification

- `rustfmt --edition 2021` parsed and formatted the implementation successfully.
- Static source census: zero production `thread::Builder`, `thread::spawn`, or `semio_framework_async::block_on` in process transport. Remaining `thread::sleep` and entrypoint bridges are inside `#[cfg(test)]` transport integration tests.
- Added decoder tests for fragmented plus concatenated frames and oversized-prefix rejection.
- The isolated `semio-framework-plugin-host` structured check run by the concurrent Phase 4 gate completed with exit code 0 after this file changed and reported no process-transport diagnostic. Full process integration tests remain queued behind the repository-wide stdio repair gate.

## File

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🚚️process-transport/🦀️component.rs`
