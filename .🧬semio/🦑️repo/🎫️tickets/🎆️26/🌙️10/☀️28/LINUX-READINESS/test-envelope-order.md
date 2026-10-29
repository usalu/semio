# Workspace Contract Envelope Registration Order

Read-only review of the actual workspace-named-input-retry.log and current Source. No tests launched or Source changed.

The contract error is a suite registration defect. Build TestScript, at library/🏗️build/📜️script.ts:89 beneath 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️, invokes Bun with 27 explicit test files. The first is 🧪️tests/🟦️.ts; the second is 🧪️tests/🕸️graph/🟦️.ts. Bun runs callbacks from the first file before loading the second, as the stripped terminal log demonstrates.

The first file’s actual lease admission test (:51) calls withBuildTestInvocationV1. This initializes the process-wide receiver ready promise without a declared birth. Later the graph barrel imports candidate (:2); candidate declares test:build-workspace-graph-laws (:14). Invocation receiver declareBuildTestEnvelopeV1 (:37) correctly rejects this late declaration. Kernel’s identical declaration is not the conflict. The terminal summary is 155 passed, 1 skipped, 6 failed, 1 error across 162 tests and 27 files; private-store failures have separate ownership.

Recommended repair: make the entire authored 27-file TestScript row establish its existing closed birth before loading any test file. An explicit owned Bun preload module declaring owner test:build-workspace-graph-laws, maximumElapsedMilliseconds 300000, capabilities {} gives that row deterministic registration, and must be included in the target executable Source closure. Reuse the same declaration in independently runnable graph entries. A shared birth side-effect import as the first import of the first suite entry is a smaller coherent alternative, provided transitive imports cannot initialize the receiver first. Merely swapping file order is dependent on test runner loading behavior. Do not change the receiver’s envelope-late guard or allow a birth to be added after the receiver has started.

Preserve inherited Original checks at invocation receiver :46–50, identical declaration checks at :36, and the single shared lifetime. Test the real ordered multi-file row (or a bounded fixture reproducing first-file invocation followed by second-file identical declaration), rather than only the graph barrel alone. The isolated kernel gate already passed and does not reproduce this ordering.

## Actual Bun Hook Placement Proof

Rootcreatedhandcraftedtwo-file diagnostic inputs under suite-probe (noSourcecopied). RegisteredBun withoutpreload passedfirsttest, ran importedsharedafterAll, thenfailedsecondbecausemodulelifetimewasalreadyclosed. The same exactfiles with--preload ofthelifetime modulepassedboth andfinalizedonceaftersecond. Thus a first-entrysideeffectimport alonewouldnotfixcrossfileOriginal/privaterepositorylifetimes. Actualpreload isrequiredhere.

Newauthoredsharedbirth preloadimportsprocess invocationandprivaterepositorylifecycle helpersatglobalpreloadscope,declarestheexistingclosedgraphbirthfromJSON, andisregisteredasanexecutableSourceentry. DefaultTestScript commandnowpreloadsitbeforethe27testfiles. MainadmissionfixtureusesactualprivateRepositoryWorkspaceV1.invoke/root forbothstoreandartifactpaths; checkoutfence staysunchanged. Actualtwo-filegenuineadmission/kernelproof isrunningbeforefurtherwholeworkspaceattempts.

## Corrected Per-File Ownership Repair

The shared preload recommendation above is superseded. The real mixed graph import chain contains incompatible authored births: graph candidate declares a capability-free graph-law birth; ledger declares its canonical native build caller grant. Preloading either birth cannot make that chain coherent. The receiver's birth-single and envelope-late guards remain unchanged.

The public workspace command now selects its files through build/🗂️suites/🌐️workspace and invokes the existing process suite runner. Each file runs in its own received Original child, with the canonical suite's budget, platforms, tools, scratch, capability requirements, launcher and worker policy retained. Group failures aggregate while cancellation stops the run. The graph barrel is now a standalone selection regression, with its former test imports represented explicitly in the selection.

The closed selection contains all 48 former leaves plus the new graph-entry regression. An independent Python SQLite recursive closure reproduces the 27-entry/22-edge expansion, and Ajv validates the selection with the same acceptance and rejection results. Initial missing-implementation red was captured; the three selection tests then passed with 106 assertions. No common birth rename, receiver guard change, or native suite removal was used. Superseded shared birth/preload files and their main import were removed.

The first genuine four-file integration attempt (main, candidate, ledger, kernel) reached the real ledger test and exposed its existing private workspace mismatch: its store operations received the checkout Original. Three ledger fixture bodies now first validate the native received test birth, then use the existing private repository invocation issuer for the private root, with their authored 60-second bound. The format guard and storage implementation remain unchanged. The normal-admission rerun is recorded in generated/workspace-suite-process-fixed.log; its status must be read from that run, not inferred from the pure selection tests.

The executable Source roster now explicitly retains all selected leaves. Its independent literal evidence accepts the closed workspace selection document as its authored dispatch owner. Two unrelated stale expected entries were updated after verifying their actual authored dispatch calls: dashboard compiled freshness and installer Go lease tests. The two-file hook diagnostic remains retained under suite-probe.

## Private Ledger Fixture Follow-Up

Normal-admission process attempts preserved the first cause at each boundary. The first correction still nested the private issuer inside the checkout Original, and the nested-envelope guard correctly refused it. The fixture now awaits received native birth validation, then enters the separately authored private repository Original outside that callback scope. Neither Original guard was changed.

Once ownership was coherent, the dataset schema correctly refused the former ticket scratch sourceRoot because it escaped the private workspace. Inputs are now handcrafted under repository.scratch. Neutral invalid root vectors (parent traversal, absolute POSIX, Windows absolute, nested traversal) agree between Ajv and the actual dataset reader.

The next failure was a stale test reader interpreting the current Brotli inference record as UTF-8 JSON. The independent physical-record check now uses bounded system Brotli decoding, validates the actual closed header with Ajv, verifies its result digest, and retains warm-result equality and exactly-one-computation assertions.

Focused genuine native ledger validation84830 is terminal0: 1 test, 36 assertions, 6 filtered tests. This is a focused result, not full ledger or full workspace acceptance. The final canonical four-file run is recorded separately in generated/workspace-suite-process-green.log.

The canonical route audit also caught an existing Python-spawning policy-literals test in the 15-second workspace-laws row. It now belongs to the existing 300-second workspace-phase-laws tree; no birth changed and the subprocess budget law remains intact. Route audit85627 passed9tests318assertions. Source-roster oracle passed3tests555assertions, including all49 selected executable entries and actual dispatch evidence.

## Final Genuine Harness Proof

Normal-admission canonical four-file run91133 completed with status0. The actual isolated leaves passed: ledger5tests102assertions (2 explicit macOS-only skips), custody kernel7tests28assertions, graph candidate2tests335assertions, main5tests3890assertions. The integration orchestrator also passed. Thus nineteen actual leaf tests passed with4355assertions; no birth-single, envelope-late, closed Original, nested Original, private format-owner, dataset-root or stale-record-reader error remained.

The Source selection, source-roster oracle and canonical route audit are green as recorded above. Every original leaf remains selected, with the former barrel replaced by one additional standalone selection regression. The authoritatively requested full canonical workspace gate is now recorded in generated/workspace-isolated-contract.log; its result is separate from this focused proof.

## Full Canonical Workspace Gate96614

The single authorized normal canonical command repo run workspace:test-workspace-build settled status1 with no OS signal. It completed graph coordination and produced immutable SDK9a41ff598616fc038e8418d7fefe9603e3bfb4ca6d987cdeb9feff164bfc1cdb, but no selected test leaf ran.

The daemon's actual daemon.log reports the existing eager second workspace-root watcher failing at watchOutputFiles watcher.js176 with the Linux OS watch limit. It shut down through its owned service path. Ordinary fallback then failed separately in library/🟨️.mjs createDependencies: SEMIO_NX_IMPLEMENTATION_IDENTITY differed from nxBuildDependencyImplementationV1 before graph publication. Neither guard was changed. The complete first-cause log is generated/workspace-isolated-contract.log; daemon diagnostics are under .cache/b/o/m/nx/graphs/9a41ff598616fc038e84/930e701156b561fef798.

The focused canonical four-file proof remains accepted; this full gate is failed before test execution, not a pass or a failed harness leaf. No restart or duplicate full command was launched.
