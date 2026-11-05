# Layout Core Execution — 2026-11-04

Read the build skill and layout instructions. The canonical mutation schema and Rust enum exclude parent-owned drawing text changes, and the TypeScript semantic suite explicitly refuses SetDrawingText. The Rust KINDS catalog still advertises set-drawing-text, creating a mismatch in the existing kinds_match_the_enum_and_the_catalog test.

Validation pending: targeted owned Cargo test has entered the repository capacity queue. No source edits yet.

## Authority And Test-First Evidence

The committed canonical mutation JSON Schema has 58 branches. The Rust KINDS array has 59 entries, including set-drawing-text. Existing TypeScript semantic coverage explicitly rejects SetDrawingText because the actual Drawing child owns that event. Added kinds_match_the_canonical_schema before any production catalog edit; it independently reads the portable primary schema with third-party serde_json. Added a language-neutral schemaRefusals vector for SetDrawingText, checked against Ajv and both native/canonical TypeScript parsers.

## Export Law Replacement Mapping

- Placed drawing frame: obsolete inline VectorScene test replaced by existing same-name actual Drawing issuer test, retaining expected PDF geometry.
- Drawing text: obsolete inline test replaced by same-name actual-source Plan extraction test; strengthened actual Canvas composition to capture glyph cluster text and assert Plan.
- Edited drawing text: obsolete parent SetDrawingText proxy law replaced by actual Draw child mutation and PDF Title extraction; strengthened actual Canvas glyph cluster text to assert Title.
- Embedded PNG and rotated embedded PNG: proxy success is intentionally excluded by the current original-source contract. Existing pdf_export_refuses_original_drawing_samples_without_a_resolution_owner checks both rotated false and true against portable sample fixtures and explicit refusal. No unresolved sample is silently omitted.
- All ten actual-source export tests remain, including styled/fill geometry, group opacity refusal, missing original-read refusal and front rectangle ordering.

Removed the stale missing parent set-drawing-text wire witness include. The underlying fixture had already been removed.

## Changed Files

- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪪️native-json/🧪️tests/🟦️.ts
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/📨️mutations/🔣️.json
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/📤️export/🧪️tests/🔬️unit/✒️pdf-drawing-laws/🦀️.rs

## Native Gate

Command: repo cargo --store=shared --configuration=prototype -- test --manifest-path ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/Cargo.toml -p semio-s-artifact-layout-layout-mutations kinds_match_the_enum_and_the_catalog --lib

Session 91879 remains queued at repository machine capacity after more than 300 seconds, without a compiler or test result. Red/green native execution is not yet claimed. Follow with filter kinds_match after the catalog repair, then full repo run @semio-tech/layout-layout-rs:test. Duplicate normal node proof was cancelled through its owned abort control; no other sessions were stopped.

## Native Gate Result

The targeted gate completed with exit 1 before compiler admission: semio-s-artifact-layout-layout-mutations Source extent exceeded at existing 🧪️tests/🧫️fixtures/🧬️structure/🔣️.json, observedBytes 218479 against maximumBytes 111285, remainingFiles 5. This is an infrastructure refusal, not a catalog test red. No Rust result is claimed. The test-first source test and direct 59-vs58 primary schema observation justify removing the single obsolete set-drawing-text entry in Rust KINDS and the oracle wire catalog. Both source catalog files are additional changed files.

## Source Boundary Ownership Finding

The refused portable fixture is correctly compiler Source: mutations/🧪️tests/🏗️structure/🦀️.rs line 7 includes it with include_str!. The admission owner computes per-unit cumulative Source bytes and passes only remaining allowance to readPhysicalCargoSourceV1. Thus 111285 is remaining crate allowance, not a per-file limit, and the 218479-byte fixture crosses the aggregate boundary. It must not be relabeled freshness while embedded. The clean remedy is a dedicated public mutations-tests crate, analogous to existing model-tests and validation-tests, moving laws from the production aggregate compilation unit without changing fixture semantics or increasing policy caps. A runtime fixture loader would require explicit declared data freshness observation and is a different supported approach only if the owning native target contract is updated consistently. No build infrastructure was edited.

The duplicate Canvas tuple call was corrected and all five helper definition/call occurrences reviewed. A requested rg query through repo tools was refused because rg is not registered; filesystem reads through registered Node provided the equivalent source inspection.

## Same-Package Public Integration Target

Following source-ownership review, moved only the public structure law from the aggregate unit to Cargo [[test]] name structure in the same owning mutations package. The physical test file and embedded portable fixture remain in place. Replaced inherited private imports with explicit public LayoutMutation, LayoutSnapshot, LayoutDiff and protocol imports; removed the lib module mount. No new package or copied source, cap increase, runtime fixture loader or test-only public hook. Additional changed files: mutations/📦️packages/🦀️rust/Cargo.toml and mutations/🧪️tests/🏗️structure/🦀️.rs. Full owner must run --all-targets to retain this acceptance law. Root owns next full native gate.

Further source observation: the remaining public graph test embeds a 1076881-byte graph corpus, plus 142013-byte word and 317992-byte semantic corpora. Future gate may expose this separate oversized compiler Source unit; no change made speculatively.

## Full Declared Owner And Orphan Review

Started normal repo run @semio-tech/layout-layout-rs:test, session 49856; currently queued on host capacity. The package router previously omitted the mutations package from testPackages, leaving catalog and newly public structure law unexecuted by the owner. Added existing semio-s-artifact-layout-layout-mutations package to that list.

Removed the unmounted 🏷️set-drawing-text/🦀️.rs implementation and its two unit laws after layout-wide references confirmed no production mount. Removed stale artifact-root import semio_s_artifact_layout_layout_model::set_drawing_text. All actual Draw child mutation/Canvas/PDF editing acceptance remains mounted. No old behavior adapter added.

Supported runtime data observation pattern found in mathematical-equation Rust tests: std::fs::read_to_string with Path::new(env!(CARGO_MANIFEST_DIR)).join(relative). Any eventual use here should return owned serde_json data instead of leaking strings, keep exact fixture files, and declare native test fixture inputs plus sourceInputObservation freshness metadata. No runtime data-loader edits yet.

## Portable Corpus Data Separation

Converted eleven graph/word/semantic fixture include_str parses in the existing graph law file to one private system std::fs loader returning owned serde_json::Value. Exact existing physical fixture files and every assertion remain. All manifest-relative paths were resolved and observed to exist. Declared test cache=true, explicit default/^default/nativeSources inputs plus framework IEEE fixture, and target metadata.semio.sourceInputObservation=freshness. These corpus bytes now remain observed test data rather than embedded Rust compiler Source; Rust code continues through ordinary compiler admission. No cap or fixture data edits, copies, leaks, hooks or dependencies introduced. Original full owner session49856 remains alive for source recapture.

## Compiler Observation And Parent Refusal Roster

The owning test target explicitly declares execution {version:1,kind:native,scope:test} and runtimeSourceEntries [{projectRoot}/📜️script.ts]. The graph owner buildNativeSourceObservationV1 returns compile for selectedNative independently of data freshness, so broad nativeSources inputs remain observed data and cannot suppress actual Rust root discovery/compiler admission. The declared script runtime root remains compiler-owned.

The existing parent graph law referenced deleted child.content and set_drawing_canvas_size. Current LayoutDrawingMember contains only handle. Replaced its two invalid-content/reference branches with wrong artifactKind and wrong subset; retained unchanged-command inverse and both refusal inverse assertions. This follows current parent reference authority. Actual Draw export-document unit test still checks viewport width=0 refusal on the Drawing producer; it is a child/export viewport check, not a claim that parent mutations validate child document geometry. Existing actual Draw mutation/Canvas/PDF Title editing law remains.

## Live Native Gate Handoff

Exact command: repo run @semio-tech/layout-layout-rs:test

Live unified exec session: 49856. Invocation launched once; never cancelled or restarted. Last sampled state: still repository capacity queued at 429 seconds. No compiler result from this full gate yet. There is no separate disk log: original stdout remains in the live unified exec session, and root can resume write_stdin session_id 49856. Keep future generated logs under ticket 🗑️generated and remove them at completion per repository policy. Previous targeted Source refusal is documented above.

Unresolved static risks: other embedded validation fixtures may still cumulatively cross the per-crate Source boundary; native compiler and integration test results remain unverified pending this full gate. No assertion or fixture was removed from the retained graph/structure/canonical suites; only obsolete parent-owned drawing proxy tests/implementation were removed with replacement mapping above.

Additional current changed paths since the earlier roster: artifact root 🦀️.rs; artifact Rust package 📜️script.ts and 📋️project.json; mutations graph law; deleted orphan parent mutation implementation and two-law source file.

## Complete Mutation Test Data Inventory

A full remaining include_str inventory found16 actual wire witnesses, validation and canonical schema/oracle data, plus retained corpora up to3554190 bytes. Converted40 remaining includes across13 physical test files through one private cfg(test) root test_fixture_text function, using exactly resolved manifest-relative paths. Graph11 portable corpus reads reuse this system filesystem interface. All original wire-witness entries and schema equality assertions remain. No public hook, fixture copy, cap change or test removal. Only structure integration retains its one218479-byte embedded portable fixture in its separate bounded compiler target.

Updated physical test files using the shared private fixture interface:

- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/✅️validation/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️kinds-catalog/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🕸️graph/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🎚️simple/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🏭️prepare/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/📄️document/⚖️equality/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/📎️overrides/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/📓️spread/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🔎️target/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🖼️frame/👥️batch/⚠️message/🔣debug/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🖼️frame/👥️batch/⚠️message/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🖼️frame/👥️batch/🗺️plan/🧪️tests/🦀️.rs
- ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧳️retained/🗄️parent/🌱️creation/🧪️tests/🦀️.rs

## Session Scope Correction

Unified exec sessions are agent-scoped; root cannot resume49856 directly. This lane resumed the original process (never duplicated) and now appends available output chunks to ticket 🗑️generated/fleet-native-2026-11-04.log. Initial chunks can be tool-truncated; subsequent chunks are captured with larger budgets. Current original run admitted at~440 seconds and is graph facts/closure verification, no compiler/test result yet. Generated log must be removed when the ticket finishes, retaining this audit.

## First Full Native Gate Result And Owned Recovery

Original49856 terminated exit1. The exact20-package selection included mutations, editor transform transactions, engine/export/editor/model and companions; none reached compiler or assertions. The three declared prerequisite generators passed. Native workspace synchronize preparation failed because its nested owned capture finalized an invalid UTF-8 stream (ERR_ENCODING_INVALID_ENCODED_DATA at owned-execution/📥️capture/🟦️.ts:93). Upstream prep reports only its last2048 stderr characters, so exact nested producer is not yet identified. The decoder implementation uses correct streaming fatal UTF-8 decode plus final flush; no concrete decoder bug was verified and no process infrastructure was changed. Existing language-neutral capture vectors already cover split multibyte UTF-8, invalid UTF-8 refusal, and independent CrossSpawn output.

Per build-skill original-owner recovery, started exactly one normal full-owner retry after terminal failure (session47617), with full stdout/stderr redirected from birth to 🗑️generated/fleet-native-retry-2026-11-04.log. No caps, receipts, caches or peer controls changed. Original sampled log also retained until ticket completion. No native test pass is claimed.

## Supported Backend Switch

The once-retried Nx command47617 remained in graph namespace/tool startup, then completed only its three prerequisite generators; it never dispatched native owner preparation/compiler/assertions. At root coordination request, cancelled only this owned invocation through its original controller. It terminated130 and explicitly reported Cancelled (SIGINT); leases released. This is cancellation of unexecuted native acceptance, not another UTF-8 failure.

Started identical full20-package declared target once through supported repo --tool=bazel run @semio-tech/layout-layout-rs:test with existing SEMIO_CARGO_PREPARATION_TIMING=1 diagnostic environment. Session70033, full stdout/stderr redirected from birth to 🗑️generated/fleet-native-bazel-2026-11-04.log. No source/caps/policy/receipt/cache/peer changes. This preserves full suite and structure integration selection while avoiding the Nx graph namespace bottleneck.

## Supported Bazel Native Result

Session70033 terminated exit1. It selected the identical20-package suite and dispatched actual native owner preparation after three prerequisite generators. Preparation again failed before compiler/assertions with ERR_ENCODING_INVALID_ENCODED_DATA while strict separated capture finalized a child stream. This reproduces the failure across supported Nx and Bazel backends, not a decoder bug diagnosis. No mutations unit, structure integration, export drawing or mounted editor transaction assertion ran.

Full own Bazel action log preserved in 🗑️generated/fleet-native-bazel-action-2026-11-04.log, copied read-only from own t_75c0ef4f026d50dd action test.log. Outer complete redirect remains fleet-native-bazel-2026-11-04.log. Existing preparation timing telemetry is hidden by upstream separated capture and its2048-character failure tail; exact offending nested producer/stream is not yet identifiable from current stderr. No further identical retry launched and no capture validation weakened. Next diagnosis must preserve strict UTF-8 and original child ownership while exposing precise bounded producer/stream failure context.

## Narrow UTF-8 Causal Investigation

The fatal decoder is streaming each channel with decode(chunk,{stream:true}) then flushing once on end. Repeated failure occurs at final flush, consistent with an incomplete ending sequence; this does not prove whether producer truncation or runtime decoding is causal. Native prep nested strict capture sites are custody recipe script execution and Cargo lock tool execution; neither should be treated as binary by default. Existing split-UTF8 portable capture tests can serve as regression reference; no current pass is claimed.

The timing flag alone cannot reveal precise recipe identity because prepareCargoWorkspaceInvocation captures the synchronize child stderr and then truncates to2048 chars on failure, dropping its earlier telemetry. A temporary timing-gated forwarding of already-bounded stderr (with DEBUG prefix) was proposed for root coordination; not implemented. No validation policy was weakened and no speculative process rewrite was made.

## Targeted Preparation Diagnostic

At root coordination request, added temporary timing-gated preparation failure forwarding via existing writeOwnedCommandOutputV1 under the same Original control. It emits DEBUG-prefixed already-bounded failed child stderr before the unchanged2048-character status wrapper. Strict decoder,16MB/65536-line capture caps, argv/environment privacy, cancellation and actual failure status remain unchanged. This temporary edit in repo workspaces/🦀️cargo/🟦️.ts must be removed once causal diagnosis is complete.

Started one normal declared diagnostic nativeBazel invocation with timing, session4872, full redirect 🗑️generated/fleet-native-diagnostic-2026-11-04.log. Existing framework-process:test-owned-execution baseline session11269 separately exercises current portable splitUTF8/invalidUTF8 and CrossSpawn cases with no test/source expansion; log fleet-capture-baseline-2026-11-04.log. No speculative delayed-stream or decoder changes were made.

## Diagnostic Dispatch Blocked By Shared Build Source

Capture baseline11269 terminated1 before target dispatch: bootstrap authorizer ancestry read failed ENOENT at repo/library/📖️storybook. Native diagnostic4872 terminated1 before target dispatch: authorizer Source closure capacity exceeded files1788/2048, bytes16780262/16777216. Full redirected logs preserve both exact errors. These add no capture or native test result and are not another UTF-8 reproduction.

Timing-gated stderr forwarding remains temporary and must be removed after diagnosis; no cap or unrelated Source change was made in response. Further normal diagnostic execution requires correcting actual shared build Source ownership/closure rather than increasing limits or bypassing bootstrap.

## Matched-Lock Wasm Toolchain Repair

Host99766 actual failure observed ambient wasm-bindgen0.2.126 although admitted Cargo-home metadata already recorded wasm-bindgen-cli0.2.129 and Cargo.lock selected0.2.129. Provisioning previously probed bare PATH then accepted cargo install no-op without verifying the installed command; WGPU independently probed bare PATH and prepended only Binaryen.

Authored neutral bindgen fixture/schema and real owned routing regression first. Preimplementation85421 stopped in shared Source admission (1822files16783971bytes), so no behavior-red result is claimed. Root authorized focused implementation after this test-first source checkpoint while shared Source was corrected.

The existing locked-bindgen resolver now observes external tooling through an owned value-only port, prefers admitted CARGO_HOME/bin and the declared owned Cargo installation over ambient PATH, and strictly checks locked identity. Provisioning probes and verifies that exact install destination, uses targeted --force only after observed owned mismatch/absence, and refuses stale postinstallation output before claiming readiness. Explicit conflicting generator selection refuses without acquisition. No global version was changed.

WGPU now invokes the resolved absolute matching generator and uses the existing wasmPackEnvironment interface for Trunk. That interface preserves one canonical PATH key across Windows/native hosts and explicitly carries the selected tool identity. Current source/cache/fingerprint fixtures were hand-updated to the single registry override SEMIO_TOOL_WASM_BINDGEN; the old parallel spelling is not retained as an alias. Generic funnel cache behavior was not changed because explicit selected commands already bypass its bare-name cache correctly.

Regression target repo-lib:test-native-dependencies now declares workspace:deps-trunk so zero-touch real generator availability is explicit. Tests cover three platform path projections, exact identity/mismatch/refusal with Ajv+semver witnesses, stale bare-cache priming followed by actual absolute owned process execution and CrossSpawn comparison, and REAL root-locked generator --version through the owned command interface after stale priming. Native mock cases also cover install metadata succeeding while the owned binary stays stale (must refuse). Existing Trunk fixtures now follow the actual implementation source and Cargo-home contract.

Normal supportedBazel GREEN10728 remains capacity queued; no test success is claimed yet. Host lane is holding canonical activation until this actual proof completes. Temporary timing-only preparation stderr forwarding remains separately tracked and must be removed at completion.

Matched-bindgen changed-file roster:

- Repo library ⚡️caching/🦀️cargo/🕸️wasm-bindgen/🟦️.ts
- Repo library 🏃️process/🕸️wasm/🔣️policy/🟦️.ts and its public cargo/process/wasm/policy facade
- Repo library ⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/🟦️.ts
- WGPU renderer 🏗️compiler/🌐️wasm/📜️script.ts
- Repo library caching policy and Nx plugin environment policy JSON
- Wasm tool fingerprint source/test and Bazel action environment fixture
- Native dependency test source/fixture and Trunk lockfile test source/fixture
- New caching test 🕸️wasm/bindgen.ts and neutral bindgen.json/bindgen.schema.json
- Repo library package project test-native-dependencies prerequisite metadata

Neutral readiness cases include correct owned skip, verified install, stale owned output after successful installation, and explicit conflicting tool refusal without installation. Real version proof is still pending queued GREEN10728; no activation readiness has been declared.

### Actual Registered Bindgen Selection Witness

Normal registered CLI probes both exited zero: `repo tools exec wasm-bindgen -- --version` produced `wasm-bindgen 0.2.126`; the same command with canonical `SEMIO_TOOL_WASM_BINDGEN` selecting the existing owned Cargo-home binary produced `wasm-bindgen 0.2.129`. This verifies the ownership mismatch and canonical absolute selection through the real registered interface without changing PATH or cache. It is a diagnostic witness, not a passing regression gate. Gate 10728 remains the original normal Bazel test invocation; the latest sample is verifying input closures after normal recapture.

### Bindgen Actual Gate 10728

Original normal Bazel gate10728 terminated1 after actual test dispatch (4.8s test action), not Source refusal. Ajv portable schema/path/identity/selection checks completed; stale process output assertion then failed because the new test asked stdout ignore, which intentionally disables onLine observation in the owned command contract. Removed that incorrect capture option from the three process assertions so default piped stdout is observed. No process/runtime selection implementation changed in response. Full original output remains generated/fleet-bindgen-green-2026-11-04.log. A same-target rerun follows this test-harness correction.

Corrected same-target normal command: `repo --tool=bazel run @semio-tech/repo-lib:test-native-dependencies`, session63576; full stdout/stderr redirected from birth to `🗑️generated/fleet-bindgen-green2-2026-11-04.log`. Initial state capacity queued behind layout-js:test. Production changes and all assertion rosters are unchanged by the capture correction.

### Bindgen Actual Regression Proof And Remaining Owner Failure

Corrected normal gate63576 terminated1 after actual test execution. All new portable bindgen schema/path/identity/selection assertions, stale cached bare process0.2.126, absolute selected synthetic0.2.129 and independent CrossSpawn comparison, REAL root-locked owned executable0.2.129 through runOwnedCommand after stale priming, and independent actual CrossSpawn comparison passed. The existing native dependency matrix and all four new readiness laws (skip, repaired install, stale postinstall refusal, explicit mismatch refusal) also completed before later Cargo fixture code. These are actual executed assertions, not inferred prerequisite success.

The whole target remains RED: existing rawCargo fixture lines122-128 calls Cargo fetch with a private test CARGO_TARGET_DIR/build environment while inheriting default refusing compiler ingress. Cargo probes rustc -vV and correctly refuses Original compiler ingress requires repository launcher and authentic enforcing store. No compiler enforcement was removed and no success is claimed for the complete native dependency target. A minimal repair must use existing authentic compiler owner/store interface rather than clearing RUSTC or constructing raw tool paths. Full log: generated/fleet-bindgen-green2-2026-11-04.log.

Root retained the native causal evidence: valid resolution JSON exceeds the unchanged65536-byte text-input contract at custody resolve-program, not a decoder defect. Host owns ordered bounded batching. Removed only temporary timing-gated full-stderr forwarding and its import from workspaces/cargo after root checkpoint. Existing timing API, strict decoder, original failure status and all finite limits remain.

### Native Fixture Compiler Owner Repair

Actual63576 ingress refusal led to a test-only ownership repair in caching/tests/native-dependencies: the real Cargo transaction portion now runs inside existing withRepositoryCompilerLaneV1, refines its issued child control for phase operations, and uses that authentic store environment and owned lane target/build directories. Removed test-private CARGO_TARGET_DIR/build overrides; did not strip or stub RUSTC enforcement. Earlier neutral mocks and strict negative custody children are unchanged. Existing dependency/no-fixture-deliverable laws remain. Same normal full declared target rerun session58845; full output redirected from birth to generated/fleet-bindgen-green3-2026-11-04.log. This run is pending, not a full GREEN claim.

No-compilation law preservation under owned compiler relocation: retained both original fixture target absence checks, and added before/after comparison of paths,size,mtime of every file in the exclusively held compiler lane target. Only neutral-fixture-declared Cargo metadata .rustc_info.json and CACHEDIR.TAG is excluded. Thus synchronization cannot quietly compile into the new owned output location. Same58845 invocation remains live; no duplicate/cancel.

### Proper Compiler Ingress Proven; Suite Level Corrected

Normal58845 actual test opened authentic compiler owner and progressed through real Cargo fetch/update/metadata and owner preparation beyond the prior ingress refusal. It then exited1 at the existing outer default fundamental15-second test deadline; no assertion failure or whole-suite success is claimed. Full log generated/fleet-bindgen-green3-2026-11-04.log retains actual work and budget refusal.

The TypeScript package native-dependencies owner route now declares minimum quick through existing resolveTestLevel([],quick), following the neighboring nx-owner-arguments owner convention. Quick is the smallest fitting existing level (300000ms): this suite already bounds the integration body at90000ms and its following native law child at120000ms. No budget constants, custom timeout, fixed inner90-second cancellation, Original authority or output limits changed; child budget remains clamped by scriptInvocationBudget. All portable selection/readiness/no-compile/custody laws remain selected. Added route source to changed-file roster: library/tests/typescript-package-commands/build-and-direction-routes/TypeScript.

One same-target normal retry21643 now live, without command environment override or extra level flag, proving the declaration itself. Full birth redirect generated/fleet-bindgen-green4-2026-11-04.log. No production bindgen changes.

### Quick-Level Actual Result And Strict Negative Diagnosis

Normal21643 terminated1 after authentic Cargo synchronization, no-compile target snapshots, and multiple actual strict negative custody receipts. The supported quick floor worked; no timeout occurred. After the input-mutation receipt, a later negative child had no completion receipt and the helper discarded its already-bounded stderr when reporting that protocol failure. Added permanent bounded controlled-fixture context (purpose,id,status,signal,1024-character stdout and8192-character stderr tails) to that unchanged refusal; no receipt matching/status/deadline/aggregation validation weakened. Full result is generated/fleet-bindgen-green4-2026-11-04.log. Additional changed test-support file: library/caching/tests/native-dependencies/purpose/TypeScript. Next same-target normal run diagnoses precise producer failure; no whole-suite success claim.

### Strict Member Case Causal Evidence (58002)

Normal58002 terminated1. Bounded diagnostic names mutation/member, status1/no signal, stdout empty. The child first entered cargo-preparation-replan:1, then refused Preparation custody changed at its generated Cargo.toml. The fixture expects Prepared member roster changed with exactly one tool call. Current controlled replan semantics and producer/receipt expectation need review; no regex/call-count/receipt validation was relaxed and no further fixture run was launched while critical WGPU admission took priority. Full stderr remains generated/fleet-bindgen-green5-2026-11-04.log.

### WGPU Missing Issued Profile Input

Activation8496 actually fails Cargo unit-graph profile prototype undefined. WGPU compileTrunkRenderer calls bare admitCargoSelectionV1 at line72 before its Cargo preparation hook; unlike generic wasm-build and native component-pass, it has no repositoryCargoCompilerInputsV1 port around the selection. Root manifests intentionally omit profile definitions. The existing genuine owner factory captures Original workspace/mode, reads Source profile policy, issues exact Cargo selectors and retained env/control/writer custody; caller CARGO_PROFILE_* values are explicitly refused.

Primary Trunk0.21.14 source also proves a second propagation boundary: its rust pipeline hardcodes nested Cargo build flags and separate JSON artifact query without generic --config forwarding; its Build schema has no cargo_args field. Sources: [Trunk rust pipeline](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/pipelines/rust/mod.rs), [Trunk Build config](https://raw.githubusercontent.com/trunk-rs/trunk/v0.21.14/src/config/models/build.rs). Issued profile selectors therefore must reach actual nested compile as well as admission; merely wrapping admission is incomplete. Normal repo tools exec trunk -- --help refuses unknown registered tool; no raw Trunk command was used. Investigating the proper owned compiler-input projection rather than fake profile env, caller shadow config, manifest profiles or bypassing Original/store authority.
