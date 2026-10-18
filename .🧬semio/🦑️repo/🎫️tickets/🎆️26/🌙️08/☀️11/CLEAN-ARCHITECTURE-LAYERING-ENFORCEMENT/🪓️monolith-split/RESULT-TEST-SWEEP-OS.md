# TEST-SWEEP-OS (`tso`): baseline, buckets, fixes

Status: IN PROGRESS (baseline recorded; fixes landing). Scope: `🧰️framework/🛍️products/💻️os/**` excluding `📺️renderer/**`.
Tools: `F/tso-tools/` (`vitest.sh`, `vtall.sh`, `bunall.sh`, `vtsum.py`, `vtfail.py`). Logs: `F/🗑️generated/tso-*`.

## 0. Verdict

(updated at the end)

## 1. Baseline (2026-10-17 22:2x, before any edit)

| Suite | Runner / config | Result |
|---|---|---|
| os (`@semio-tech/framework-os`) | vitest, `os/🧪️tests/🎚️config/🟦️.ts` (node) | 35 files, 727 tests: 613 passed, **114 failed** (identical to `RESULT-APP-CHANNEL-DECOMPOSE.md`) |
| mcp | vitest `mcp/🧪️tests/🎚️config` | 8 files: 3 passed, 5 failed (28 passed, 5 failed tests) |
| plugin registry | vitest `plugin/📇️registry/🧪️tests/🎚️config` | 21 files: 4 passed, 17 failed (49 passed, 52 failed tests) |
| registry composition | vitest `…/🧩️composition/🎚️config` | 8 files: 4 passed, 4 failed (156 passed, 9 failed tests) |
| shell | vitest `shell/🧪️tests/🎚️config` | 2 files: 1 passed, 1 failed (6 passed, 1 failed) |
| dev | vitest `dev/🧪️tests/🎚️config` (node) | 19 files: 11 passed, 8 failed (183 passed, 37 failed, 2 skipped) |
| bun test (216 `bun:test` files, one process per file) | `F/tso-tools/bunall.sh` | see section 3 |

## 2. Bun baseline (216 `bun:test` files, one process per file, before any edit; no `SEMIO_TEST_ARTIFACT_DIR`)

155 files exit 0, 61 files non-zero (6 Bun 1.3.14 segfaults `Segmentation fault at address 0x80xx`, 2 hang past the 180 s alarm); 864 tests passed, 182 failed. Several of the 61 failed only because the scripts' harness sets `SEMIO_TEST_ARTIFACT_DIR`/`SEMIO_TICKET_DIR` and the bare run does not; `tso-tools/env.sh` and `bt.sh` now export both. Per-file table: `F/🗑️generated/tso-bunall-base.tsv`.

## 3. Interim log (IN PROGRESS, final tables at the end)

Done so far (each re-run green): see section 5 once written. Work order: os vitest (done), registry/dev/mcp/shell vitest configs, then the Rust-source-text readers in `bun:test` files.
