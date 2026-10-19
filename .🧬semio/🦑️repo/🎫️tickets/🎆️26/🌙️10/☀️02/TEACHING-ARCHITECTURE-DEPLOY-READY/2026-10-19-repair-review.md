# Independent Repair Review — 2026-10-19

## Result

No actionable defect introduced by the reviewed repairs was confirmed by source inspection. This is a source review, not runtime or release certification.

## Reviewed Scope

Reviewed the current Flow manifest/bootstrap extraction, actual build source registry, heavy publisher consumption, new bootstrap regression and manifest schema; the outer dependencies-reader removal and retained inner bootstrap/task ordering; all proctor fixture/helper/schema changes and the architecture content parity regression; and the duplicate navigation import removal. Read the dependency, release, integration, runtime and earlier audit reports. Unrelated lockfile, dependency and coordination edits were excluded.

The new source-only bootstrap imports only system modules and authored JSON; the publication owner consumes its manifest function directly. The registry supplies the repository root required by the new bootstrap signature. The outer dependency-reader removal preserves diagnostics environment restoration and lease cleanup. The inner build owner still performs bootstrap before acquiring its task dependencies reader.

## Read-Only Verification

Executed a Python filesystem assertion against the actual Rust helper literals. The base unit helper and HTTP harness resolve to the same existing catalog under `🎓️teaching/🛂️proctor/🧪️tests/🧫️fixtures/📚️catalog/🔣️.json`. Both catalog quiz files exist; all three authored schema references resolve to the existing shared course schema. Console output with `[DEBUG]` confirmed those assertions. Searched relevant Rust/JSON/TypeScript consumers: no remaining proctor reference to the removed duplicate course fixture was found. The architecture-course path in the HTTP edge suite is a separate intended source.

No compiler, managed test, browser or build was run by this reviewer. Focused regression outcomes in other reports are attributed to their owning workers and were not independently rerun.

## Remaining Evidence

Actual managed course/core/react tests, architecture typecheck/content checks, Rust proctor unit and HTTP suites, full Flow ownership suite, static publication and both browser topologies still require their owning workers’ results. The pre-existing Flow ownership fixture drift documented in the release report remains outside this reviewed repair scope. The native Linux release environment and image/stack/runtime gates remain necessary for a fresh deployable backend claim. Pending validation must not be reported as passing.

## Follow-Up: Offline Redeployment and Release Identity

No staged `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/dist` exists at review time, so no actual CDN payload, archive checksum or backend image identity can be verified yet. This is a concrete missing deliverable, not a failed artifact check.

The CDN stager (`🚀️deploy/🟦️.ts:431`) validates the retained source, copies its selected public files and adds `_headers`. It exports neither a release identity manifest nor checksums. The backend stager (`:1069`) exports compose/Caddy files, `.env`, certificate directory, README and the two required images plus Caddy’s portable local digest tag. Its image labels are logged but no release identity/checksum record is written alongside the archive. Thus final handoff must record the actual exported file hashes and Docker image ID if transfer and deployed-release identity are to be verified; hashed asset names alone do not identify the whole CDN release. This need can be met by a handoff report; no new runtime mechanism is required.

### Confirmed Operator Documentation Gap

README `:286–288` tells an offline updater to preserve `.env`, copy only the new archive/compose/Caddy files and repeat load/up. However, `stackBundleEnvironment` (`🚀️deploy/🟦️.ts:1014`) pins `.env` to the bundle’s chosen `--tag`. If the previous bundle used tag A and the replacement uses tag B, following that update procedure leaves `PROCTOR_TAG=A`; Compose continues running A despite loading B. The documented `--tag` option makes this a supported scenario. Fix the offline update instruction to set `PROCTOR_TAG` to the new bundle tag while retaining all other overrides. No product edits were made by this reviewer.

### Exact Local Handoff Procedure

Registered build operations are `bun repo run @teaching/architecture-course:publish --configuration=release`, `bun repo run @teaching/architecture-course:docker-image-build --configuration=release`, `bun repo run @teaching/architecture-course:docker-image-check --configuration=release`, `bun repo run @teaching/architecture-course:docker-stack-check --configuration=release`, and `bun repo run @teaching/architecture-course:docker-stack-bundle --configuration=release`. These write no Git state or remote publication. Use the same explicit tag for image and bundle operations when choosing a tag.

Copy the complete CDN directory, including dotfiles, from `dist/release/pages/course` to the configured static host. For a fresh backend deployment copy all of `dist/proctor`, enter that directory, run `docker load --input proctor-image.tar`, supply a certificate if required, then `docker compose up --detach --wait` and `curl --fail https://semio.iek.uni-hannover.de/instance`. For an update first run `docker compose exec -T proctor proctor backup - > proctor-before-update.sqlite`, copy the new archive/compose/Caddy files, update only `PROCTOR_TAG` if it changed, load the archive and run the same Compose command. Preserve certificate files and existing volumes. Confirm `docker compose images proctor` names the intended image and inspect its ID against the handoff record.

The authored full browser gate already covers outage/recovery, language selection, phones, presence and CSP. This additional inspection established no further local learner runtime defect without execution.

## Follow-Up: Backup, Restore and Dashboard Operations

Inspected the proctor CLI backup/restore implementations, storage snapshot/inspect/adopt/offline guard, the real HTTP operator test, proctor operator README, architecture deployment runbook, Compose data mounts and registered proctor/course project targets. No further confirmed actionable source/README divergence was found in this scope.

The source matches the documented normal operator sequence: backup uses SQLite `VACUUM INTO`, verifies format/integrity and event-count bounds, streams binary output through stdout with progress on stderr, and removes its spool. Restore stages and verifies the input before adopting it, refuses a database held by the serving process, and replaces stale WAL siblings. The HTTP operator suite exercises backup while serving, refusal to overwrite backup files, refusal to restore while serving, restore into another directory, equality of learner/run/leaderboard/crowd/handle/course views and appending after restore. These are authored tests; this reviewer did not execute them.

Compose fixes the project name and retains named `proctor-data`, `caddy-data` and `caddy-config` volumes across ordinary update/stop/start; the runbook correctly prohibits `down --volumes`. The registered proctor targets route backup/restore/erase/prune/rebuild to `📜️script.ts`, preserve uncached operator execution, and expose the documented required restore file and erase/prune parameters through dashboard metadata. Architecture publish/image/check/bundle/readiness targets are present in the package project registry. No active `launch.json` was found in the repository file inventory; command registration here is established only through the existing dashboard/project registry, not a newly verified VS Code launch surface.

This scoped source review is exhausted. No additional edits or builds were performed. The previously reported offline tag update documentation finding remains owned by the release worker.

## Independent Review of Offline Tag Repair

Reviewed the emitted `stackBundleReadme` change, course README change, neutral bundle update vector and deployment test assertions. The emitted guide now tells an operator to preserve other overrides and certificates, set the actual new `PROCTOR_TAG`, load, start and probe. The neutral different-tag vector checks the expected selected image using the existing YAML parser. No introduced production behavior issue was found; production execution is unchanged. Tests were inspected, not run by this reviewer.

One confirmed documentation occurrence remains: README `:485–488` still independently tells offline updaters to copy only archive/compose/Caddy, load instead of the registry commands and run `up`, claiming the tag now names another image. It omits the required retained `.env` tag change for different-tag bundles. Apply the same instruction there or refer to the corrected offline update paragraph at `:286`. Reported to the coordinator. The new regression checks the first occurrence but does not detect this second divergent instruction.

## Cross-Language Negative Validation Review

Confirmed one narrow diagnostic divergence: for an otherwise valid pooled task with `pool.seconds = 0`, TypeScript `🔨️modules/✅️validation/🟦️.ts:353` reports `below-minimum` through `integer(..., 1)`, while Rust `🔨️modules/✅️validation/🦀️.rs:145` reports `value-invalid` for the same field. Both reject the document; this does not block the currently valid authored course or accept an invalid quiz. The validators document stable issue codes, so a shared negative vector should select one schema-aligned diagnostic and enforce parity. This finding is established from the explicit branches, not an executed Rust test.

Inspected proctor loading and validation aggregation, shared sheet generation (shuffle consumption, pool slot selection, sorting rotation, timed seconds), challenge rules and the existing neutral answer-validation/sheet adapters. No confirmed acceptance, sheet or submission divergence affecting the authored course was found in that inspection. Rust still must parse and run the actual course and neutral negative corpus through the managed gate before cross-language runtime parity is established. No build or test was run by this reviewer.

## Final Review of Parity and Runbook Corrections

The shared timing vector actually resides under `🧫️fixtures/🧺️balanced-pools/🩺️validation.json`. Inspected all five boundaries (0, 1, 300, 3600, 3601), the TS/Ajv test wiring and Rust unit test include paths. Read-only Python assertions confirmed both Rust include paths resolve to existing parseable JSON. Expected zero diagnostic is `/tasks/0/pool/seconds: below-minimum`; above maximum remains `value-invalid`. Rust now matches these explicit TS branches. Accepted range 1–3600 is unchanged; the repair changes only the zero-value diagnostic and therefore changes no valid authored quiz semantics. No further issue confirmed. This reviewer did not compile or execute either language’s tests.

Rechecked all relevant course README update paragraphs and emitted bundle guide. Both offline occurrences now explicitly update the retained `.env` tag while preserving other overrides and certificates. Registry instructions select the intended tag before pull and derive stack extraction from that same Compose-selected image. The earlier hard-coded latest extraction and second stale offline paragraph are corrected. No remaining concrete runbook selection divergence was found by this source inspection. No Docker or redeployment execution was performed by this reviewer.

## Final Changed-File Input and Test Wiring Audit

No new concrete missing input or test registration was confirmed. Read-only assertions verified the new Flow bootstrap owner, bootstrap test and build source registry exist in both the browser ownership target inputs and its neutral ownership fixture; the ownership test calls the new bootstrap test before application-owner checks. The bootstrap owner still has only system imports and authored JSON in its closure.

The generic build-mode test schema requires the newly added original-mode and fixed-target cases, and the test imports that schema and fixture together, validates them with Ajv, and exercises both actual generic and Nx resolvers against the SQLite oracle. This is source wiring evidence; it does not establish a passing managed resolver suite.

The course Rust project named inputs include all course Rust sources and the complete authored fixture tree. The new timing vector is included literally by the mounted validation unit test and resolves to an existing JSON file. The native source closure follows literal include references and captures embedded bytes/digests, so the vector is inside the compiler input contract when the test cfg selects that unit module. The TypeScript named inputs likewise cover the authored fixture tree. Course and architecture test routers point to their existing Vitest configurations; the changed tests remain in those selected test trees. Deployment tests import the neutral bundle fixture, and the associated course project includes the course source/fixture tree.

A focused source search reconfirmed no remaining consumer of the removed proctor duplicate fixture in the reviewed proctor/course trees. No compiler, managed build/test or retained artifact capture was executed by this reviewer. Previous full Flow ownership drift remains the separately documented existing issue; this inspection adds no passing claim for that gate.

## Current Integration Addendum After Shared Infrastructure Changes

Re-read the latest release and E2E-mode evidence and the other coordinator’s Linux backend decision. Inspected current Flow bootstrap implementation/registry, proctor fixture helper/catalog, course E2E original-mode boundaries and both corrected deployment update procedures. Read-only assertions again confirmed the registered Flow owner exists with the required root argument and imports only system modules plus authored JSON; the proctor catalog and both referenced quiz documents still exist. The bootstrap now counts Unicode characters consistently with its new schema, while retaining a finite UTF-16 precheck. No new concrete omission or integration defect was established in this review.

Current course rehearsal source still explicitly requires original release authority, builds and retains the release site artifact and validates it before serving. The generic E2E-mode correction does not remove that course boundary. The corrected registry/offline update instructions continue selecting one intended image tag for extraction/start while retaining host settings and data. Prior source findings have been corrected; no product or infrastructure source was changed by this reviewer.

The other coordinator is actively changing the Linux cache backend. Its announced directory-backed design is not runtime evidence for the previously authored course Linux image gate. This review neither reverts that work nor recertifies deployment requirements from it. Sole dependency preparation is owned by the parent (latest reported session 78699); this reviewer started no installer, compiler or managed gate. No current staged release artifact was inspected and no full test/build/redeployment pass is claimed. Pet deeds and installer diagnostic/tri-state changes were intentionally left to their assigned auditor.

## Offline Submission and Recovery Source Audit

No concrete mismatch with the documented short-shortage/local-first delivery contract was confirmed. Inspected the course React owners `🎯️targets/⚛️react/🔨️modules/📮️outbox/🟦️.ts`, `🧭️session/🏛️decision-routing/🟦️.ts`, `🧭️session/🫡️deputising/🟦️.ts`, `🧭️session/🎯️run-play/🟦️.ts`, `🧭️session/💾️restore/🟦️.ts` and settlement behavior, together with proctor `🎓️teaching/🛂️proctor/🔨️modules/🗄️storage/🦀️.rs` and the three architecture shortage/away/fault browser sources.

Answers apply locally before queued delivery. Each command has a persistent record and stable idempotency id; only superseded answers coalesce, while start/open/submit retain causal queue order within the session. A deputy submission is queued behind outstanding commands, and proctor views cannot overwrite held decisions while that run remains ahead. Restore reads held run records and the outbox; retries release a record only after a terminal verdict. The proctor storage explicitly commits receipts/events/outbox in one authority transaction and keeps rebuildable views separate. This inspection establishes source intent, not actual disk durability or successful network recovery.

The authored `🧪️tests/🔌️connection-shortage/🟦️.ts` checks partial answer reload and resumed delivery; `🧪️tests/📴️proctor-away/🟦️.ts` checks first-arrival registration/run/result/badges on the device, reload, reconnect, server-derived standing and a fresh device resolving the same identity/result; `🧪️tests/🛟️proctor-faults/🟦️.ts` covers repeated expert-task outages, submission-time failure and contract mismatch. These are the minimum meaningful runtime acceptance cases after dependencies settle: run all three against actual proctor stop/start in both configured topologies and require fresh-device result/badge assertions, not merely the local success screen or empty queue. The authored gate already supplies this acceptance path; no additional feature or speculative test was proposed. None of those browser/runtime cases was executed by this reviewer.
