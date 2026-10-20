# October 19 Independent macOS Audit

Read-only source audit; no builds or runtime claims. Read root AGENTS.md and the repository-product instructions. This audit intentionally leaves source edits to the parent and assigned implementers.

## Findings

No additional concrete live IDE projection or developer-tool execution blocker was established beyond the fixes already assigned by the parent. This is a bounded source review, not proof of cross-platform runtime completion.

Confirmed stale live references already covered by the fleet:

- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json:88-89` names removed `.vscode/launch.json` and `.vscode/tasks.json` as inputs.
- `🧰️framework/🔨️modules/🏃️process/🏗️build/🎛️environment/🧬️schema/🔣️.json:29-30` names removed launch and seed inputs. Parent already assigned the environment schema correction.

## Evidence Reviewed

- No root `.vscode` exists. Hidden-file inventory found no active launch/task/workspace projection outside historical ticket output in its bounded results.
- `.devcontainer/devcontainer.json` has no editor customizations, extension recommendations, or editor task controls. Its lifecycle dispatches registered workspace setup targets. `.devcontainer/README.md:39-45,71-73` describes dashboard ownership and forbids editor installation during attach.
- Plugin registry `📜️script.ts` has no launch-generation route. Its generate routes import `📽️projection/🟦️.ts`.
- VS Code extension `⌨️commands/🟦️.ts` registers source/tree navigation, filter, clipboard, refresh, ticket-file opening and external-link operations. No process/build/test/debug launch handler was observed in the registration module. `compose.ticketOpen` opens an existing ticket file; it does not create a ticket.
- Developer tools `🟦️.ts` validates requests against the owned schema, resolves existing workspace-relative files/directories, rejects traversal and external symlink targets, creates fixed argument vectors, and propagates the original cancellation signal.
- Developer tools `📜️script.ts` executes terminal/debug operations through `runOwnedCommand`, with unlimited duration for interactive work. `runOwnedCommand` inherits stdin and pipes output. Thus the source supports forwarding dashboard task input; interactive runtime still needs the parent native verification.
- Debugger profiles are schema-owned Node Inspector, Python Pdb, GDB and LLDB declarations; dedicated `debug` and `debuggers` target metadata is present. Windows GDB Unicode restrictions are explicit in both plan validation and availability output. Missing debugger executables receive a concrete localized diagnostic. Availability only claims executable presence, correctly avoiding a target-support guarantee.
- `🧪️tests/🧭️authority/🟦️.ts:354-390` asserts launch projections and launch generation targets are absent, root operations retain dashboard declarations, and the extension exposes no prohibited developer-operation commands.

## Documentation Follow-up

`README.md:697` still sends debugging users to Workspace Terminal even though developer tools now declare dedicated Program Debugging and Debugger Availability targets (`📋️project.json:607,719`). This remains a usable route, so it is a discoverability/documentation inconsistency rather than a proven runtime blocker. The parent may update the daily guide and root row to expose the concrete dedicated action.

## Limits

No tests were run by this audit because parent/native and other agent validation were already active. Broad filesystem scans were slow because of the large checkout and were not treated as exhaustive negative evidence. Historical `.cursor/plans` references were deliberately not treated as active infrastructure. Cross-platform debugger execution, Windows process-tree cancellation and native dashboard interaction require the assigned runtime checks.

## Follow-up: Runtime Environment Integration

Reviewed the new environment schema, seven-case JSON corpus, Gherkin feature, Bun tests, owner target and script route. No concrete import, schema, or expectation failure was found by source inspection. The `../../🟦️typescript/🟦️.ts` relative import correctly resolves from the nested tests directory; the schema-validator workspace package exports the named validator; its third parameter accepts the root schema. Ajv uses the draft-07-compatible default export and `allowUnionTypes` for additional string/null values. SQLite JSON assertions independently check the fixed runtime flags, required values, and allowed value types. The compiler owner implementation only overlays the fixed cache flags, sets SEMIO_BUILD_MODE, removes NODE_COMPILE_CACHE and freezes a fresh record, matching the SQLite json_set oracle. Both prototype and release are exercised. The numeric negative case remains invalid after compiler preparation because unrelated values are preserved, and the test explicitly expects that result.

The owner target command, cwd, source execution declaration, uncached output and script suite key match. Its sources include both the neutral environment contract and actual compiler owner. The script executes Bun tests from this.repoRoot, so the test's process.cwd() reads resolve to the intended repository files. No test was executed in this read-only review, and compile success is not claimed. A useful future strengthening is checking the original required declarations against the registry admission schema, but no demonstrated target-admission failure warrants blocking this change.

## Concrete Debugger Documentation Wording

Recommended rows in the root daily-work table:

| Action | Dashboard command | Form inputs |
| --- | --- | --- |
| Check installed debuggers | `@semio-tech/repo-developer-tools:debuggers` | None |
| Debug a program | `@semio-tech/repo-developer-tools:debug` | Profile, workspace-relative program file, working directory, JSON argument list |

Recommended replacement for the existing debugging paragraph:

For debugging, choose **New Task**, search **Debugger Availability**, and run it to see installed profiles. Then search **Debug a Program**. Choose Node.js Inspector for JavaScript, Python Pdb for Python, or an installed native debugger (LLDB is the usual macOS choice). Select a workspace-relative program and working directory; build native programs with debug symbols first. Enter program arguments as a JSON list, for example `["--name","Ada Lovelace"]`. Toggle task input with `Ctrl+B t` to enter breakpoints and stepping commands. The task menu provides **Restart**, **Stop** and **Kill**. The Windows GDB profile requires ASCII program and working-directory paths; check installed LLDB when debugging native targets under Unicode paths.

These action labels, profile constraints, fields and argument example come directly from developer-tools metadata (`📋️project.json:607-745`) and schema/plan validation. Workspace Terminal remains the action for a custom installed debugger or advanced shell workflow.

## Bounded Independent Review of IDE Retirement and Execution Fixes

Read the current diffs for the neutral .NET/course inputs and dashboard authority dispatch, their schema/JSON/Gherkin/tests, plus the current package TestScript and relevant process helpers. No tests or builds were run.

### Remaining Actionable Gap

Dashboard package `📜️script.ts` still calls synchronous `runCmd` for the windows Bun precheck, the quick control-plane check, and the quick registry/CLI checks (current lines 52,74,78-79). `📚️library/🏃️process/🟦️.ts:131-170` implements that helper with `spawnSync`; its options contain no Original AbortSignal. Those operations cannot process the Original cancellation callback while the synchronous child is running. This predates the authority fix and does not invalidate the new authority ownership behavior, but it leaves a concrete gap if completion is described as cancellation across every dashboard test route. Replace these subprocess calls with awaited owned runners using the original invocation environment, control budget and signal, and expand dispatch tests to assert that custody. Current dispatch test ports intentionally record synchronous commands and cannot detect this gap.

### Authority Dispatch Review

The new authority subprocess is awaited inside `withInstalledDashboardV1`, receives the leased executable rather than a separately returned path, and uses the same options.control signal and budget. `scriptInvocationEnvironment` transmits the current invocation. The fixture, schema and Gherkin explicitly encode authority custody and call counts. Tests transpile and execute the actual TestScript class rather than a duplicate routing implementation; SQLite supplies independent dispatch expectations. Promise settlement and refusal tests ensure the reader remains held until completion and is released on failure, and selected suites cannot continue after authority refusal. No import or schema inconsistency was identified by source reading. The unit test simulates lease/process ports; it does not establish actual native cancellation or installation integrity at runtime.

### Window Owner Selection Review

The split crate selection now points the dashboard docking filter at the terminal crate. The selected UI unit-law manifest declares the native feature used by the command. The shared fixture enumerates native package/manifests and argv, and independent TOML parsing checks names and terminal core dependency. The tests cover actual dispatcher argv and the existence of a moved docking test. This is valid ownership evidence, not a substitute for running current native test owners. No concrete changed-selection error was found.

### IDE Retirement Review

The .NET policy retains cs/csproj sources, replaces removed Monorepo.sln with existing build/cache policy files, and the course removes only the two dead editor configuration inputs while retaining the dashboard registry schema. The language-neutral JSON fixture and Gherkin capture these expectations; repository schema compiler and Ajv compare positive and malformed vectors. JSON.parse and the independent TypeScript config parser compare the authored input manifests. Non-glob workspace policy files must exist, and a regression scan rejects editor/solution inputs. No lost active source input or compatibility layer was observed. The test preserves the existing build-source patterns and does not relax a failure/ceiling to obtain success.

## Shared Dependency Image Bootstrap Block

Read-only observation: `ps -p 41423 -o pid,ppid,etime,state,%cpu,command` returned only its header during this audit, so the previously reported holder was no longer present at that instant. No process was signalled, killed, bypassed or altered. The reported earlier 32-minute exclusive hold is evidence supplied by the parallel live investigator, not independently reproduced here.

### Exact Lease Semantics

The fixed image opener in process `📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts:121` initially acquires shared custody and immediately returns a mounted image under that shared owner. When image preparation needs a writer, `completeFixedReceiverV1:260` explicitly releases the exclusive guard, reacquires shared custody, captures the original physical facts across that gap and returns a ready reader. Ordinary command consumption therefore does not intentionally hold an exclusive image for its whole runtime. No defective downgrade was established.

Conversely, cache store maintenance deliberately does: `⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/🏪️store/🟦️.ts:118` calls `opened.withExclusiveMaintenance`, invokes the complete maintenance body under that custody, drains registered work, rechecks storage and closes only afterwards. The physical fixed-image maintenance implementation at `fixed/🟦️.ts:115` obtains exclusive image custody and awaits its supplied body before closing. A holder in this route is ongoing maintenance, not a stale downgrade, until body and cleanup have settled.

### Concrete Global Serialization Point

`🚀️bootstrap/📦️dependencies/💾️storage/🟦️.ts:72` wraps the entire JavaScript dependency producer in queued dependencies-image maintenance, then takes the repository binding writer. Its full producer (`runJavascriptDependencyStoreProducerV1`, following line 422) performs complete ownership/planning observation, immutable input staging, potentially Bun installs for each fresh owner, installed-file checks, repeated fences, ready receipt publication and Source binding publication before returning. The global image writer is retained throughout all of those phases. Schema `📥️install.json:4772` gives that maintenance route a maximum 21,600,000 ms (six-hour) request. That is a concrete reason unrelated shared bootstrap readers can queue for tens of minutes without a lease bug.

Build entry `🏗️build/📜️script.ts:128-130` awaits `ensureBuildEnvironmentV1` before entering the ordinary shared dependencies store and launching the selected task. `🏗️build/🚀️bootstrap/📜️script.ts:21,32,55-58` uses a shared inspection on a warm receipt, otherwise invokes the all-owner installer and publishes the parser receipt under writer custody. A wrapper whose argv names course test-e2e can therefore still be in global dependency preparation; argv alone does not prove Playwright/e2e execution owns the exclusive image. The course task itself contains no direct dependency-image maintenance invocation.

### Safe Structural Direction

The coarse-lock issue is that ordinary dependency generation materialization is classified as whole-image maintenance. A long-term fix should retain shared physical image lifetime while producing a uniquely owned immutable generation, serialize its mutations with a generation-specific writer lease and Original reservation, and acquire the global Source-binding publication writer only for the atomic checked transition and receipts. True image attach/detach/layout/retirement work should retain whole-image exclusive maintenance. Existing readers must retain their old immutable generation and binding custody until settled; a producer must revalidate complete Source ownership before publishing. Failure must retain current preservation/cleanup semantics. This requires changing the store authority/schema interfaces and concurrency tests together; simply downgrading the current maintenance owner or releasing its EX guard around an untracked installer would break the present custody contract.

Before implementing that redesign, the safe immediate action is to let the current owner's admitted maintenance finish or have its own user stop it through its dashboard task. New bootstrap attempts should report the current phase and holder without spawning duplicate installers. No cleanup, forced unlock, PID kill or file deletion is justified by elapsed time alone. The independent audit cannot label the actual 32-minute body's phase as a defect without the live investigator's child/output evidence.

## Final Independent Source Acceptance

Re-read the converged dashboard package script and dispatch regression, process runtime-environment contract/test and neutral IDE-input gate. **Accepted at source-review level: no remaining concrete defect identified in these changes.** This review does not report runtime test success; the slice reports still say admitted tests are awaiting bootstrap.

The earlier synchronous dispatch finding is now resolved. TestScript has no runCmd or spawnSync reference. Its single awaited Bun helper supplies scriptInvocationEnvironment(this.invocation,...), the exact options.control.signal and compilerAdmissionBudgetV1(options.control) for windows, authority, execution, control-plane, registry and CLI checks. Authority and registry/CLI calls remain inside their installed reader callbacks until the awaited child settles. Native windows calls receive the original invocation and begin after the Bun precheck completes. The new dispatcher test omits any runCmd port, checks each child's received invocation and signal, forbids overlapping selected children, records settlement-before-release ordering, and verifies refusal closes custody before any later suite starts. Its shared corpus now includes quick, long, exhaustive and default routes in addition to explicit suites. The SQL oracle matches expanded-route counts. The actual window test uses fixed Cargo names validated through third-party TOML parsing. No missing port or event/count mismatch was found by inspection.

The runtime-environment tests still match their fixed cache-flag schema, SQLite oracle, compiler environment implementation, owner target and source route; no IDE catalog remains in that contract. The IDE-input gate retains the .NET source/project patterns and existing build/cache policies, preserves the course registry schema, rejects removed editor assets and compares schema/JSON acceptance against third-party parsers. No additional mismatch was identified. README now exposes Debugger Availability and Debug a Program directly, resolving the prior discovery inconsistency.

The global dependency-image contention analysis is a separate bootstrap architecture finding. It is not evidence against these focused source repairs. Parent should retain the actual admitted test result or explicit bootstrap limitation when closing the ticket.

## Native Dashboard Routing Follow-up

The parent observed `bun run dashboard commands --check` fail before native startup. Source confirms the installed-only condition in build `📜️script.ts` dispatches `dashboardInvocation(...)` only when dashboardInstalled is true, so a missing installation falls into generic task argument admission and interprets native flags as build flags. This is a routing defect before the existing lazy installer can run.

Sent concrete invariants to the IDE retirement implementer: dispatch whenever dashboardInvocation returns a defined vector, independently of installation existence; native launch owns stale/missing installation preparation. Only run/daemon/workflow/preferences are launch targets. build/compile/install/test must remain ordinary backend targets so launchDashboard → rebuildDashboard → package BuildScript → repo :build → declared compile terminates rather than recursively launching. Keep graph/batch/parallel/exclude-dependencies in backend routing, pass the current Original workspace/mode/network/control, and preserve the opaque post-`--` tail. Scoped run-target help must reach its target; only runner-level help should return before storage. Test missing/present installation against exactly one native launch with no generic validation/bootstrap/graph fallback, alongside authoring-target and backend-option negative cases.

An existing separate edge is that mayRunStale deliberately skips staleness for stop/status/task/log operations: if no installation exists those operations currently reach the concrete installedDashboard diagnostic, rather than lazy rebuild. That behavior does not form a rebuild loop and should not be silently broadened in this focused fix.

## All Tracked IDE-Configuration Filename Audit Including Tickets

Completed `git ls-files -z` inventory, with no directory exclusions, selecting conventional IDE directories, exact launch/tasks/seed names and solution/code-workspace suffixes. **Ten tracked files match; all ten physically exist and all are historical ticket inputs. No active conventional IDE configuration matches.** Nine contain full editor launch definitions or launch-row fragments; one is a launcher-contract metadata fixture. This is not a claim that arbitrary prefixed historical evidence JSON contains no editor data.

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/actor-publication-port-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/catalog-closed-cache-retirement-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/fem2d-surface-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/i41/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/mounted-owner-turn-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/n8-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/n9-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/pb-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/s-composition-execution-inputs/launch.json`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️14/ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/commands/launch.json`

The first nine are under CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT: three objects containing `configurations`, four launch-row arrays, and two `rows`/`retireNames` objects. Their rows contain node-terminal editor commands, workspace/editor variables and launch presentation fields. They can be retired as obsolete historical IDE projections under the explicit remove-all instruction, while retaining the ticket reports and underlying script/config inputs. They are outside active owner discovery and must not be represented as current developer entry points.

The tenth, ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/commands/launch.json, is not directly consumable VS Code configuration: its configurations only name EN/DE suffix/order metadata and it includes freshEnvironment and argv cases. It is nevertheless specifically an obsolete IDE launcher fixture. Its sibling commands/📜️script.ts:7 reads it and also unconditionally reads already-removed root .vscode/launch.json. Thus that historical launcher-check path is already obsolete; deleting the fixture alone should not be claimed to preserve replayability of that helper. Retire its IDE-specific checker branch with the fixture if historical helper replay is in scope, or retain an explicit report saying it was intentionally retired. Do not relabel it as an active neutral runtime contract.

The earlier active-only no-IDE conclusion remains correct, but a literal no-launch-anywhere assertion is false while these ten files remain. No files were modified by this inventory.

## Final Cold-Launch Review and Historical Consumer Census

The current missing-install change is accepted by bounded source review. The native branch checks nativeArguments!==undefined and no longer gates on dashboardInstalled. The existing dashboardInvocation restriction keeps build/compile/install/test on the backend, preventing rebuild recursion. Runner help returns early only when no native dashboard vector exists, allowing native target --help through. The actual branch-body regression checks missing/installed states, Original invocation/signal/mode/network/workspace, nonzero native return and backend fallback against a SQLite oracle; help tests execute the actual entry body with the real parser and stop at storage admission to establish correct help ownership. No concrete new defect found; no test/build executed here.

A completed tracked-code census read .ts/.mjs/.py/.sh/.ps1 files and looked for exact names of the nine historical input directories beside launch.json or direct local launch.json file reads. The sole identified direct consumer of the ten retiring local fixtures is `ABSTRACT-REPO-BUILD-TOOLS-AND-ADD-BAZEL/commands/📜️script.ts:6-22`: its `if(process.argv.includes("--launch"))` branch reads the sibling launch.json, reads removed root .vscode/launch.json, validates IDE helper picks and writes ticket-launch-source-current.json before exit. Remove that branch while preserving the script's --embedded-quotes branch and subsequent neutral authored-tokenizer diagnostic. The remaining imports may be cleaned only if unused after that removal.

No local executable consumer of the other nine launch files was identified. Actor-publication-port-inputs/final-source-list.json lists its launch.json as historical source inventory; it is evidence DATA, not executable code or a current entry point, so preserve it with the reports. `pb-inputs/📜️script.ts:40` checks root .vscode files rather than its sibling launch.json: this is a separate obsolete GUI assertion, not a dependency on the retiring local fixture. Other old root .vscode readers were observed in historical helpers (for example ap13 through ap18); deleting the ten local fixtures does not introduce those already-existing missing-root failures. Do not remove complete input scripts merely because those independent historical assertions exist.

## Retired File Verification and Lazy-Rebuild Intent Closure

Re-ran the exact no-exclusion git-ls-files conventional IDE-path predicate after parent retirement, checking physical existence of every selected tracked path. **Zero physically present tracked conventional IDE configuration files remain.** The ten index entries remain visible until changes are committed, as expected for read-only git inventory of deleted working-tree files. This conclusion is bounded to conventional directories and launch/task/seed/solution/code-workspace filenames, not arbitrary historical JSON evidence.

The parent's next real canonical launch reached lazy rebuilding and was refused by exact Original launch admission. Source explains the failure: build `📥️caller/📤️child/🟦️.ts:20-33` declares closure for initial args, selected Nx operations, watch callbacks and authored watch successors. It does not declare the native dashboard launcher’s lazy `run @semio-tech/repo-dashboard-rs:build` descendant. The rebuild path preserves the Original and package BuildScript correctly routes through the build owner, so the undeclared child reaches `readOwnedBuildInvocationV1:78` and is refused. The assertion is correct; its initial declarative closure is incomplete.

Coordinated with execution implementer: add only the exact native launcher preparation child to the pre-issuance closure when the actual dashboardInvocation recognizer selects a native launch, then process that child through the existing bounded closure machinery. Keep exact runtime assertion, tool/mode/network/native resources and Original deadline unchanged. Do not append capabilities during rebuilding. Test initial launcher intent, projected build child, unrelated/authoring/backend-option cases and refusal of sibling/extra-argument requests. Since child projection uses the same expansion function, the same declaration must remain stable under projection. No source edit or test execution was made by this auditor.

## Prebirth Child Declaration Plan Review

Reviewed the execution implementer’s proposed manifest-based plan. Run/daemon/preferences should declare their build child in authored `metadata.semio.execution`; the current build target already declares compile. Reading those fixed owner declarations before issuing Original capabilities and sharing closure expansion with child projection preserves the exact runtime launch assertion. This is accepted as the appropriate architecture; implementation is not yet reviewed in this section.

Coordinated exact requirements: helper reads only the fixed dashboard package project manifest, validates its exact owner name and closed orchestration declaration, and emits literal bounded child vectors. Native program arguments must not be appended to the build child, and caller-supplied capability objects or arbitrary caller paths must not stand in for authored manifest authority. Build must not declare itself; compile must remain its final authored child. Current dashboardInvocation also recognizes workflow, but the current manifest has no workflow target; the helper must handle that explicitly rather than assume every recognized name has an authored target.

The existing installation module has no observed import-time installer/compiler invocation; its visible top-level operations are owned imports, constants and function declarations. It imports the tooling-store and published-Cargo reader chains, so a claim of a fully dependency-free bootstrap requires checking that transitive closure, not just the helper body. No concrete import-time ambient dependency failure was observed in this read-only inspection. A minimal source-only helper is preferable if the new call would otherwise expand the initial bootstrap dependency surface.

## Final Authored Bootstrap Closure Acceptance

Reviewed the implemented dashboardBuildDescendantsV1, authored execution metadata, shared prebirth pending queue, phantom workflow removal, missing/stale status rule and new bootstrap regression. **No concrete defect identified by source inspection.** The helper reads the fixed physical project manifest, bounds its size/path, refuses symlink/nonregular/multilink files, requires the exact owner name, validates closed orchestration declarations and accepts only same-owner existing child targets. It does not read caller-provided manifest bytes or extend issued capabilities at runtime. The queue processes those literal descendants through its existing bounded seen-set; both initial issuance and child projection use it. Runtime exact launch admission is unchanged.

Current metadata is run/daemon/preferences/install → build → compile, with ten installed-executable test variants → compile. Unit/windows/compile have no such children. This terminates cleanly and supplies the actual lazy compile descendants. Removing phantom :workflow does not remove native workflow argv through :run. mayRunStale now avoids rebuilding only when the installation actually exists; absent status/stop operations can prepare the binary, while an installed stale binary can still stop/status a daemon without compiling broken current sources.

The main execution test imports the new bootstrap test. Its corpus is admitted by Ajv; a recursive SQLite query independently derives descendants from authored owner edges; actual createBuildBootstrapCapabilitiesV1 runs both backends/modes, checks unchanged initial/native intent and passes derived vectors to the actual strict reader body with isolated issuance ports. Malformed owner, foreign/missing child and closed-field vectors exercise helper rejection without editing the authored manifest. No test was run by this auditor and source acceptance does not imply native runtime success.

### Git Workflow Surface Read

Root workspace manifest has setup-git but no dedicated commit/push/pull/fetch/branch targets. Developer-tools metadata provides read-only Git inspection and a supervised terminal; no typed write-Git action was found in those two manifests. Developers can use the terminal route, so this is a direct-action discoverability limitation rather than proof they cannot complete Git work through the dashboard. No Git mutation, source implementation or scope expansion was performed.

## Native Owned RowClose Selection Invariant

For the active257-row refusal fixture, current forward helper grant(d) is still exact quoted copy and was not globally changed by close-only repair. In list receiver RowClose, demands(original,body512) quotes child.close_demands(512−metadata). advance passes child_grant(g,metadata), so actual child copy is d.copy−metadata, the quoted cost rather than the selected child budget. The concrete record receiver close_step reselects close_demands(grant.maximum_copy_bytes), hence execution can choose a different event. Furthermore parent enlarge adds metadata to a default child receipt, so paid outer progress cannot establish child progress or trigger an empty-outer-receipt idle fence. This is a concrete source selection mismatch; it does not establish the live runtime frontier. RowAdvance has a different explicit payload-body convention and must not be blanket rewritten. Root and Controls informed.

Actual focused29678 later established RowAdvance/index0, quote41copy/40release/depth4, receipt41copy/0release, finite66816 refusal. Independent source decomposition is33bytes list metadata plus8bytes record phase2 overhead plus a zero-copy field closure quote. NativeIntrinsicSharedField phase2 quotes its cursor.close_demands(body512) but receives/calls cursor.close_step with copy0 after both metadata subtractions. That is the active internal closure selection mismatch, distinct from the earlier latent list RowClose observation. Outer metadata can produce paid progress while child release never advances. Controls owns the owner-specific correction; no blanket forward funding or production guard weakening was recommended.

Controls narrowed the active cause further: StringCursor has no close_demands override, so the trait default reconstructs independent copy/capacity/release/depth getters and can quote copy0/release40 instead of the exact selected binding event. Independent read confirms the default reconstruction and that RetainedCloneClose.next_owner_demands delegates the actual factory/binding selected demand, matching String close_step execution. Therefore the512→0 caller budget observation is downstream of a bad child quote and is not alone the corrective target. The supported minimal fix is selected-close quoting in StringCursor with a real regression; no blanket parent grant increase.
