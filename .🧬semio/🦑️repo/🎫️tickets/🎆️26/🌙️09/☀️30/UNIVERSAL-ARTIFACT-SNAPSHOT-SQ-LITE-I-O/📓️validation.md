# Validation

## Initial Red Run

Ran `bun nx run @semio-tech/framework-rs:test-snapshot-sqlite` with `SEMIO_TEST_ARTIFACT_DIR` set to this ticket's generated folder. Nx launched the registered task and its existing three generation prerequisites. Bun reached the new shared-corpus unit test and failed because the TypeScript codec had not yet been implemented (`Cannot find module '../../🟦️.ts'`, 0 pass, 1 fail). This confirms the test precedes the implementation and that task/launch routing is in place. The Rust and cross-language stages were not reached.

## Environment

Client date rolled over to 2026-10-01 during the work. Ticket remains in its original 2026-09-30 folder. Concurrent unrelated edits are present throughout the workspace; only task-specific hunks are authored or audited. No modifying Git commands or worktrees are used.

## First Complete Codec Run

Ran the same registered Nx target after the codecs were implemented. Result: 26 TypeScript unit tests passed (178 assertions), 4 native Rust unit tests passed under Nextest, and 15 cross-language interoperability tests passed (520 assertions). The native oracle executable was built and actually invoked. Each shared vector was exported by Rust and TypeScript, queried with independent Bun SQLite including `PRAGMA integrity_check`, imported by the other language, and independently authored by SQLite at page sizes 512, 1024, 4096 and 65536 with a displaced table root. Native wrong-dialect import was rejected. One oracle unused-qualification compiler warning was assigned back for repair. This run does not yet verify I/O registry/assembly integration, which is still being implemented.
