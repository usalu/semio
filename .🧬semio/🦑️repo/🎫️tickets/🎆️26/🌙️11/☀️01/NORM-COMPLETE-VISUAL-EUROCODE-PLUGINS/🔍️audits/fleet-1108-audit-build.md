# 🔍 Fleet 1108 Audit: Build Failures (static, no cargo)

Lane `audit-build`, read-only. No builds started, no source edited. Sources: highest-numbered `<n>.txt` per `🗑️generated/build-en199x/` and `build-en1990/…`, static checkers in `🧰️scripts/`. Staleness test: the rustc snippet logged for each error is searched in the current file (`present=True` means the exact offending line still exists); the file mtime is compared with the log time.

Scripts: `🧰️scripts/audit-build-extract.py`, `audit-build-resolve.py`, `audit-build-stale.py` (run with `python3 -I`).

## Static checkers (no cargo)

| Checker | Result |
|---|---|
| `🧰️scripts/check-cargo-authority.py` | 37 crates, 37 members, **0 errors** |
| `🧰️scripts/source-boundary-scan.py` | **3 crates over budget**, all by file count (limit 32), bytes and lines within budget |

Over budget (still present):
- `semio-s-artifact-norm-en1993-mut-fastened-joints`: 67 files, 96140 bytes, 1670 lines (`🗿️artifacts/🔩️en1993/…/🧬️mutations/🪛fastened-joints/📦️packages/🦀️rust`)
- `semio-s-artifact-norm-en1993-mut-buckling-lengths`: 34 files (`…/🧬️mutations/🪜buckling-lengths/📦️packages/🦀️rust`). Fix: merge two small mutation files or move one module out of the closure.
- `semio-s-artifact-norm-en1993-mut-fatigue-stresses`: 34 files (`…/🧬️mutations/⏳fatigue-stresses/📦️packages/🦀️rust`). Fix: same as above.

## Compile errors from the newest logs

Paths are relative to `✏️s/🔌️plugins/📕️norm/`. Status key: STILL PRESENT, STALE, UNVERIFIED (log elided, cannot be checked statically), INFRA (not a code error).

### Shared crates (app-surface, used by several artifacts)

| Crate | Log | Count | Status | file:line | Suggested fix |
|---|---|---|---|---|---|
| `semio-s-plugin-norm-app-surface-panels` | en1991 17.txt | 2 E0502 | STALE | `📇️registry/…/🗂️design-case-surface/✏️case-edits/🧪️laws/🦀️.rs:79,89` | Already rewritten: `duplicate_case_edit(&cases(host.document()), …)` is now bound with `let` before `apply(&mut host, …)`. Re-check on next build. |
| `semio-s-plugin-norm-app-surface-visual-chrome` | en1995 30.txt | 3 E0425 | UNVERIFIED | locations elided (`52625 diagnostic line(s) not shown`, cargo log file no longer exists) | Re-run `check` for `norm-en1995-rs` once and capture the `error[E0425]` lines. Names referenced in the log (`device_profiles::*`, `accessibility::focus_order::*`, `published_nodes::*`, `viewport3d::config::DeviceClass`) all resolve in the crate. |

### Per artifact

| Crate | Log | Count | Status | file:line | Suggested fix |
|---|---|---|---|---|---|
| `semio-s-artifact-norm-en1990-checks` | en1990 16.txt | 2 E0716 | STALE | `🗿️artifacts/⚖️en1990/…/⚖️checks/📇️catalogue/🦀️.rs:502,666` | `envelope()` now returns an owned `Result<Option<Combination>, String>` (`📇️catalogue/🦀️.rs:486`); the borrowed temporary is gone. |
| `semio-s-artifact-norm-en1992-diff` | en1992 20.txt | 1 E0425 (`OpeningKey`) | STALE | `🗿️artifacts/🏛️en1992/…/🧬️schema/🔺️diff/🦀️.rs:229` | The `by OpeningKey` text is no longer in the `list_delta!` at line 230; `OpeningKey` is only used in `row_key!` (line 43). |
| `semio-s-artifact-norm-en1992-checks-core` | en1992 20.txt | 1 E0433 (`semio_s_plugin_norm_mechanics`) | STALE | `…/⚖️checks/🧱️materials-and-time/🦀️.rs:98` | Dependency is now declared: `🧬️core/📦️packages/🦀️rust/Cargo.toml:24` (`semio-s-plugin-norm-mechanics = { workspace = true }`). |
| `semio-s-artifact-norm-en1993-model` (sqlite snapshot) | en1993 13.txt | 73 reported, 17 captured (16 E0308, 1 E0063) | STALE | `🗿️artifacts/🔩️en1993/…/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs:200-217` | File was rewritten 2026-11-07 03:15 (after the log, 11-03). The logged lines no longer exist, and `frame_actions` / `frame_member_bases` are now initialised (line 244). The 56 uncaptured errors need a rebuild to confirm. |
| `semio-s-artifact-norm-en1994-diff` | en1994 17.txt | 30 reported, 10 captured (E0308) | STILL PRESENT | `🗿️artifacts/🧩️en1994/…/🧬️schema/🔺️diff/🦀️.rs:56,60,79,80,97,98` | The `RowPatch` macro (`commit_into`, `absorb`, `inverse`, `is_empty`) is unchanged since the build, so the mismatch remains. Compare the `$row` / `$f` field types in the `nested_patch!` / `scalar_patch!` call sites with the working `vdi3805` diff crate (`🗿️artifacts/🏭️vdi3805/…/🔺️diff/🦀️.rs:110,237`). |
| `semio-s-plugin-norm-ndp-en1990` (build graph, en1999 log) | en1999 11.txt | 1 (duplicate project) | STALE | `⚖️compliance/📜️annex-and-tables/🔖️ndp/🗃️table/en1990` | Second root `📜️annex-and-tables/🗃️table/en1990` no longer exists. |

### Infrastructure (not code errors)

| Artifact | Log | Status | Cause | Note |
|---|---|---|---|---|
| en1996 | 18.txt | INFRA | `SIGTERM` after 350 s, no compile error reached | Re-run once the host is free. |
| en1997 | 16.txt | INFRA | Nx daemon `watcher lost coverage: watch-limit` | Raise the host inotify `max_user_watches`, not a code fix. |
| en1998 | 12.txt | INFRA | `build:graph-plan` SIGKILL (likely OOM) | No compile error in the log. |

## Summary

Still present (code): **1 crate** (`norm-en1994` diff, 30 errors, 10 captured, macro at lines 56–98) plus **1 unverified** crate (`visual-chrome`, 3 E0425, locations elided).

Stale (code): app-surface-panels (2), en1990-checks (2), en1992-diff (1), en1992-checks-core (1), en1993-model (17 captured of 73), en1999 duplicate project (1).

Budget: 3 en1993 mutation crates over the 32-file limit.
