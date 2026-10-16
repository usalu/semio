# Context Execution Independent Audit

Read-only Source audit on 2026-10-16. No observer, compiler, image, download, SDK, maintenance, retirement, test or physical guard operation was executed. Only this report was written. Scope is the single prototype `build-mode/📜️script.ts cargo prototype` selected by `SEMIO_PROFILE_CONTEXT_ONLY=1`.

## Concrete Cleanup Blocker

The actual context observer creates `directory = await mkdtemp(...)` and then constructs `database = new Database(":memory:")` before entering `settleCargoContextObservationV1`. If the synchronous SQLite constructor refuses, the exact newly created `cx-*` directory has no registered removal path. This is a small but concrete violation of exact owned temporary cleanup on every refusal. Establish directory cleanup immediately after successful mkdtemp, with database cleanup conditional on successful construction. No runtime failure is asserted here.

## Source Findings

The wrapper creates one fresh process envelope using Date.now with an authored 120000 ms maximum and offline capability. It captures the selected Bun test child through the original compiler admission control with 1 MiB/65536-line output. This Source construction does not establish that a future launched Original is fresh or that an earlier process is terminal.

The observer uses the actual `(store, control)` callback, not store.control, for writer admission, capacity, children, descriptors, inventory and continuations. The store preserves the private reservation binding. The compiler owner opens the image with an outer control, obtains reservation custody, runs the body in a canonical child, drains that canonical child and writer operations, then closes the image while its opening phase remains live. Canonical settle checks remaining control after drain. The observer awaits its complete writer operation; its body has no intentionally detached child work.

The compiler writer resource is acquired once around the full fixture operation. Its same-root reentry uses AsyncLocalStorage and refinement rather than reacquiring the same EX lock. Capacity CPU1 is held only around each actual Cargo compilation and released in finally. Unit-graph and lockfile queries remain real Cargo children outside that CPU reservation; they are control-bounded, but CPU1 is not an OS-wide hard CPU limit on those commands or Python. No Source-only proof excludes stale external locks or contention on tools/Rust/provider resources.

The toolchain is the actual tools-store privately issued immutable Rustup generation, with exact executable paths, source pin checks and generation custody. The Source path may attempt bootstrap/provisioning if missing. Offline authority must produce a terminal refusal when required assets are absent; do not substitute PATH Cargo, fake store or neutral registration authority. No installed-generation or executable availability was observed in this audit.

The five authored phases are cold, unchanged, root-body, leaf-body and context; they use two ordinary rlib crates. Source edits are capped at 16 KiB; descriptor reads and final inventory use 2 MiB; inventory additionally bounds 4096 files and 8192 entries, checks control, rejects symlinks/unknown kinds and settles directory closure. Per-command and retained record output is bounded. These inventory checks run after Cargo produces its files: they are observation/refusal bounds, not a 2 MiB live filesystem quota. Physical containment remains the real funded Rust image, unchanged Rust16 envelope and initial 4 GiB compiler allocation/headroom admission.

Cleanup inside the established settlement scope awaits database close and exact recursive fixture removal, attempting both even after a primary failure. The setup gap above precedes that scope. Output, unsupported Cargo behavior, expired control, inventory overflow or cleanup refusal must remain terminal; no retry is qualified by this audit.

## Runtime Gates Remain Open

Root reports nothing physical queued; this audit created no child process handle requiring later physical settlement. The supplied host df value of 115,633,732 KiB says nothing about free bytes inside the actual Rust backing. Actual provider opening, original guard custody, real Rust statfs initial-floor admission, current sealed toolchain, Python availability, five-phase compiler behavior, bounded complete output and terminal exact cleanup remain unobserved. A physical attempt requires Root scheduling and independently established terminal predecessor/child facts. Source review and prior cheap laws are not runtime evidence.
