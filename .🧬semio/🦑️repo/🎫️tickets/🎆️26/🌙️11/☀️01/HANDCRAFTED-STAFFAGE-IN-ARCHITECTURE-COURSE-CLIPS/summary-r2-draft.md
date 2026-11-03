# Round-2 summary draft (from agent sd, to be reconciled at close)

Draft base for `summary.md`. The final version supersedes this file.

## Known leftovers to reconcile (sd draft, may be stale)

| Item | Draft status | Source |
|---|---|---|
| TTS key unset, OI-110/111 | Open, owner | `r2-tt-report.md` |
| Docker daemon stopped, OI-84 | Open, infra | `r2-px-report.md` |
| Linux vitest needs sudo bootstrap | Open | `r2-lx-report.md` |
| Energy series German on screen | By design | `r2-tx-report.md` |
| `repo dependencies ensure` refuses, d3-format unlinked | Open, infra | `r2-ty-report.md` |
| Gendered titles | As authored | `r2-bc-report.md` |
| BIM geometry over 2 h, OI-01/02/80/81/118 | Check bv | `r2-rb-report.md`, bv |
| OI-04 energy_tank | Ruled: keep | rulings |
| Nakagin memory | Check xl3 | `r2-xl2-report.md`, xl3 |
| OI-10/11 cast memory and frame budget | Open | sz, pf |
| OI-23 kettle mass | Check kr | `r2-kr-report.md` |
| OI-27/112/113/114/134 | Ruled or fixed by ap/cl | rulings, `r2-ap-report.md` |
| OI-124/125 test imports, oracle docstrings | Open | owner |
| OI-126 untracked files | Commit only if asked | owner |
| OI-115/131 renders re-checked | Check cp3 | cp3 |
| Guard is name-based | Limit | `r2-hz-report.md` |
| `_WATT_DEVICES` orphans | Check | `audit-final-static-r2.md` |

## Verification snapshot (sd draft)

| Suite | Latest | Source |
|---|---|---|
| Library vitest | 1161 passed, 11 skipped | `r2-ed-report.md` |
| React vitest | 117 passed, 1 skipped | `r2-ed-report.md` |
| Catalog check | 148 assets, 0 failures | `r2-qa-report.md` |
| Staffage Python and guard | 64 OK | `r2-fb-report.md` |
| Cast export | 98 casts, 16 039 frames, 270.6 MB | `r2-sz-report.md` |
| Optimization | 92 OK | `r2-op-report.md` |
| Path length | 0 violations, max 226 | `r2-pl-report.md` |

## Performance (sd draft)

| Measure | Before | After | Source |
|---|---|---|---|
| Full manifest CPU / wall | 198 s / 258 s | 97 s / 158 s | `r2-pf-report.md` |
| Full manifest peak | 1309 MB | 733 MB | `r2-pf-report.md` |
| Oak CPU | 96 s | 9.4 s | `r2-pf-report.md`, `r2-fm-report.md` |
| Total cast bytes | 312.5 MB | 270.6 MB | `r2-sz-report.md` |
