# Final verification sweep (2026-11-28)

Scope: non-Rust checks of the sourcing plugin on the current tree. No product code edited, no modifying git commands, no processes killed.

## 1. TypeScript tests and typecheck

- `repo run @semio-tech/sourcing-js:test` failed before running anything: `Internal error: Closed host capacity Grant required: /format is not allowed`. Log: `🗑️generated/sweep-pkgtest.log` (first attempt, overwritten by fallback).
- Native fallback (the test script itself runs `bun test` on these files): the bun binary pinned by `repo tools list` (bun 1.4.2, `.cache/b/t/m/bun/90987a3a.../bun`) was called directly. Bun is not on PATH.
- `bun test` needs an explicit `./` prefix on the file path; without it bun treats the path as a name filter and runs nothing.

| File | Pass | Fail |
|---|---|---|
| `✏️s/🔌️plugins/🪵️sourcing/🧪️tests/🧭️dev-route/🟦️.ts` | 3 | 0 |
| sourcing `demo-session/🧪️tests/🧩️example/🟦️.ts` | 1 | 0 |
| sourcing `demo/🧪️tests/🧩️example/🟦️.ts` | 1 | 0 |
| sourcing `🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 49 | 0 |
| **Total** | **54** | **0** |

Matches the 54 passing from the earlier run. Logs: `🗑️generated/sweep-pkgtest.log`, `🗑️generated/sweep-sqlite-bun.log`.

Typecheck: `repo typecheck` works (not blocked). Run on the script plus 4 touched test files: `passed, 5 files (0 importers), 31 errors elsewhere not counted`. Exit 0. Log: `🗑️generated/sweep-typecheck.log`. The 31 errors sit outside the touched files and are not counted by the tool. They were not investigated here.

## 2. Python oracles

`repo test-file` on the three curation oracles, exit 0: 3 passed, 0 failed, 0 skipped. The locked Python env was already in place, so `repo dependencies ensure` was not needed. Log: `🗑️generated/sweep-oracles.log`.

- `🚪️io-curation-1/🐍️.py`: passed (sqlite 19 semantic-cells cases; Rust codec re-read skipped because `SEMIO_CURATION_IO_OUT` is unset).
- `🧮️stock-curation-1/🐍️.py`: passed.
- `🗂️mutate-curation-1/🐍️.py`: passed.

## 3. Journey-kit and surface TS tests

All run with the pinned bun. Logs: `🗑️generated/sweep-surface-bun.log`.

| File | Pass | Fail |
|---|---|---|
| `🧰️framework/.../🌐️World3dHost/🧪️tests/🧩️component/left-press-orbit.test.ts` | 4 | 0 |
| `🧰️framework/.../📺️renderer/🧑‍🎨engine/🧪️tests/🏁️operation-fault-settle/🟦️.ts` | 2 | 0 |
| `🧰️framework/.../🔌️PluginRuntime/📬️typed-result/fault-notices.test.ts` | 15 | 0 |
| **Total** | **21** | **0** |

## Failures

None.

## Open

- `repo run` for the sourcing test target and `repo show project` are blocked by the host capacity grant (`/format is not allowed`). Retry once capacity clears. The equivalent files were run directly above.
- The pinned bun is not on the shell PATH. The `repo` tooling reaches it through its own cache path.
- The 31 typecheck errors outside the touched set are unexamined.
- The Rust side (cargo) was out of scope.
