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

## Resumed Execution — Fresh Unit Verification

The user resumed the goal and explicitly required resolving the memory blocker without stopping other agents. A fresh `repo run @semio-tech/mit-bestand-demonstrator:test --output=stream` was started with both `SEMIO_TICKET_DIR` and `SEMIO_TEST_ARTIFACT_DIR` under this ticket. The new transport session is `24066`; neither terminated prior session is reused. Its initial admission observation is one active grant, zero requests ahead, 3,584 MB required and 132 MB spare above the 6,553 MB reserve. The capacity owner is being investigated by another agent through legitimate owner controls; no memory policy or foreign process was changed by this work.

Fresh unit session `24066` was admitted with four of eight jobs, then failed with exit status 1 during selected compile closure preparation before Vitest. The concrete refusal is `Canonical generated import path required` in `readBuildGeneratedCompileOutputV1`, called by `discoverBuildOwnerAdmissionFactsV1` (owner-discovery source lines 141 and 226). No browser or nested Cargo stage ran. Diagnostics are retained in `🗑️generated/unit.log`; the invocation is terminal. The path guard is now being investigated at its owner.

## Generated Import Refusal Diagnostics

Added a bounded structured error diagnostic at the generated-import guard and carried the original importer/specifier through selected compile import checks. The canonical guard remains unchanged. A schema-first language-neutral diagnostic matrix covers bracket and percent filesystem names and an escaping request; AJV validates the context schema and SQLite independently produces the same diagnostic object. The fresh single-worker unit rerun will identify the real rejected physical path before any guard classification change.

Additional diagnostic source paths: `🏗️build/🕸️graph/📥️inference/🔎️owner-discovery/🟨️.mjs`, `🏗️build/🧬️schema/🚪️admission.json`, `🏗️build/🧪️tests/🕸️graph/🎯️source-demand/🟦️.ts`, and `🏗️build/🧪️tests/🧫️fixtures/🕸️graph/🔣️.json`, relative to the repo library.

### Resumed Diagnostic and Package Resource Identity

Fresh unit session 24066 admitted four jobs and failed before Vitest at canonical generated import classification. A schema-first bounded diagnostic was added without changing its safety predicate. One-job diagnostic session 78888 admitted after about 147 seconds and isolated the physical candidate `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🌎️hub-source/👷️service-worker/🟦️.ts?worker&url`, imported by `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🪝️host-ports/🟦️.ts` through `@semio-tech/plugin-registry/hub-source/service-worker?worker&url`. Both sessions exited 1 before application tests. Generated diagnostic output is retained under `🗑️generated/unit-diagnostic.log`.

The import resource adapter previously split suffixes only for relative imports. Its generic package path now separates actual query/fragment suffixes before resolution, preserves the first `#` in package import aliases and percent sequences in package identities, and leaves URI schemes unchanged. The canonical generated-output path guard remains unchanged. Twenty-two language-neutral schema-first cases cover the failing package worker, plain/raw/url/sharedworker forms, fragment query distinction, `#` aliases, encoded package identity, URI schemes, and relative URL behavior. The Bun regression compares outputs with the existing third-party WHATWG URL implementation. No dependency installation or manifest change was made.

Independent source review caught URI scheme case sensitivity before execution. Two additional neutral cases preserve uppercase and mixed-case schemes, and the scheme predicate now uses case-insensitive matching. The adapter suite now contains 24 cases. Unit session 28576 remains in admission, so its deferred source envelope will observe this correction.

### Active Resumed Verification

Unit session 28576 admitted one job and passed the prior query guard into selected compile-import, catalog normalization and native preparation. Source admission continues; Vitest has not yet started. At parent direction a single independent declared E2E run was started with one job, workers off, the same ticket environment, and distinct E2E artifact directory. Session 27300 admitted immediately with 3584 MB reservation and 13273 MB available above a 6553 MB floor. Both remain live; no duplicate or foreign invocation was cancelled. Full streams are retained in `🗑️generated/unit-normalized.log` and `🗑️generated/e2e-resumed.log`.

### Physical File Glob Prefix

Unit session 28576 exited 1 before Vitest after clearing package resource normalization. Its next concrete failure was `ENOTDIR` while `readBuildSourceInputMatchesV1` attempted `readdirSync` on the real file `/Users/ueli/Documents/semio/✏️s/tsconfig.json`. The traversal previously checked existence alone. A schema-first language-neutral case now covers file and absent glob roots alongside a real compiled source; the filesystem test double now correctly raises ENOTDIR for file readdir probes. Existing picomatch comparison independently validates the resulting matched sets. The traversal now observes the actual root kind and only reads directories. No generated-path grammar or memory rule was changed. E2E session 27300 remains live with its original process.

### Implementation Revision Turnover During Execution

Unit session 29945 admitted one job after its wait, then exited 1 during module evaluation. E2E session 27300 also exited 1 during selected owner assembly, without reaching application target execution or Playwright. Both reported `Bounded physical Source revision required` from the revision reader self-check (`current()` invokes `capture(ownPath, loaded.byteLength, false)`). The current revision-reader module is an ordinary single-link 8795-byte file updated at 2026-10-21T01:15:39Z; its inventory loader was updated in the same instant. The arguments schema is an unchanged 2541-byte ordinary file, and Cargo.lock is an ordinary 708100-byte single-link file. A growing implementation after module load can hit the old byte bound before the stale-revision comparison. The team is confirming source turnover before another fresh invocation; no source guard was weakened or bypassed. Native session streams were captured in `unit-file-prefix.log` and `e2e-resumed.log`. The local Node REPL was reset after a stalled read-only request; this affected no native process handle.

The parent confirmed no fleet edits to the revision reader; other agents in the shared repository may have changed it concurrently. The stale loaded-module explanation requires a fresh process, without altering the guard. After the capacity owner confirmed its authority tracing sources saved and stable, fresh unit session 40923 and E2E session 19584 were launched exactly once with `SEMIO_BUILD_TRACE_AUTHORITY=true`, `--parallel=1 --workers=off`, and the ticket/test artifact environment. Initial admission waits requested 3584 MB with 658 MB spare, three active runs and one/two ahead. The prior failed sessions remain terminal. Streams are in `unit-trace.log` and `e2e-trace.log`.

### Literal Physical Import Names and Generated Lookup Scope

Traced unit session 40923 admitted, cleared previous blockers, then exited 1 on a resolver request for the mutations directory itself. The authored import in the EN1992 mutation aggregate is `./#️⃣change-bar-layer-count/🧬️schema/🟦️.ts`: URL parsing treated the literal hash keycap directory as a fragment and discarded the remaining file path. This is an existing physical file, rather than a missing module. E2E session 19584 admitted independently and remains owned/live under its loaded source.

Schema-first physical fixtures now cover literal hash/keycap directories, percent and bracket names, full literal versus stripped source both present, and encoded versus decoded filename both present. Each fixture writes bounded caller-owned temporary files and compares the adapter with actual TypeScript and esbuild resolution. The adapter first retains a real exact relative physical candidate, without decoding it; only absent literal candidates use the resource URL/suffix interpretation. Existing 24 URL/package cases retain a no-file adapter environment.

The generated-output helper previously applied its generated-expression grammar to every ordinary physical probe. Workspace structural containment remains mandatory; all authored output roots and producer ownership retain strict validation. A source probe with no matching generated contract now returns undefined, preserving valid literal hash/percent/bracket/brace names. Matching generated candidates still reject those ambiguous generated-expression characters before any producer source read. Five ordinary source probes and existing outside-workspace/matched-generated refusal diagnostics cover this distinction. The strict revision-reader and memory guards were not changed.

Independent saved-source review found no concrete defect in physical-first import identity or generated lookup separation. E2E session 19584 confirmed exit 1 on the same old loaded hash-directory bug, without application assertions. A fresh traced unit 90096 then admitted immediately (one job, 3584 MB reservation, 11865 MB available, 6553 MB floor); fresh traced E2E session 9812 waits with 1704 MB spare, five active runs and one ahead. Both use workers off and their ticket-owned artifact environment. Distinct logs are `unit-physical.log` and `e2e-physical.log`.

### Catalog Producer Self-Dependency

Unit session 90096 exited 1 after the literal physical path fixes cleared discovery. The next refusal is `Generated task cannot consume its own output`, target `@semio-tech/s-dev-composition:generate`, importer `✏️s/🧑‍💻dev/🧩️catalog/🏗️builder/🟦️.ts`, generated root `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated`, compile phase. The builder imports generated plugin and playground values. The generate command runs through a shared launcher with many literal dynamic branches, while its target has no exact runtime source-entry or freshness declaration. Its actual generation branch only loads the authored catalog publication and generic projection implementation. The integration owner received exact paths to repair producer source authority; the self-consumption guard was preserved. E2E 9812 remains live; neither route has executed application assertions.

Vitest worker-policy review found argument forwarding is preserved by the canonical runner, but neither its policy nor the demonstrator config caps worker count. The next fresh unit invocation will forward `--maxWorkers=1` as target arguments in addition to the repository one-job limit. Browser E2E already declares `fullyParallel: false` and `workers: 1`. No current process was restarted for this review.

E2E session 9812 subsequently exited 1 on the same catalog generation self-consumption guard, confirming the literal import repair cleared both declared application routes. No application target execution, Vitest or Playwright assertions completed. Both owned native sessions are now terminal. The integration owner is splitting catalog publication into its own domain entry, with route fixtures, while preserving the generator guard; fresh execution waits for that save boundary.

### Dedicated Catalog Publication Retry

The parent split the four catalog publication commands into a domain-owned entry, preserving the same owning roots and Original receive/run mechanism. Both independent source reviews found no concrete defect. All previous owned sessions were confirmed terminal before fresh launches. Traced unit session 44090 and E2E session 83439 were then launched once with one repository job, workers off, and the ticket artifact environment. The unit target forwards `--maxWorkers=1` through the declared target argument separator. Initial unit wait: four active runs, two ahead, 3584 MB needed and 1734 MB spare above the 6553 MB floor. Streams are retained separately in `unit-publication.log` and `e2e-publication.log`; no application assertions are claimed until actual test output.

Both publication retries subsequently admitted: E2E 83439 reserved one job/3584 MB with 14377 MB available and unit 44090 with 13462 MB available; the floor remained 6553 MB. Both native handles are live in selected source/compile-import/native-preparation passes at the latest poll. No new refusal and no Vitest or Playwright header has appeared. No new invocation has been queued by this owner.

### Remaining Authored Generator Contract Root

Both unit 44090 and E2E 83439 exited 1 on the same generated catalog self-consumption guard, still before application assertions. Static authority tracing found `generatorContracts.plugin-registry` in the taxonomy still names the old Specific root launcher in `inputPatterns` and `inputDiscovery.implementationEntryPaths`, and still declares preview arguments `catalog-publication preview`. Project assembly adds those input patterns to the producer target. Owner discovery promotes matched TypeScript inputs to compile roots by default, so the old launcher remains compiler authority despite the new command entry. The parent received all three concrete declaration references to update.

This case does not involve inherited TypeScript project expansion: `declaredProjectClosureV1` returns undefined unless the target explicitly declares a nonempty `metadata.semio.closure.typescript`. Runtime source entries are additive to the bound command entry, and therefore cannot replace an impure launcher. A source execution declaration suppresses native preparation; freshness observation suppresses matched-input promotion while preserving the real command closure. The dedicated pure entry remains necessary independently of those declarations. No further invocation was queued before the corrected owner contract save boundary.

### Corrected Generator Contract Verification

After the parent saved and independently reviewed all three taxonomy references, the Generate freshness declaration and neutral controls, and the default one-worker Vitest config, exactly one fresh pair was launched: unit 74822 and E2E 98132. Both use source-authority trace, one repository job, workers off, and caller-owned ticket artifacts; unit also retains its explicit one-worker argument. Initial unit admission waits behind two requests with three active runs, 3584 MB needed and 1144 MB spare above the 6553 MB floor. E2E initially returned a live handle without output. New streams are `unit-contract.log` and `e2e-contract.log`. The command closure and self-output guard remain mandatory.

At parent direction the existing live E2E session 98132 was handed to the acceptance owner without restarting or cancelling it. Its complete `e2e-contract.log` stream through the final 307-second capacity wait was transferred; no poll was in flight. The recipient is verifying the same native handle once and will retain its stream. This owner retains unit 74822, runtime catalog and canonical architecture verification.

The recipient native service rejected session 98132 as unknown, proving exec handles are agent-scoped. No output was consumed by that attempt. This owner therefore retains the native E2E handle and complete stream appends; the acceptance owner handles shared-log/browser/asset triage via compact admission/start/terminal notifications. No new process or restart was introduced.

One unit 74822 progress burst exceeded the 18000-token capture limit and its inline filesystem append hit the Node tool 64000-byte review limit. That consumed progress chunk could not be fully persisted; the log records an explicit gap with its token metadata and observed stages. The process remained live, and the filtered output contained no surfaced error or terminal result. Persistence now stores each new result before writing bounded chunks, avoiding the inline request limit. E2E 98132 output was unaffected.

### Optional Graph Quota Admission

Unit 74822 exited 1 before Vitest after one membership refresh (`graph-retry` version 1, reason membership, before/after null, facts empty). The new failure is `Closed optional graph cache quota required` in `build/📥️inputs/💾️storage/🟨️.mjs` line 24, called at line 148. The call constructs the exact four-field state: maximumBytes from the 1 GiB optional policy, usedBytes from the original ledger observation, availableBytes from native statfs bavail*bsize, and minimumAvailableBytes from the 1 GiB floor policy. Validation rejects schema errors, used bytes above maximum, or free bytes below minimum. The ledger itself already validates its observed used bytes against the same maximum. The error reports no concrete state values, so no disk-pressure or malformed-statfs root cause is claimed without further diagnostics. The parent owns the storage repair and received the exact call/validation path. This owner made no storage, policy or cache mutation. E2E 98132 remains live and its full stream is preserved.

Static concurrency follow-up: storage maintain() sets its maintained flag before awaiting maintainBuildGraphOptionalCachesV1. The helper uses exclusive maintenance with wait false and returns false when maintenance was not acquired. That false result is currently ignored, leaving maintained true and suppressing the later retirement retry before quota creation. This is a concrete call-flow concern under other live readers, but the terminal error lacks actual quota values, so it remains an evidenced hypothesis rather than a confirmed pressure state. Parent/High received this observation; no storage edit was made by this owner.

### Reviewed Optional Quota Retry

The parent saved the narrow optional-quota admission repair after schema-first neutral pressure cases and independent review: valid observed free-space pressure can be represented, while reservation still refuses optional writes below the same floor. No floor or required publication rule changed. Exact unit session 45442 was launched once against that reviewed boundary with trace, one job, workers off and the now-authored one-worker Vitest config. Output goes directly to `🗑️generated/unit-quota.log` at launch, avoiding tool payload capture limits. E2E 98132 remains unchanged/live under its own loaded source.

### Concurrent Source Integration Audit

Read-only review after outside changes to admission, script-inputs and project-assembly confirmed the package resource and physical identity repair remains intact: URI schemes are preserved case-insensitively; an exact existing relative physical candidate precedes URL interpretation; leading package-import hash identity and package percent sequences remain unchanged; actual package suffixes are separated. Generated lookup still rejects outside/NUL structural paths, validates authored output roots strictly, returns undefined for unmatched ordinary physical probes, and validates matched generated paths and producers strictly. The source-role API retains its selected/declared/freshness behavior and runtime source entries remain additive. New outside work adds an explicit closed source-resource DATA API; project assembly now passes self-producer evidence through admission instead of filtering it, strengthening the self-dependency guard. No own change was lost and no concrete regression was found; no source was edited for this audit.

The quota retry unit 45442 admitted one job/3584 MB with 18879 MB available and the unchanged 6553 MB floor, then queued for shared access to the native graph image `.cache/b/g`. The current queue is position 4, with live reader PID 37466 ahead. The parent correlated it with a foreign browser verification flow and the other live reader with this task E2E, and its native lease diagnostic found no wait cycle. This is image lease admission rather than another memory-gate failure; no stale-holder or deadlock claim is made. E2E 98132 remains active in compiler/source discovery. All subsequent E2E bursts are fully preserved by bounded writes, and the quota retry log is written directly.

A later E2E 98132 pretest progress burst reported 61069 original tokens against a 60000-token tool capture limit. The returned text was fully persisted using bounded appends, but its truncated middle cannot be recovered. This distinct capture gap is explicitly marked in the E2E log. The process remained live and no test result is claimed. Subsequent polling uses a larger capture limit; future process launches already write directly to ticket logs.

### Reviewed Unit Source Boundary

Parent saved and Low independently reviewed the explicit source/freshness contract for the unit and canonical architecture targets, with sixteen exact runtime entries, the owned Vitest leaf, and test-only runtime variant imports inside the test guard. Build and E2E producer authority remains unchanged. Existing unit session 45442 and E2E session 98132 remain live; no cancellation or duplicate command was issued and no assertion has yet executed.

### E2E Loaded Authority Investigation

After the foreign graph-image reader released, the unit image writer reported E2E PID 40609 as its remaining shared reader. Read-only inspection found no Original or compiler-capture hash in the E2E stream. The mounted selected authority receipts predate this invocation (newest receipt belongs to the separate flow target), and E2E has not published a process-specific graph receipt. The source graph implementation binds storage module and schema bytes in loadedGraphModuleAuthorityV1 at module initialization and checks this authority before publication, but its current in-memory capture is not exposed in the existing output. Historical receipt differences and elapsed time alone are insufficient proof that this particular invocation is obsolete. No cancellation was issued during this investigation.

### Obsolete Owned E2E Preparation Ended

At parent direction, session 98132 (PID 40609) received SIGINT through its owned native handle and exited 1 with `Original Script process interrupted`. All returned output was persisted in `e2e-contract.log`, retaining the previously documented capture gap. No browser or application assertions executed. The basis was known module initialization ordering: graph static storage imports and loadedGraphModuleAuthorityV1 initialized before the 02:07 writer lease, while the storage module/schema and app manifest/router were subsequently saved. This makes the invocation preparation baseline obsolete for current-source verification. No emitted process-specific hash mismatch is claimed. The unit session, all agents and all foreign processes were preserved; no replacement E2E was launched.

### Unit Quota Retry Actual Result

Unit 45442 exited 1 normally after the graph image queue and headroom authentication progressed. It reached selected normalization and refused `@semio-tech/mit-bestand-demonstrator:test` at target-policy targetExecutionKindV1 line 97, called by project-assembly line 328. The current authored test and canonical architecture execution metadata both contain only `{kind:"source"}`. No Vitest assertion executed; full direct output remains in `unit-quota.log`. Parent received this concrete contract refusal before any retry or source mutation.

### Required Source Execution Version Regression

After the actual native refusal, the neutral fixture schema was extended first to require executionVersion 1 for both source targets. Its two neutral cases and the regression were saved before the manifest repair. The regression now validates each actual execution object with Ajv against the owning repository Source execution schema, checks the fixture version explicitly, and rejects a kind-only source object. Then only the test and canonical architecture execution objects were changed to `{version:1,kind:"source"}`. Their sixteen runtime entries, freshness roles, commands and build/E2E controls remain unchanged. No native retry was launched pending independent review.

### Pending Runtime Catalog Target Scope

Read-only inspection found test-runtime-catalog has no execution or source-input-observation metadata. Its root-script class launches the runtime helper through a constructed owned Bun-test argument. Before this later verification, the parent was notified that broad inherited default inputs may still promote unrelated app sources, and that a finite source/freshness declaration must include the actual runtime helper entry. This is a declaration review finding, not an executed catalog failure; no extra target was changed or run.

### Catalog Source Contract Save Boundary

The existing runtime schema gained a closed catalog-test contract and its neutral fixture names the exact source execution version, freshness role, Bun helper entry, command/router route, and required s-dev catalog generation dependency. These and the Bun regression were saved before manifest changes. The regression validates actual execution against the owning Source schema with Ajv, rejects kind-only execution, and uses the existing TypeScript parser as an independent AST oracle for the registered router class and its owned Bun test/helper argument. The catalog target now declares `{version:1,kind:"source"}`, freshness observation, the exact physical test helper entry and `@semio-tech/s-dev-composition:generate` ordering. Its existing dynamic generated projection consumption is preserved; a generated file was not falsely declared as a physical runtime entry. No native Rust producer is invoked by the actual helper; generated producer authority remains required. A semantic check proves all other manifest fields unchanged. No native retry was launched pending custody repair and independent review.

Independent review found that the helper computed runtime and asset-module imports were not literal Source edges under freshness. Their two actual module specifiers were added to the neutral contract/schema and a TypeScript AST assertion before changing the helper. Then the same runtime namespace and asset-source function imports became static relative imports, preserving the code actually executed while making its Source and generated dependencies discoverable. The single helper runtime entry remains sufficient through those literal module edges; the real catalog-generation prerequisite is retained. No native command was issued.

### Reviewed App Boundary Awaiting Current Storage Integration

Parent confirmed the unit/catalog Source contracts and E2E metadata review are complete. Native launches remain on hold because an outside storage Source edit changed the custody protocol after its earlier review; the acceptance owner is aligning the schema-first tests with the current implementation while preserving others work. This owner has no live native handles or pending Source edits and has not launched a duplicate or bypass command.

Concurrent app-contract readback after the acceptance owner changes confirmed all three source execution objects retain version 1, both unit neutral cases retain executionVersion 1 and the required field, and the actual-owning-schema Ajv regression remains present. The E2E control now expects freshness while execution remains undeclared, and the catalog generator dependency remains explicit. This was source readback only, not an executed test pass.

### Current Reviewed Unit Verification

After the parent reported clean combined review of the current seventeen custody cases and all app contracts, exactly one fresh unit invocation was launched as session 6759. It uses prototype mode, source-authority trace, one repository job, workers off, ticket-owned artifacts and the authored single Vitest worker. Output is redirected directly at launch to `🗑️generated/unit-current.log`; no forwarded Vitest override is needed. Parent separately owns the exact quota verification. E2E remains unlaunched until the first current source-check boundary. Runtime catalog and canonical architecture remain sequential after the actual unit result.

Current unit session 6759 subsequently admitted and reached selected source-input discovery under the reviewed source contract. No application test result has yet appeared. The direct log retains the exact admission figures and all subsequent Source progress.

### Current Unit Inference Roster Trace

The explicit launch argv is exact run, not run-many, with no graph verification flag. Source routing buildInvocationGraphSelectionV1 selects kind tasks for any canonical run, and loadBuildInvocationGraphV1 passes that exact selection. The Graph verification environment branch runs only in the kind-all, non-baseline generation path, so its mere existence does not explain the current selected roster. The existing log reports the later inference jobs counter as 3610; a counter alone does not prove fallback or native execution.

The actual sixteen unit roots include runtime/🟦️.ts and activation/🟦️.ts. Runtime imports generated playground/plugin projections; activation statically imports runtime/assets plus generated plugin projections. Assets also imports those generated projections. The canonical owned producer is s-dev generate, whose registry-catalog discovery records package metadata, descriptor/example and owner membership. These are legitimate potential Cargo manifest Source/data inputs even though unit callbacks do not invoke Native producers. In selected project inference, Native preparation still requires the closed buildNativeSourceObservationV1 result compile; a Source execution or input-only freshness observation does not itself enable it. No actual Native compiler invocation has appeared. Exact attribution of all 3610 jobs remains a separate read-only correlation, and no code change or restart follows from this trace.

## Quota71477 Published Current Graph

At roughly03:58UTC the retained exact quota invocation completed4157/4157 portable input closures and emitted its graph-publication summary after one earlier analysis-fact retry. Actual timing document: `[graph-phase] {"summary":[["workspace-catalog-lookup",395619],["workspace-membership-hash",1143],["source-parser-bootstrap",0],["graph-original-inference",242443],["workspace-catalog-lookup",412980],["workspace-membership-hash",1198],["graph-serialize",628],["graph-expansion",44464],["graph-publication",56124]]}`. Its next stage is a shared graph-image queue, position5/5 behind shared8301, so Bun assertions have not begun. Source-demand EX advanced to3/5; no process was restarted or stopped. This is graph publication evidence only, not a quota/test/release pass. Direct quota-custody.log remains retained.

### Outside Owner Discovery Source Turnover

Low detected an outside-fleet owner-discovery change at 04:04:55Z. Comparing the current file with this owner retained earlier read showed the semantic edit: cargoTargets for one owner is computed once into nativeDeclarations and reused for the initial Cargo project and target merge, instead of being invoked twice. The file changed from 33804 to 33798 UTF16 code units. The physical Source-probe and generated-import diagnostic repairs remain intact. This owner wrote or reverted no Source and retained unit 6759 at parent direction while immutable authority implications are independently checked. No emitted unit hash mismatch or application assertion is claimed.

### Post-Use Owner Discovery Baseline Replacement

At parent direction, obsolete unit 6759 received SIGINT through its native handle and settled with exit 1 and `Original Script process interrupted`. Its complete direct log was archived as `unit-old-owner-discovery.log`. The concrete basis was the independently established old owner-discovery SHA 0e0cf2156222230a5a4fa7a4505d530fb89e4323cc8d331748bc5c34694cac94 used by selected admission before the 04:04:55 Source turnover; that module belongs to producer implementationRevision/loadedImplementationRows. The current SHA is ce393e3238d9db8ab2d2ccc691dbeca8e54abdfec05657d647c0889eafea345a. No emitted process-specific pinned receipt mismatch or test assertion is claimed. All agents and foreign processes remained running. Exactly one replacement unit invocation follows against the saved current Source.

Replacement exact unit session 89827 is live with the same prototype/trace/one-job/workers-off/ticket controls, authored one-worker Vitest config and direct unit-current.log. No other verification request was launched by this owner.

Replacement unit 89827 admitted one job/3584 MB with 18767 MB available and the unchanged 6553 MB floor. It then queued for shared graph-image access, position 5 of 5 behind live reader PID 19334. No assertion, Source refusal or current process-specific mismatch was emitted at this point.

## Actual Physical Headroom Refusals

Root quota71477 settled terminal exit1 naturally after publication and image access: authenticated physical available971575296 bytes, required1130774528, retired0 bytes/files0 at caching/storage/enforcement/budget/graph/headroom/TypeScript line48. Direct quota-custody.log is full. Fresh unit89827 also settled exit1: available971575296 required1073741824 retired0/files0. Source-demand42083 terminal exit1: available971608064 required1073741824 after retiring38432768 bytes/twofiles. All are pretest physical disk headroom refusals, not host-memory admission errors, assertion failures or claimed passes. No current goal-owned native job remains live. High acceptance owns authenticated-headroom diagnosis; High runtime owns fixed-image policy/supported control discovery; Low independent audit. Preserve floors, finite backings, custody, receipts and all foreign activity.

### Fresh Unit Required Graph Headroom Refusal

Replacement unit 89827 exited 1 normally before tests: authenticated graph headroom reported availableBytes 971575296, minimumAvailableBytes 1073741824, retiredBytes 0 and retiredFiles 0, at the graph headroom owner line 48. Its complete direct unit-current.log is 17456 bytes. Parent quota verification independently reached the same owner with a larger required amount. No retry or Source edit followed. Parent assigned bounded owner-policy/recovery research, preserving all memory floors and foreign processes and prohibiting raw cache reads/mutations or new build launches pending a concrete plan.

## Integrated Headroom Repair Native Launch

Both High execution owners saved and froze their integrated source/tests; Low final combined review found no concrete mismatch. Old quota71477 complete direct log (1407392 bytes) is archived as generated/quota-before-headroom-repair.log. Fresh exact repo-graph-maintenance:test will reuse quota-custody.log only after that settled terminal; prototype/trace/ticket artifacts/one job/workers off unchanged. Start this one owner first, then resume app unit and Source-demand only after actual repaired physical admission, preserving all foreign activity. Actual assertions remain pending.

## Authenticated Graph Headroom Repair Boundary

Fresh exact unit session89827 settled exit1 before assertions at authenticated graph headroom: available971575296, minimum1073741824, retiredBytes0, retiredFiles0. Complete direct log is now generated/unit-before-headroom-repair.log (17456bytes). Older obsolete-owner source preparation session6759 remains separately archived in generated/unit-old-owner-discovery.log. Both are terminal; neither is a unit assertion result.

The selected publication repair and optional retirement integration are documented in graph-headroom-recovery-2026-10-21.md. Headroom schema, neutral fixtures, actual-function AST/third-party oracles, and actual-image selected scenario were saved before the implementation boundary. Source is frozen while parent owns fresh exact repo-graph-maintenance:test session58346. No new unit is launched until actual physical headroom admission is observed; subsequent exact checks remain unit, runtime catalog, canonical architecture in order after actual success.

## External Reclamation Counter Contract Turnover

Fresh quota58346 was admitted one job/3584MB with20160MB available and progressed selected Source work. After launch, unknown-author saves changed headroom/Nx at04:26:03, headroom schema04:26:41, storage defining module04:27:22 and test files. Both High owners compared retained exact save strings and confirm these were not their pending edits. Preserve the new design: queued optional reclamation returns5-field pressure DATA and actual allocated retirement counts; Nx projects closed2counts; authenticated headroom validates/accumulates those actual counts. Existing floor, private writer proof and stream/hash/physical custody remain. Low reviewed source/declaration parity. Three real harness assumptions need alignment: lstat.blocks, exhausted reclamation DATA and returned callback counts. Character escaping was checked and is correct; no speculative escaping edit.

Because this invocation already used the loaded storage writer before its defining module changed, Root ended ONLY own58346 with supported handle Ctrl-C; actual tool returned terminal exit1. The agent fleet and all foreign jobs remain active. Its complete direct log remains quota-custody.log until archived before a fresh run. No actual source refusal or process-specific pinned digest mismatch is claimed, and selected Source progression alone is not claimed as repaired physical headroom proof. Runtime/Acceptance review and align current tests, Low reviews final current contract; no further native launch until that concrete integrated boundary.

## Current Verification Gate

Combined optional and selected headroom repair review is clean. Parent's maintenance session58346 settled exit1 through supported SIGINT after concrete defining Source turnover; this agent owned no live process at that point and made no stop/restart decision. Parent now sequences exact SourceDemand first to establish actual physical recovery before a fresh exact application unit. This agent remains ready for unit → runtime catalog → canonical architecture on actual success. No unit, runtime catalog, architecture, or browser assertion pass is claimed.

## Fresh Source-Demand34653 Initial Physical Gate

Fresh Source-demand34653 admitted normally one job/3584MB with16511MB available and6553MB unchanged floor, then completed1040/1040 project inference. Low traced actual control flow: build/script unconditionally awaits prepareBuildToolStorageV1(snapshotBytes0) before loadBuildInvocationGraphV1; the Nx storage owner delegates to prepareNxGraphCacheStorageV1 with Original checks before/after. Therefore initial1GiB physical headroom admission returned. This does not prove cleanup executed (no recovery/retirement telemetry), and the later withRepositoryBuildGraphArtifactV1 additional snapshot allocation+floor gate remains pending. Root holds broad quota/unit readers until immutable snapshot/backend/Bun execution is observed, avoiding a new reader delay before first current execution. All current Source remains frozen.

## Source-Demand Later Image Admission Queue

At04:40:45UTC after4112/4112 portable expansion, the retained34653 invocation reached a new graph-image exclusive queue, position2/2 behind foreign shared64604 (31s own wait at that observation). Initial1GiB gate is already proven to have returned, but the later snapshot+floor/immutable backend gate remains pending. No new own broad reader, restart, lease mutation or foreign process action was introduced. Published phase document: `[graph-phase] {"summary":[["workspace-catalog-lookup",47575],["workspace-membership-hash",595],["source-parser-bootstrap",0],["graph-original-inference",121672],["workspace-catalog-lookup",149062],["workspace-membership-hash",783],["graph-serialize",166],["graph-expansion",27450],["graph-publication",37255]]}`. Actual Bun assertions are still unobserved.

## Fresh Exact Unit After Current Recovery and Cancellation Boundary

Parent authorized a fresh exact unit after supported live leases proved shared readers queue behind the pending SourceDemand exclusive recovery, so this invocation cannot overtake it. Native session9640 was launched with SEMIO_BUILD_MODE=prototype, SEMIO_BUILD_TRACE_AUTHORITY=true, ticket environment, distinct generated/unit-current artifact directory, --parallel=1 --workers=off and direct generated/unit-current.log. No live duplicate unit exists. Parent owns quota30980 and Acceptance owns SourceDemand; no other process is stopped or modified. This agent will retain session9640 to natural terminal, then run exact runtime catalog and canonical architecture sequentially only after actual unit success.

## Current Three-Check Queue — 2026-10-21T05:00–05:01Z

Fresh unit9640/PID81573 passed normal one-job/3584MB RAM admission after an initial floor wait. Root quota30980/PID77066 is also normally admitted at one job. Supported lease snapshot at1792558810049ms shows only unrelated teaching PID64604 holding graph image SH. SourceDemand66551 is EX position2; root quota77066 SH position3; unit81573 SH position4. The new readers depend on SourceDemand queued writer and have not overtaken it. No wait-for cycle or enforced violation. SourceDemand has published its selected graph but still needs snapshot peak recovery and actual Bun execution; quota/unit have no assertions yet. No new failure, foreign action or source bypass.

## Isolated Release Preflight Repairs While Unit Queues

Parent authorized two map-domain repairs while session9640 is queued before graph/source use. First, schema-first cancellation cases now prove the issued invocation signal is forwarded to prefetch and artifact publication, already-aborted work never starts fetch, and cancellation during prefetch prevents build. Second, the old vector canary3/1/2 was not included in the Switzerland prefetch plan. It is corrected to3/4/2 after neutral canary/cold-warm cases were authored and the full actual zoom0..10 plan was compared with the existing d3-geo Mercator oracle. Detailed evidence is in release-serve-plan-2026-10-21.md; five touched map-domain files are inventoried. No storage Source, dependency manifest, live process, or unit invocation was changed/restarted by this agent. The latest unit observation remains live shared-image queue position4/4 behind unrelated64604, with no assertions.

## Acceptance Spec Text Authority

The console oracle reads its acceptance spec as UTF-8 text. The generic URL classifier regards a `.ts` literal as executable unless closure.resources marks DATA; both unit owners now declare that exact project resource, preserving the sixteen entries. Schema/neutral fixture/AST projection were saved first. Corrected double-escaped regular expressions in the same fixture schema to actual JSON Schema regex escapes; no native assertion claim.

## Concurrent Selected Repair Awaiting Native Verification

The selected-only bounded recovery, exact canonical private opening binding, shared state lock helper integration and Nx pre-coarse fallback are saved for independent review. The native proof keeps an unrelated actual image reader plus a busy selected state alive and requires only selected-state timeout-zero EX attempts. This supplements existing whole-image optional/snapshot retirement. No floors, capacities or foreign lifetimes changed. UNIT9640 remains live pretest (last direct log154503bytes, ownEXposition4/5 behind foreign64604SH); no assertion result is claimed. Existing optional-storage compositor harness is being aligned by its owner.

## Unit9640 Natural Terminal

UNIT9640 ended naturally with exit1 before any Vitest assertion. Complete180917-byte direct log retained as generated/unit-before-concurrent-selected.log. Actual refusal: Original resource loan deadline exhausted, stack remaining→checkResourceLeaseControl→persistent native image command owner. Last preterminal state was queued image EX behind unrelated64604SH. No process cancellation was needed, and no foreign lifetime changed. Concurrent selected repair now independently reviewed clean; fresh invocation can consume its current Source boundary through normal admission.

## Natural Deadlines Before Reviewed Concurrent Recovery

SourceDemand34653, quota30980 and unit9640 all settled naturally with exit1 on Original resource loan deadline exhaustion while waiting for graph-image EX behind unrelated reader64604. No assertions ran and no own or foreign process was interrupted at this boundary. Full logs retained as the focused owner archive, quota-before-concurrent-selected.log and unit-before-concurrent-selected.log. Combined source review of new selected-only recovery is clean; exact uncached source/freshness maintenance verification is the next first run.

## Fresh Unit Selected Invocation

After reviewed concurrent recovery boundary, exact demonstrator:test launched once as session52585, prototype/trace plus both ticket environment fields, parallel1/workersoff. Direct generated/unit-selected.log and generated/unit-selected artifacts retain output. This request follows existing fair queue; no prior job remains live and no foreign job was interrupted. Catalog and canonical-architecture remain sequential after actual unit success.

## Handoff Documentation Source Reconciliation

Updated both README language sections to the current twenty-case authored roster, automatic Energy finalization, fixed plain release build, and explicit Flow/Canvas build prerequisites. Removed stale historical twelve-case/raw-Bun instructions and kept all current end-to-end execution evidence explicitly pending. This is documentation reconciliation, not a new test pass.

UNIT52585 passed normal RAM admission at one job/3584MB reserved with14501MB available above6553MB floor. It advanced dependency generation verification through86016 observations, then queued graph-image shared access position4/5 behind unrelated64604SH. No selected recovery, graph inference, Bun or Vitest assertion boundary has been reached in this invocation yet.

## Selected-First Lifetime Correction

Root quota session 1997 naturally exited 1 before assertions after the old optional-first receiver was consumed. Complete output is retained under generated/quota-before-selected-first.log. SourceDemand session 45267 independently reached the same canonical-image failure. The corrected selected-first source passed independent review; fresh exact checks are required.

Correction to the preceding interpretation: the exact canonical-image failure is independently caused by the repository partition facade passed to a canonical-only operation. The opening owner retains the actual image privately; recovery must enroll through that owner. The optional-first consumption defect was real but is not the direct guard shown by these logs. Fresh selected-first jobs may reproduce the facade refusal.

Root fresh selected-first session 62109 naturally exited 1 at the partition facade canonical-image guard before test assertions. Complete log archived as generated/quota-before-partition-bridge.log. Runtime is implementing the owner bridge test-first and independent review is pending.

## Unit52585 Actual Canonical Refusal

Old unit52585 settledexit1 with Exact canonical Native image required (platform33→binding35→selected66→Nx73), no assertion header. Complete83013-byte log archived unit-before-selected-first.log. The supported SIGINT request found immediate terminal state; log already contains concrete natural refusal, so no cancellation is asserted. Fresh unit launch held for the now-saved owning partition facade bridge review.

## Canonical Partition Bridge Reviewed

The narrow owner bridge and selected-first compositor passed independent source review. It enrolls through the existing private partition operation and passes its actual retained canonical image/refined control; copied facades remain refused. Three neutral actual-function factory/copy/closed/drain cases and a real native image ingress proof are saved. Root is launching fresh exact maintenance once, with quota-partition-bridge.log. Runtime is authorized for one fresh exact unit run. Acceptance is correcting independent finite DATA/Source test declarations before its own next run.

## Fresh Partition Bridge Unit

After independent bridge review, exact demonstrator:test launched once as session19495 with prototype/trace, both ticket paths, parallel1/workersoff. Output generated/unit-partition-bridge.log; artifacts generated/unit-partition-bridge. All defining own sources are frozen. No duplicate unit exists. Actual success remains required before catalog and canonical-architecture.

## Actual Concurrent Reclamation

At 2026-10-21T06:26:00.580Z root maintenance57825 emitted seven selected-graph-headroom-retirement progress records after authenticating receipts/content through the private partition bridge. Independent read-only statfs observed 1136500736 bytes available on the actual graph image (filesystem4084162560), above the unchanged1073741824-byte floor. This is actual recovery progress, not a test pass. No foreign process or agent was stopped and no floor/image policy changed.

## Unit19495 Capacity Premise Cancellation

The actual root quota check reclaimed seven selected pairs and observed1,136,500,736 bytes available above1GiB floor. Unit19495 had already entered queued whole-image EX from an earlier deficient observation. Parent directed cancellation of only this now-unneeded owned pre-assertion cleanup so the retained root shared request could advance. Sent normal SIGINT through session19495 and confirmedexit1. Complete log archived generated/unit-concurrent-capacity-race.log; no assertion pass claimed. This was a concrete changed-capacity premise, not an age restart. No foreign process/agent was touched; fresh unit remains held for long-term race review and actual root admission.

## Owned Stale Recovery Request Settled

Unit19495 had already queued coarse exclusive image maintenance after skipping concurrent busy selected pairs; its floor check preceded root retirement. After root restored the physical floor, that owned exclusive ticket was no longer needed and was blocking later shared admissions. Parent deliberately authorized normal SIGINT of only unit19495; Runtime confirmed terminal1 with Original Script process interrupted and archived full29513-byte output as generated/unit-concurrent-capacity-race.log. No assertion ran and no foreign process/agent stopped. Root maintenance57825 was retained and advanced to selected source/compile preparation immediately after the owned ticket released. Runtime and independent reviewer are designing the robust private observation/withdrawal/fresh-shared revalidation path test-first; busy retries alone do not close the race.

At06:41:55 root57825 had reached the later immutable-publication coarse EX wait, with actual1100517376 bytes free. A subsequent read-only selected-directory inventory is empty: concurrent selected retirement has exhausted those eligible pairs. This later peak requirement is distinct from the earlier stale-unit race. Root/Runtime/Low are auditing exact peak funding and remaining resource-safe reclaimable data; no guard or foreign work changed.

## Graph Withdrawal Save Boundary

Queued coarse recovery now withdraws only its owned writer when Native observes sufficient physical space, then closes and reopens through ordinary shared admission before accepting the floor. Twelve neutral actual-function cases and two retained-reader Native scenarios were authored, including nonwaiting observer omission and capacity being consumed again before revalidation. Existing scoped optional compositor proofs now cover three funding paths with SQLite parity. See concurrent-capacity-revalidation-2026-10-21.md. No new command or assertion outcome at this save boundary.

## Fresh Combined Recovery Verification

The combined Native withdrawal, graph revalidation and scoped optional recovery passed final independent source review. Root57825 is terminal1; a settle-only SIGINT tool call returned immediately, while its complete1024783-byte log already ends with Original resource loan deadline exhausted (not an interruption), so the evidence is recorded as deadline terminal rather than claiming deliberate cancellation. It had reached graph publication, retired eight selected pairs and entered real exclusive optional cleanup after the older reader released naturally; no Bun assertion ran. Full log is archived as generated/quota-before-withdrawal-scoped.log. Fresh exact maintenance uses quota-scoped-final.log after these defining Source corrections. No foreign process or agent stopped.

## Combined Source Review Complete

Independent review found no remaining concrete issue in the queued Native withdrawal integration, twelve neutral graph cases, two retained-reader Native scenarios, scoped optional compositor paths, or real scoped optional reclamation proof. The small fixture explicitly projects only its opening budget and request range; production1GiB floor, actual durable ledger/gate and private custody remain unchanged. Root started fresh exact maintenance verification24207 (`quota-scoped-final.log`); this agent has no live unit process and waits only for that actual image admission gate before its authorized unit→runtimecatalog→canonicalarchitecture sequence. No Native or browser assertion passed at this checkpoint.

## Fresh Unit Dispatch After Combined Recovery Review

Started one exact `repo run @semio-tech/mit-bestand-demonstrator:test --parallel=1 --workers=off --output=stream` invocation, native session55974, with prototype mode, authority trace, the existing ticket and unique generated/unit-withdrawal-scoped artifact directory. Complete stdout/stderr redirect directly to generated/unit-withdrawal-scoped.log. Root separately owns maintenance verification. No duplicate unit exists; natural completion retained, success-only runtime catalog and canonical architecture follow.

Root24207 naturally ended1 after memory admission and before graph/tests with Bounded physical Source revision required in the physical reader self-check. Its defining reader changed externally at07:14:31 while this invocation waited; exact loaded hash is unavailable, so the stale-load cause remains an evidence-based interpretation rather than a proven byte attribution. Current arguments/schema files remain regular bounded files. Complete10049-byte output is archived as generated/quota-source-reader-change-1.log. The guard stays intact; a fresh invocation loads current Source, with current reader/schema hashes observed separately. No defining Source edit by this fleet followed final review.

## Fresh Unit Actual Recovery Stage

Unit55974 passed normal one-job memory admission (3584MiB reservation;16604MiB available above the unchanged6553MiB floor policy). Its log emitted961 scoped inventory markers and no scoped retirement marker, then entered coarse graph-image EX waiting behind a retained shared request. At the latest observation it remained live at227s of image wait, position2/4, with no application assertion. Root and Low are auditing candidate retention and coarse per-leaf cost. The valid underfloor wait does not trigger the new funded-observation withdrawal, as intended. No process cancellation, foreign action or Source change by this agent. Full direct log remains generated/unit-withdrawal-scoped.log.
