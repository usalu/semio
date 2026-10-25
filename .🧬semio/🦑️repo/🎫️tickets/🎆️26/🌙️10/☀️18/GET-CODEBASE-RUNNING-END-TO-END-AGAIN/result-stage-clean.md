# stage-clean result (wave 1023b, 10-23)

Status: DONE and verified. Staged source copies removed from the ticket; residual patches written; other tickets listed (not touched). No sub-agents, no git-modifying commands, no builds.

## Outcome

- Inventory: 142 `staged*` roots (top-most; dirs ending in `-tools` such as `staged-merge-tools` are tool dirs, excluded), 4970 files, scan by `os.walk(followlinks=False)` limited to C (generated folder pruned).

- Deleted: **4315 files, 59.8 MiB** of source copies, plus 1160 now-empty dirs. Remaining under `staged*` dirs and `staged-merge-tools`: 661 files, 70.6 MiB, longest path 179 bytes (mirrored long repo paths are gone); 60.5 MiB of it is `gran-hubs-tools/staged/plans/*.json` (edit-plan data, 7 files > 1 MiB, inputs of `apply.py`; kept, coordinator may drop them if gran-hubs is finished).
- Kept (tool inputs, not source copies): 388 scripts/manifests/plans (`apply*.py|sh`, `plan|manifest|ops|edits*.json`, `make-*.py`, oracles ...), 258 byte-identical duplicates of scripts that still exist elsewhere in C (`ticket-dup`), 9 tool-data files > 1 MiB with no counterpart.

- Patches: 124 unified diffs in `stage-clean-tools/residual/<root-slug>.patch` (47.6 MB text, `/` in root replaced by `__`). Every diff was round-trip verified by an independent applier (old text + patch == staged bytes, 0 failures); a sample also applied cleanly with `patch -p1 --dry-run` at the repo root. Orphans are new-file diffs (`--- /dev/null`), header comment per file: `# stage-clean <class> staged=<root>/<rel> map=<how>`.

- Full per-file inventory (class, mapping method, counterpart): `stage-clean-tools/residual/inventory.tsv`.

## Classification (counts over all 4970 files)

| class | files | meaning |
|---|---:|---|
| identical-live | 2193 | identical to live (deleted, no patch needed) |
| head-live-changed | 28 | identical to HEAD blob, live changed since (deleted, patched) |
| different | 966 | differs from both HEAD and live (deleted, patched) |
| orphan | 1128 | no counterpart / counterpart gone (deleted, patched as new-file diff) |
| tool-input | 388 | script/manifest/plan (kept) |
| ticket-dup | 258 | duplicate of a kept ticket script (kept) |
| tool-data | 9 | > 1 MiB, no counterpart (kept) |

Counterpart mapping, in order: manifest/plan JSON references (sha256 or id -> repo path), repo-relative path suffix, git blob hash equal to a HEAD or index blob outside C, same-basename file with >= 0.5 line-set similarity. Counterparts inside C or inside another `staged*` dir never count (3 files in `course-bzl-tools/rebased/x-paths/staged` whose only twin is a staged copy in ARCHITECTURE-COURSE-ON-BAZEL were therefore patched as orphans).

## Per staged dir

Columns: files = all files in the root; ident = identical to live; headchg = identical to HEAD, live changed; diff = differs from both; orphan = no counterpart; kept = scripts/dups/data left in place.

| dir (under C) | files | ident | headchg | diff | orphan | kept | patch |
|---|---:|---:|---:|---:|---:|---:|---|
| `agent-session-tools/staged` | 18 | 13 | 0 | 4 | 0 | 1 | `residual/agent-session-tools__staged.patch` |
| `audit-fix-tools/staged` | 15 | 0 | 0 | 0 | 6 | 9 | `residual/audit-fix-tools__staged.patch` |
| `backend-contract-tools/staged` | 28 | 18 | 0 | 0 | 9 | 1 | `residual/backend-contract-tools__staged.patch` |
| `boot-fast-tools/staged` | 18 | 9 | 0 | 0 | 8 | 1 | `residual/boot-fast-tools__staged.patch` |
| `boot-sweep-tools/staged` | 17 | 13 | 0 | 0 | 4 | 0 | `residual/boot-sweep-tools__staged.patch` |
| `bworkers-tools/staged` | 23 | 7 | 0 | 0 | 15 | 1 | `residual/bworkers-tools__staged.patch` |
| `bzl-coverage-tools/staged` | 8 | 5 | 0 | 0 | 3 | 0 | `residual/bzl-coverage-tools__staged.patch` |
| `bzl-default-tools/staged` | 10 | 3 | 1 | 4 | 1 | 1 | `residual/bzl-default-tools__staged.patch` |
| `bzl-hermetic-tools/staged` | 12 | 1 | 0 | 4 | 7 | 0 | `residual/bzl-hermetic-tools__staged.patch` |
| `bzl-hermetic-tools/staged-paths` | 11 | 5 | 0 | 0 | 0 | 6 | - |
| `bzl-incr-tools/staged` | 10 | 1 | 0 | 2 | 5 | 2 | `residual/bzl-incr-tools__staged.patch` |
| `bzl-lazy-tools/staged` | 20 | 3 | 0 | 0 | 16 | 1 | `residual/bzl-lazy-tools__staged.patch` |
| `bzl-nocache-tools/staged` | 2 | 0 | 0 | 0 | 0 | 2 | - |
| `bzl-real-tools/staged` | 4 | 0 | 0 | 0 | 3 | 1 | `residual/bzl-real-tools__staged.patch` |
| `bzl-rust-tools/staged` | 5 | 0 | 0 | 5 | 0 | 0 | `residual/bzl-rust-tools__staged.patch` |
| `bzl-stage-tools/staged` | 16 | 1 | 0 | 0 | 15 | 0 | `residual/bzl-stage-tools__staged.patch` |
| `bzl-tests-rest-tools/staged` | 27 | 21 | 0 | 1 | 4 | 1 | `residual/bzl-tests-rest-tools__staged.patch` |
| `bzl-xplat-tools/staged` | 6 | 2 | 0 | 0 | 4 | 0 | `residual/bzl-xplat-tools__staged.patch` |
| `cache-fix-tools/bz/staged` | 1 | 0 | 0 | 0 | 1 | 0 | `residual/cache-fix-tools__bz__staged.patch` |
| `cache-fix-tools/staged` | 21 | 1 | 0 | 17 | 3 | 0 | `residual/cache-fix-tools__staged.patch` |
| `cache-gc-tools/staged` | 11 | 2 | 0 | 1 | 8 | 0 | `residual/cache-gc-tools__staged.patch` |
| `cache-gc-tools/staged2` | 9 | 0 | 0 | 1 | 8 | 0 | `residual/cache-gc-tools__staged2.patch` |
| `cache-gc-tools/staged3` | 13 | 2 | 0 | 2 | 9 | 0 | `residual/cache-gc-tools__staged3.patch` |
| `cache-gc-tools/staged4` | 14 | 9 | 0 | 0 | 5 | 0 | `residual/cache-gc-tools__staged4.patch` |
| `cache-gc-tools/staged5` | 1 | 0 | 0 | 0 | 1 | 0 | `residual/cache-gc-tools__staged5.patch` |
| `cargo-fix-tools/staged` | 79 | 53 | 1 | 23 | 1 | 1 | `residual/cargo-fix-tools__staged.patch` |
| `cargo-pressure-tools/staged` | 9 | 4 | 0 | 0 | 5 | 0 | `residual/cargo-pressure-tools__staged.patch` |
| `ci-bazel-tools/staged` | 12 | 3 | 0 | 3 | 5 | 1 | `residual/ci-bazel-tools__staged.patch` |
| `cli-latency-tools/staged` | 11 | 6 | 0 | 0 | 3 | 2 | `residual/cli-latency-tools__staged.patch` |
| `cli-zt-tools/staged` | 18 | 4 | 0 | 6 | 8 | 0 | `residual/cli-zt-tools__staged.patch` |
| `conc-bazel-tools/staged` | 16 | 2 | 0 | 6 | 8 | 0 | `residual/conc-bazel-tools__staged.patch` |
| `conc-cargo-tools/staged` | 28 | 14 | 0 | 10 | 4 | 0 | `residual/conc-cargo-tools__staged.patch` |
| `conc-deps-tools/staged` | 126 | 4 | 0 | 56 | 62 | 4 | `residual/conc-deps-tools__staged.patch` |
| `conc-graph-tools/staged` | 33 | 11 | 0 | 2 | 16 | 4 | `residual/conc-graph-tools__staged.patch` |
| `conc-harness-tools/staged` | 10 | 7 | 0 | 2 | 0 | 1 | `residual/conc-harness-tools__staged.patch` |
| `conc-leases-tools/staged` | 21 | 4 | 0 | 17 | 0 | 0 | `residual/conc-leases-tools__staged.patch` |
| `conc-leases-tools/staged-optional` | 1 | 0 | 0 | 1 | 0 | 0 | `residual/conc-leases-tools__staged-optional.patch` |
| `conc-nx-tools/staged` | 19 | 13 | 0 | 0 | 5 | 1 | `residual/conc-nx-tools__staged.patch` |
| `conc-tests-tools/staged` | 20 | 13 | 0 | 4 | 3 | 0 | `residual/conc-tests-tools__staged.patch` |
| `conc-ts-tools/staged` | 6 | 5 | 0 | 0 | 1 | 0 | `residual/conc-ts-tools__staged.patch` |
| `conc-ts-tools/staged2` | 2 | 0 | 0 | 0 | 1 | 1 | `residual/conc-ts-tools__staged2.patch` |
| `conc-ts-tools/staged3` | 6 | 3 | 0 | 1 | 0 | 2 | `residual/conc-ts-tools__staged3.patch` |
| `course-bzl-tools/rebased/x-paths/staged` | 3 | 0 | 0 | 0 | 3 | 0 | `residual/course-bzl-tools__rebased__x-paths__staged.patch` |
| `course-bzl-tools/staged` | 4 | 2 | 0 | 0 | 0 | 2 | - |
| `deps5-tools/staged` | 6 | 3 | 0 | 0 | 3 | 0 | `residual/deps5-tools__staged.patch` |
| `docs-1020e-tools/staged` | 21 | 0 | 0 | 0 | 20 | 1 | `residual/docs-1020e-tools__staged.patch` |
| `docs-1021a-tools/staged` | 41 | 0 | 0 | 0 | 40 | 1 | `residual/docs-1021a-tools__staged.patch` |
| `docs-build-tools/staged` | 3 | 0 | 0 | 0 | 2 | 1 | `residual/docs-build-tools__staged.patch` |
| `env-pass2-tools/staged` | 2 | 0 | 0 | 0 | 2 | 0 | `residual/env-pass2-tools__staged.patch` |
| `fem-finish-tools/staged` | 16 | 7 | 0 | 8 | 0 | 1 | `residual/fem-finish-tools__staged.patch` |
| `fixups6-tools/staged` | 12 | 4 | 0 | 8 | 0 | 0 | `residual/fixups6-tools__staged.patch` |
| `fnsplit4a-finish-tools/staged` | 182 | 82 | 0 | 97 | 1 | 2 | `residual/fnsplit4a-finish-tools__staged.patch` |
| `gran-hubs-tools/staged` | 30 | 0 | 0 | 0 | 7 | 23 | `residual/gran-hubs-tools__staged.patch` |
| `gran-keys-tools/staged` | 9 | 7 | 0 | 1 | 1 | 0 | `residual/gran-keys-tools__staged.patch` |
| `gran-value-tools/staged` | 23 | 0 | 0 | 0 | 8 | 15 | `residual/gran-value-tools__staged.patch` |
| `graph-mem-tools/staged` | 18 | 9 | 0 | 0 | 6 | 3 | `residual/graph-mem-tools__staged.patch` |
| `graph-parallel-tools/staged` | 56 | 0 | 0 | 0 | 37 | 19 | `residual/graph-parallel-tools__staged.patch` |
| `graph-perf-tools/staged` | 36 | 19 | 0 | 0 | 12 | 5 | `residual/graph-perf-tools__staged.patch` |
| `graph-speed-tools/staged` | 36 | 22 | 1 | 0 | 10 | 3 | `residual/graph-speed-tools__staged.patch` |
| `hot-cache-tools/staged` | 105 | 38 | 2 | 2 | 25 | 38 | `residual/hot-cache-tools__staged.patch` |
| `hub-bzl-tools/staged` | 4 | 1 | 0 | 3 | 0 | 0 | `residual/hub-bzl-tools__staged.patch` |
| `i18n-core-tools/staged` | 115 | 80 | 0 | 34 | 0 | 1 | `residual/i18n-core-tools__staged.patch` |
| `i18n-core-tools/staged-c1-commands` | 16 | 0 | 0 | 12 | 3 | 1 | `residual/i18n-core-tools__staged-c1-commands.patch` |
| `i18n-core-tools/staged-c2-caching` | 23 | 0 | 0 | 19 | 3 | 1 | `residual/i18n-core-tools__staged-c2-caching.patch` |
| `i18n-core-tools/staged-c3-dependencies` | 8 | 0 | 0 | 4 | 3 | 1 | `residual/i18n-core-tools__staged-c3-dependencies.patch` |
| `i18n-core-tools/staged-c4-build` | 14 | 0 | 0 | 10 | 3 | 1 | `residual/i18n-core-tools__staged-c4-build.patch` |
| `i18n-core-tools/staged-c5-authoring` | 7 | 0 | 0 | 3 | 3 | 1 | `residual/i18n-core-tools__staged-c5-authoring.patch` |
| `i18n-core-tools/staged-c6-cleanup` | 9 | 0 | 0 | 5 | 3 | 1 | `residual/i18n-core-tools__staged-c6-cleanup.patch` |
| `i18n-core-tools/staged-c7-misc` | 16 | 0 | 0 | 12 | 3 | 1 | `residual/i18n-core-tools__staged-c7-misc.patch` |
| `i18n-core-tools/staged-config` | 32 | 0 | 0 | 12 | 19 | 1 | `residual/i18n-core-tools__staged-config.patch` |
| `i18n-core-tools/staged-defaults` | 21 | 0 | 0 | 20 | 0 | 1 | `residual/i18n-core-tools__staged-defaults.patch` |
| `ide-free-tools/staged` | 56 | 32 | 0 | 9 | 12 | 3 | `residual/ide-free-tools__staged.patch` |
| `iface-wrap-tools/staged` | 231 | 228 | 0 | 0 | 2 | 1 | `residual/iface-wrap-tools__staged.patch` |
| `ingress-perf-tools/staged` | 22 | 5 | 0 | 0 | 14 | 3 | `residual/ingress-perf-tools__staged.patch` |
| `input-precision-tools/staged` | 8 | 0 | 0 | 0 | 7 | 1 | `residual/input-precision-tools__staged.patch` |
| `input-precision-tools/staged2` | 8 | 0 | 0 | 4 | 2 | 2 | `residual/input-precision-tools__staged2.patch` |
| `inputs-closure-tools/staged` | 10 | 2 | 0 | 0 | 6 | 2 | `residual/inputs-closure-tools__staged.patch` |
| `langs-conc-tools/staged` | 19 | 5 | 0 | 0 | 13 | 1 | `residual/langs-conc-tools__staged.patch` |
| `law-drift-tools/staged` | 26 | 9 | 0 | 9 | 7 | 1 | `residual/law-drift-tools__staged.patch` |
| `lawcov-build-tools/staged` | 56 | 34 | 0 | 0 | 13 | 9 | `residual/lawcov-build-tools__staged.patch` |
| `lawcov-cli-tools/staged` | 2 | 0 | 0 | 0 | 0 | 2 | - |
| `lay16-tools/staged` | 4 | 0 | 0 | 0 | 2 | 2 | `residual/lay16-tools__staged.patch` |
| `lay17-tools/staged` | 12 | 0 | 0 | 0 | 7 | 5 | `residual/lay17-tools__staged.patch` |
| `lease-fair-tools/staged` | 12 | 5 | 0 | 7 | 0 | 0 | `residual/lease-fair-tools__staged.patch` |
| `legacy-purge-tools/staged` | 243 | 213 | 3 | 26 | 0 | 1 | `residual/legacy-purge-tools__staged.patch` |
| `lib-defects-tools/staged` | 2 | 0 | 0 | 0 | 2 | 0 | `residual/lib-defects-tools__staged.patch` |
| `lock-sync-tools/staged` | 6 | 2 | 0 | 2 | 1 | 1 | `residual/lock-sync-tools__staged.patch` |
| `loop-hunt-tools/staged` | 12 | 8 | 0 | 0 | 1 | 3 | `residual/loop-hunt-tools__staged.patch` |
| `mem-gov-tools/staged` | 24 | 15 | 0 | 4 | 5 | 0 | `residual/mem-gov-tools__staged.patch` |
| `orphan-child-tools/staged` | 4 | 2 | 0 | 0 | 2 | 0 | `residual/orphan-child-tools__staged.patch` |
| `own-crash-tools/staged` | 4 | 0 | 0 | 2 | 2 | 0 | `residual/own-crash-tools__staged.patch` |
| `own-crash2-tools/staged` | 1 | 0 | 0 | 0 | 1 | 0 | `residual/own-crash2-tools__staged.patch` |
| `own-output-tools/staged` | 3 | 0 | 0 | 0 | 0 | 3 | - |
| `parity-tools/staged` | 25 | 8 | 0 | 1 | 15 | 1 | `residual/parity-tools__staged.patch` |
| `perf-slo-tools/staged` | 10 | 5 | 0 | 0 | 4 | 1 | `residual/perf-slo-tools__staged.patch` |
| `pg-chain-tools/staged` | 2 | 1 | 0 | 0 | 1 | 0 | `residual/pg-chain-tools__staged.patch` |
| `play-bzl-tools/staged` | 6 | 0 | 0 | 3 | 3 | 0 | `residual/play-bzl-tools__staged.patch` |
| `pm-seam-tools/staged` | 24 | 8 | 0 | 14 | 1 | 1 | `residual/pm-seam-tools__staged.patch` |
| `pm-seam-tools/staged2` | 81 | 6 | 0 | 23 | 51 | 1 | `residual/pm-seam-tools__staged2.patch` |
| `pm-seam-tools/staged3` | 22 | 3 | 0 | 3 | 15 | 1 | `residual/pm-seam-tools__staged3.patch` |
| `policy-admit-tools/staged` | 6 | 4 | 0 | 0 | 0 | 2 | - |
| `polyglot-units-tools/staged` | 25 | 22 | 1 | 1 | 1 | 0 | `residual/polyglot-units-tools__staged.patch` |
| `r6-docs-1020e-tools/staged` | 21 | 0 | 0 | 0 | 20 | 1 | `residual/r6-docs-1020e-tools__staged.patch` |
| `r6-docs-1021a-tools/staged` | 41 | 0 | 0 | 0 | 40 | 1 | `residual/r6-docs-1021a-tools__staged.patch` |
| `r6-legacy-purge-tools/staged` | 243 | 215 | 3 | 24 | 0 | 1 | `residual/r6-legacy-purge-tools__staged.patch` |
| `r6-runtime-seam-tools/staged` | 44 | 41 | 0 | 3 | 0 | 0 | `residual/r6-runtime-seam-tools__staged.patch` |
| `r6-tool-iface-tools/staged` | 163 | 70 | 4 | 88 | 0 | 1 | `residual/r6-tool-iface-tools__staged.patch` |
| `rcache-server-tools/staged` | 12 | 10 | 0 | 1 | 1 | 0 | `residual/rcache-server-tools__staged.patch` |
| `rcache-tools/staged` | 11 | 1 | 0 | 10 | 0 | 0 | `residual/rcache-tools__staged.patch` |
| `repo-doctor-tools/staged` | 26 | 23 | 0 | 2 | 1 | 0 | `residual/repo-doctor-tools__staged.patch` |
| `repo-mcp-build-tools/staged` | 29 | 10 | 2 | 17 | 0 | 0 | `residual/repo-mcp-build-tools__staged.patch` |
| `rexec-inhouse-tools/staged` | 27 | 22 | 0 | 0 | 5 | 0 | `residual/rexec-inhouse-tools__staged.patch` |
| `rexec-tools/staged` | 9 | 1 | 0 | 5 | 3 | 0 | `residual/rexec-tools__staged.patch` |
| `round2-tools/staged` | 122 | 21 | 0 | 2 | 48 | 51 | `residual/round2-tools__staged.patch` |
| `round3-tools/staged` | 77 | 6 | 0 | 27 | 22 | 22 | `residual/round3-tools__staged.patch` |
| `round4-tools/staged` | 364 | 169 | 3 | 79 | 34 | 79 | `residual/round4-tools__staged.patch` |
| `round6-tools/staged` | 772 | 208 | 3 | 59 | 241 | 261 | `residual/round6-tools__staged.patch` |
| `runtime-seam-tools/staged` | 44 | 38 | 0 | 6 | 0 | 0 | `residual/runtime-seam-tools__staged.patch` |
| `show-perf-tools/staged` | 3 | 0 | 0 | 0 | 0 | 3 | - |
| `storage-seam-tools/staged` | 21 | 19 | 0 | 0 | 2 | 0 | `residual/storage-seam-tools__staged.patch` |
| `sync-incr-tools/staged` | 6 | 4 | 0 | 0 | 2 | 0 | `residual/sync-incr-tools__staged.patch` |
| `sync-incr-tools/staged2` | 9 | 4 | 0 | 0 | 2 | 3 | `residual/sync-incr-tools__staged2.patch` |
| `sync-incr-tools/staged3` | 9 | 6 | 0 | 0 | 0 | 3 | - |
| `tax-stage-tools/staged` | 32 | 17 | 0 | 12 | 1 | 2 | `residual/tax-stage-tools__staged.patch` |
| `tax4-tools/staged` | 4 | 2 | 0 | 0 | 0 | 2 | - |
| `tax5-tools/staged` | 5 | 3 | 0 | 0 | 0 | 2 | - |
| `tax6-tools/staged` | 3 | 2 | 0 | 0 | 0 | 1 | - |
| `testfix-defect-tools/staged` | 13 | 11 | 0 | 0 | 2 | 0 | `residual/testfix-defect-tools__staged.patch` |
| `testfix-law-tools/staged` | 1 | 0 | 0 | 0 | 0 | 1 | - |
| `testfix-real-tools/staged` | 4 | 3 | 0 | 0 | 0 | 1 | - |
| `testfix-real-tools/staged-1` | 3 | 2 | 0 | 0 | 0 | 1 | - |
| `tool-direct-code-tools/staged` | 39 | 30 | 0 | 8 | 0 | 1 | `residual/tool-direct-code-tools__staged.patch` |
| `tool-direct-manifests-tools/staged` | 17 | 7 | 0 | 1 | 7 | 2 | `residual/tool-direct-manifests-tools__staged.patch` |
| `tool-iface-tools/staged` | 163 | 70 | 4 | 88 | 0 | 1 | `residual/tool-iface-tools__staged.patch` |
| `triage-r4-tools/staged` | 4 | 0 | 0 | 0 | 4 | 0 | `residual/triage-r4-tools__staged.patch` |
| `tsc-stage-tools/staged` | 24 | 7 | 0 | 3 | 14 | 0 | `residual/tsc-stage-tools__staged.patch` |
| `walkperf-tools/staged` | 5 | 0 | 0 | 0 | 5 | 0 | `residual/walkperf-tools__staged.patch` |
| `xplat-linux-fix-tools/staged` | 29 | 18 | 0 | 1 | 10 | 0 | `residual/xplat-linux-fix-tools__staged.patch` |

(4 of the 142 roots held no files at scan time and are not listed.)

## Staged dirs in OTHER ticket folders (listed only, NOT deleted)

Search: `rg --files -g '**/staged*/**'` over `.🧬semio/🦑️repo/🎫️tickets/` minus this ticket: 62 files, 0.79 MiB in 11 dirs, all in `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/`. Not checked for identity with HEAD.

| files | KiB | dir (under tickets/) |
|---:|---:|---|
| 4 | 8 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-dev-tools/staged` |
| 9 | 502 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-graph-tools/staged` |
| 7 | 21 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-ingress-tools/followup/staged` |
| 6 | 30 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-ingress-tools/staged` |
| 12 | 68 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-inputs-tools/staged` |
| 5 | 13 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-keys-tools/staged` |
| 3 | 58 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-paths-tools/staged` |
| 3 | 13 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-proctor-tools/staged` |
| 8 | 53 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-release-tools/staged` |
| 5 | 41 | `🎆️26/🌙️10/☀️20/ARCHITECTURE-COURSE-ON-BAZEL/x-site-tools/staged` |

Other copy-like dirs in C that are not named `staged*` and were out of scope (they also hold repo-source copies, e.g. `r6-tool-iface-tools/src`, `tool-iface-tools/src/files`, `integrate-tools/final*`, `lawcov-build-tools/work/pristine`, `*/src/fixture.json`): worth a follow-up sweep with the same scripts.

## Commands (all headless, nice 10, no builds)

`python3 stage-clean-tools/scan.py <root> <C> 🗑️generated/stage-clean/inv.json` -> `patch.py <root> <C> inv.json stage-clean-tools/residual summary.json` -> `delete.py <C> inv.json residual/inventory.tsv [--go]` (dry run first: 4315 to delete, 0 skipped; `--go` re-hashes each file and refuses anything not under a `staged*` dir of C). Scripts re-runnable; `🗑️generated/stage-clean/` holds only inv.json + summary.json (tool output, deletable).

## Open
- Apply scripts in the kept tool dirs still reference the removed copies; they are obsolete (all work is live edits), nothing re-runs them.
- Patches for `conc-deps-tools__staged` (13.7 MB) and `round6-tools__staged` (5.8 MB) are the largest; they hold edit snippets and pre-images that have no live counterpart.
