# Component driver: cargo exit 101 with 0 failed units

## Cause

Run `wuZBGHM`, pass `prototype-00`: `l/cargo-prototype-00.log` (the pass's own stderr log, never surfaced) holds nine
`error: could not compile ... process didn't exit successfully: rustc-wrapper ... (exit code: 1)` blocks. Each is preceded by:

    semio-build-scheduler: build broker unreachable at \\.\pipe\semio-broker-<pid>-<id>: All pipe instances are busy. (os error 231)

The rustc wrapper (`native-build/🎟️admission/🦀️.rs`, `Broker::connect`) opens the Windows named pipe once; when the node
server has no free pipe instance between two accepts (a burst of parallel rustc wrappers) Windows answers ERROR_PIPE_BUSY (231)
and the wrapper exits 1 before running rustc. No `compiler-message` exists, so the driver counted 0 failed units and only
recorded `cargo exit 101`. Not a source, lock, registry (`geo`) or RUSTFLAGS problem.

## Fixes

- `🎟️admission/🦀️.rs`: `connect` retries ERROR_PIPE_BUSY (`retry_while`, `pipe_busy`, up to 600 x 50 ms); other errors fail at once. Rust unit test `retries_only_busy_pipes`.
- `🧩️component-pass/🟦️.ts`: per pass the last `policy.stderrTailLines` non-"Compiling" stderr lines (each cut at 400 chars) are kept; on any nonzero cargo exit they go into the thrown `Native build failed` error and a `## Cargo diagnostics` section of `report.md`. Policy key `stderrTailLines: 120` added to `🧬️schema/🔣️.json`.
- Test `🧪️tests/🟦️.ts`: stub mode `stderr-exit` (links all, writes stderr, exits 101); new case asserts error and report carry the stderr line.

## RUSTFLAGS duplication

`CARGO_ENCODED_RUSTFLAGS` doubled only in cargo's planning/lock-refresh "environment variables may affect" note (log line 1869); the
real build environment (line 12474) is single (`-Zthreads=0 -Clink-arg=--max-memory=536870912 -Clink-arg=-zstack-size=8388608`).
Cause of the doubled planning value not located; flags are idempotent, so it did not cause the 101. Left unchanged.

## Verification state

Neither the Rust test nor the bun test was executed: the bun suite needs the repo test harness (direct `bun test` fails with
"Original parent Script process invocation required") and Cargo is gated behind the repo launcher. Run via `repo` test targets of the native-build projects.
