# Result CENSUS5-C (prefix `c5c`)

Executor Sonnet 5.5, foreground, no sub-agents, no modifying git command, no ticket tool. `F` = this folder. `PL` = `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin`. Tools: `F/c5c-tools/` (descendant of `c5b-tools`). Logs: `F/🗑️generated/c5c-*` (nothing deleted). Sandboxes: `/Users/ueli/.cache/c5c/`.

## 0. Verdict (IN PROGRESS, started 2026-10-17 23:2x)

- Packages 7, 8, 9 of `MONOLITH-CENSUS-5.md` section 4 (14 files, 21,036 live lines). One package at a time. Nothing edited yet.

| Package | Files | Lines | Verdict |
| --- | ---: | ---: | --- |
| 7 (layout editor engine) | 5 | 8,729 | in progress |
| 8 (plugin framework) | 6 | 7,203 | pending |
| 9 (directory client, dsl derive) | 3 | 5,104 | pending |

## 1. Found state (fresh slice)

- No earlier c5c work. Snapshots: `F/c5c-tools/orig/<tag>.rs`, hashes in `orig/SHA256`. Tag table: `F/c5c-tools/files.tsv`. Crate table: `F/c5c-tools/crates.json`.
- Re-measured: all 14 files are above 1,000 lines (1,428, 1,456, 2,524, 1,322, 1,999 | 1,336, 1,728, 1,008, 1,021, 1,097, 1,013 | 2,404, 1,295, 1,405), none under a `🧊️wgpu` tree.
- Baseline census of the owning crates (lib and cfg(test) closures): `F/🗑️generated/c5c-census-own-before.jsonl`. Pre-edit copy of the layout artifact for the layout gate: `/Users/ueli/.cache/c5c/pre-layout/`.
