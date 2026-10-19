# Result docs-products: developer docs of course, play and demonstrator (2026-10-20)

Executor `docs-products`. Static reads plus short `bun test` runs on explicit files. No `bun repo run`, no cargo, no activation, no dependency sync, no sub-agent, no modifying git command. HARD FREEZE paths untouched (no `package.json`, `bun.lock`, library module, root `📜️script.ts`, taxonomy). The codex course ticket did not edit the course README between my reads; `git diff` of the course README shows only my hunk and my appended chapter.

## Files changed

| File | Change |
|---|---|
| `🎓️teaching/🏛️architecture/📚️course/README.md` | `## Develop` rewritten (intro, Prerequisites, Commands, The chain, dashboard registry, "Not yet verified"); the end-to-end gate gained the exact CLI call; new final chapter `## 🇩🇪️ Deutsch` (Entwickeln, Voraussetzungen, Befehle, Die Kette, Ende-zu-Ende-Prüfung, Veröffentlichen). The deploy runbook sections were not touched (`### Readiness gate`, `### Update`, `### Rollback` stay unique and in place, as the deploy law slices them). |
| `🏢️semio-tech/🎡️play/README.md` | new (EN chapter plus `## 🇩🇪️ Deutsch`), the product had no README |
| `♻️mit-bestand/🧺️demonstrator/README.md` | new (EN chapter plus `## 🇩🇪️ Deutsch`), the product had no README |
| `🎓️teaching/🛂️proctor/README.md` | `## Run` gained "Tools, prerequisites and cache" (neutral, `--tool=bazel`, cache per target, port, not-yet-verified); new final chapter `## 🇩🇪️ Deutsch` (command table and the same notes) |
| `🎓️teaching/README.md` | new `## Develop` table and pointers, plus `## 🇩🇪️ Deutsch` |
| `🎓️teaching/🏛️architecture/README.md` | one table cell now names `bun repo run @teaching/architecture-course:<target>` and the German chapter |

Convention followed: English first, one German chapter `## 🇩🇪️ Deutsch` at the end of the file, German headings that mirror the English ones, informal "du" (as in the repo README). The German course chapter covers the developer part only and says so; the operator runbook (Proof of concept by hand, Deploy and below) stays English.

## What the docs state, and where it was read

| Statement | Source read |
|---|---|
| Commands, dependencies, `cache`, `continuous`, ports of the 16 course targets, the compound `course-with-proctor`, `execution-mode` on 5 targets, `test-e2e` and `docker-stack-*` parameters | course `📋️project.json` (dumped target by target), `📜️script.ts` router |
| Cached targets: course `test`, `typecheck`; proctor `build`, `test`; play `prepare-prototype`, `activate-prototype`; demonstrator `prepare-prototype`, `prepare-release`, `activate-lanes-prototype`, `activate-prototype`; everything else uncached | the four manifests; lane tasks from `🎮️playground-targets/🟨️.mjs:215-231` (`prepare-*` driver-owned uncached, `activate-*` deterministic cached) |
| Default test levels: course `quick`, play and demonstrator `fundamental`, `proctor` argument `quick|long|exhaustive` | `📜️script.ts` of each, `resolveTestLevel` and `TEST_LEVELS` |
| Course gate: default both topologies, option order, `--` boundary, release needed for `rehearsal`, `ensureBrowser` downloads Chromium into `.🧬semio/🦑️repo/⚡️cache/tools/ms-playwright` | `🎭️e2e/🟦️.ts` (`endToEndPlan`, `runCourseEndToEnd`, `ensureBrowser`), `🏗️builder/🎚️mode/🟦️.ts` and its fixture |
| `docker-image-build` refuses non Linux x64; `checkCourse` builds the proctor | `🚀️deploy/🐳️docker-image/🟦️.ts:32`, `🚀️deploy/🟦️.ts:227-236` |
| Play and demonstrator chain, ports and env (`SEMIO_TECH_PLAY_PORT` 6033, `MIT_BESTAND_DEMONSTRATOR_PORT` 6029, `0.0.0.0` under `DEVCONTAINER=true`, port 0 for `serve-e2e`, frozen play e2e server, report directories, Playwright config, GPU switches) | `🔨️modules/🧩️runtime/📜️script.ts`, `🔨️modules/🧪️e2e/🟦️.ts` and `🎚️config/🟦️.ts` of both products |
| Play: 18 groups, 149 panes; demonstrator: 8 panes, German-locked; 35 lane tasks; 13 release jobs; `workspace:deps-wasm-opt` prerequisite | counted from `🔨️modules/🧩️runtime/🔣️.json`, `📋️project.json` `build` metadata, `🪧️brand.ts` |
| `deps-browsers` runs `playwright install chromium` into the repo tool cache | root `📋️project.json` target and the `sync browsers` branch of the native dependencies script, `repoToolCacheEnv` |
| Dashboard registry entries (readiness, `generation=fresh`, global axes) | manifests and the repo README "Command registry" (itself checked by `docs-fix`); `result-cmdreg.md` |
| Global flags, `--tool` position, `-c`/`--configuration` | `bun repo --help` run on 2026-10-20 |

## Marked "not yet verified" in the docs

- No run of any chain under Nx or Bazel (activate, serve, test-e2e, dev, deploy-check). All names, ports, cache flags and registry entries are read from manifests and scripts.
- The Rust proctor did not compile on 2026-10-19 (`result-course-img.md`: framework value crates mid-refactor); `dev`, `check`, the dev topology of `test-e2e` and the image path are therefore unrun. `test` and `typecheck` of the course need no proctor.
- Chromium 1248 and ffmpeg were not installed on 2026-10-19 (`audit-products-1019.md`), so the first `test-e2e` downloads; revision numbers are deliberately not written down (they follow the installed `playwright-core`).
- `build-system.yml` has never run; Windows/Linux paths of the setup are not run.
- Cold-run durations of the 35 play lanes and 9 demonstrator lanes are "not timed".
- `test-release` and `release-verify` of play have no dependency on `build` (they read pages published earlier): stated as is.

## Verification (exact commands, 2026-10-20)

| Command | Result |
|---|---|
| `bun test ./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎛️dashboard/🟦️.ts` (README must name `dashboard` and the project, must not contain the retired editor phrases) | 4 pass, 0 fail, before and after the final edits |
| `bun test ./🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🟦️.ts` (host and image drift in the course and proctor READMEs, target names, `### Readiness gate` / `### Update` slices; the file also runs `testCoursePreflightAlignmentV1`, which checks the README preflight tokens) | 23 pass, 1 fail: `publishes exactly a CDN artifact that bakes the proctor origin` (`siteArtifactProblems` returns two entries in another order; unrelated to any README, not touched) |
| `bun test --timeout 240000` on `🧪️tests/🏠️owner-projection-policy/🟦️.ts` and `🧪️tests/🔖️readme-current-source-revision/🟦️.ts` under `📚️library`, with `SEMIO_TEST_ARTIFACT_DIR` set in the ticket `🗑️generated` folder | 67 pass, 1 fail: `actual scoped normalization preserves reserved documentation and observes cancellation` hit its own 30 s timeout while the scoped `git ls-files` for `🧪️test/README.md` was running (reproduced 3 times at 35 to 52 s under host load; the same git command alone returns at once). No file of this task is in that scope. No approval needed re-pinning: none of the six READMEs is in the owner projection catalog or in the current-source approvals. |
| `SEMIO_WORKSPACE_ROOT=/Users/ueli/Documents/semio bun test ./♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts` | 1 pass (the `41 !== 35` mismatch of `result-bzl-man.md` is gone); with a relative root the esbuild step refuses, so the docs name the absolute path |
| Relative links of all six files (script walk), balanced code fences | all resolve, all balanced (the proctor README link prefix was wrong once and fixed to `../../`) |
| `bun repo --help` | usage printed, flags as documented |

Not run: `bun repo show projects`, `bun repo run …`, taxonomy/`verify` laws over the new files (frozen paths, long walks), the dashboard build (`semio commands --check`).

## Open items

1. The two new READMEs rely on `README.md` being a reserved basename in `🔣️taxonomy.json` (`reserved-readme-markdown`); the inventory/`path-emoji-statutes` law was not run on them. Sibling READMEs in `🎓️teaching/**` are in the same state (not in the owner projection catalog).
2. Heading anchors with emoji (`#-build-and-run`, `#-command-registry`, `#-deutsch`) follow the existing convention of the repo and are unchecked on GitHub.
3. Course `test-e2e` does not depend on `workspace:deps-browsers` (play and demonstrator `prepare-e2e` do; proposal in `result-e2e-prep.md` section 6, `project.json` owned by bzl-man). The docs describe the hidden download inside the gate.
4. When the Rust value family compiles and the first chain runs, drop the matching "not yet verified" sentences (course, proctor, play, demonstrator; EN and DE each).
5. `package.json` scripts of the three products call `bun --cwd … repo run` while `AGENTS.md` says `nx` (decision pending, deps owner); the docs follow the manifests.
6. The English-only parts of the course README (runbook) have no German version by design of this task; a full translation is a separate decision.
