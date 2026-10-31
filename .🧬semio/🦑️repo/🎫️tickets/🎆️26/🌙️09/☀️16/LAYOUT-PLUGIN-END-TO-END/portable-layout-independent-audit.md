# Portable Layout Guarded Packet Audit

Read-only source/packet review2026-10-29; no apply, semantic test, install/setup/publication/cache operation executed. Packet published19files, authority=false/sourceApplied=false, before/after SHA256 and exact expected splices/pointers. Source edits remain root-held.

Concrete defect: install.original.json fixture packet has pointer empty string for whole-document JSON replacement. portable-layout-edit.py alter() derives steps=[] and accesses steps[-1], causing IndexError before outputs write. Apply must handle root replacement with exact complete expected-value precondition and return the replacement to its caller. Owner notified. generation.original.json changes=[] row is only formatting and does not cause this exception.

Executor now rechecks each source hash immediately before write in addition to whole-packet preflight, reducing stale-input overwrite risk; root still must keep production sources stable through approved rollout. Recipe6to7, isolated-to-hoisted descriptor/config/schema/producer arguments/provider fixture paths and bootstrap inclusion of full recipe are represented together. Existing expected-byte vectors are regenerated rather than accepted alongside legacy recipe. This is source coherence evidence only, not real Bun layout/setup acceptance. Final corrected apply source remains pending reread.


## Corrected Source And Fresh Packet

Fresh source reread confirms alter() now checks whole-document expected equality and returns a deep copy on empty pointer; apply() assigns that returned root. This closes the reported IndexError/root-replacement defect. Raw UTF8 reads preserve CRLF, rendering uses minimum actual indentation, and each target is hash-checked again before write. No new actionable defect found in this focused correction.

Fresh packet SHA256 `5b94f243b1f4c2dcd2426f3bc7415d532eebffed85c3f54f2cf76e790d9b9ea2`,18files89operations, maximum declared absolute179UTF16. The whitespace-only generation.original row is absent. Packet retains authority=false/sourceApplied=false; root-pointer install.original fixture replacement remains and is now supported by inspected executor. Owner reports preparation55093 EXIT0 and exact-byte --check-prepared48866 pending; reviewer did not execute either checker or apply. All production rollout remains root-held.


## Supported Post-Recipe Ordering

Read-only operational source trace2026-10-29; no command/test/setup executed. Normal `repo tools exec bun -- test <manager-test-entry>` is an existing early owner path and can be attempted after the approved source recipe change before full tools setup all. It does not pass through the ordinary installed-generation readiness guard.

Evidence: `library/🏗️build/📜️script.ts:117-126` parses the genuine CLI then handles tools exec/list at123 and returns before admitted build/runtime command routing. The tool-registry command creates ordinary host capacity admission and calls runToolV1 with heavy=true unless caller asks --light. `process/🧰️tools/🏃️run/🟦️.ts:120-126,150-181` resolves the registered executable, sanitizes environment, waits capacity and executes through owned process handling. That path contains no ensureBuildEnvironmentV1 or dependency-generation receipt assertion. Tool resolver selects pinned/runtime/provisioned executable by its own registry; this is not a fabricated Script Original or alternate build adapter.

The later runtime branch `library/🏗️build/⌨️command/⚙️runtime/🟦️.ts` is distinct: native recovery is checked first, dependency-requiring commands call ensureBuildEnvironmentV1, tools setup all invokes its authored setup-publication hook before opening the dependencies store. That later store/readiness route does not retroactively apply to the earlier exec return. runNativeRepoRecoveryV1 itself accepts only closed authored run targets and is not needed or reused for tools exec.

Bun preloads still apply: runtime-pin validates/selects the existing pinned runtime; test store fence protects live checkout stores. Inspected preload sources add no recipe-key/generation readiness check. Manager test entry imports existing Ajv/smol-toml/jsonc-parser packages from the currently installed layout. Thus ability to start before setup is source-supported, conditional on that existing installed runtime/dependency package set remaining available; this trace does not guarantee a particular host can resolve every import or pass every test after recipe edits.

Exact supported order after Root releases source application: (1) apply the reviewed guarded source packet only; (2) run normal managed focused manager tests with `repo --tool=bazel tools exec bun -- test <ABS_MANAGER_TEST_ENTRY>` (no --light); (3) run the existing `repo --tool=bazel tools setup all` once under its real setup owner; (4) run declared ordinary dependency-requiring acceptance gates. If focused test import/runtime resolution refuses, retain that refusal and use existing setup owner next; never inject a loader, replace receipt, fabricate control or bypass source ownership. No setup, ordinary recipe flip or runtime success is authorized by this audit itself.


## Final Matcher Typing Packet

Fresh final packet SHA256 `52a175ed02f93d4d4880a46057e8140a2c2fc8502c8e60626ec6cc91c1ab613b` independently matches owner publication;18files89operations retained. Proposed equality law now uses expect<unknown> for recipe.linker versus schema values, acknowledging unknown input typing without changing test runtime semantics or production acceptance. Root-pointer/executor fix remains unchanged. No additional actionable defect found in this narrow final delta. Normal --check-prepared48866 remains owner-managed pending admission; no duplicate check, patch apply, setup or runtime execution performed by reviewer. This final SHA supersedes earlier packet identities for rollout.


## Actual Root Strict Evidence

Root reports genuine session61699 terminal EXIT0. Independently read generated/final-bootstrap-strict-3.log: actual normal capacity admission occurs after roughly885seconds; no TypeScript diagnostics follow. The stdout/stderr log itself contains no exit-code receipt, so EXIT0 provenance remains roots terminal tool result rather than inferred from silence. Independently parsed final-bootstrap-strict-input.json contains exactly14root entries and strict/noUncheckedIndexedAccess/noEmit flags. This covers the named bootstrap/storage source roots at admission, with borrower production edits already present as reported by root; it is a TypeScript compile result, not native runtime, exact portable patch application or setup success. Portable48866/full borrower45981/focused borrower66928 outcomes remain pending owner evidence.


## Exact-Byte Check Refusal

Owner48866 TERMINAL EXIT1 before writes, reporting proposed-byte mismatch in dependency neutral fixture JSON. This disproves any claim that52a packet was reconstructed-byte validated. Owner identifies text-spliced JSON incorrectly classified as structural and reserialized; preparer now tracks structural modifications separately so raw JSON text hunks retain exact formatting. Fresh22963 preparation/identity and a new exact-byte check remain pending. No source application occurred; prior source/hash inspection remains separate from this failed checker.


## Final Reconstructed-Byte Packet

Fresh packet SHA256 `589b27671fa2bea62c783bea1d0bd5606c94779599040874d1f7e5bf257a56a2` independently matches owner22963 terminal report;18files retained. Source reread confirms an explicit structural set marks modify()/generated structured vectors, while raw text JSON replacements are tagged text and retain splice bytes. Whole-document root fix remains. This closes the reported serializer-classification defect in inspected preparation source. Owner reports22963 executed preparation and no-write apply(False) together, EXIT0, guardsValid=true/sourceApplied=false and the same parsed-byte SHA. Terminal summary receipt is separate provenance from raw output; no full log was reconstructed or claimed by reviewer. Final589b packet supersedes52a for rollout. No production apply or reviewer semantic execution.


Validation receipt independently read after publication: observed-normal-terminal-summary, authority=false, session22963/terminalTool1e19d4, exitCode0, normal admission, sourceApplied=false, guardsValid=true,18files89operations/max179UTF16 and exact589b packet SHA. It honestly labels itself an authored observed summary rather than reconstructed raw stdout. Values align with fresh packet read and owner terminal report.


## Actual Applied Source Identity

Root reports guarded18file source batch EXIT0, no setup/cache effect. Independent fresh byte read confirms packet SHA `589b27671fa2bea62c783bea1d0bd5606c94779599040874d1f7e5bf257a56a2` and all18current target SHA256 values equal their exact afterSha256; mismatches=0. Current Bun descriptor is recipe7/hoisted with node_modules canonical provider layout; root bunfig install linker is hoisted. Complete manager recipe participates in bootstrap key, and structural schema/provider/vector edits are exactly the reviewed guarded bytes. This confirms source application identity only, not installed generation, setup, real manager tests or cache publication. Preserve shared hunk attribution: only exact packet target bytes are covered by this match; unrelated caller/borrower/launcher work remains in their owning packets. Root reports prewrite lease snapshot had no dependency sync/bindings action; reviewer did not independently interpret that snapshot or run effects.


## Actual Applied Manager Gate Failure Audit

Independently read generated/portable-manager-green.log for41178:85pass4fail837expectations,89tests. Portable recipe/layout law passes; embedded independent Python oracle passes. These do not make the full manager gate pass. Root separately reports9297Python pass. Setup remains held.

1. Actual directory fixture is stale: manager tests :233 creates node_modules/.bun/<entry> and TypeScript+hash suffix; :236 removes the old .bun path. Current hoisted expectedProviders returns plain/scoped package entries, so all5providers are missing. Build/remove fixture locations from current installed descriptor; retain real filesystem traversal and missing-provider refusal.
2. Protocol pinned-fixture mismatch is exact: :324 excludes /$defs/ManagerProtocol/const only from actual installRegions, while regenerated pinnedInstall includes that region. Log diff shows precisely the missing protocol object, not changed runtime behavior. Align intentional filtering on both sides or generation while preserving explicit protocol assertion :326 and recipe key-order checks. Do not delete the protocol law.
3. Real-tool flags case :275-285 timed out after5000ms and killed1dangling process. It synchronously probes installed managers and help for lock/sync/verify, plus repeated npm config. Actual log does not identify the slow tool, so attributing this to one executable or harmless fixture behavior is unsupported. Preserve real installed-tool flag verification and current deadlines/caps; retain per-child identity/outcome or safely avoid identical redundant probes before choosing a repair. No reviewer reproduction executed.
4. Zero-ratchet correctly finds7tokens across4current files under library: setup retirement schema :17-18 bun.lock/bunfig.toml; sealed/🔣️.json :16 bun.lock; precision/🔣️.json :1781,1820,1864 bunfig.toml; private-repository/🟦️.ts :18 comment bunfig. Root owns exact schema-neutral cleanup. Keep ratchet files={} and production found={} unchanged; adding allowances or excluding these files would hide the defect.

Owner coordination sent exact fixture/protocol/help diagnostics to dependency lane and exact4ratchet paths to root. No code changes, semantic execution, deadline/cap changes, installation or setup performed by reviewer.


## Actual Help Timing Diagnosis

Independently read93071 generated/portable-help-timing.log:26actual command observations, every status0, total7795.958ms. Per-manager totals: Bun30.33ms, npm4248.90ms, Cargo226.65ms, Go344.16ms, uv141.72ms, dotnet2804.20ms. Slowest individual call is npm install --help1729.63ms; no individual observed call approaches5000ms. Diagnostic covers original version probes, all declared lock/sync/verify help and repeated npm config calls; it does not perform the original flag assertions itself.

This grounds the aggregate-timeout explanation for this actual observation: serial six-manager work exceeds the one5000ms test boundary even while all calls succeed. It does not prove future individual tools cannot hang or that every loaded host will finish one adapter within5seconds.

Sound correction is parameterizing the existing real-tools law per manager at unchanged default5000ms per test, preserving all26version/help/config invocations, unavailable-tool behavior, rendered leading-command checks and every real flag assertion. This changes aggregation from one six-adapter case to six independent adapter cases; do not call it an unchanged total5second budget. It raises no numeric timeout/cap and introduces no fake tool adapter, exception waiver or reduced checks. Full normal manager rerun is still required. Timings and diagnosis were read only; reviewer executed no command/probe.


## Retirement Neutralization Source Review

Read current defining retirement schema/source/tests/fixture after root publication. Literal manager lock/config names become generic lockFile/scopeConfig placeholders. Production selectedRetirementSchema is materialized from the actual selected javascriptInstallSchema ManagerProtocol manager descriptor using existing materializeSchemaV1/schemaViewV1. Both retirement wire definitions and runtimeProtocol derive from that resolved schema; selector actually checks its resolved unreadyPayloadRoles. No fallback accepts broader metadata names and no acquisition/release authority changes.

New Bun/npm fixture rows compare actual production materializer output against independent SQLite extraction of descriptor primary lock/scope configuration, then invoke current selector with each resolved protocol for allowed payload roles and unknown-role refusal. Existing plan/settlement extraction ports now use resolved schema. No actionable source defect found; genuine70209 driver test outcome remains pending. Reviewer made no source edits or semantic execution.


Actual retirement70209 owner terminal EXIT0: independently read generated/retirement-neutral-green.log confirming5tests passed/0failed/133expectations, including current descriptor-role SQLite law. This closes earlier pending driver-test result at its actual scoped5law coverage; no setup/reclamation/runtime authority effect is inferred.


## Sealed And Precision Neutral Source Review

Read Rust final source/schema/test/graph consumer packet; no semantic execution. Sealed policy replaces literal primary lock with typed javascript/locks role, retaining package.json. sealedStampedFilesV1 resolves every selected lock/config role through existing canonical CJS accessor. Capture hashes present stamped files and records absent selected lock paths; drift checks selected semantic names are represented among captured files/absences, then detects file edits and absence appearance. Manager swaps cannot silently reuse a tree lacking their selected names. Original lock sealing is preserved and expanded to all selected locks; no literal input is simply discarded.

All3original precision scope-configuration positions now express javascript/scope reads. Engine requires provided managerFiles, rejects empty roles and malformed/escaping relative names, and emits exact workspace-prefixed selected paths. Graph project-assembly supplies actual CJS selectedManager lock/config roles and reads every accessor sourceFiles member under memoized observed rule computation, retaining policy/catalog/descriptor source accounting. Sealed startup adds only existing first-party CJS accessor using Node builtins/JSON, with no new external package dependency. Existing sealed snapshot fixtures preserve that actual CJS/JSON closure.

New2manager neutral fixture independently compares role names against SQLite and actual selected descriptors, checks lock edit, absent lock appearance, selected manager change, exact scope input patterns, missing host provider and escaping host refusal. Production handlers, not a literal-token allowance, implement neutral data. No actionable coverage-dropping/obfuscation/ratchet allowance found. Focused normal64114 and final full manager94case outcome remain pending; source inspection alone does not establish those passes.


Final manager36311 actual log independently read:94passed0failed845expectations, including per-manager realtool checks, portable recipe law and zero-ratchet. Python34021 final log reports vectors equal oracle; terminal EXIT0 is roots tool evidence. Earlier41178 remains a failed historical run, not overwritten acceptance. Rust focused64114 separately reports31pass1skip1fail10184expectations; copied-source snapshot case is still failing and under root-directed handcrafted refactor. Manager pass does not substitute for that fixture/strict acceptance or real setup.


## Focused Failure Classification Correction

Root reports64114 captured-loader failure is a production CJS named-export incompatibility exposed by the snapshot law; it must not be dismissed as fixture-only. Earlier failed copy-based fixture also violates intended source-copy/temp constraints and is being replaced, but a replacement must still expose the actual defining ESM dependency shape through real snapshot runner and retain negative control. Planned sealed/graph ESM registry change also alters manager catalog closure, so previous94/845 pass becomes historical prechange coverage until rerun. Final source audit pending; no production fix acceptance inferred.


## Final ESM Registry And Handcrafted Snapshot Review

Fresh source now imports the existing TypeScript ESM registry from sealed and graph MJS, removing incompatible CJS named imports and the temporary CJS declaration facet. Graph MJS already imports multiple first-party .ts modules on its existing runtime path; this change follows that established Bun/Node-owner loader route and introduces no standalone raw-Node compatibility promise. Registry/core/adapters/formats import only relative first-party modules, JSON and Node builtins; selecting manager does not start spawnProbe. No startup workspace package dependency added.

New packageManagerSourceFilesV1 explicitly lists ESM catalog, neutral core, catalog/policy/schema, formats and every registered adapter module+descriptor. Inspected adapter imports match that set. Graph observes these actual source paths before memoized precision rules; selected role behavior and descriptor-source accounting remain intact.

Refactored snapshot law authors only minimal entry/helper fixture text, imports original sealed core/registry at actual checkout paths, captures checksum-bound snapshot in memory and streams it to actual loader. Assertions retain captured codeA while live helper changesB; actual sealedTreeDrift sees B while naive captured read stays blind. Loader now defines import.meta.url as original file URL consistently with original path/dir fields. No checked-in source text is copied into the ticket in current testcase.

Concrete remaining temp defect sent to root/Rust: testcase invokes fixtureCaptureV1(checkout,...), but helper still computes memo file join(root,.memo.json). Therefore memo targets checkout/.memo.json despite configured fixture root under ticket generated. Root .memo.json currently exists, but ownership/preexistence is not inferred and reviewer did not remove it. Supply separate ticket-generated memo path while retaining actual checkout capture root. Existing generated neutral-manager directory was empty at observation because fixtures clean themselves, so no fabricated actual maxpath measurement is claimed. Full path bound check remains required for actual generated fixture paths.

Observed new focused66533 log:32pass1existing opt-in skip0fail10188assertions; this does not waive the concrete temp-location issue above. Subsequent current manager/strict receipts are reviewed separately.


Current post-ESM verification receipt independently read:67262 actual log94pass0fail845assertions;64720 strict12root owner EXIT0 with actual normal admission/no diagnostics;34021 oracle unchangedcurrent inputs as owner attests. All3retained log SHA values independently match the authored receipt. Generated dependency-layout-packet.json SHA a2b0911301ed73266ce25d3840a146e8cd89ab68fa3089cde9bf15dc7b24cb9b matches receipt53files/max194UTF16, authority=false, actualRematerialized=false. No actual setup or installed-generation claim. Final memo-location correction remains separate pending source review.


Memo leak source correction independently reread: fixtureCaptureV1 now accepts explicit fourth memoPath and real-checkout test supplies join(w.root,.memo.json). Actual Source root stays checkout, fixture/memo files stay configured ticket-generated root, and no checked-in Source copy returns. Production ESM code unchanged. Normal32276 same3root test rerun pending; earlier66533 pass precedes this test-only location correction. Root memo ownership/cleanup is separately root-owned and reviewer removed nothing.


Final memo-corrected focused32276 actual log independently read:32pass1existing opt-in repository-scan skip0fail10188expectations across33cases/3files. Handcrafted actualsnapshot+originalESMclosure/live-drift negativecontrol passes with ticket-generated explicit memo path. This closes pending test-location rerun; no Sourcecopy/tempoutsideTicket remains in that inspected case. Root.memo preservation remains separate root operation, not reviewer cleanup. Current strict12 retry after test-only memo edit remains owner-managed; no setup/rematerialization claim.
