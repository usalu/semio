# Phase 0 Gate Report — Observability & Dependency Freeze

Baseline commit `95b8688ee2f62f4056b6403c282bf0c76172c37c`. Host: 10 logical cores ⇒ `cpu_worker_count = 9`.

## Gate status: MET

| Deliverable | Status | Evidence |
| --- | --- | --- |
| Tracing module | done | `semio-framework-trace` at `🧰️framework/🔨️modules/⏱️trace/`; `cargo check -p semio-framework-trace` exit 0 (verified by coordinator, not only by the implementing agent); 12/12 unit tests; native + wasm32-unknown-unknown + wasm32-wasip2 clean |
| Forbidden-call audit | done | `bun ./📜️script.ts verify interactivity` — WARN severity, one-line flip to DENY |
| Dependency freeze | done | `bun ./📜️script.ts verify dependencies` + committed baseline `🔒️dependencies.json`; ratchet failure-mode was actually exercised |
| Dependency inventory | done | `📓️p0-inventory-dependencies.md` — all 238 |
| Command inventory | done | `📓️p0-inventory-commands.md` + `…-part2.md` — ~357 commands |
| Thread census | done | `📓️p0-inventory-threads.md` — 28 sites |
| Async census | done | `📓️p0-inventory-async.md` + `🔧️async-census.json` — 53,338 classified |

## Headline numbers

**Async surface (the dominant finding).** 53,338 classified `async fn` (54,601 raw `async fn ` occurrences repo-wide against only 24,734 `.await` occurrences):

| Class | Count | Share |
| --- | ---: | ---: |
| A — genuinely suspending | 6,251 | 11.72% |
| A-shallow — awaits only non-suspending callees | 883 | 1.66% |
| B — decorative, simple | 39,796 | 74.61% |
| C — decorative + long-running CPU work | 6,408 | 12.01% |
| D — unparseable | 0 | 0.00% |

**88.28% of the async surface is effectively non-suspending.** 1,085 category-C functions sit in the four simulation hot spots (FEM, Energy, Puzzle, Procedural/WFC); the top 100 by body size are ranked in `top100_hotspot_C.json` / `top100_table.md`. Largest single offender: Energy `advance_timestep`, 307 lines with `for` + `.map()`.

**Interactive commands.** ~357 total (~180 in part 1: puzzle 2D/3D, procedural, cad, draw, space; 177 in part 2: FEM 37, Process 31, Block 24, Layout 21, framework 21, Animate 17, Sourcing 14, VCS 10, Energy 2).

**Dependencies.** 238 unique third-party (134 JS, 104 Rust). **Zero genuinely unused** — the earlier "5 unused" claim (criterion, neo4rs, rusqlite, spade, sqlx) was retracted after direct reference counts disproved it (rusqlite 84+, sqlx 56+, neo4rs 29+ references).

**Threads.** 28 creation sites (14 `std::thread::spawn`, 7 `thread::Builder`, 3 `tokio::runtime::Builder`, 4 Web Workers). On a 10-core native interactive session: 15 deterministic threads (3 I/O, 1 compute, 5 shard executors, 5 shard forwarders, 1 epoch ticker) **plus unbounded ad-hoc threads** (DB actors, store-sync actors, submit bridges, HTTP range fetches).

**Audit first run.** 180 findings: 122 blocking-bridge, 36 sync-filesystem, 10 thread-pool construction, 6 sync-clipboard, 6 sync-process, 0 sync-net/db. **121 of the blocking-bridge findings are not allow-listed** — mostly plugin geometry-kernel bridges and `pollster::block_on` in the OS renderer's wgpu glue.

## Findings that change the plan

1. **De-async must be a codemod, not hand edits.** At ~46k B/C functions, per-file agent editing is not viable. `🔧️async-census.json` (one record per function, with body extents) is the codemod driver. Phases 6–7 must budget a mechanical conversion pass with compiler-verified, span-keyed edits — never name-keyed, which silently hits unrelated production code.
2. **`AGENTS.md:44` mandates the async style** — verbatim: "You SHOULD implement everything async when it makes sense." AGENTS.md must not be edited. The refactor narrows "when it makes sense" to *genuine suspension only*; that narrowing is recorded here and in the master ticket rather than in AGENTS.md.
3. **Phase 1 scope is larger than planned.** Beyond the four known pools, there are ad-hoc per-actor DB threads, a **per-document embedded tokio runtime** in store/sync, submit bridges and HTTP fetch threads. Five sites need genuine restructuring rather than a call-site swap: `ShardExecutor` (5 ms poll loop), shard outcome forwarders (250 ms poll loop), DB actor threads, store-sync actor (per-doc thread + embedded runtime), and the epoch ticker (1 ms poll loop, needs a wasmtime callback redesign — which interacts with the owned-interpreter decision).
4. **`ThreadBudget::checkout` wraps silently in release** (`⏳️async/…/🦀️component.rs` ~line 359) — `debug_assert!` only. Confirms the checked-permit-ledger requirement.
5. **Three allow-list entries needed correction** versus the master plan's stated paths; the audit config holds the corrected records.

## Verification commands

```
cargo check -p semio-framework-trace
bun ./📜️script.ts verify interactivity
bun ./📜️script.ts verify dependencies
bun nx run @semio-tech/framework-trace-rs:test
```

## 2026-08-21 current-tree reverification

| Command | Result |
| --- | --- |
| `bun nx run @semio-tech/framework-trace-rs:test-quick --skip-nx-cache` | PASS — 13/13 debug |
| `bun nx run @semio-tech/framework-trace-rs:test-long --skip-nx-cache -- --release` | PASS — 12/12 release; the deliberate overrun test remains quick/debug-only |
| `cargo clippy -p semio-framework-trace --all-targets -- -D warnings` | PASS |
| `cargo rustc -p semio-framework-trace --lib --target wasm32-unknown-unknown -- -D warnings` | PASS |
| `cargo rustc -p semio-framework-trace --lib --target wasm32-wasip2 -- -D warnings` | PASS |
| `bun ./📜️script.ts verify interactivity` | PASS in DENY mode; one approved entrypoint bridge remains |
| `bun ./📜️script.ts verify dependencies` | PASS — 209 current versus 238 baseline; no additions |

## Deliverable index

Phase folder: `📓️p0a-trace-module.md`, `📓️p0b-guardrails.md`, `📝️p0b-audit-baseline.txt`, `📝️p0b-freeze-baseline.txt`, this report.
Master folder: `📓️codebase-map.md`, `📓️p0-inventory-async.md`, `📓️p0-inventory-commands.md`, `📓️p0-inventory-commands-part2.md`, `📓️p0-inventory-dependencies.md`, `📓️p0-inventory-threads.md`, `🔧️async-census.ts`, `🔧️async-census-selftest.ts`, `🔧️async-census.json`, `🔧️async-census-summary.json`, `top100_hotspot_C.json`, `top100_table.md`.
Repo root: `🔒️dependencies.json` (committed freeze baseline).
