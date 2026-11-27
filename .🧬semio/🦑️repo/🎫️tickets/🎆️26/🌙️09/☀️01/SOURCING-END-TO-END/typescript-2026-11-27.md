# TypeScript agent report (2026-11-27)

## Ran (`repo test-file`, log `🗑️generated/ts-agent-testfile.log`)
- `🧪️tests/🧭️dev-route/🟦️.ts`: 3 pass, 0 fail (earlier `createReadTakenMemoV1` link error is gone, peers fixed it).
- curation `demo/🧩️example`: 1 pass; `demo-session/🧩️example`: 1 pass; `🪶️sqlite/📸️snapshot`: 49 pass.
- `🪪️document-contract` and `🔬️rust-package-commands`: 0 registered tests, exit 0 (top-level asserts hold).
- `🗂️mutate-curation-1/🐍️.py`: not a standalone oracle; `repo test-file` says "no entry point and no oracle above it imports it"; it needs the `semio_repo_test` harness, so it runs only through its owning Rust/oracle case (other agents' scope).
- Total: 54 pass, 0 fail.

## Open
- `repo run @semio-tech/sourcing-js:test` (script prints only "sourcing ts ok") and `repo typecheck` over sourcing files: started in background (`ts-agent-pkgtest.log`, `ts-agent-typecheck.log`), stuck >5 min in host capacity queue (shared memory contention) after an earlier "replica not settled" retry loop (partition-census module, now loads fine).
- Launch entry `sourcing-react` -> `@semio-tech/framework-os-dev:serve-sourcing-react-prototype`: no static target of that name in the dev project.json (only `serve-hold`; serve targets are likely generated/dynamic), and `repo run ... --dry_run` queued on capacity without resolving. Not resolved; coordinator uses `serve-sourcing-native.ts` on :6081 (pid 97632 observed listening, untouched).

## Fixes
None needed; no files edited. Note: my first `repo run ...serve-sourcing-react-prototype --dry_run` was stopped before it started anything.
