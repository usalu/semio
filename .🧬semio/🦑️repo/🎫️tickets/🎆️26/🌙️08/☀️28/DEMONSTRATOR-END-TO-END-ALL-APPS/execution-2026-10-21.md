# Demonstrator Runtime Execution — 2026-10-21

The declared `repo run @semio-tech/mit-bestand-demonstrator:test-e2e --output=stream` was started with `SEMIO_TICKET_DIR` set to this ticket. The first observed blocker is native capacity admission: the run requires 3584 MB above the 6553 MB reserve; the host initially had less than zero spare capacity and three active tasks. The admission queue remains owned by `repo`.

The E2E graph declares profile preparation, exact component-union activation, browser preparation, an invocation-specific continuous Vite service, and eight-pane Playwright coverage. The consumer supplies its generation-specific readiness URL and directs artifacts beneath this ticket’s `🗑️generated` directory. No browser acceptance result is claimed until the declared graph runs.

## Explicit Site Publication Profile

The `build` graph prepares release artifacts, while the site implementation reads the strict `SEMIO_BUILD_MODE` environment contract for Vite and every staged artifact path. The site target previously did not declare that environment value. Added the language-neutral pipeline fixture’s `site.profile` assertion to the runtime catalog regression, then made the target explicitly supply `SEMIO_BUILD_MODE=release`. This prevents a caller’s prototype intent from selecting a different artifact profile than its declared prerequisites. The runtime-catalog command is also waiting at native admission; no test pass is claimed yet.

Mode injection was traced to `repo` target normalization: `dev`, `serve`, `prepare-e2e`, and `serve-e2e` are explicitly fixed to prototype, so their environment is correct. Generic `build` is normalized to prototype and that generated configuration overrides the target option environment. The initial explicit site environment declaration therefore also requires a graph-owner fix to honor declared default compiler mode. This is coordinated with the integration owner before implementation.

## Compiler Workflow Declaration

The graph normalizer now preserves an authored `defaultConfiguration` and accepts schema-declared `metadata.semio.buildMode.fixed`. Named fixed workflows retain precedence over a soft default and reject conflicting authored fixed intent. Generic targets remain prototype by default. The Demonstrator site publication declares release as both its fixed workflow and its default; development serving remains fixed prototype through existing target-name rules.

A language-neutral case matrix was added to the existing invocation fixture, validated through the owned project schema and AJV, and checked against SQLite selection results. Cases cover the generic fallback, authored release default, fixed release publication, fixed prototype serving, contradictory fixed modes, and invalid mode declarations. The regression also verifies normalization is idempotent and every generated configuration retains its own environment mode. The regression was written before the normalizer was changed. Execution remains gated by host admission.

The public invocation selector also selects a compiler mode before graph loading. Its existing Demonstrator route leaves `build` neutral, so a plain caller would receive prototype authority before discovering the fixed release workflow. The integration owner is coordinating that original-authority boundary; the runtime patch does not bypass an issued compiler mode. Both backend consumers already inspect `metadata.semio.buildMode.fixed`.

`repo --help` completed successfully with exit code 0 after the build-mode source edits, confirming the native frontend still boots. This check does not exercise graph normalization or browser rendering.

## Original Invocation Integration and Verification Route

`readSelectedBuildTargetModesV1` uses the existing canonical `BuildOwnerRegistryV1` validation for exactly `run <canonical-project-name>:<target>` and reads that owner’s JSON manifest. It reads no Cargo manifests, no unrelated owners, and no directory inventory. Unknown owners and inferred targets remain neutral. Every `run-many`/`affected` selection and help reads no registry; project selector aliases, exclusions, and changed-file filtering remain owned by the backend. Their existing explicit, inherited, named-workflow or generic prototype mode semantics remain intact. Fixed declarations participate in conflict authority; soft defaults are used only when no explicit, named, authored, or inherited mode exists. The existing Demonstrator renderer/watch route no longer supplies a special compiler-mode environment.

The native Rust `run_native_build` and Go `nativeBunBuildTool` inherit arguments and environment and enter the same build script. `runBuildCliV1` waits for capacity; then `runAdmittedBuildCliV1` awaits `createBuildBootstrapCapabilitiesV1`, whose child helper calls the full resolver before creating native capabilities. Only afterward is `createScriptProcessEnvelope` called. Thus the canonical declaration is selected before Original authority is issued, including the native frontend path.

The focused verification route is `repo run @semio-tech/repo-lib:test-build-mode --output=stream`. It runs the existing invocation suite with added language-neutral mode/declaration matrices, AJV and SQLite parity, exact read-boundary checks, actual native bootstrap grants for Nx and Bazel, explicit-conflict refusal, and source-chain ordering checks. The target explicitly registers the runtime test source entry. This route is registered but not launched while the current unit and E2E runs wait for admission.

## Files Changed by Runtime Execution Work

- `♻️mit-bestand/🧺️demonstrator/📋️project.json`
- `♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🎚️mode/🟨️.mjs`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🎚️mode/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/📋️project/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🧪️tests/📋️schema.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🔬️typescript-package-commands/🏗️build-and-direction-routes/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/📋️project.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/execution-2026-10-21.md`

For the focused mode route, set `SEMIO_TEST_ARTIFACT_DIR` to this ticket’s `🗑️generated/build-mode` along with `SEMIO_TICKET_DIR`: the shared repository test artifact interface honors `SEMIO_TEST_ARTIFACT_DIR`, while the Demonstrator E2E consumer directly honors `SEMIO_TICKET_DIR`. No additional run has been queued.

The build README documents fixed/default declaration selection and the canonical reader scope in English and German. Additional changed file: `🧰️framework/🛍️products/🦑️repo/README.md`.

## Read-Only Review Resolution

The reviewer identified that pre-authority cross-products for scoped `run-many`/`affected` could pin excluded or unaffected owners and fail to resolve backend project-selector aliases. The canonical declaration reader was narrowed to exact canonical `run` selections, the boundary approved by the integration owner. It now observes no registry or owner manifests for any multi/affected selection. Added language-neutral cases cover exclusions, changed-file filters, `name:` and `directory:` aliases, bare-root selectors, globs, and explicit inherited release intent. These cases preserve backend task selection rather than claiming the pregraph reader has computed final affected tasks. English and German documentation state this exact boundary.

## Admission Observation After Runtime and Mode Repairs

The declared E2E run remains owned and live in its native admission queue after more than 1,179 seconds. The latest observed request has one waiter ahead and two active grants; it still requires 3,584 MB above the 6,553 MB reserve, with negative spare capacity in the latest sample. Earlier samples also refused active swap pressure. No compiler, Vite listener or Playwright pane assertion has started, so the report claims no application execution pass. The separate read-only capacity audit confirmed fresh peer receipts and no admission-owner defect; no peer task, policy, lease or cache receipt was changed.

## Approved Own-Queue Reordering

The integration owner requested that its unit regression run receive feedback before the cold nine-lane compilation. Session `12655` was verified to still contain only capacity-wait output at 1,437 seconds, then interrupted cleanly with SIGINT while still waiting (last queue line at 1,442 seconds). The owner reported `Host capacity wait cancelled by SIGINT`; no compilation or prepared runtime had begun. The same E2E command and ticket environment were immediately requeued behind the integration owner’s unit run. No foreign process or lease was touched.

The replacement E2E transport session is `83869`; its first observation is the native capacity wait with four requests ahead and two active grants.

The replacement invocation was started after the final exact-run mode reader and focused-test edits. This also avoids relying on an older waiting process to reload statically imported build modules after source changes. While capability issuance happens after capacity admission, modules imported before the wait can remain cached for that process.

## Final E2E Session State — External Capacity Blocker

After the integration owner’s third consecutive blocked goal-turn audit, it requested cancellation of only this pending E2E invocation. A fresh authoritative poll of `83869` showed only native capacity waiting at 784 seconds: 3,584 MB required, −225 MB spare above the 6,553 MB reserve, two active grants and three requests ahead. SIGINT was then sent to this own waiting process. It exited with terminal status 1 and the explicit diagnostic `Host capacity wait cancelled by SIGINT`; its final queue observation was 789 seconds, −328 MB spare, load 10.52, still two active grants and three ahead.

Session `83869` is terminated and has no running E2E child, listener or browser. Earlier session `12655` also terminated while waiting; the redundant catalog-test session `55239` was cancelled while waiting. No Demonstrator compiler preparation, server startup or browser acceptance assertion ran in these invocations. The retained native diagnostics are in `🗑️generated/runtime-e2e.log`. No foreign task, resource policy, cache partition, or receipt was changed. The ticket remains open and actual execution remains required.

The source wave is ready for the parent’s next capacity-admitted continuation: run Demonstrator unit and runtime catalog regressions, the focused `@semio-tech/repo-lib:test-build-mode` route with ticket-scoped artifact environment, canonical architecture checks, actual E2E coverage, release publication, then final serving and visual audit. This report claims no pass for those pending checks.
