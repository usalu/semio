# Q2 report: stepladder residual and dependency ensure (2026-11-01)

## 1. Stepladder `climb-and-work@1.500: force residual -5.63` (fixed)

Root cause: engine, not the asset binding and not nondeterminism.
- The check is deterministic (no random sampling). The sample instants are `durationS * i / (samples - 1)`. The default is 9 samples, which includes t = 1.5 s of the 12 s clip. The runs that "passed" used `--samples 5` (t = 0, 3, 6, 9, 12), which skips t = 1.5. That explains the "passes alone / fails in the category run" reports.
- The clip has `smooth` tracks `load:occupant` (keys 0, 0, 1 ...) and `load:lateral-thrust`. `smooth` is a Catmull-Rom spline, so it undershoots below the flat 0 keys before the rise. At t = 1.5 the scale was about -0.07. The occupant load (-981 N) became +69 N (upward) and the thrust became -5.6 N, so both feet had to pull, the rear foot lifted, the front foot slid and the residual stayed unbalanced. Other track targets (operating, emitter, region fractions, activityMet) were already clamped to their domain. Only `load:` scales were not.
- Fix: `applyTracks` in `🎞️animation/🟦️.ts` now uses `loadScales[id] = Math.max(0, value)`, and the module doc says so. Asset JSON is untouched.
- Regression test: `🧪️tests/🎞️clip-sampling/🟦️.ts` ("never lets spline overshoot of a rising load track reverse the load"; it first asserts that the raw spline does go below 0), plus a scenario in `🥒️.feature`.
- Verification: `--asset stepladder` (9 samples) is green. The full catalog check (all categories, default 9 samples, physics on): `✓ manifest` plus 131 `✓` assets, 0 `✗` (all four of Q1's category fixes included).

## 2. `repo dependencies ensure` fails with `Current immutable input capture work refused` (not our footprint)

Reproduced: `repo dependencies ensure` ran 16 min (06:17 to 06:33) and ended with the same error at `💾️storage/🟦️.ts:3041` (`captureJavascriptDependencyDescriptorInputsV1`, trace via `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts:2172/2260/2316`, `withArtifactWindowsRelativeResourceV1`). Preceded by `Store dependency sync stale ... reason: absent`.

Evidence that it is infrastructure, not this ticket:
- The function captures the dependency Store group: `directory = store.root` and the in-memory `plan.files` of the dependency scopes (see `material()` near line 633). It never walks the staffage catalog, the ticket folders or `.🧬semio/🎓️teaching/⚡️cache/staffage` (2.4 GB, only read by the staffage cache code).
- `limits = SourceOwnerStoreProtocol`: `maximumWork` 4194304, `maximumBytes` 64 MiB, `chunkBytes` 64 KiB, at most 64 roots and 512 files per root. Even 64 MiB of reads is about 2 K work units (two checks per chunk). Reaching 4 M therefore needs a runaway count of per-call filesystem facts (the `check` is also passed as the per-OS-call callback of `withArtifactRelativeDirectoryV1`) on the Windows relative-resource path, not file volume.
- No `package.json` or `bun.lock` is changed in the working tree (the git status holds no manifest changes), so the scope inputs are unchanged by this ticket.
- Not changed: no limit raised, no bootstrap edit. Owners of `📦️dependencies/💾️storage` and the Windows descriptor observation module need to look at the work accounting in `withArtifactWindowsRelativeResourceV1` / `facts`.

## 3. Staffage suites run without the ensure (through `repo tools exec bun -- x vitest`)

`d3-format` and `@semio-tech/framework-transient` stay unlinked in the root `node_modules`. Two ticket-local vitest configs add only resolve aliases (d3-format to `node_modules/.bun/d3-format@3.1.0/...`, framework-transient to `🧰️framework/🔨️modules/🫧️transient/🟦️.ts`) on top of the real configs:
- `q2-vitest-alias.config.ts` (library): run from `🧍️staffage/📦️packages/🟦️typescript`: **51 of 51 files, 954 tests pass, 10 skipped** (incl. `print-size`).
- `q2-vitest-react-alias.config.ts` (React): run from `🎯️targets/⚛️react/📦️packages/🟦️typescript` (the support helper uses `process.cwd()`): **17 of 17 files, 107 tests pass, 1 skipped**.
- Command: `repo tools exec bun -- x vitest run --config <T>/q2-vitest-alias.config.ts` (or the react one).

React fixes needed once the imports resolved:
- `🧪️tests/🗂️grid-scroll/🟦️.tsx`: the root grid expected a hard-coded 12 cells; it now expects `ROOT_PAGES.length` (the appliances category made it 13). Import of `ROOT_PAGES` added.
- `🧪️tests/🎚️config/🟦️.ts` (React): `testTimeout` and `hookTimeout` 180 s, like the library config, because the phone-swipe case exceeded the 5 s default on the loaded host.

## Files
- `🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🎞️animation/🟦️.ts`
- `🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎞️clip-sampling/🟦️.ts` and `🥒️.feature`
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🗂️grid-scroll/🟦️.tsx`
- `🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react/🧪️tests/🎚️config/🟦️.ts`
- Ticket: `q2-vitest-alias.config.ts`, `q2-vitest-react-alias.config.ts`, this report. `T/🗑️generated/q2/` deleted.

Not re-run: `tsc --noEmit` (the missing-package errors stay until the install links them or a paths mapping is used).
