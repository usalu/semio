# Result: x-release (P1-1, build mode axis under Bazel)

Status: implemented and verified by focused tests. The generic Bazel part is STAGED (adapter frozen); the course part, the invocation message and the README are LIVE.
Paths: `BZ` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel`, `BUILD` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build`, `SITE` = `🎓️teaching/🏛️architecture/📚️course`, `T` = this ticket folder.

## 1. Finding: the audit's P1-1 premise is only half right

How the mode travels (same for Nx and Bazel, one code path):
1. `resolveBuildModeV1` (`BUILD/🧭️invocation/🟦️.ts`) reads every public form (`--configuration=release`, `--configuration release`, `-c release`, `-c=release`, `:release` task suffix, `SEMIO_BUILD_MODE`), refuses disagreement, and exports `SEMIO_BUILD_MODE`.
2. `createBuildBootstrapCapabilitiesV1` / `createNativeBuildCallerCapabilitiesV1` (`BUILD/📥️caller`) store it as `buildMode` in the caller capabilities (tool `nx` or `bazel`). That is the authority.
3. `buildTaskEnvironmentV1` (`BUILD/📥️caller/🟦️.ts`) overwrites `SEMIO_BUILD_MODE` of every task environment with `capabilities.buildMode`; the course reads it with `readNativeCallerAuthorityV1(...).original.buildMode` (`SITE/🏗️builder/🎚️mode/🟦️.ts`), which accepts the Bazel caller (`tool` present, `buildCallerSchema`).
4. The Bazel fingerprint already contains the mode (`taskFingerprint`: `mode: original.SEMIO_BUILD_MODE`), pinned by the existing `📥️fingerprint` suite (cache groups differ by grant mode).

Why the audit saw "Unknown build configuration": the manifests declare no `configurations`, but graph inference adds `prototype` and `release` plus `defaultConfiguration` to EVERY inferred target (`targetBuildModes`, `BUILD/🎚️mode/🟨️.mjs`, applied at `BUILD/🕸️graph/📥️inference/📦️project-nodes/🟨️.mjs:149` and `🧱️project-assembly/🟨️.mjs:231`). So for inferred course targets `publish:release` should plan fine. That is NOT proven by a run (see 5). The adapter still failed closed for any target without declared configurations (synthetic `repo-exec`, unavailable projects, hand-made graphs) and silently dropped `request.configuration` there. Design decision, per the README ("Choose the compiler configuration with --configuration=prototype|release"): build mode is a repository-wide invocation axis, not a per-target configuration. Every tool honors it for every task.

## 2. Design (revised after review R1-R5)

- Axis: `prototype|release` (`buildModesV1()`), resolved once from the invocation, held as the caller's `buildMode`. The caller's mode reaches the planner as `request.authority`.
- Entry points bind the authority (`bazelAuthorizedRequestV1`, reads `capabilities.buildMode` of the issued invocation): `runBazelTasksV1`, the orchestration planner injected into `withBuildOrchestrationTaskEnvironmentV1`, and the dry-run plan in `BUILD/📜️script.ts`. `planBazelTasksV1` itself stays pure and defaults the configuration to `request.configuration ?? request.authority`. (R1: the selection module is NOT changed: its return value is deep-equality-tested in several suites, and the request already had a `configuration` slot that is always empty in real runs.) Result: every planned task of every target, declared or not, carries the mode in id, label and, through `selected`, the workspace namespace.
- Undeclared targets (no `configurations` at all): the mode is their only configuration; `bazelTargetV1` exports `SEMIO_BUILD_MODE=<mode>` into the task's `execution.env`. Targets that declare some configurations keep Nx's rule (a requested configuration they do not declare falls back to `defaultConfiguration`; `:release` on them is refused), so the existing Nx dependency-parity corpus still holds (12/12 cases re-run against the emitted adapter).
- Plan-time refusals, EN then DE, prefix "Conflicting build modes" (`requireBazelModeAdmissionV1`, called in `include` before anything is fingerprinted): (R3) a task whose mode differs from `request.authority`; (R2) a task, explicit or reached as a dependency, whose target is fixed (`metadata.semio.buildMode.fixed`, for example `dev` = prototype) to another mode than the one the task would run with. The run-time check in `issueBazelTaskEnvironmentV1` stays as defense in depth.
- Action key: unchanged and sufficient (mode in task id/label, manifest and fingerprint).
- Unknown configuration names are refused in EN and DE ("Unknown build configuration" / "Unbekannte Build-Konfiguration").
- R5: only `prototype` and `release` can be selected at the command line (`resolveBuildModeV1` runs `readBuildMode` on every value); other configuration names (the `staging` fixture rows) exist only in the adapter. Stated in the README text.
- Risk to check with a dry run before relying on R2: a release plan whose graph reaches a mode-fixed target (for example Play/Demonstrator `test-e2e` -> `serve-e2e`, fixed prototype) is now refused at plan time where it used to run silently with the caller's mode. No manifest pins a mode in `dependsOn`, but inferred edges may. Run `--dry-run -c release` for those `test-e2e` targets after applying.
- R6 (English-only `Build mode must be prototype or release` in `🏃️process/🏗️build/🎚️mode`, `Explicit compiler configuration value required` in the invocation module): not changed, outside the P1-1 files; recommend a follow-up.

## 3. Files

LIVE (done, before the review; nothing live was edited since):
- `SITE/🏗️builder/🎚️mode/{🟦️.ts,🔣️.json,🧬️schema/🔣️.json,🧫️fixtures/🔣️.json}`, `SITE/🧪️tests/🎚️mode/🟦️.ts`, include entry in `SITE/🧪️tests/🎚️config/🟦️.ts`.
- `BUILD/🧭️invocation/🟦️.ts` (`buildModeConflictMessageV1`, EN then DE, names fixed targets) with `BUILD/🧭️invocation/🧪️tests/{🟦️.ts,🔣️.json,📋️schema.json}`.
- `README.md` lines 164/515: currently V1 wording, which OVERSTATES (it says the adapter accepts undeclared modes and refuses mismatches). `apply.py --readme-now` restores the true-now wording (bilingual conflict message + course gate only); allowed as a tiny edit once the tree is no longer quiet.

STAGED under `T/x-release-tools/staged/` (nothing applied):
- `mode_patch.py`: 10 anchored adapter edits (import, request.authority, identity + `bazelModeOnlyV1` + `bazelAuthorizedRequestV1` + `requireBazelModeAdmissionV1` + `bazelTargetV1`, plan, explicit, select, admission, run entry, orchestration planner, environment) + 1 edit in `BUILD/📜️script.ts` (dry-run plan) + the three README states (original, now, full).
- `apply.py [--check]` (two-phase: reads and computes everything, refuses without writing on a missing/ambiguous anchor, then atomic replaces; also installs `tests/📥️mode/` into `BZ/🧪️tests/📥️mode/`) and `apply.py --readme-now`.
- `tests/📥️mode/{🟦️.ts,🔣️.json,🧬️schema.json}`, `check.sh`, `emit.py`, `planner-parity.ts`.
- Harness (in `T/x-release-tools/`): `vitest-as-bun.ts`, `no-nlink-revision.ts`.

## 4. Tests and results

- Course (live): `bun test --preload vitest-as-bun.ts [--preload no-nlink-revision.ts] SITE/🧪️tests/🎚️mode/🟦️.ts`: 20 pass.
- Invocation (live): 3 existing + 1 new test pass (with the nlink stand-in while the mirror exists).
- Adapter, `sh check.sh live` (red) vs `sh check.sh patched` (green), test-first. Fixture rows: declared, undeclared, partial (only prototype declared) and pinned (fixed `dev`) graphs; suffix, flag, authority and combinations; refusals; authority from a real issued Bazel caller; mode never shares id/label/namespace. Independent checks: Ajv strict on the fixture, `bun:sqlite` recursive CTE for mode inheritance.
  - live: 31 pass / 21 fail (undeclared modes throw or are dropped, no German, no plan-time refusal, mismatch accepted).
  - patched: `📥️mode` (33 tests) + regression `📥️fingerprint`, `📥️actions`, `📥️coverage`: 52 pass / 0 fail (2 skipped need `SEMIO_TEST_ARTIFACT_DIR`); Nx-derived planner corpus (`planner-parity.ts`, the Bazel half of `🕸️dependencies`): 12 pass / 0 fail. `check.sh` ends with `apply.py --check` (all anchors found, including `BUILD/📜️script.ts`).
- `apply.py --check`: 3 pending targets, tests 3 of 3 to install. `mode_patch.readme()` verified idempotent for states now/full and full -> now.
- Logs: `T/🗑️generated/x-release/check-live*.log`, `check-patched*.log`, `check4-*`, `check5-*`.

## 5. Environment hazards found (not caused by x-release)

- x-inputs' hardlink mirror (`T/🗑️generated/x-inputs/mirror`) gave ~1,580 live files nlink=2; the Source revision reader (`BUILD/📥️inputs/🔏️revision/🟨️.mjs:27`) refuses them: every `bun repo` run and every bare `bun test` of anything that imports it dies with "Bounded physical Source revision required". Reported in `coordination.md` and to the coordinator. `check.sh` detects it and preloads `no-nlink-revision.ts` (a stand-in with the same return shape minus the nlink rule) so my tests still run; remove the mirror and the preload becomes unnecessary (check.sh then runs without it).
- Consequence: the `--dry-run` proofs below were NOT obtained. Logs: `T/🗑️generated/x-release/dry2-*.log` (the earlier 14-minute run died on the storage allocation fault, fixed by a peer at 15:47, then on the nlink fault).

## 6. Apply order and verification (coordinator)

1. Clear the nlink hazard (see 5) and wait for `show projects` green and the freeze to lift.
2. README first, any time the tree is not quiet: `python3 -I T/x-release-tools/staged/apply.py --readme-now`.
3. `python3 -I T/x-release-tools/staged/apply.py --check` (expect 3 pending), then without `--check`; run again (expect nothing pending). Rebase `mode_patch.py` if an anchor moved (the script refuses and writes nothing).
4. Tests: `sh T/x-release-tools/staged/check.sh patched` before applying, or after applying `bun test ./<BZ>/🧪️tests/📥️mode/🟦️.ts ./<BZ>/🧪️tests/📥️fingerprint/🟦️.ts ./<BZ>/🧪️tests/📥️actions/🟦️.ts ./<BZ>/🧪️tests/📥️coverage/🟦️.ts`; `bun test ./<BUILD>/🧭️invocation/🧪️tests/🟦️.ts`; course: `bun repo --tool=bazel run @teaching/architecture-course:test -- 🧪️tests/🎚️mode/🟦️.ts`.
5. Plans (expect `…:publish:release` tasks with release dependencies and `selected` = the release id): `publish:release`, `publish --configuration=release`, `publish -c release`, `SEMIO_BUILD_MODE=release … publish`, each with `--dry-run`; the same for `test-e2e`, `docker-image-build`, `docker-image-publish`, `deploy-check`. Also dry-run `-c release` for the mode-fixed chains named in section 2 (R2 risk) and a prototype `publish` real run for the EN+DE gate refusal.
6. A real `publish:release` run is the only proof that the gate passes inside the Bazel action.

## Backend edits

- `BUILD/🧭️invocation/🟦️.ts` (not in the freeze list): message only, plus tests. Live adapter, `BUILD/📜️script.ts` and selection untouched; everything else is staged.

## Open points

- Nx for a target without declared modes was not examined; inference declares them for all inferred targets.
- Windows/Linux not run; the patch is pure TypeScript.
