# result-cicover (lane cicover `cv`, wave 1027b, 10-27 ~18:30): CI coverage staged, nothing live edited

`C` = this ticket folder. Tools `C/cicover-tools/`. Batch `ci1` is staged (12 files, base = live of 18:14, `stage.py check` says all `base`). Nothing under `.github/` or any other tracked path was edited, no product run, no sub-agent, no modifying git command. Mirror `cv1` (mine, one) was used for every law and is dropped at the end. Committing and pushing is the user's decision: every `.github` file is still only in the index/working tree (`M`/`A`, never committed), and CI can only run what is pushed.

## FOR coordinator
- Apply: `python3 -I C/cicover-tools/staged/ci1/stage.py check|apply|revert` (keeps modes; `python3 -I C/xplat5-tools/tests/test_stage_mode.py C/cicover-tools/staged/ci1/stage.py` OK).
- Rebuild on the current live bases (hot files `build-system.yml`, `dashboard.yml`, the workflow law): `python3 -I C/cicover-tools/mkbatch.py --batch ci1` (anchored edits; a moved anchor stops the build with exit 2). Mirror: `M=$(sh C/cicover-tools/mirror-up.sh); python3 -I C/cicover-tools/mirror-apply.py C/cicover-tools/staged/ci1 $M`.
- Laws (mirror only): `sh C/cicover-tools/staged/ci1/verify.sh $M --remote` (workflow laws + CI report tests + dashboard CI test, ciwf `run-law.sh`, direct-tool law, `cover-check.py`, 9 negative controls, stage mode law). Result today: exit 0.
- No product run proves this batch (the only proof is a run on GitHub). After landing, commit and push; the first pushes to `⛳wip` and the first pull request start every workflow. FOR the user after the first run: read the job summaries of `go / launcher (windows)` (the Windows evidence lines) and flip the advisory rows listed below to blocking when they are green.
- Land WITH the rest of `.github` (the workflow law and the build-system fix `jobs.bench.env` are only in the index today).
- Pre-existing red, not mine: the dashboard `🏗️ci` law "CI Source gate" fails on live (project.json inputs of `test-ci-source` gained `{projectRoot}/📋️project.json`; the fixture lists 9 inputs, the target has 10). Owner: td-products/dashboard lane; the fix is one fixture entry plus the count `inputs:9` in `🏗️ci/🟦️.ts`.

## What the batch changes (12 paths: 9 edited, 3 new)
| path | change |
|---|---|
| `.github/workflows/build-system.yml` | trigger `push.branches: [main, "⛳wip"]`; dispatch `base` default empty = the default branch (`DEFAULT_BRANCH` env of the plan step, `origin/$DEFAULT_BRANCH`); runner images pinned (`ubuntu-24.04`, `macos-26`, `windows-2025`); `suites` and `devcontainer-suites` 350 -> 240 min, `devcontainer` 120 -> 90; every `tools setup`/`dependencies sync` step `timeout-minutes: 30`, both cache proofs 45; Bun store restore added to `storage` and `host` (they synced dependencies without it); header documents runners, timeouts, caches |
| `.github/workflows/dashboard.yml` | same trigger and pins; job timeouts 360 -> 180; `SEMIO_BUILD_BUDGET_MS` 21,600,000 -> 9,000,000 (150 min < 180); Go build cache restore/save around the native bootstrap |
| `.github/actions/repo-launcher/action.yml` | restores/saves the launcher runtime cache (`<user cache>/semio/runtimes`, where `Ensure` downloads Bun) and the installer's Go toolchain (`~/.local/share/semio/go`, `%LOCALAPPDATA%\semio\go`) through the new action; key from the runtime registry and the bootstrap pins |
| `.github/actions/devcontainer-suites/action.yml`, `devcontainer-repo/action.yml` | new input `budget-ms` (210 min / 150 min) replaces the literal 360 min `SEMIO_BUILD_BUDGET_MS`; the budget is now below the job timeout (`devcontainer-suites` was killed at 350 min, 10 min before its own budget) |
| `.github/dependabot.yml` | `/.github/actions/tool-cache` joins the github-actions directories |
| NEW `.github/actions/tool-cache/action.yml` | restore / save one tool cache; key `<name>-<os>-<arch>-<hash>`, prefix fallback; saves only on the default branch (push or dispatch), only when the exact key is absent (`lookup-only` first) |
| NEW `.github/workflows/go.yml` | `launcher` (Linux, macOS, Windows, blocking): `repo run @semio-tech/repo-launcher-go:test --skip-cache`; Windows adds three evidence steps (never fail the job); `modules` (services = every `@semio-tech/repo-*-go` except the launcher, 22 projects incl. `repo-yaml-go`; clients = `repo-client`, `repo-mcp`, `repo-coordinator`): Linux/macOS blocking, Windows advisory |
| NEW `.github/workflows/products.yml` | 20 rows of `repo run-many` (below) |
| `🧪️tests/🔀️workflow/🟦️.ts` (law) | pinned runner labels, 7 new laws (below) |
| dashboard `🏗️ci/🔣️.json` + `🧬️schema/🔣️.json` (2 paths) | the 3 host runner names/consts follow the pins |

## Coverage map (product family -> workflow, job, platforms, blocking)
| family | CI after the batch |
|---|---|
| build system (`repo`, `repo-lib` route tables) | `build-system.yml`: smoke/storage/host x 3 OS, `suites` 21 shards x 3 OS (Windows included, `platforms` verdict reports SKIPPED never green), `devcontainer` (must-run) and `devcontainer-suites` 5 shards on Linux, affected, parity, bench. Unchanged in scope; Windows has no devcontainer (hosted Windows cannot run a Linux container), the `build` table is library logic and runs on the plain hosts only |
| launcher Go | `go.yml/launcher` Linux, macOS, Windows, blocking (was: no workflow) |
| other Go (22 `repo-*-go`, client, mcp, coordinator) | `go.yml/modules`; Linux and macOS blocking, Windows advisory |
| dashboard | `dashboard.yml` (3 OS + devcontainer, journeys/load/cold start/windows/process deadline), unchanged scope, now fires on the default branch |
| course (typecheck, check, build, test) | `products.yml/course` Linux blocking, macOS and Windows advisory (course `test` additionally runs on all 3 OS in `smoke`'s cache proof) |
| course libraries (framework, course-react, pets, pets-react, ui-react) | `course-libs` Linux blocking, macOS/Windows advisory |
| course Rust (ui-contract-rs, course-rs, pets-rs, ui-rs), framework Rust (framework-server, course, framework-rs, framework-actor-rs), ui-styling | `course-rust`, `framework-rust`, `ui-styling`, Linux blocking |
| proctor | `proctor` Linux blocking (a server) |
| repo Rust crates (24 `repo-*-rs`) | `repo-rs-1/2/3` Linux blocking |
| print | `print-rs` (Linux blocking, macOS/Windows advisory), `print` check/build/test, `presentation` (+react), `bericht` (3 reports), `projektetage` (deck) Linux blocking, TeX bundle and fonts cached |
| course release | `architecture-course.yml` unchanged (manual); it already pins `ubuntu-24.04` |
| NOT covered (reason) | os shell and plugin registry, hub, 304 `s` plugins, semio-tech play, mit-bestand demonstrator, leutwiler: no green run on the official route exists (activation of play/demonstrator never completes, hours; os/hub never run on Bazel), so a row would be permanently red. They still run through `affected` when a change touches them. Rows can be added to `products.yml` in one line each after a first green gate run |

Gate products (prodmap Tier A): course build+test = `course` row; print-rs = `print-rs` row; dashboard = `dashboard.yml`. Expanded jobs per full run: build-system 43, dashboard 4, go 9, products 20, course release manual.

## Triggers, pins, timeouts, caches
- Triggers. `on.push.branches` named `main` while the default branch is `⛳wip` (U+26F3, checked against `gh api` and `origin/HEAD`). The five failing runs were pushes to `🐙ueli/⛳wip` (headBranch), not to `⛳wip` as `result-ciwf` wrote; personal-branch pushes stay without CI (pull request or manual dispatch covers them), `⛳wip` is reached by merges (last on origin: 06-04), so the push runs are few. The four gating workflows now fire on a push to `main` or `⛳wip`; the dispatch comparison base defaults to the default branch.
- Runner images (README of actions/runner-images read today): `ubuntu-latest` = `ubuntu-24.04` today and is announced to move to 26.04, `macos-latest` = `macos-26` (arm64), `windows-latest` = `windows-2025`. Pinned to exactly what the aliases mean today, so behaviour does not change now and a later migration is a reviewed diff. A law rejects any `*-latest` label.
- Timeouts. The suite policy (`maximumShardBudgetMinutes 180` + `suiteSlackMinutes 10`) bounds a shard at 190 min; the sweep jobs now have 240 (law: between 210 and 300). Provisioning steps 30 min, the two cache proofs 45, product rows 60-180 by the measured warm figures of `result-prodmap.md` (course 120, print-rs 150, proctor 150, repo crates 150, presentation 60). Dashboard 180 (was 360). `affected` stays 120: a range that needs more is an incident, not a timeout.
- Caches. Existing: Bun store (smoke saves, others restore; storage/host/new jobs now restore too), the local task cache of `affected`, the bench baseline. New: launcher runtimes + installer Go (every job, through the action), Go build cache (`go`, `dashboard`), TeX bundle + fonts (print rows). All four directories are loose trees of the cache ledger (`⚡️cache/go`, `tectonic`, `print-fonts`) plus the Bun store path the README documents (`cover-check.py` compares them with `🔣️policy.json`). Deliberately NOT cached: the partitions (`.cache/b/t` tools, `.cache/b/p` Bazel, the Rust partition with the 48 GiB unit cache): on macOS they are mounted sparse images, the README says even the Bun store must be restored after `tools setup` because the partition has to be attached first, and a restored tree could meet a half-made partition; the GitHub cache limit is 10 GB per repository (0 used today, `gh api .../actions/caches`). Experiment for later, after one green run per OS: restore `.cache/b/t/m` after `tools setup` and compare the second run's setup time. Docker layer caching of the devcontainer image needs a registry push and `packages: write`, which the read-only policy forbids.

## Windows checks of `result-windows.md` (7)
| # | check | state |
|---|---|---|
| 1 | launcher Go tests incl. the five Windows laws | `go.yml/launcher (windows)`, blocking (the Windows-only laws appear when batch `winlaunch` lands; nothing fails before) |
| 2 | root spelling end to end (lower-case drive, upper-case path) | evidence step: counts the state directories before/after `repo service status` from the other spelling; fails the step (not the job) when a second directory appears |
| 3 | installer replacing a running launcher | evidence step: holds a `repo tools exec node` process, runs `🔵️.ps1`, expects exit 0 and `repo.exe.old-*`, runs it again and expects none left (needs `wininstall`) |
| 4 | `.cache/b` swap vs the running runtime | NOT staged: no command asks for a partition detach; needs a tiny repo verb or a Go test hook first |
| 5 | rename over an open file under Bun | evidence step (one-liner through `repo tools exec bun -- -e`) prints `renamed` or the error code into the summary |
| 6 | timing | evidence step prints the CIM process snapshot and `repo --version` in ms; the three hot no-op runs are not added (the `smoke` cache proof already prints the second run per OS) |
| 7 | RDCW overflow, NTFS index above 2^53 | not automatable in a workflow step |

## Laws and evidence (all on mirror `cv1`)
- New laws in the workflow law file (7 tests: runner pins, SHA pins incl. composite actions and local action existence, push triggers + dispatch base, timeouts + sweep window + provisioning limits, launcher on three OS + Windows evidence non-blocking + Go row shape, cache saves only on the default branch, artifact names per workflow). RED on the base workflows: 6 fail (the 4 policy laws, the launcher/Go law, the cache law; the pin law and artifact law pass on base by design). GREEN with the batch: workflow law 14/0; CI report tests (dry-run render of the five scenarios, plan script, pins) green; dashboard CI test 4/1 where the 1 is the pre-existing red above.
- Negative controls (`negative.py`): 9 seeded violations (alias label, tag-pinned action, push filter without `⛳wip`, sweep job at 350, provisioning step without limit, Windows evidence step that can fail, launcher on two OS, cache saved by every run, duplicate artifact name), every one fails exactly its law, file restored.
- actionlint-style: ciwf `run-law.sh` (TS context-availability law, regex model, schemastore schema oracle): 0 findings on all 5 workflows + 6 actions; no `runner` context in job env; every `uses` SHA pinned (9 distinct, all existing upstream, checked with `gh api`); local action paths and inputs exist; direct-tool-declarations law over `.github` (live library code): 12 files, 0 findings; project/target facts (`cover-check.py`): 26 rows, every named project exists in a tracked manifest with every named target, `@semio-tech/repo-*-go` expands to 22 (23 minus the launcher), 4 cache directories equal ledger areas.
- actionlint itself is not installed (not pinned in the tool registry; I did not download it).

## Not verified (needs a real runner)
Everything that runs on GitHub: memory of the heavy Linux rows (cold print-rs peaks near 16 GB on a 16 GB runner; the repository variable `SEMIO_LARGE_LINUX_RUNNER` swaps the label of the nine heavy Linux rows, unset = `ubuntu-24.04`), the 7 GB macOS runner on advisory rows, disk (every product row prints `df -h`, floor 8 GiB), `repo run-many` accepting the wildcard + `!` negation list with Bazel and `--parallel=2`, `tools setup toolchains` on a fresh runner, the cache restore into emoji directories and `~` expansion on Windows, `lookup-only` in the pinned `actions/cache` v6.1.0 (the input exists in its `restore/action.yml`), PowerShell syntax of the Windows evidence (no `pwsh` on this machine), whether `repo service start` returns without holding the step's handles (the step has a 10 min limit and a `finally` that stops the service).

## Open decisions
1. Path filters: pull requests that only touch `.🧬semio/**` (ticket folders) or `*.md` still start ~80 jobs; a `paths-ignore` on the four workflows is one line each if wanted (I did not add it: it also hides docs-law regressions).
2. Advisory rows (macOS/Windows of course, course-libs, print-rs; Windows of Go modules) go blocking after a first green run: remove `advisory: true` / the flag.
3. Docs lane: the README CI paragraph (EN near line 590, `In CI`, and the DE twin) still describes a two-job `products` workflow that does not exist; proposed EN text: "CI runs `build-system.yml` (smoke, storage, host laws, suite sweep, devcontainer, affected, parity, bench), `dashboard.yml`, `go.yml` (launcher laws on Linux, macOS, Windows; every `repo-*-go` project) and `products.yml` (course, proctor, print, repo crates) on every pull request and every push to `main` or `⛳wip`; runner images are pinned." DE: "Die CI führt `build-system.yml` (Smoke, Speicher, Host-Gesetze, Suite-Durchlauf, Devcontainer, affected, Parität, Bench), `dashboard.yml`, `go.yml` (Launcher-Gesetze unter Linux, macOS, Windows; jedes `repo-*-go`-Projekt) und `products.yml` (Kurs, Proctor, Druck, Repo-Crates) bei jedem Pull Request und jedem Push auf `main` oder `⛳wip` aus; die Runner-Images sind fest eingebunden."
4. Windows `W8-W11` and the Windows installer remain with their own batches (`winlaunch`, `wininstall`, `fleetpy`); the Windows evidence steps pick up their effects once they land.

## Files
`C/cicover-tools/`: `mkbatch.py`, `src/` (new files, `law-additions.ts`, `verify.sh`), `staged/ci1/` (manifest, files, base, `stage.py`, `verify.sh`), `mirror-up.sh`, `mirror-apply.py`, `negative.py`, `cover-check.py`, `scan-direct.ts`. `C/🗑️generated/cicover` removed.
