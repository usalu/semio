# 📏️ Wave 7 Crate Source-Gate Census, Re-run 2

Read-only re-run of the wave-7 crate source-gate census (`AUDIT-WAVE7-CRATE-GATE.md`) after the fleet's crate rollout, the test-closure rollout and the reader fix for `include!`-relative module mounts (TEST-CLOSURE-T4). Auditor: Haiku `cgc8`, run on 2026-10-17 from about 14:08 CEST. No source file, git state or cargo build touched; nothing run under cargo or rustc except `rustup which rustc` and `rustc --print cfg`.

## Method

- Same census as `🗑️generated/cgc7-census.ts`, byte-identical except the output prefix (`cgc8`), so the cgc7 baseline outputs were not overwritten. Script: `🗑️generated/cgc8-census.ts`. Reader: the canonical `readRustSourceClosureV1` with the current host cfg, `debug_assertions`, and each crate's enabled features. Limits 32 files, 262,144 bytes, 4,000 active lines, tightened only by a crate's own `rust-source-boundary` metadata.
- Manifest list: `🗑️generated/cgc8-manifests.txt` (3,010 `Cargo.toml` paths, every directory outside hidden dot-directories, `🗑️generated`, `target` and `node_modules`). The baseline list had 2,562 paths. No baseline crate disappeared; every one of the 2,419 baseline crates was measured again. The repo is being changed by the fleet during the run, so counts are a snapshot at about 14:10 to 14:30 CEST.
- Compare: `🗑️generated/cgc8-compare.ts` matches the two runs by manifest path (`cgc7-crate-census.csv` vs `cgc8-crate-census.csv`). Owners are the fleet slices from the last 100 lines of `📓️FLEET.md`.

## Totals: before to after

| Metric | Before (cgc7, 08:25) | After (cgc8, run 14:10 to 14:30) |
| --- | ---: | ---: |
| Workspaces (`[workspace]` roots) | 287 | 287 |
| Crates measured | 2419 | 2866 |
| Workspace members | 2373 | 2820 |
| Non-member crates reached through path dependencies | 46 | 46 |
| Compilable (lib closure within limits) | 1868 | 2454 |
| Compilable (test closure within limits) | 1811 | 2403 |
| Oversized lib | 58 | 8 |
| Oversized test-only | 43 | 1 |
| Oversized lib and test together | 58 | 8 |
| Crates whose closure the reader refuses (any target) | 10 | 0 |
| Path dependencies resolving to a missing directory | 1 | 1 |
| Directories with a manifest but no `[package]` | 144 | 144 |

Read the compile counts with care: the crate population grew by 447 (2866 vs 2419) because the carves created new crates, so `Compilable` rose partly through new, small crates. Every baseline crate was re-measured. The headline reduction is: oversized lib 58 to 8, oversized test-only 43 to 1, reader-refused 10 to 0. Oversized lib roots that blocked dependents fell from 58 to 8.

## Still oversized or refused (9 crates)

Blocked count is the number of crates blocked by this crate (lib or test). The owner is the slice in `📓️FLEET.md` (last 100 lines). `before` is the cgc7 status.

| Crate | Workspace | Kind | Limit and ratio | Blocked (any) | Before | Owner |
| --- | --- | --- | --- | ---: | --- | --- |
| `semio-s-artifact-writer-writer` | `✏️s/🔌️plugins/✒️writer/🗿️artifacts/✒️writer` | lib | files 4.844x (155/32 files, 428871 B, 7483 lines) | 5 | lib 4.844x | r7c |
| `semio-s-artifact-flow-flow` | `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow` | lib | files 4.094x (131/32 files, 530825 B, 8480 lines) | 3 | lib 4.094x | UNOWNED |
| `semio-s-artifact-bim-model` | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model` | lib | files 41.25x (1320/32 files, 6148821 B, 87089 lines) | 3 | lib 40.625x | r9a |
| `semio-s-artifact-trinity-rewriting` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting` | lib | files 5.25x (168/32 files, 505178 B, 7093 lines) | 4 | lib 5.031x | r7b |
| `semio-s-artifact-stdio-pdf` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf` | lib | files 23.5x (752/32 files, 3351126 B, 53114 lines) | 263 | lib 22.563x | r5b |
| `semio-s-artifact-stdio-semio-kit` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` | lib | files 3.469x (111/32 files, 380076 B, 4892 lines) | 341 | lib 3.469x | r3 |
| `semio-s-artifact-puzzle-2d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d` | lib | files 10.813x (346/32 files, 2628259 B, 25353 lines) | 5 | lib 10.625x | r8a |
| `semio-s-artifact-sourcing-curation` | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation` | lib | files 4x (128/32 files, 491278 B, 6943 lines) | 12 | lib 4x | r7c |
| `semio-framework-plugin` | `.` | test-only | bytes 16.14x (380/32 files, 4231119 B, 46591 lines) | 0 | test-only 16.445x | pi2 |

Notes:
- **UNOWNED:** `semio-s-artifact-flow-flow` (lib 4.094x, 3 blocked). The baseline audit records it as a FLEET-CLAIMS wave-4 item, but no slice in the last 100 lines of `📓️FLEET.md` covers it. The R3 slice covers the stdio `semio-flow` crate, not this artifact root.
- The baseline's 43 test-only rows are now down to one (`semio-framework-plugin`, pi2). Its test ratio fell 16.445x to 16.14x.
- No stdio, draw, layout, block, framework, teaching or hub test closure is oversized in this census; only the pi2 row above remains test-only.

## Crates whose status got worse (5)

| Crate | Workspace | Change |
| --- | --- | --- |
| `semio-s-artifact-bim-model` | `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model` | lib ratio 40.625 -> 41.25 |
| `semio-s-artifact-trinity-rewriting` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting` | lib ratio 5.031 -> 5.25; test ratio 8.281 -> 8.531 |
| `semio-s-artifact-stdio-pdf` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf` | lib ratio 22.563 -> 23.5; test ratio 39.156 -> 40.219 |
| `semio-s-artifact-stdio-semio-base-examples` | `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio` | test ok -> blocked |
| `semio-s-artifact-puzzle-2d` | `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/◻️2d` | lib ratio 10.625 -> 10.813; test ratio 36.469 -> 36.656 |

Only one crate changed status. `semio-s-artifact-stdio-semio-base-examples` was test-compilable and is now blocked: its dev-dependency `semio-s-artifact-stdio-semio-snapshot` (testkit feature) reaches `semio-s-artifact-stdio-semio-kit`, which is still oversized (lib 3.469x, R3). It is not oversized itself. The other four rows are still-oversized crates whose ratio rose slightly: `bim-model` (40.625x to 41.25x), `stdio-pdf` (22.563x to 23.5x lib; 39.156x to 40.219x test), `trinity-rewriting` (5.031x to 5.25x lib; 8.281x to 8.531x test) and `puzzle-2d` (10.625x to 10.813x lib; 36.469x to 36.656x test). These are growth in still-failing crates, not new failures. No crate newly exceeds the gate: 0 new oversized crates since the baseline.

## Outputs

- `🗑️generated/cgc8-census.ts`, `cgc8-census.out`, `cgc8-census.err` (run log), `cgc8-manifests.txt`
- `🗑️generated/cgc8-census-summary.json`, `cgc8-crate-census.csv` (one row per crate)
- `🗑️generated/cgc8-compare.ts`, `cgc8-compare.json` (per-crate before/after comparison)
- `🗑️generated/cgc8-extract-baseline.ts`, `cgc8-baseline-oversized.json` (baseline extract)
- `🗑️generated/cgc8-report.ts` (this report renderer)
