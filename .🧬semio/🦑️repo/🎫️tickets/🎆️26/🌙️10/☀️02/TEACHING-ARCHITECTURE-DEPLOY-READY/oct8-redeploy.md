# Teaching architecture redeploy — 2026-10-08

Ticket `2026/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY`, reopened on checkpoint 676 (`18782fa4b19`). Goal `🎯runningframework🎯runningproducts`.

## What failed on the current tree

`bun nx run @teaching/architecture-quiz:test -- quick` first reported 5 failures in 3 files (247 tests).

| Failure | Cause | Change |
|---|---|---|
| catalog, physics and demand: `property-unknown` at `/short` | the contract and the Rust struct already allow a short title on a quiz and a catalog; the TypeScript checker still treated it as unknown, and the Rust checker did not bound its length | both checkers accept `short` and bound each language to 40 code points |
| deploy drift seed for `package.json` | root `package.json` no longer names `@teaching/architecture-quiz:docker-stack-check`, so the replace seeded nothing | the seed appends an unknown target name, which the checker reports |
| dev-proctor source roots | `cargo metadata --offline` in `🎓️teaching` had to download `rsqlite-vfs` (pulled in by `sqlite-wasm-rs`) | `cargo fetch --locked` on that workspace; the checker stays offline |

Rust constructors of `Quiz` and `Catalog` in the quiz tests now set `short: None`, and a Rust unit test covers a valid short title and one of 41 code points.

## Gates so far

| Gate | Result |
|---|---|
| `@teaching/architecture-quiz:test -- quick` | 5 files, 247 tests passed |
| `semio-framework-quiz` library tests | 192 passed, including the new short-title case |
| `@semio-tech/quiz:test -- quick` | 11 files, 451 tests passed |
| `@teaching/architecture-quiz:typecheck` | passed (20m 9s; `framework-rs:generate` 16m 17s, then `tsc --noEmit`) |
| `@teaching/proctor:test-quick` | 126 passed (90 unit, 15 conformance, 21 end-to-end), 0 failed |
| Docker engine | Desktop 4.93.0, engine 29.8.1, linux/amd64, answering |
