# P8yz-g Flow Retained VCS Source/Static Implementation

Date: 2026-08-26  
Status: production reachability cutover source/static-ready for a fresh independent audit; focused Rust execution/typecheck remains deferred

## Scope

This packet initially owned the framework Flow `🌿️vcs` retained component, its three language-neutral fixtures, and this report. The fourth audit authorized the exact production cutover in Flow's Wasm component/protocol/schema, JavaScript bridge/browser package, emitted package manifest/host, and Flow-local package build router. It still does not edit Puzzle, Store guard, the root script, `.vscode/launch.json`, central typed-command code, dependencies, Cargo manifests/lock, renderer, or peer applications.

## Fourth Independent Audit Production Cutover

The fourth independent audit `📓️terra-p8yz-g-flow-retained-vcs-fourth-independent-adversarial-audit-2026-08-26.md` found one blocking defect: `FlowRetainedVcs` was reexported but had zero production callers. The real browser package drove `FlowRetainedFeature`/`FlowProgramState`, so no production request could reach the retained VCS authority, page lease, ACK, control, or close code.

That split is removed for the adopted VCS operations:

- session admission now calls `FlowDomain::bind_session`, and `FlowDomainAdapter` constructs one `FlowRetainedVcs` with the actual `AbiHandle` generation;
- operation 2501 is schema-named `vcsCheckpoint`, 2502 is `vcsFault`, and 2503 is `vcsRetryCheckpoint`; every one accepts exactly `sessionGeneration:u32`, `baseRevision:u64`, and `parentRevision:u64` after the outer session handle;
- the old `loadFixtureJson`, `resyncFixtureJson`, and `fixtureJson` schema/JavaScript entries, `FlowAction2501/2502/2503` structs, and their `flow_action` dispatcher arms are deleted rather than retained behind aliases;
- `FlowVcsFeature` performs fixed-width parsing and exact session/request-generation/authority preflight without mutating retained VCS state; only the first valid feature grant, after outer request/resource/event admission, calls `begin_checkpoint`;
- each Wasm grant performs one retained semantic unit: `poll`, `fault`, `take_page`, `resume_page`, `retry_page`, VCS ACK, `close_operation_step`, or `close_retired_step`;
- progress, checkpoint, preview, page-ready, terminal, fault, and cancellation remain distinct states; and
- the fixed page encodes outer session slot/generation, request generation, exact VCS authority, retained operation id/slot/generation, page sequence/operation/session, revision/parent/document generation, counts, and semantic digest.

The protocol now has a dedicated `RetainedPage` transfer. It emits the output ledger before the page, retains the exact `AbiPage`, rejects wrong/out-of-order/repeated ACK before feature mutation, calls `FlowVcsFeature::preflight_acknowledge`, then calls the real `FlowRetainedVcs::acknowledge_page`. Only after that ACK does it poll terminal and incrementally return operation, page, event, output, item, byte, control, history, and retired-owner credits. Cancellation invokes the real `cancel`; operation 2502 invokes the real `fault`; both incrementally close before the outer terminal reply.

Any retained `poll` fault after inner admission is captured as an owned terminal failure, driven through the real `FlowRetainedVcs::fault`, and exposed only after `close_operation_step` plus `close_retired_step` have returned the exact retained owners. The outer bridge can therefore never discard a live inner VCS slot merely because a concurrent publication made its authority stale.

If another exact VCS operation still owns a live slot, `close_retired_step`'s `ClosePending` response is retained as pending work rather than surfaced as a terminal failure. Both operations continue receiving bounded turns until all operation slots are handed back, after which each can observe terminal-empty without losing the shared retired-owner cursor.

The existing generic `AbiPageReader` remains only for non-VCS feature output. The removed whole-document operations cannot reach `FlowProgramFeature`; the retained VCS page does not pass through or pre-ACK the generic reader.

### Production package reachability

The real renderer `WasmSessionLoader` imports `@semio-tech/flow-core/🟨️flow-browser.js`. That package entry imports `createFlowFeatures`, installs every schema operation on `FlowSession.prototype`, and therefore installs all three VCS operations on the actual production session class. Both the source host and emitted `🫀️core/pkg/🟨️flow-host.js` contain the same authority codec and exact VCS page decoder. The emitted package manifest now publishes the browser and host subpaths, and the Flow-local build router rewrites those `files`/`exports` entries after every Wasm package copy so a rebuild cannot silently remove production reachability.

The mounted factory/registration/dispatch chain is explicit: the exported Wasm bridge owns `FlowBridge::new(FlowDomainAdapter::default)`; `FlowDomainAdapter::start_feature` registers operation codes 2501–2503 to `FlowVcsFeature`; the generic bridge dispatches accepted typed requests through `D::start_feature`; and the host drives the exported `flow_bridge_send`, `flow_bridge_poll`, `flow_bridge_begin_close`, and `flow_bridge_terminal_is_empty` functions. Cancel, close, and ACK are emitted as distinct controls by the same installed host.

### Language-neutral reachability law

`🌿️vcs-production.tsv` owns six production protocols: successful checkpoint, injected fault, explicit retry checkpoint, cancellation, stale authority, and wrong session. Each fixes operation code, session slot/generation, request generation, base/parent authority, the exact ordered retained units, page outcome, and terminal outcome.

The focused Rust law is source-ready to open the real `FlowBridge<FlowDomainAdapter>`, admit operation 2501, observe progress/checkpoint/preview/output, reject a wrong ACK, accept the exact ACK, drain terminal and incremental retirement, then run fresh operation-2502 fault and operation-2501 cancel requests before terminal-empty. A separate hostile source law verifies the production constructor/caller census, every lifecycle caller, removal of old dispatcher mappings/names, equality of source and emitted host/browser files, manifest export, persistent build-router installation, and all fixture units. Caller needles are assembled from split fragments so the law cannot satisfy itself with its own literal strings.

## Third Independent Audit Remediation

The third independent audit `📓️terra-p8yz-g-flow-retained-vcs-third-independent-adversarial-audit-2026-08-26.md` found one exact evidence defect: the five `afterRollback*` fault rows were duplicate calls after a primary cancellation. The language-neutral ledger and live source law now create a fresh retained session for each control at each of those five boundaries.

For `afterRollbackVisibility`, `afterRollbackSurface`, `afterRollbackHistory`, `afterRollbackRedo`, and `afterRollbackSemanticOwner`:

- the `cancel` row invokes `FlowRetainedVcs::cancel` as the first live control;
- the `fault` row independently invokes `FlowRetainedVcs::fault` as the first live control;
- the fixture fixes one, two, three, six, or seven incremental rollback grants respectively;
- the post-step cursor fixes phase, history/surface/visibility flags, exact redo-retirement count, edit ownership, and semantic-mutation ownership;
- each row fixes `Cancelled` versus `Faulted`, the exact session/base/parent authority, and a complete boundary-state reference;
- the boundary state resolves canonical document, explicit null page, exact history, and all sixteen handback fields; and
- the exact surface owner is compared in either the retired owner page or the restored document owner slot, including all fourteen surface/host/backing counters.

Only after those fresh primary-control and boundary assertions does each row repeat its own control to preserve the separate `duplicateControl` idempotence law. Incremental close then compares the complete terminal `redoThree` state.

The five intermediate states add exact 18-byte SetLayout credit, undo/redo/retired-action/retired-surface counts, revision, parent, generation, prior digest, version, edit, retention, and closing fingerprints. The uncommitted `publishedLayoutBoundary` document is fixture-owned and distinguishes the first four rollback positions from the final semantic restoration. All changed transfer, fingerprint, expected-state, and protocol-document signatures were regenerated and independently recomputed.

## Second Fresh Audit Remediation

The second fresh audit `📓️codex-p8yz-g-flow-retained-vcs-second-fresh-adversarial-audit-2026-08-26.md` confirmed the retained production source route and all-thirteen-feature oracle GREEN. It rejected only the hostile fixture evidence. All four requested items are now implemented in the fixtures and executable test source.

### 1. Fixture-owned byte-cap I/O

`📒️lifecycle.json` now owns three complete byte protocols:

- a literal `é界🌊️` source with four Unicode scalar values and exactly twelve UTF-8 bytes;
- `é` repeated 32,768 times with exactly 65,536 bytes; and
- `a` repeated 65,537 times with exactly 65,537 bytes.

Each record fixes the initial document/session/authority, feature, encoding, value/unit, repetition count, character count, byte length, expected handle or explicit null handle, result, source-retained result, exact admission state, cleanup control/grant, and exact final state.

The live fixture law constructs the specified string, compares both character and UTF-8 byte counts, calls the real typed `begin_remove_widget`, compares accepted/limit and retained ownership, resolves and compares the exact admission fingerprint, then performs the fixture-defined cleanup and compares the complete final state. Exact admitted fingerprints include operation/page/item/byte/output/event/control credits and every retained fingerprint field.

### 2. Resolved transfer protocols and exact results

All twenty-four transfer/rollback records now contain:

- exact initial document reference;
- session generation, revision, and parent revision;
- setup undo/redo owners and optional surface/host/generation;
- typed operation feature, authority, and full input;
- expected operation handle;
- named grant;
- persistent cursor target with every relevant phase/index/flag/candidate-count/rollback-step value; and
- both cancel and fault controls, each with a specified result and exact final-state reference.

There is no `exactPreOperation`, `byteExactPreOperation`, or descriptive `expectedHandback` label.

Every referenced expected state resolves to:

- a complete canonical document;
- an explicit null page;
- exact undo/redo history owners; and
- a complete handback fingerprint containing all seven credits, active operations, leased pages, undo/redo owners, retired action/surface owners, revision, parent revision, document generation, digest, document-version count, active version, edit owner, document retention, and closing.

The live law creates a fresh retained session for every boundary/control pair (48 total), applies all setup owners, begins the fixture operation with its fixture authority, verifies the fixture handle, advances the real persistent cursor until every specified target field matches, invokes the specified cancel/fault, compares its result, proves duplicate controls preserve the exact captured resource fingerprint, incrementally closes and retires owners, resolves the referenced complete expected state, and compares document/history/every fingerprint field.

Replacement-boundary input is a fixture-owned two-widget/two-synapse/two-layout multilingual document, so widget/synapse transfer and reversal targets are reachable and distinguishable by exact candidate counts.

### 3. Live authority, malformed, grant, and terminal parsing

Four authority protocols execute through live `FlowRetainedVcs`:

- wrong operation handle;
- stale generation handle;
- ABA generation;
- stale publication authority after the exact number of polls.

Each fixes the admitted and forged handles, authority, grant, result, exact at-result state, cleanup, and exact final state.

Three malformed/omitted protocols execute the real typed `begin_patch_widget` path:

- identity mismatch;
- omitted widget source;
- omitted ID source.

They compare the specified fault, null handle, both retained-source booleans, canonical document, history, and complete fingerprint.

All five grant records execute a real checkpoint poll using every fixture field: valid, zero fuel, expired deadline, over-window deadline, and interrupted. Each compares progress/fault, exact admitted fingerprint, cleanup, and exact final fingerprint.

The all-feature oracle no longer hard-codes terminal zeros or booleans. `flow_oracle_expected_handback` resolves the named `terminalFingerprints` fixture and parses every fixed credit/active/leased/retired/edit/retained/closing value; page/history/version values are parsed from each operation ledger. Hostile states likewise parse every complete fingerprint field from `📒️lifecycle.json`.

### 4. Field/value hostile mutations

`📒️lifecycle.json` contains stable canonical 64-bit signatures for:

- every byte vector;
- every authority vector;
- every malformed vector;
- every grant vector;
- every transfer vector;
- every complete fingerprint;
- every expected state; and
- every protocol document.

The executable hostile law recomputes each signature, recursively enumerates every scalar path, mutates each null/boolean/number/string value independently in memory, and requires every mutation to change the signed result. It therefore detects changed results, grants, handles, authority fields, source values, byte lengths, boundary phases/counts, control values, state references, history, digest, version, credit, and ownership values—not only deletion or array-count changes.

`🗂️owners.json` preserves all sixty-two owner categories and records the eight signed-vector groups plus actual document/page/history/handback extraction omissions.

## Preserved GREEN Evidence

The owned `FlowSemanticOracle` still independently evaluates the thirteen fixture operations in its own test-only `serde_json::Value` document/history/version model. The subject still drives each real retained request through page take, exact ACK, and incremental close before extracting canonical document/page/history/complete handback output. The former literal-label matrix remains absent.

The complete live `//#region 🌊️RetainedVcs` production route remains unchanged by this evidence remediation:

- fixed operation/page/item/byte/output/event/control/history/depth/deadline caps;
- typed preflight before transfer;
- schema-fixed scalar census/digest and unrolled slots;
- one semantic unit per grant;
- persistent action/replacement cursors;
- exact redo/history/surface/visibility/edit/semantic rollback;
- split history/surface/visibility/page publication; and
- no whole action, recursive scan/digest, whole clone/serialization, atomic insert/remove, or whole document replacement route.

## Safe Evidence Executed

### Rust parse/format

~~~sh
rustfmt --edition 2021 --check '<Flow VCS component.rs>'
~~~

Result: PASS. This is Rust syntax/format parsing, not a typecheck or test-execution claim.

The production-cutover check includes the retained VCS component plus the Flow Wasm component and protocol. All three parse and are rustfmt-clean.

### Production schema, package, and caller evidence

Node parsed the component schema and emitted package manifest. The schema has exactly 109 operation entries and 109 argument entries; 2501/2502/2503 resolve to the three VCS names with three fixed authority fields each; all three old whole-document names are absent. `node --check` passed for the source host, source browser, emitted host, Flow host test, and schema-oracle test.

Byte comparison reports exact equality for source versus emitted `🟨️flow-host.js` and source versus emitted `🟨️flow-browser.js`. The package manifest exports both subpaths, and the Flow-local build router persists those entries after copying the authoritative source files.

The production source caller census now finds `FlowRetainedVcs` in the Wasm component as an owned domain field and exactly one constructor, followed by ten non-test lifecycle call shapes: `begin_checkpoint`, `poll`, `take_page`, `resume_page`, `retry_page`, `acknowledge_page`, `cancel`, `fault`, `close_operation_step`, and `close_retired_step`. The exact mounted subset reports one bridge factory, three VCS registrations, one `D::start_feature` dispatch, one renderer production consumer, a persistent package export, six reachability fixture rows, and zero old-route matches. The former production-caller count of zero is resolved.

The official `bun ./📜️script.ts verify interactivity tool-jobs` command was executed. It completed its static census and self-tests but remains globally RED on concurrent out-of-scope migration work: `admitted=0`, `remaining=884`, `factories=13`, `registrations=0`, `dispatches=3`. Its emitted failure ledger contains no Flow/Flow-VCS production-reachability failure and the current verifier has no dedicated Flow predicate. The exact Flow factory/registration/dispatch/consumer subset above was therefore run separately and passed; the global RED is not represented as a pass.

~~~json
{"status":"PASS","factory":1,"registrations":3,"dispatches":1,"lifecycleCallers":10,"controls":3,"exports":4,"productionConsumer":1,"packageExport":"persistent","oldRoute":0}
~~~

The language-neutral production ledger parses as six data rows and contains all ten required lifecycle units. The source-ready real-bridge law is not reported as executed because Cargo was prohibited.

### Exact hostile fixture census

~~~json
{"status":"PASS","bytes":[12,65536,65537],"authority":4,"malformed":3,"grants":5,"boundaries":24,"controls":48,"rollbackBoundaries":5,"freshPrimaryCancel":5,"freshPrimaryFault":5,"fingerprints":11,"completeFingerprintFields":16,"signatures":"exact"}
~~~

The evaluator parsed all three JSON fixtures, recomputed every stored canonical signature, verified byte lengths, required complete per-boundary protocols/state references, and required every complete fingerprint field.

The current signature census is eight signed groups and sixty-three signed fixture values. `🗂️owners.json` contains seventeen hostile omission laws, including missing fresh rollback cancel/fault, mismatched rollback stage/authority, and mismatched exact surface owner.

### Resolved hostile protocol census

~~~json
{"operations":13,"perTransferProtocol":true,"perTransferExactState":true,"fingerprints":true,"fingerprintFields":16,"signatureGroups":["byteVectors","authorityVectors","malformedVectors","grantVectors","transferControlLedger","fingerprints","expectedStates","protocolDocuments"]}
~~~

### Complete retained route

The unchanged complete-route static law remains GREEN:

~~~json
{"status":"PASS","routeLines":1799,"forbidden":23,"required":10,"fullGrantChecks":5}
~~~

### Diff hygiene and raw census

`git diff --check` passed for the component, all three fixtures, and this report. The read-only raw caller census remains exactly eight: Store guard, directed DAG, Shooting, FEM 2D, FEM 3D, CAD, Puzzle 5D, and Puzzle 3D. Flow remains absent.

## Deferred Gates

Cargo, focused Rust unit execution/typecheck, Nx, Wasm rebuild, browser, native runtime, timing, and cache-touching gates were prohibited and were not run or claimed. The new production bridge and hostile reachability laws are source/static-ready but must be compiled and executed once the embargo lifts. Production adoption is no longer deferred in source: the real browser package subpath is installed and its Wasm dispatcher constructs and drives `FlowRetainedVcs`.

## Changed Paths

1. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🦀️component.rs`
2. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/📒️lifecycle.json`
3. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/🗂️owners.json`
4. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌿️vcs/🪞️fixtures/🔮️oracle.json`
5. this report

Production-cutover additions:

6. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️component.rs`
7. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🦀️protocol.rs`
8. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🧬️schema/🔣️component.json`
9. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🧪️fixtures/📒️ledger.tsv`
10. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/🧪️fixtures/🌿️vcs-production.tsv`
11. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/📦️packages/🟨️javascript/🟨️flow-host.js`
12. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/📦️packages/🟨️javascript/🟨️flow-browser.js`
13. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌉️wasm/📦️packages/🟨️javascript/🧪️tests/🧪️flow-host.test.js`
14. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/🟨️flow-host.js`
15. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/🟨️flow-browser.js`
16. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/pkg/package.json`
17. `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust/📜️script.ts`

No modifying Git command was run. Concurrent unrelated work was preserved.
