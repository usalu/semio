# 🧹️ Hygiene Audit 1023c (read-only)

Auditor `hyg-c`, wave 1023c, 2026-10-23. Read-only: nothing was edited, staged, deleted or stopped except this report. No git-modifying command was run. Scratch intermediates went to the session scratchpad (`/private/tmp/claude-501/-Users-ueli--claude-spares-spare-pSPkwj/ad7098a3-86cf-42c9-85e1-65386eec4277/scratchpad`). The `du` runs that hit the 60 s cap were stopped by their own alarm; no audit process is left running.

## Summary

- Working-tree `[DEBUG] ` lines: 0 unstaged. 6 staged lines in 3 source test files, plus about 80 in ticket scripts and outputs (section 1).
- The three uncommitted `[DEBUG]` lines recorded in earlier result files (`inspect receiver`, `__spinTrack`, `SEMIO_DEBUG_SPIN`) are gone from the tree.
- `C/🗑️generated` is 6.4 GiB: 4.7 GiB in 266 subdirs plus 1.75 GiB of top-level files, mostly `deps4-sync*.log` (section 2).
- AGENTS.md violation: the ticket holds repo source copies. 210 files byte-identical to tracked repo files outside the ticket, plus import-rewritten derived copies in `integrate-tools/final*` (section 3).
- Disk and memory are fine: 130 GiB free on `/`, swap 498 MiB used of 2 GiB. `.cache/b` is about 16.7 GB measured, with 3 of 8 children not measured (section 4).

## 1. `[DEBUG] ` lines

Method: `git diff -U0` (unstaged) and `git diff --cached -U0` (staged), added lines only, with new-file line numbers. Plus `git ls-files --others --exclude-standard` for untracked files.

- Unstaged working tree: 0 lines.
- Untracked files: 4 ticket scripts (below). No untracked source files contain the prefix.
- Staged (index vs HEAD): 886 lines.

### 1a. Staged source test files (new in the index, not committed)

| File (repo path) | Line(s) | Kind |
|---|---|---|
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🏙️bin-01/🔬️unit/🏷️catalog-staging-command-refusal-and-direct-fixture-laws/🦀️.rs` | 45 | test marker |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🧪️tests/🔬️unit/🧲️docking/🦀️.rs` | 107, 131, 164, 259 | `println!` test markers |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/⚡️recovery/🧪️tests/🟦️.ts` | 55 | test marker |

These follow the committed test-marker convention (see 1d), not temporary logs. Decision for the coordinator: keep them as evidence or strip the prefix.

### 1b. Gone since earlier notes

`rg 'inspect receiver|__spinTrack|SEMIO_DEBUG_SPIN'` over the repo returns nothing. The three lines listed in earlier result files (`🏃️process/📦️artifacts/.../cleanup/🎛️command/🟦️.ts:135`, `routing/📥️invocation/🏃️process/🟦️.ts:17,194`) are no longer in the tree.

### 1c. Ticket scripts and outputs (temporary logs, must be removed or kept under the rule)

Staged:

- `apps-c-tools/probe.mjs:18`; `apps-c-tools/text.mjs:3,5,6`
- `boots-b-tools/activate.sh:7,9`; `boots-b-tools/serve-probe.sh:9,10,12,14`
- `print-b-tools/api.ts:3`; `print-b-tools/quick.ts:4`
- `repo-tests-c-tools/intent-dbg.ts:13,16`
- Output files, which AGENTS.md says to delete: `nx-b-tools/spin-hunt-proc.txt` (6 lines), `nx-b-tools/spin-hunt-timers.txt` (3), `nx-b-tools/spin-hunt-lease.txt` (2)
- `stage-clean-tools/residual/*.patch`: 76 lines across 8 patch files. `conc-deps-tools__staged` 21, `tsc-stage-tools__staged` 18, `bzl-stage-tools__staged` 14, `round6-tools__staged` 8, `pm-seam-tools__staged2` 8, `lease-fair-tools__staged` 3, `tax-stage-tools__staged` 2, `fixups6-tools__staged` 2

Untracked:

- `boots-c-tools/serve-probe.sh:9,10,12,14`
- `boots-c-tools/activate.sh:8,10`
- `boots-c-tools/interact-forms.mjs:29,142`
- `boots-c-tools/interact-note.mjs:28,181`

Markdown that quotes the prefix (documentation, not logs): `audit-leftovers-1023.md` (768 staged lines, 361 KB), `result-nx-b.md` (5), `result-rcore-b.md` (2), `result-demo-b.md` (2), `coordination.md` (1), `brief-wave-1023b.md` (1).

### 1d. Committed convention (context only)

`git grep -c '\[DEBUG\] '` at HEAD over `*.ts` and `*.rs` sums to 5083 matching lines. These are test pass markers already committed and are not part of the uncommitted diff. An earlier note counted about 1096 at HEAD. The difference is in scope, not checked further.

## 2. `C/🗑️generated`

`du -sh` on the whole folder: **6.4 GiB** (1130 direct entries: 266 dirs, the rest files).

- Subdirs total: 4.7 GiB (`du -sk` per direct child, `perl` alarm 60 s; none capped).
- Top-level files: 1.75 GiB. Largest: `deps4-sync60.log` 308 MB, `deps4-sync50.log` 301 MB, `deps4-sync31.log` 167 MB, `deps4-sync23.log` 119 MB, `deps4-sync14.log` 116 MB.

The 20 largest subdirs:

| # | Dir | MiB |
|---|---|---|
| 1 | gran-hubs | 376.2 |
| 2 | input-precision | 270.0 |
| 3 | mcp-b | 243.1 |
| 4 | wsr | 215.0 |
| 5 | gran-value | 201.4 |
| 6 | rs-sweep3-run | 174.5 |
| 7 | rust-carve-x | 174.1 |
| 8 | cargo-fix | 167.5 |
| 9 | manifest-batch2 | 154.0 |
| 10 | chain | 152.2 |
| 11 | rust-health-logs | 141.6 |
| 12 | fnsplit4a-census | 134.2 |
| 13 | rcore-b | 120.8 |
| 14 | apps-a2-x | 103.9 |
| 15 | integrate3-apply | 98.2 |
| 16 | chain10 | 94.4 |
| 17 | chain17 | 87.8 |
| 18 | store-b | 74.2 |
| 19 | window-config-1022 | 71.0 |
| 20 | ts-health-run | 64.7 |

## 3. Source copies under the ticket

AGENTS.md: MUST NOT copy source files into the tickets folder when they are checked into version control.

Method: 114 candidate dirs were found by name (`*-tools/src`, `*-tools/staged*`, `integrate-tools/final*`, `resolutions`, `residual`, `snapshot*`, `staged-merge-tools`). For each, every `.rs/.ts/.tsx/.mjs/.js/.py/.toml/.json/.patch` file was hashed with `git hash-object` and matched against blobs at paths outside the ticket. Files that match only ticket-internal blobs are not counted.

Result: **17 dirs, 210 of 1340 checked files** are byte-identical to tracked repo files outside the ticket. Dirs with the most such copies:

| Dir | Size | Files | Identical to repo outside ticket |
|---|---|---|---|
| `integrate-tools/final` | 4.7 MiB | 189 | 38 (rest are import-rewritten derived copies, e.g. `A/0097.ts` with relative paths into `🔨️modules/...`) |
| `r6-runtime-seam-tools/src` | 0.3 MiB | 43 | 37 |
| `runtime-seam-tools/src` | 0.3 MiB | 43 | 37 |
| `integrate-tools/final2` | 2.4 MiB | 146 | 16 (rest derived) |
| `r6-tool-iface-tools/src` | 0.5 MiB | 28 | 18 |
| `tool-iface-tools/src` | 0.5 MiB | 28 | 18 |
| `integrate-tools/final3` | 0.3 MiB | 33 | 17 |
| `iface-wrap-tools/src` | 0.1 MiB | 11 | 5 |
| `backend-contract-tools/src` | 0.3 MiB | 28 | 4 |
| `tool-direct-code-tools/src` | 0.1 MiB | 6 | 5 |
| `conc-harness-tools/src` | 0.7 MiB | 9 | 4 |
| `perf-slo-tools/src` | 0.6 MiB | 9 | 2 |
| `bzl-incr-tools/src` | 0.2 MiB | 9 | 1 |
| `lawcov-build-tools/work/snapshot(-fixed)` | small | 6 each | 1 and 3 |
| `legacy-purge-tools/src`, `r6-legacy-purge-tools/src` | small | 3 each | 2 each |

Also large, but not repo copies: `stage-clean-tools/residual` (124 `.patch` files, 46.5 MiB) carries source hunks and is staged; `gran-hubs-tools/staged` (23 files, 60.6 MiB, largest candidate) and `round6-tools/staged` (221 files) are tool scripts and data (no matches outside the ticket). Ticket-wide: 4528 files are staged under the ticket and 1752 of them have source-like extensions. Most are tool scripts.

## 4. Disk, swap and cache

- `df -h /`: `/dev/disk3s1s1` 926 GiB size, 13 GiB used, **130 GiB available**, 10 % capacity, 0 % inodes used.
- `sysctl vm.swapusage`: total 2048 MB, used 497.75 MB, free 1550.25 MB (encrypted).
- `.cache/b` top-level children (`du -sh`, `perl` alarm 60 s):

| Child | Size |
|---|---|
| c | 4.1 MB |
| d | not measured (cap 60 s hit) |
| g | 11 GB |
| l | 68 MB |
| n | 2.5 GB |
| o | 3.0 GB |
| p | not measured (cap 60 s hit) |
| t | not measured (cap 60 s hit) |

Measured total is about 16.7 GB. The three uncapped children need a longer run.

## Follow-ups for the coordinator

1. Remove the source copies in `integrate-tools/final*`, `*-tools/src` and `stage-clean-tools/residual` from the ticket, or unstage them, to satisfy the AGENTS.md rule.
2. Strip `[DEBUG] ` from the ticket scripts in section 1c, or delete them. Delete the `nx-b-tools/spin-hunt-*.txt` outputs.
3. Decide whether test markers in the 6 staged test lines (1a) and at HEAD (1d) keep the prefix.
4. Shrink `C/🗑️generated` by deleting `deps4-sync*.log` (1.75 GiB) when done.
