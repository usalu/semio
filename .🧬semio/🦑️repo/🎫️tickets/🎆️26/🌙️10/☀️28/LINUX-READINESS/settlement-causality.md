# Host Hook Settlement Causality

Receiver global snapshot removal was already present at takeover. The original dynamic-hook probe ran after removal and failed: PID615942, status1/null, 1 pass/1 fail, 120000ms beforeEach/afterEach hook timeout. The same pre-removal probe PID613803 failed identically. Snapshot removal alone is not a qualifying repair.

Investigating dynamic afterAll registration independently before further full Cargo runs.

The predecessor snapshot removal was restored exactly to the current Git baseline after read-only diff inspection, preserving implementation-pin and runtime-entry context. The bare dynamic-host hook probe is queued under normal Bun capacity; no duplicate run was launched.

Static sandbox cleanup requires preserving receiver fixture lifetime: a hook registered by sandbox import can run before receiver drainage, so unconditionally disposing all live resources there must be verified against retained fixture release ordering, not only command completion. Existing sandbox release remains idempotent and process-exit disposal remains installed.

Primary runtime documentation: [Bun lifecycle hooks](https://github.com/oven-sh/bun/blob/main/docs/test/index.mdx) describes file or preload registration. [Parallel lifecycle semantics](https://github.com/oven-sh/bun/blob/main/docs/test/parallel.mdx) distinguishes file scope from shared module registries and preload hooks wrapping every file. These support probing file registration separately; they do not prove the pinned runtime causal result.

Exact pinned-runtime cause candidate: [Bun issue44552](https://github.com/oven-sh/bun/issues/44552) reports the same1.4.2+744846f84 and same timeout diagnostic. AsyncLocalStorage registration wraps callbacks; runner arity detection then treats zero-argument hooks as completion-callback hooks. A scoped helper can therefore execute fully yet leave Bun awaiting completion. Narrow candidate retains existing dynamic sandbox cleanup registration/order and explicitly calls the supplied completion callback. This avoids discarding implementation-pin context or shortening fixture lifetime.

Added a regression in existing receiver corpus/schema/harness/owner: first actual callback, second callback creates and disposes a sandbox, cleanup follows; filesystem existence and independently ordered SQLite events must agree with the neutral callbackOrder vector. Red PID621140 queued normally, output receiver-sandbox-scope-red. No runtime fix applied before red.

Bare host control30206 completed0/null:2pass0fail2assertions57ms, showing dynamic hooks outside issued ALS are healthy. Permanent receiver red PID621140 ended1/null08:01:33: second actual callback completed scope events and lease release, receiver cleanup ran, but Bun hook waited5141ms and child failed1pass/1fail. Applied only explicit completion registration in sandbox root atTestEnd. Green PID622756 is actual normal receiver regression, unchanged ownership and deadlines.

Genuine budgeted command PID623355 accepted0/null:2pass0fail2assertions151ms. Full receiver owner within combined command623554 completed16tests, but the combined command failed because the second file attempted to reuse a closed receiver. A separate bare isolation launch624574 was correctly refused for missing parent envelope. Correct genuine isolation PID624840 accepted0/null:5pass0fail46assertions10.40s, including authored preparation/body separation and expected deadline failure observation. Neither failed launch is accepted as a gate.

Temporary receiver/kernel cleanup DEBUG traces removed before fullCargo run625340; immutable300s file and30/60s bodies retained. Full Cargo launch is unchanged genuine birth, not cold Cargo and not a selected pattern.

Standalone receiver626101 accepted0/null16pass0fail212assertions10.42s. Sandbox625602 failed its existing explicit-parent session fixture under deep ticket TMPDIR; fixtureBaseV1 retry627605 reproduced because that helper only avoids managed mounts. Final fixture resolves its short host base through dirname(testSandboxParentV1()), preserving maxRootLength. Sandbox628390 accepted0/null19pass2explicitMac-onlyskip418assertions.

FullCargo625340 failed before laws on the native metadata predicate shadow TDZ. Course independently repaired that current Source to inner result precise before this owner edited Source; no algorithm/source rename was applied here. Current SHA256 6985ae27b05beef179e2e0f905b4e197928bebb4f2acd2bbcf875861ffc0a474. Added permanent actual-system-stat regression to existing usage owner using existing neutral root files against Python lstat oracle. Full usage629747 labeled red actually accepted current repaired Source:0/null12pass0fail204assertions1193ms. This is not claimed as a separately observed usage red. The earlier actual Cargo TDZ receipt remains the red evidence.

## Final Change Scope

- Process testing sandbox root: explicit completion callback for its existing dynamically registered physical cleanup hook. Scope, disposal, process-exit cleanup and Original context are unchanged.
- Existing receiver corpus/schema/harness/test owner: neutral sequential callback/sandbox-release ordering and independent SQLite event oracle.
- Existing sandbox test owner: explicit session fixture uses the host base selected by existing bounded sandbox-parent resolution, keeping the authored root bound.
- Existing directory usage test owner: actual filesystem stat regression consumes the existing neutral file vector and Python oracle. Course owns the independent predicate-shadow Source repair.
- Receiver and Cargo test owner: removed temporary DEBUG tracing only.

Temporary diagnostic input scripts/reports remain retained at the ticket root. Generated receipts remain under the shared generated directory for Root consolidation and final ticket cleanup; no shared outputs were pruned during active peer work.

## Current Full Cargo Boundary

Fresh coherent Source fullCargo630279 ended1/null08:12:03,13pass3fail235assertions216.50s. All three Kernel target-selection cases completed without the former120s done-hook stall. Full Cargo is not accepted: empty-selection isolated child attempted direct Nextest argv as compiler consumption and was refused before its intended primary; actual owner-policy lint also found undeclared/missing clippy.

Empty-selection oracle now opens an admitted native Cargo test compilation selection, then renders the independent Nextest step through the existing issued admit method under the same held configuration and compiler writer. The closed consumption schema, Source custody and lifetime scopes remain intact. Removed temporary Cargo admission fixture DEBUG tracing. Root and Rust owner notified about adding clippy to the authored root component set; no ambient rustup command used.

Root and the Rust/catalog owner confirmed component Source ownership. Added only clippy to rust-toolchain.toml components; nightly date, minimal profile and targets are unchanged. FullCargo640589 is the single normal-funded retry for these two observed failures and is currently queued before fixture start. No artificial grant, policy ceiling or timeout adjustment was made.

| Accepted Verification | Outcome | Scope |
|---|---|---|
| Receiver regression622756 |0/null|1 parent / 2 child passes, 15 parent assertions|
| Genuine budgeted command623355 |0/null|2 passes, 0 failures, 2 assertions, 151 ms|
| Receiver lifetime626101 |0/null|16 passes, 0 failures, 212 assertions, 10.42 s|
| Genuine isolation624840 |0/null|5 passes, 0 failures, 46 assertions, 10.40 s|
| Sandbox lifecycle628390 |0/null|19 passes, 2 explicit Mac-only skips, 0 failures, 418 assertions|
| Native usage629747 |0/null|12 passes, 0 failures, 204 assertions, 1193 ms; Course repaired Source|

FullCargo remains unaccepted until its own actual final receipt. Earlier failed full630279 had13pass3fail and must never be summarized as green.

FullCargo640589 ended1/null08:19:00 before laws. Its actual declaration red exposed the Rustup installation Source exact-set check, which still demanded only llvm-tools-preview/rust-src and refused the now-authored clippy set. The public ToolchainRequest schema already admits unique nonempty strings; the exact-set Source and neutral Rustup fixture need coherent update by the coordinated Course owner. No installation Source/schema change has been made by settlement owner yet, and the same failed invocation will not be restarted without a coherent Source repair.

Read-only Rustup follow-through found three coherent component extents in installation Source: sourceRequest exact set, pinned distributionPackages list (clippy-preview on host), installed component census (clippy on host), plus existing neutral request fixture. These were relayed for coordinated Course repair so the new component remains inside verified manifest/download/installed census checks. No digest/census check is bypassed and no expensive Cargo retry is launched while Source remains incoherent.

Root assigned the coherent Rustup seam to settlement owner and notified Course. Neutral request fixture updated first to the three declared components; new permanent test compares actual root TOML against that vector using TOML parsing and independent SQLite projection. Installation Source then retains the exact three-component set, adds clippy-preview to verified distribution package downloads and clippy-host to installed component census; every other Source/digest/capacity/target/profile check is unchanged. Genuine Rustup650103 accepted0/null:12 passes,0 failures,240 assertions,985ms. This does not substitute for actual installer/clippy execution; unchanged full Cargo is now the next verification.

FullCargo651641 ended1/null08:30:20 with14passes2failures239assertions264.91s. The coherent three-component installer/census completed and independent empty-selection oracle passed. Sole primary was stale consumed-lint expectation omitting required --locked/--offline; assertion now requires both flags. It remains strict about all other forwarded arguments.

A root Course run exposed immutable generation identity reuse: metadata addresses include request digest, but physical Rustup home used only channel/host. Different component requests on the same channel therefore point at a previous generation without their own exact intent, correctly refused as foreign. Diagnosis and request-addressed physical home remedy were relayed; no foreign generation deletion or adoption occurs.

Namespace causal TDD: neutral generation-home vectors and independent SQLite projection were added before implementation. Genuine Rustup675057 ended1/null,12passes1failure because request-addressed home function was absent. Source now derives bounded RUSTUP_HOME under authenticated bootstrap home from the same request digest used by receipt metadata and explicitly passes that home to all compiler consumers. Shared bootstrap CARGO_HOME, exact intents/seals/directory identity, global home custody and foreign-directory refusal remain intact; no old generation is adopted or deleted. Genuine Rustup675459 accepted0/null13passes0failures248assertions819ms. Actual root reuse still needs the Course retry, and the next unchanged fullCargo verifies this physical layout plus the strict locked/offline assertion.

## Freeze Handoff

At Root quiet-window coordination, settlement Source/report/input edits are complete and frozen. FullCargo675825 continues under its original invocation and produces generated output/receipt only. The final fullCargo result must be read from generated/cargo-full-request-home.outcome.json and its log; this report does not predict acceptance. No further Source, report or input edit is planned until Root ends the formal quiet period or explicitly reopens a repair boundary.

Additional changed existing paths for Root consolidation: root rust-toolchain.toml; library caching/bootstrap/tools/rustup installation Source, Rustup neutral test fixture and owner; Cargo main owner strict locked/offline assertion and admitted Nextest oracle; Cargo admission fixture diagnostic trace removal. No new test schema authority or public package entry was created.

Formal Source/report/input quiet was respected until Root explicit release08:46:26. FullCargo675825 ended1/null08:38:33 before laws: the old archive census charged a package prefix that Rustup never materializes. [Pinned Rustup1.29 package Source](https://raw.githubusercontent.com/rust-lang/rustup/1.29.0/src/dist/component/package.rs) strips the first archive component during extraction and installs component-relative files into the toolchain.

After release, existing closed archive fixture/schema gained actual final-generation paths, an accepted long nonmaterialized prefix and a final-install-only overflow refusal. Genuine Rustup688506 is red12passes1failure against old Source. The validator now checks both actual temporary extraction and final installation paths through unchanged256-character/portable-segment guards. Caller passes its authenticated final directory. Genuine Rustup690022 accepted0/null14passes0failures255assertions852ms, including Python tarfile/pathlib and independent SQLite replay of the same neutral vector. Request namespace, complete archive/source/digest/census and Original custody remain strict.

Root also reported a distinct Course89311 physical generation identity refusal; no identity Source change, foreign adoption, or deletion is made without its authentic directory trace. That diagnosis remains distinct from the private archive-path acceptance.

FullCargo692654 is retained1/null08:59:22: real archive/census/installation and empty-selection passed, then the unchanged300s shared Original expired during owner-policy Source admission. No budget was raised. Root authorized bounded parallel verified acquisition of the same eight pinned packages inside its Original and existing operation/capacity controls. Neutral vectors/schema now define parallelism2, full eight-package completion, exact error/undefined, cancellation/pre-cancellation, admission bound and accepted-sibling drainage before return.

Read-only sandbox forensic paths sent to Root: session fixture owner line299 creates a unique mkdtemp child below dirname(testSandboxParentV1()), and line314 removes only that returned child. Layout owner defines parent selection at54, sandbox root disposal at82 only removes its captured root, home rmdir at83 is nonrecursive, and reclaim owner at38/44 uses the explicit session fixture parent. No forensic deletion cause is claimed and no cleanup Source is changed.

Bounded acquisition TDD751482 is actual red14passes1failure before implementation. Genuine Rustup755378 accepted0/null15passes0failures308assertions987ms after implementation. The existing protocol now caps parallel verified archive acquisition at2 within existing32 operation bound and same funded Original; each active archive uses a genuine issued control refinement. The batch stops new admission, cancels accepted siblings on first error/cancellation, drains them all, and preserves exact first failure including undefined. Neutral cases retain all eight normal packages, peak2, no new admission after stop, pre-cancellation and over-bound refusal. No deadline, memory grant, source/digest/census/identity or package set was weakened. Next fullCargo is the justified real performance integration, not a missing-history rerun.

FullCargo759167 ended1/null09:12:10:14passes2failures249assertions221.28s, comfortably inside the unchanged300s Original after bounded acquisition. Sole new primary was exact-mode normalizer misclassifying llvm-cov Nextest profile options. [Official cargo-llvm-cov changelog](https://github.com/taiki-e/cargo-llvm-cov/blob/main/CHANGELOG.md) documents --cargo-profile for nextest and nextest-archive; their --profile remains the independent Nextest configuration.

Schema-owned exact prefixes and existing nextestModeFlags vectors now cover nested coverage, archive output versus snapshot input, wrong/duplicate compiler modes and opaque tails. Initial809056 ran old vectors after an edit-script parse failure and is not causal;813978 exposed a test schema-import mistake, also not causal. Actual820378 red is the original compiler-mode refusal against valid new vectors. Source now classifies only the exact declared prefixes and recognizes prebuilt input only for run/list, preserving the exact mode refusal and tail. Focused825240 accepted0/null2passes0failures72assertions1010ms (Nextest and native-release families), with Node parseArgs as independent option oracle.

Rustup772527 also accepted the strengthened independent SQLite window oracle for batch started/completed/peak/pending observations. Full Cargo remains unaccepted until its new actual terminal receipt.

Second quiet-window handoff: all current settlement Source/report/input edits are complete. Focused mode-prefix acceptance825240 is retained; fullCargo828855, complete profile policy829001 and complete profile consumption829006 remain the same normally funded queued handles. Generated output may continue during Root formal quiet. No additional authored Source/report/input edit or test launch is planned until explicit release; terminal outcomes must be read from their generated receipts and never inferred here. Current production Source edits also include existing mode/profile normalizer/schema and existing consumption corpus/owner; no new owner/schema file was created.

## Full Cargo 828855 and Forwarding Follow-up

The unchanged full Cargo owner 828855 settled 1/null at 09:38:06 UTC: 13 passed, 3 failed, 249 assertions, 208.09 seconds after normal admission. The previous nested profile conflict no longer occurs. The new primary refusal is `Declared Cargo compiler or program forwarding intent required` for the physical `llvm-cov nextest` command with an opaque program tail. Exact nested receiver vectors and independent SQLite projection were added to the existing forwarding corpus/owner before production changes; the retained `cargo-coverage-forwarding-red` invocation remains normally queued. The raw Cargo command API must continue returning `llvm-cov`; only the forwarding receiver projection may recognize the exact nested test/nextest prefixes. Unknown/report/archive/compiler forms retain refusal.

The profile policy owner 829001 passed 11 tests and 477 assertions. Consumption 829006 had 22 passed and 2 failed, 1,388 assertions: its stale message substring was replaced with the actual closed refusal code/key/values assertion, and its source-boundary chain now includes the actual artifact-plan wrapper and input-owner edge. These assertion corrections still require a current complete consumption run.

A separate failed-build split-Unicode retention-removed case reached its fixed 10,000 ms body limit and received SIGTERM instead of the expected exit 101; five sibling failed-build cases passed. This is not a green full Cargo result. The body performs real admission/planning before its Node output oracle. No budget was increased, and no blind full retry is authorized. A causal performance observation is required before changing implementation.

Authored Source/report/input quiet 09:41–09:47 UTC was honored; work resumed only after Root explicitly released at 09:47:37 UTC.

### Exact Nested Forwarding and Original Home-Container TDD

Forwarding red 885510 settled 1/null at 10:00:18 UTC (1 pass, 1 fail, 78 assertions): the exact coverage-nextest opaque-tail case reached the intended declared-forwarding refusal. Production now preserves the existing global-option scan as a private index projection. The public raw command still returns the physical command; only forwarding maps exact schema-declared llvm-cov test/nextest prefixes to their existing program intent. No report, archive or compiler-prefix route was added. Green 893160 remains normally admitted/queued.

Initial unpublished home ownership needed the original base-container refusal after request-owned homes were introduced. Four neutral actual-filesystem rows distinguish absent homes, foreign bootstrap Rustup container, foreign Cargo container, and both. Red 892527 settled 1/null at 10:01:32 UTC on the absent production helper (0 pass, 1 fail, 2 assertions). The controlled helper checks the authentic ResourceLeaseControl and exact existing paths without writing or deleting; production invokes it on bootstrap RUSTUP_HOME and CARGO_HOME before h.json publication. Existing request generation, identity and intent guards remain unchanged. Complete Rustup green 898115 is retained.

The exact prior failed-build case is queued as diagnostic 895223 with genuine birth, exact isolation selection and existing phase profiling. The unchanged 10-second body and 300-second fixture limits remain in force. This diagnostic cannot count as full Cargo acceptance.

Forwarding green 893160 settled 0/null at 10:01:46 UTC: 2 passed, 102 assertions, 294 ms, including the unchanged baseline compiler selector refusals. This qualifies the narrow nested forwarding repair.

The direct isolated phase diagnostic 895223 settled 1/null at 10:03:35 UTC with the exact expected `node exited with status 101` primary, no overflow, seven assertions and completed body checkpoint, 9,640.07 ms body time. It did not reproduce the timeout. Only issuer CLI phase JSON was emitted; the actual test child did not receive that phase profiling environment, so no body attribution follows from this artifact. A single exact targeted JSC observation 911790 uses the existing closed profiling request and first-party preload with unchanged body/file limits. It remains normally queued. No performance Source edit is justified yet.

Complete bare Rustup 898115 settled 1/null at 10:08:10 UTC (15 passed, 1 failed, 249 assertions, 3.17 s). All new actual foreign-home rows passed. A concurrent existing durable-directory test now calls withBuildTestInvocationV1 and correctly refuses this bare launcher for absent Original parent; the owner is being rerun through the genuine-birth route as 949007. This is not full-suite green.

Consumption 898943 settled 1/null at 10:08:13 UTC (23 passed, 1 failed, 1,392 assertions, 1.97 s). The refusal-key assertion now passes. The library native producer acquired the same artifact-plan wrapper as the framework producer: its fixture now asserts both buildRepositoryCargoArtifacts→withRepositoryCargoArtifactPlanV1 and withRepositoryCargoArtifactPlanV1→repositoryCargoCompilerInputsV1. No production Source change was made for this stale chain. Current complete consumption owner is 948812.

### Current Focused Acceptance and Actual CPU Causality

Current complete consumption 948812 passes 24 tests and 1,393 assertions in 2.64 s, 0/null. Current complete genuine Rustup 949007 passes 16 tests and 323 assertions in 3.72 s, 0/null, including the new original-home-container refusal. These qualify their exact focused scope; full Cargo remains unaccepted.

Actual JSC observation 911790 ran child 953584 with the expected first-party --preload (observed in its real /proc argv). It reproduces the fixed body timeout at 10,021.88 ms, during actual cargo metadata --locked --offline before the Node output command: only four assertions, no completion checkpoint. Original cancellation kills Cargo with SIGTERM. Profile `🗑️generated/failed-cpu/repo-953584-1.jsc.json` is 5,440,948 bytes, 3,203 traces across 183.646 s. Read-only reduced results are `🗑️generated/failed-cpu-analysis.json`. Final 15-second observed stacks include 279+36+28+24 samples in lease SQLite COMMIT through initialize:221, and substantial samples in complete nativeCompilerSourceClosureV1 readFileSync:69, SHA256:72 and lstat:39. These sampled stacks establish actual expensive Source/lease work, not measured operating-system CPU time or proof that one alone consumes the whole deadline.

The repository native-compiler wrapper calls a complete native source closure at initial entry and again on each profile.recheck; held configuration admission invokes those checks. No Source hashes, lifetime checks, leases, or deadlines were removed. This Source seam was reported to Root and the Source validator owner for ownership coordination before any optimization. No additional full Cargo is running.
