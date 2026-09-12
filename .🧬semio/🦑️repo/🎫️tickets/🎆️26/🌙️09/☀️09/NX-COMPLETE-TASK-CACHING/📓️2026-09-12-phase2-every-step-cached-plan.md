# Phase 2: Every Step Cached (2026-09-12)

Dev request: delete old session builds; finish until absolutely every dev/build/test/etc step is cached on package level.

## Old session builds

Deleted every `target*` dir in the dead sessions' scratchpads `9f5f6952…` (target-idle 58G, target-wgpu-boot 44G,
target-wgpu 37G, target-asm 29G, target-realloc-trace 11G, target-realloc 7G) and `9e1e818a…` (target-p3d 52G,
target-p3d-f 5.5G) — 8 dirs, ~244G. `lsof` showed only log files open (orphaned `wgpu-dev.sh` dev server on 6118 and a
`disk-watchdog.sh` loop); scripts/logs kept.

## Inventory (repo:audit projects.json, 2026-09-12 01:14)

6770 targets, 5402 cached, 1368 uncached (624 continuous).

| group | count | disposition |
| --- | ---: | --- |
| `test-exhaustive` | 250 | test-domain plugin `🧪️test/🟨️.mjs:164` hard-codes `cache: level !== "exhaustive"` → lane P1 |
| other `test-*` (report, inventory, metrics, discover, doctor, gc, clean, fixture-*) | 15 | classify per implementation → lane P1 |
| `activate-*` | 245 | dev runtime publication → cached Nx outputs the servers read → lane P2 |
| `prepare-*` (wgpu + remaining) | 126 | same → lane P2 |
| continuous `dev`/`serve`/`watch`/`start`/`native`/`daemon`/`logo` | 624 | cannot replay; must be thin and depend on cached package targets for every build they need → lane P3 audit + fixes |
| one-off tail (oracles, `*-check`, `*-source`, `*-generate`, reports, write-baseline, preview-generated, cpp, catalog-root, workspaces-write, …) | ~55 | cache every deterministic one with exact outputs; keep network/live/fuzz/bench/external-mutation uncached with a recorded reason → lane P4 |
| mutating by nature (deps, setup, clean, prune, new, publish, commit, format, purge, reset, nx) | ~50 | stay uncached (side effects are the purpose) — verified list in lane P4 report |

Launch configs: `.vscode/launch.json` 2482 configs → 2382 `nx run`, 84 script/`nx exec`, 10 raw `cargo`, 6 external
tools; `.claude/launch.json` 10 direct `📜️script.ts` dev entries → lane P5.

Shared build-dir 113G > 80 GiB budget: 94G is rustc incremental state (debug 57G, wasm32-wasip2/wasm-dev 20G,
wasm32-unknown-unknown 12G, wasm32-wasip2/debug 4.5G) → lane P6 (separate incremental budget, `-working` session guard).

## Lanes (Sonnet 5, file ownership)

- P1 test domain: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/**`.
- P2 dev runtime activation: `🧑‍💻dev/**`, `🔌️plugin/📦️packages/🟦️typescript/**`, `♻️activation/**`, `🟨️.mjs` `playgroundPreparationTargets()` only.
- P3 continuous targets outside os-dev: `📓️print/**`, `🌎️hub/**` serve/dev, `♻️mit-bestand/**`, `.storybook`, every other continuous owner.
- P4 tail + policy: `⚡️caching/🔣️policy.json` lists, `🟨️.mjs` `mutatingName`/`liveName`/`cacheableFamily`, owning `📋️project.json`s of the tail.
- P5 launch configs: `.vscode/**`, `.claude/launch.json`.
- P6 pruner incremental budget: `⚡️caching/🧹️pruning/**`, `⚡️caching/📜️script.ts` prune/report, `policy.json` `storage` + schema.

Deletion verified: no `target*` left in either scratchpad (305 MB + 110 MB of scripts/logs remain); `df -h /` free 46 GB → 250 GB.

## Final state (2026-09-12 04:10)

Lanes P1–P6, Q1–Q3, R1 reports: `📓️2026-09-12-lane-*.md`; coordinator review fixes 8–9 in `📓️2026-09-11-coordinator-log.md`.

| | start of phase 2 | end |
| --- | ---: | ---: |
| targets | 6770 | 6794 |
| cached | 5402 | 6073 |
| uncached continuous (servers/watchers; depend on cached package targets, audited by P3/Q2) | 624 | 624 |
| uncached non-continuous | 744 | 97 |

Remaining 97 non-continuous uncached, all irreducible (side effect or non-determinism is the purpose), each with a
recorded reason in the lane reports:

| category | count | examples |
| --- | ---: | --- |
| installs toolchains/dependencies | 22 | `deps-*`, `setup*`, `cpp-setup`, `oracle-setup` — write node_modules/.venv/tool stores |
| arbitrary-arg or unscoped execution | 19 | `os`, `semio`, `run`, `workflow`, `describe`, `scale-fixture`, `catalog-root`, `test-inventory`, `test-fixture-*`, energy `oracle-*` (free-form output paths / live EnergyPlus) |
| measures machine or live state | 15 | `bench*`, `perf-baseline`, `ci-baseline`, `cache-report`, `cache-verify`, `disk-report`, `audit`, `doctor`, `test-doctor` |
| publishes externally | 11 | `nx-release-publish`, `publish`, `deploy` |
| deletes state | 11 | `clean*`, `purge`, `cache-prune`, `test-gc`, `test-clean`; `clean-taxonomy-*` take caller-chosen `--plan/--resume` paths and `apply` rewrites the repo |
| rewrites source or git | 9 | `commit`, `micro-commit`, `format`, `format-fix`, `workspaces-write`, stdio `*-wiring-generate` codemods (outputs overlap inputs), `new` scaffolding |
| live services / randomized | 10 | `*-e2e` (`prepare-e2e` mints a live session), `admin-live-journey-check`, `secure-local-smoke`, `scoped-presence-browser-serve`, `stdio-fuzz`, `parity`, os-hub `browser-actor-*-check` (boot a live Vite server) |

Verification: `NX_DAEMON=false bunx nx show projects` exit 0; `repo:audit` → `projects=700 commands=7158
artifacts=8022 violations=0`; full cache-contract suite exit 0, 78 PASS (was aborting after 16).
Pre-existing unrelated breaks found and filed by lanes (not caching): os-hub `🚀️bin.rs` E0432/E0308, os-hub-admin Vite
config strip-only TS, cad `🔺️diff/🦀️.rs` syntax error, trinity_jack_shell compile break, repo-wide test dependency
ratchet (1299 findings), energy BESTEST 5 failing simulations.
