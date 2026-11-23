# 🧪️ TypeScript Sourcing Test Suites Run (2026-11)

Repo: `/Users/ueli/Documents/semio` (shared live worktree, no git writes, no source edits). Bun 1.4.2 from `~/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun`. Raw outputs: `🗑️generated/`.

## Summary

| Test file | Result | Raw output |
|---|---|---|
| `✏️s/🔌️plugins/🪵️sourcing/🧪️tests/🧭️dev-route/🟦️.ts` | 0 pass, 1 fail, 1 error (file never ran its 3 tests) | `🗑️generated/bun-file-1.txt` |
| `…/✏️editor/📚️examples/🎬️demo-session/🧪️tests/🧩️example/🟦️.ts` | 1 pass | `🗑️generated/bun-file-2.txt` |
| `…/📚️examples/🎬️demo/🧪️tests/🧩️example/🟦️.ts` | 1 pass | `🗑️generated/bun-file-3.txt` |
| `…/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts` | 49 pass | `🗑️generated/bun-file-4.txt` |
| `…/🧬️schema/🧪️tests/🪪️document-contract/🟦️.ts` | 0 tests (plain script, top-level asserts, exit 0) | `🗑️generated/bun-file-5.txt`, `bun-document-contract-run.txt` |
| `…/🗂️curation/🧪️tests/🔬️rust-package-commands/🟦️.ts` | 0 tests (plain script, exit 0) | `🗑️generated/bun-rust-pkg.txt` |

Combined run of the three named targets: 2 pass, 1 fail, 1 error, exit 1 (the 2 passes are the two demo files; dev-route aborts the run).

Totals across all sourcing test files: 51 pass, 1 fail, 1 error. No `*.test.ts(x)` files exist under `🌎️hub/🧩️compositions/🪵️sourcing` (only `📦️packages/🦀️rust/📜️script.ts`, which is not a test). No vitest files were found, so `vitest run` was not needed.

## Failure: dev-route (`🧭️dev-route/🟦️.ts`)

Root cause is a broken import in the working tree, not the catalog:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️load/🧭️registry-follow/🟦️.ts:6` imports `createReadTakenMemoV1` from `../♻️pass-reuse/🟦️.ts`.
- `…/📥️load/♻️pass-reuse/🟦️.ts` exports only `onGraphPassNoticeV1`, `emitGraphPassNoticeV1`, `createPassReuseV1`. It has no `createReadTakenMemoV1`.
- `registry-follow/🟦️.ts:80` calls `createReadTakenMemoV1()`.
- The HEAD version of `registry-follow` has no `TakenMemo` reference at all, so this is an uncommitted in-flight edit in the shared worktree (another developer's change to one or both files).
- Bun fails at module link time: `SyntaxError: Export named 'createReadTakenMemoV1' not found`. Because the failure occurs during import, none of the 3 tests execute, so the expected catalog failure in test 2 (`🧑‍💻dev/🧩️catalog/🤖️generated/🎠️playgrounds.json`, not yet regenerated) could not be observed. Fix the import/export pair, then re-run to see the real test 2 result.

Fix location to act on: add `export function createReadTakenMemoV1` to `…/♻️pass-reuse/🟦️.ts`, or correct the import in `registry-follow/🟦️.ts:6`, whichever the owner of that in-flight change intended.

## Other failures

None. The sqlite snapshot suite (49 tests) passes.

## Not run / notes

- `document-contract` and `rust-package-commands` contain no registered tests; they were executed as scripts (`bun run`), exit 0, so their top-level assertions held.
- No source files were edited and no git write commands were run.
