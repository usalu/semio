STATUS: GREEN (non-cargo rerun after semiodsl stdio-semio pack changes). TS codec laws 13/13, Python codec laws 13/13, 13/13 Python adapter identity round-trips PASS against the 39 committed fixtures. No byte differences, no fixture changes after 2026-10-29 19:30.

# Audit: codecrerun (wave 1029)

Scope: non-cargo checks only, as in `1029-result-semiocodec.md`. No source edits, no cargo, no sub-agents.
Root: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/`.

## Results

| # | Item | Command (run from root) | Result |
|---|------|-------------------------|--------|
| 1 | TS codec laws | `bun test ./🧪️tests/🎒️record-document/🟦️.ts` | PASS: 13 pass, 0 fail, 188 expect() calls (4.96 s) |
| 2 | Python codec laws | `python3 -I 🧪️tests/🎒️record-document/🐍️.py` | PASS: Ran 13 tests, OK (82.8 s) |
| 3 | `model` adapter identity | `python3 C/1029-tools/semiocodec/run_suite.py <suite>/🐍️.py` | PASS (1 handler, failed 0) |
| 4 | `table` adapter identity | same | PASS |
| 5 | `cad` adapter identity | same | PASS |
| 6 | `document` adapter identity | same | PASS |
| 7 | `object` adapter identity | same | PASS |
| 8 | `presentation` adapter identity | same | PASS |
| 9 | `value` adapter identity | same | PASS |
| 10 | `text` adapter identity | same | PASS |
| 11 | `image` adapter identity | same | PASS |
| 12 | `graph` adapter identity | same | PASS |
| 13 | `drawing` adapter identity | same | PASS |
| 14 | `brep` adapter identity | same | PASS |
| 15 | `kit` adapter identity | same | PASS |

Adapter outputs were captured per suite (each showed one `PASS identity-round-trip::oracle` line and `handlers run: 1, failed: 0`) and deleted after the audit. Harness: `C/1029-tools/semiocodec/run_suite.py` (stub of the repo test host, not `repo test`).

## Fixture check

- The codec fixture `🧫️fixtures/🔣️.json` lists 39 packs (`packCount: 39`). All 39 exist on disk.
- `stat` mtime of all 39: `2026-10-29 11:59:14`. Count with mtime after `2026-10-29 19:30:00`: 0.
- `git status --porcelain` on the 39 paths: 0 entries (no uncommitted changes).
- No failures, so no byte-level diff was needed.

## Not covered

- Rust `cargo` checks (`record_spec_law`, the stdio-semio pack path) are out of scope.
- The TS mesh adapter identity script (`C/1029-tools/semiocodec/mesh_identity.ts`) was not rerun.
- The 4 Python adapters outside the 13 (flow, animation, video, audio) were not rerun.
- Python ran with `-I` (isolated mode), which the semiocodec run did not use. Results are the same (OK).
