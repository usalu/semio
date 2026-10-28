# Disk Census Against Declared Cache Budgets (aud-disk-g, Wave 1026g)

Read-only census, 2026-10-26 19:09 to 19:53 CET. No deletion, no gc, no builds, no git writes, no source edits.
Raw output: `C/🗑️generated/aud-disk-g/` (`progress.log`, `*.txt`). Scripts: `C/aud-disk-g-tools/census.sh` (first pass; stopped at 19:14 because mount walks were too slow for the quiet window) and `census2.sh`.

Method: `du -k` reports apparent size and counts APFS clones (from `cache dedupe`) in full, so it overstates. `df -k` on a mounted image gives the physical used space, which is the number to trust for partitions. DMG images are fully allocated, so allocated size equals capacity. macOS `du` rejects `-s` with `-d`; the first pass used that combination and returned nothing for the repo cache and home roots, so those were re-run with `-kx -d 1`.

## 1. Declared budgets (where they live)

- Ledger: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🔣️policy.json`. Roots: `.🧬semio/🦑️repo/⚡️cache` loose max 24 GiB; `.cache` loose max 256 MiB. 59 areas with budgets, 19 retirement rules. Undeclared dirs are collected after 14 days (repo) or 7 days (native).
- Partition budget: `…/⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json` (partition capacities; machine maximum 98 GiB).
- Docs: `README.md` "Shared caches and budgets" and `📓️build-guide.md` "Disk budget": default 40 GiB, file `🎚️machine-budget.json` in the cache root (absent on this machine).

## 2. Partitions (DMG images, physical use from df)

| Partition | Allocated | Used (df) | Budget (source) | Enforcer | Last run |
|---|---|---|---|---|---|
| rust `⚡️cache/rust/fixed` | 48.0 GiB | 39.7 GiB, 84% | 48 GiB; unit-cache 24, cargo-home 3, nx-artifacts 3, compile-cache-rust 0.75 GiB | unit-cache collector above 80% | no rust collector stamp |
| products `.cache/b/p` | 24.0 GiB | 15.8 GiB, 67% | 24 GiB (elastic to 40) | Bazel disk GC (40% actions, 30 d), products-collector | 10-24 23:29 |
| tools `.cache/b/t` | 8.0 GiB | 5.8 GiB, 75% | 8 GiB (policy `maximumLogicalBytes` says 64 GiB) | nx-tooling collector | 10-26 16:55, removed 0 |
| dependencies `.cache/b/d` | 8.0 GiB | 4.5 GiB, 57% | 8 GiB; generations 6 h / 24 h | sync and retention | no stamp |
| graph `.cache/b/g` | 6.0 GiB | 2.0 GiB, 35% | 6 GiB (policy) vs 4 GiB (README, build guide) | graph-collector | 10-24 04:00, nothing to reclaim |
| diagnostics `.cache/b/o` | 2.0 GiB | 1.1 GiB, 58% | 2 GiB | test-runs 1 GiB | none |
| nx `.cache/b/n` | 1.5 GiB | 0.8 GiB, 52% | 1.5 GiB | none | none |
| leases `.cache/b/l` | 64 MiB | 6 MiB, 10% | 64 MiB | none | none |
| control `.cache/b/c` | 4 MiB | n/a | 1 MiB (`maximumControlBytes`) | none | OVER: `metadata-headroom.bin` 4 MiB (10-16) |

Partition total allocated: 97.6 GiB.

## 3. Loose areas and nested budgets

Repo root `⚡️cache`, excluding rust: 4.6 GiB (tools 3.3, of which elan 2.7 of 3 GiB and ms-playwright 0.6 of 2 GiB; go 0.73 of 1 GiB; cargo 0.26 of 1 GiB; bin 0.10 of 1 GiB; tectonic, implementation, agents, dashboard within budget). Loose root is within its 24 GiB cap.

Native `.cache` loose, excluding `b`: 243 MiB of 256 MiB (95%). Bazel 182 MiB (`.cache/bazel`, retired with 24 h quiet, not collected), impl-exp 37 MiB (undeclared), command 12 MiB (budget 128 MiB), gg 11 MiB (undeclared, created today), semio, boot-recovery, test-sandbox.

| Area | Measured | Budget | Status |
|---|---|---|---|
| nx-tooling `.cache/b/t/m/nx` | 4.36 GiB (apparent) | 2 GiB | OVER; collector 16:55 removed 0 entries (tools image never idle) |
| compile-cache-rust `⚡️cache/nx/tmp/node-compile-cache` | 1.00 GiB (ledger 10-26 08:10) | 768 MiB | OVER |
| Bazel outputs `.cache/b/p/m/bazel/outputs` | 19.9 GiB (apparent) | 8% of products = 1.9 GiB | OVER by du; physical share unknown |
| Bazel workspaces | 7.9 GiB (apparent) | 10% = 2.4 GiB | OVER by du |
| Bazel actions | 7.2 GiB (apparent) | 40% = 9.6 GiB | within |
| Bazel pool | 1.0 GiB (apparent) | 6% = 1.4 GiB | within |
| nx-tmp (ledger 10-26 16:55) | 134 MiB | 256 MiB | within |
| rust partition | 84% used | 80% collector trigger | AT TRIGGER |

Bazel output_user_root is set by the adapter (`🏗️bazel/💾️storage`) to a short root inside the products image. No Bazel cache exists under `~/Library/Caches` or `~/.cache`.

## 4. Areas without any budget (undeclared or runtime-only)

- `🧰️framework/.cache/b/{t,c,l,o}`: 8.1 GiB, of which `t/fixed.dmg` is 8.0 GiB (unmounted, last write 10-23). A second partition set inside the source tree, outside both ledger roots. Gitignored only by `*.[Cc]ache`.
- Undeclared repo cache dirs: next-services 70 MiB, edfix 22 MiB, nx-admission 17 MiB, one hashed dir 13 MiB, e2b 6 MiB, p5 4 MiB, f1, e2, at (under 1 MiB). Total 0.13 GiB. The 14-day undeclared rule has not been applied.
- Undeclared native dirs: impl-exp 37 MiB, gg 11 MiB, semio, test-sandbox.
- `🧰️framework/🤖️generated/process`: 189 MiB. Declared only as `runtimeStateDirectories` (build-input exclusion), no budget, no collector. Contents: `captures` 365 entries (newest write 10-26 08:59), `cargo-target` (10-09), `leases` 20 entries (10-25). Current receipts go to `.cache/command/captures` (232 entries, newest 18:59). Suspected obsolete; confirm no live writer before removal (one writer still wrote at 08:59).
- Not cache, no budget, outside the ledger: `.🧬semio/🌐hub` 100.6 GiB, root `🌐hub` 9.3 GiB, `.git/worktrees` 5.3 GiB (worktrees exist; AGENTS forbids them for agents, observed only).

## 5. Top 15 under the repo root (du -kx -d 2, apparent)

Repo root total: 266.0 GiB.

| # | Path (relative to repo root) | Size | Class |
|---|---|---|---|
| 1 | `.🧬semio` | 156.8 GiB | container: cache root plus hub |
| 2 | `.🧬semio/🌐hub` | 100.6 GiB | hub data, not a cache |
| 3 | `.🧬semio/🦑️repo` | 55.6 GiB | repo cache root 52.6 GiB + tickets 0.5 GiB |
| 4 | `.cache` | 49.8 GiB | native cache root |
| 5 | `.cache/b` | 49.6 GiB | partitions (section 2) |
| 6 | `🧰️framework` | 24.9 GiB | source (incl. nested `.cache` 8.1 GiB) |
| 7 | `.git` | 13.7 GiB | git (worktrees 5.3, objects 4.8, modules 2.9) |
| 8 | `🧰️framework/🛍️products` | 13.2 GiB | source |
| 9 | `🌐hub` | 9.3 GiB | not a cache |
| 10 | `🧰️framework/.cache` | 8.1 GiB | UNDECLARED cache |
| 11 | `leutwiler` | 7.9 GiB | project data |
| 12 | `leutwiler/realparts-of-powers-z-n` | 7.9 GiB | project data |
| 13 | `🌐hub/compositions` | 7.9 GiB | hub data |
| 14 | `.git/worktrees` | 5.3 GiB | git |
| 15 | `.git/objects` | 4.8 GiB | git |

## 6. Top 15 under ~/Library/Caches and ~/.cache (du -kx -d 1, apparent)

Library/Caches total 19.4 GiB. Repo-related: go-build 1.62, semio 0.90, ms-playwright 0.54, bun 0.32 (about 3.4 GiB). Others (Spotify 4.47, Google 1.62, ShipIt 1.46, Cursor 1.35, Brave 1.13, Zotero 1.00, Codex 0.96, Mozilla 0.53, Homebrew 0.42, antigravity 0.34, electron 0.32, Steam 0.32) are not this repo.

~/.cache total 52.1 GiB. Probably repo-related: uv 22.43 (repo bootstrap installs uv), r5a 10.27, r4 10.21, r9a 1.11, r11 0.54, r7b 0.42, r6 0.30 (directory names anim/cad/graph/pres suggest repo experiments, unverified), mathlib 0.44 (Lean, same family as repo elan). Unknown owner: codex-runtimes 1.83, mwu 0.91, gtk 0.30, tc1 0.29, ohp 0.27, vtax 0.25, ohf 0.22.

Not measured: `~/.sinc` (125 GiB per the wave brief), outside the requested roots and too heavy for the quiet window.

## 7. Ticket output sizes (ticket C)

- Ticket total 0.475 GiB.
- `🗑️generated` 0.31 GiB (jobsrv-g 150 MiB, nx 116 MiB, oracle2-g 30 MiB). The brief's figure of about 54 GiB is not what this ticket holds.
- `*-tools` 0.15 GiB across 580 entries (largest: integrate-tools 7.7 MiB, conv-g-tools 5.5 MiB, keys2-g-tools 4.7 MiB).

## 8. Stamps and recency

- Size ledger 10-26 16:55; prune stamp 09-13 (stale); collectors graph 10-24 04:00, products 10-24 23:29, nx-tooling 10-26 16:55 (0 removed). No rust or undeclared stamp.

## 9. Findings

1. Over budget: nx-tooling (4.4 vs 2 GiB), compile-cache-rust (1.0 vs 0.75 GiB), Bazel workspaces and outputs (over by du), control dir (4 vs 1 MiB).
2. Rust partition at 84%, above the 80% collector trigger, with no recorded collector run.
3. Native loose root at 243 of 256 MiB. `.cache/bazel` (182 MiB) is retired but not collected.
4. No `🎚️machine-budget.json`: the doc default is 40 GiB, the policy file says 98 GiB, and about 110 GiB is held. The machine budget is effectively unset.
5. Documentation and policy drift: graph 4 GiB (docs) vs 6 GiB (policy); tools `maximumLogicalBytes` 64 GiB vs 8 GiB capacity.
6. Undeclared: `🧰️framework/.cache/b` 8.1 GiB, 9 repo cache dirs (0.13 GiB), 4 native dirs, and `🧰️framework/🤖️generated/process` 0.19 GiB (suspected obsolete).
7. Host: `/System/Volumes/Data` has 138 GB free of 971 GB (86% used).

## 10. Totals

- Ledgered caches: 102.4 GiB (partitions 97.6, loose repo 4.6, loose native 0.24).
- Undeclared cache inside the source tree: 8.1 GiB. Total cache under the repo: about 110.5 GiB.
- Ticket: 0.5 GiB. Home caches: 71.5 GiB, of which about 49 GiB is probably repo-related.
- Over or at budget: 5 over, 1 at trigger. Without budget or undeclared: 15 entries.

## 11. Suggested next steps (not executed)

- Run `repo cache status --json` and `repo cache gc --dry-run --area=nx-tooling,nx-tmp,undeclared` in a quiet window, then decide on collection.
- Set `🎚️machine-budget.json` for this 16 GB target; fix graph (4 vs 6 GiB) and tools (64 vs 8 GiB) in policy and docs.
- After owner check, remove `🧰️framework/.cache/b` and `🧰️framework/🤖️generated/process` (and its `runtimeStateDirectories` entry); move `metadata-headroom.bin` out of `.cache/b/c`.
