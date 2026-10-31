# Proctor Backend Verification

Date: 2026-10-30. Workspace: `/home/deck/semio`. Capacity load gate was not run.

## Packages and how the proctor is built and tested

From `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts`:

- `PROCTOR_PACKAGE` = `teaching-proctor` (binary crate).
- `PROCTOR_LIBRARY_PACKAGES` = `teaching-proctor-base`, `teaching-proctor-domain`.
- `TEACHING_WORKSPACE` = `🎓️teaching`. That workspace's members are exactly those three crates (`🎓️teaching/Cargo.toml`). The capacity gate is a separate Bun process (`repo run @teaching/proctor:capacity`), not a Cargo test.

`proctorBuildPlan` compiles with Cargo arguments `--package teaching-proctor --bin proctor --profile <mode>`. The task router `📦️packages/🦀️rust/📜️script.ts` runs unit, conformance, and end-to-end tests through `runRepositoryCargoTests` on `[PROCTOR_PACKAGE, ...PROCTOR_LIBRARY_PACKAGES]`. `check` and `health` resolve the cached product (they depend on `build`) and run a private copy: `proctor check <course>` and `proctor health`.

## Cargo tests

Command, cwd `🎓️teaching`:

`repo cargo --store=shared --configuration=release --parallelism=1 -- test --workspace -- --test-threads=1`

Exit 0. Wall time 10m 22s. Release compile finished in 9m 14s. `--workspace` covered the three proctor crates only.

| Target | Result |
| --- | --- |
| `teaching-proctor` lib | 25 passed, 1.55s |
| `teaching-proctor` bin | 0 tests |
| `conformance` | 15 passed, 0.28s |
| `end_to_end` | 21 passed, 5.95s |
| `teaching-proctor-base` lib | 41 passed, 1.50s |
| `teaching-proctor-domain` lib | 28 passed, 0.20s |
| doc-tests (all three crates) | 0 tests |

130 passed, 0 failed, 0 ignored.

One compiler warning, tests still passed: unused import `answer` in `🎓️teaching/🛂️proctor/🔨️modules/🔭️projections/🧪️tests/🔬️unit/🦀️.rs:1`.

## `proctor check`

`repo run @teaching/proctor:check` runs with cwd `🎓️teaching/🛂️proctor/📦️packages/🦀️rust`, so a repo-relative course path is joined onto that directory.

A first run passed the course as a repo-relative path. The proctor resolved it under the package directory and exited 1:

`cannot read .../📦️packages/🦀️rust/🎓️teaching/🏛️architecture/📚️course/🔣️.json: No such file or directory (os error 2)`

The verification run passed the absolute course file and exited 0 (Nx run 48.7s, wall time about 2m 55s, configuration prototype). The course was accepted:

- course `architecture`, 10 quizzes, 13 badges
- fingerprint `95b3baa21f3b49f1f1d42813bc4534f0b3650358b4acd2e1c509f186db04c0ef`

| Quiz | Tasks | Revision |
| --- | --- | --- |
| physics | 3 | `1795e5c9491d9f46f5bd591e4e10d45faea6e13188a61522e913dc811a5bae26` |
| ventilation | 3 | `9f256c7b64ca73c124633d320d76632424ec562713546ee6fa9abf53fe46762d` |
| heating | 2 | `db04b89a6218b9296f2f129290c0d675562cac470f7b46e8d30749e2ffc5a319` |
| cooling | 1 | `82320ffc1602ca2f2c29bdf768c59ab490acfd4c68a9469329e3c00350ba55c4` |
| demand | 2 | `73da95631f43d2034bc2911178e50575ad66c2cc1de47b3a4d1364d8c23b1f4c` |
| parametric-design | 5 | `40be98ea56a0e9d55b3968d1e55202af3ece0d3843cf8791710c9a6264526af4` |
| grasshopper-data | 4 | `18678a5233dd518e642192907ce962c44fa2c8d3795a09794b04bbe73b25f44a` |
| excel-workbook | 3 | `4c21b134d7b85bf86df793bd42bb1e657ba59be04dc8dbfeaf6d5aff79a860d7` |
| bim-concepts | 3 | `998144b6886fe62328c435a9114b17095bddd95e290a28a2567046c991a2096c` |
| optimization-decisions | 2 | `eeecceb730728b554cbab12fe6fc37642e4cea3191cb233bc67d7060bededd41` |

## `proctor health`

`repo run @teaching/proctor:health` exited 0 (Nx: build 1m 2s, health 15.7s, configuration prototype).

`proctor health` opens a TCP connection to the listener of this environment (`PROCTOR_PORT`, default `8791`, loopback when the bind is unspecified) and sends `GET /instance` with `X-Forwarded-Proto: https`. Exit 0 means the status line was `200`.

Before that command, `127.0.0.1:8791` was already listening (`proctor-66127-b`, pid 497520). `GET /instance` returned `id` `teaching-proctor`, `version` `0.1.0`, module `teaching.proctor`.

## Outcome

Release workspace tests passed (130 tests). The architecture course checks clean when the course path is absolute. The live dev proctor on `127.0.0.1:8791` is healthy. The capacity gate was left out.