# Status — 2026-11-22 14:15

## Done and verified

- TypeScript: trinity-jack (56), trinity-rewriting (62), trinity-js (4), jack-lsp-worker (31) and the trinity-graph-child composition (6) all pass natively on Windows. See [typescript.md](typescript.md).
- Jack document load: snapshots now carry a required `manifestId` and point to one interned manifest. Before, each snapshot held a full copy of the 42-kind manifest, which made a load take about 8 minutes. Schemas, codecs, fixtures and the Rewriting side are updated, with an Ajv oracle test. See [document-load-performance.md](document-load-performance.md). The load time after the change has not been measured yet.
- `repo` fixes:
  - Sealed reads now skip the capacity and disk check, with a test.
  - A test checks the shipped host-capacity policy against its schema.
  - The cargo provenance recheck is no longer quadratic: about 27 s became 35 ms in a microbenchmark, with linear-cost tests.
  - Error detail now includes every child of an AggregateError, with tests.
  - The descriptor-emitter freshness logic came from another session.

  See [repo-query-hang.md](repo-query-hang.md) and [provenance-verification-performance.md](provenance-verification-performance.md).
- Framework `semio-plugin`, `-instance-render` and `-instance-commands` pass `cargo check --all-targets`.
- Fixes on the Rust side:
  - Jack graph window config: added `store_disposer_birth`.
  - Ingress fixture: added the missing field.
  - Operation wires: three now carry `with_original_target`.
  - Framework: removed a duplicated re-export and fixed the reserved-commit drop.
  - PDF plugin: fixed a borrow error and a lost `pub use`.
  - Theme tokens and graph manifests regenerated.

  See [native-rust.md](native-rust.md) and [framework-plugin-build.md](framework-plugin-build.md).

## In progress (agents)

| Track | State | Expected |
|---|---|---|
| Jack child-edit publication (16 failing dev graph tests) | Fixes written for about 11, rebuild running | 1–3 h (build ~1 h on the loaded host, then iterate on 4 undiagnosed tests) |
| VcsArtifactApp stack size (927 KB inline → box owners) | Working, framework edits landed | 2–3 h |
| Remaining Rust suites: jack-rs, lsp + wasm, shell, rewriting-rs, trinity-plugin, the 3 slow `reorganize` tests | Building | 2–4 h |
| Puzzle 5D after the manifest change | Full run 6; all Puzzle-owned errors fixed so far | 1–2 h, then its tests |
| Browser E2E (s loads Trinity; Jack and Rewriting editor and viewer, query, undo) | Blocked twice by the build pipeline; now `Cargo.lock` is stale and the run is being relaunched on G: | 2–4 h; the largest risk |

Realistic end to end: this evening, if the build pipeline lets the Trinity component build through.

## Problems hit

1. The session opened in `C:\git\semio`, a stale clone where `repo` is broken. All work moved to the live fleet tree `C:\semio`.
2. `C:` filled to 0 bytes three times. Parallel agents' cargo targets grew to 106–120 GB. They are on G: now with `CARGO_INCREMENTAL=0`. `C:` is at 21 GB again, partly from repo-pipeline caches and other sessions.
3. `repo run` and graph sealing are unusable: half-landed modules from other sessions, a dependency sync of another session holding the build lock for over 3 h, and no sealed graph. All work falls back to native cargo and bun.
4. Many sessions edit live, so builds keep failing on other sessions' half-finished edits (about 10 times). Agents wait briefly and retry.
5. The host is saturated: builds that take minutes elsewhere take 1–3.5 h here. `link.exe` cannot handle emoji target paths, so targets are ASCII on G:, and share-generics is off.
6. The Trinity component build is still blocked by the build pipeline: stale emitter, provenance (78 min), the restart-code launcher bug (fixed), a prerequisite AggregateError, and now a stale `Cargo.lock`.

## Anticipated

- More breakage from other sessions mid-edit, which only costs retries.
- The browser run may surface runtime issues the native tests miss, such as WebGPU in headless Playwright, which the audit flagged.
- The 4 undiagnosed child-edit failures may need framework changes.
- Open for the owner: `C:\cargo-dl` (1.7 GB) needs manual deletion, because the safety check blocks it. Another session's idle dependency sync blocks `repo graph` sealing.
