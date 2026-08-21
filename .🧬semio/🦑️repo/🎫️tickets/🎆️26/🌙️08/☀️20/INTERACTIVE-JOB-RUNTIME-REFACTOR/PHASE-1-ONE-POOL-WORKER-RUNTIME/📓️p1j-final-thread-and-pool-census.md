# P1j Final Thread and Pool Census

## 2026-08-21 final production thread-owner update

The process-transport reader pair has now been replaced by bounded nonblocking pipe cursors (see `📓️p1k-nonblocking-process-transport.md`). A fresh `bun nx run workspace:verify-interactivity` reports **zero `thread-pool` findings**. The audit is at 53 total: 13 blocking bridges, 4 clipboard, 31 filesystem, and 5 process findings, all assigned to the stdio/renderer and Phase 3 UI-isolation packets. The literal static production thread-owner portion of the Phase 1 gate is now met: the scanner sees no subsystem thread or pool creation outside the sanctioned process-wide WorkerPool root.

Date: 2026-08-21

## Outcome

The repo dashboard daemon's unbounded per-client OS-thread path is removed. Accepted Unix-domain clients are now persistent nonblocking connection cursors driven by the daemon's existing finite tick. Each connection turn reads at most 64 KiB and decodes at most 32 frames. Fragmented frames remain buffered, malformed or disconnected clients are isolated, and stable client IDs keep read and write halves paired when either half fails.

The neural engine's default Rayon path is also removed. Its topological evaluator now uses the existing deterministic sequential path; there is no Semio production source use of Rayon or a Tokio multi-thread runtime. Parallel neural evaluation may only be reintroduced as explicit jobs on `WorkerPool`.

This packet narrows, but does not claim to close, the strongest Phase 1 process-thread gate. The two registered blocking child-pipe readers in plugin process transport remain production OS threads. They require the owned platform I/O reactor before the literal `UI thread + WorkerPool workers` invariant is true.

## Bounded daemon cursor

- `ipc::try_decode_frame` incrementally decodes a complete length-prefixed frame without performing a blocking read.
- The 16 MiB protocol maximum is shared by blocking and incremental decoders; an oversized prefix fails before body allocation.
- `ClientReader::turn` bounds reads to 64 KiB and decoded frames to 32 per daemon tick.
- One malformed client no longer terminates the daemon and client count no longer changes OS-thread cardinality.
- The supervisor assigns checked stable client IDs, removes a writer when its reader closes, and removes a reader if a broadcast already discovered its writer was dead.

## Verification

Passing:

```text
bun nx run @semio-tech/repo-cli-rs:test-quick
24 tests run: 24 passed, 0 skipped
```

The new focused tests cover fragmented and concatenated frames, oversized-prefix rejection, and a real Unix-domain daemon/client attach-and-ping exchange through the nonblocking connection cursor. The existing supervisor attach/ping/event-log test remains green.

Attempted latest-tree kernel gate:

```text
bun nx run @semio-tech/framework-os-kernel:test-quick
blocked by 468 pre-existing Phase 1.5 de-async test-target diagnostics
```

The diagnostics are stale `.await` and signature mismatches in pack/DSL/stdio test code; none names the neural engine change. This gate must be rerun after Phase 1.5 reaches zero.

## Exact static census

The raw literal census is retained in `📝️p1j-thread-census.txt`. It has 38 matches across the entire repository, including the new end-to-end daemon test's one test-harness thread, compose, fixtures, documentation, and tests.

Production classifications outside `compose`:

- `⏳️async/🦀️component.rs`: one intended `WorkerPool` worker constructor.
- plugin `process-transport/🦀️component.rs`: two registered blocking child-pipe I/O boundary readers; strict gate blocker.
- repo CLI daemon: zero production thread constructors after this packet.
- renderer kernel: zero production thread constructors after the concurrent renderer packet; its remaining snapshot/Shell matches are tests.
- procedural WFC: zero production thread constructors after its scoped-thread removal.

The implicit-pool census is retained in `📝️p1j-implicit-pool-census.txt`. Its only remaining match is the `asyncprobe` host-turn fixture's Tokio runtime. Production Rayon source usage is zero; production Tokio multi-thread runtime builders are zero.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust/📦️glue.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust/Cargo.toml`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/🦀️component.rs`

## Scratch policy

Both census outputs are `.txt` files inside this Phase 1 ticket. No permanent script was added.

The strengthened Nx-wired audit output is `📝️p1j-interactivity-audit.txt`: it excludes build/test/fixture/ticket sources, masks nested block comments, detects `thread::Builder`, scoped threads, and all direct Rayon source use, and reports exactly the two process-transport production thread constructors. It remains in warn mode because the wider Phase 3/8 UI-blocking inventory is not yet zero.
