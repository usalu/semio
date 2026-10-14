# Cargo Execution Contract Early Audit

Read-only static audit of in-progress General Cargo execution contract. No tests, imports, native commands, frontend, resource operations, or Git mutations were executed. This report does not establish Source or native proof. Native frontdoors remain held pending freeze.

## Snapshot

General execution TypeScript SHA-256: `8b94795184b50eed4f6ab470d74ccc8282aee1bb2346ad764dd88a1eca2b0747`.

Execution schema SHA-256: `ba93ff74e57bb741658fd2e8d0f0de53e5e38603874b90a2f06c6ee6b07d3fb8`.

Paths below are relative to `🧰️framework/🔨️modules/🏃️process` unless stated otherwise. Concurrent changes may invalidate line numbers or findings.

## Confirmed Static Binding Issue

`🧪️testing/🦀️cargo/🏃️execution/🟦️.ts:4` imports `advanceScriptInvocation` and `scriptInvocationEnvironment` from direct `🧭️routing/🟦️.ts`. That module privately imports the former at line 3 but its export list at line 4 excludes it; it does not declare or export the latter. The environment helper is declared at `🧭️routing/📥️invocation/🏃️process/🟦️.ts:64`. The public routing package facade exports the needed bindings. Sent precise locations to HighScope and Root. This is a static finding only; compilation has not been run.

## Conditional Race Requiring Issuer Inspection

Execution lines 33–38 validate the mutable request/backend and compare request equality before line 39 awaits caller publication and continuation. Lines 42 then read backend executable, arguments, and cwd. A caller callback can mutate a shared descriptor across that await unless the eventual issuer deeply freezes or independently snapshots it. The issuer is still in progress, so this is an integration checkpoint rather than a final defect. Readonly TypeScript types alone do not establish runtime immutability. Sent this checkpoint to HighScope.

## Observed Contract Shape

The canonical domain schema owns closed request, declaration, and backend definitions and has an explicit ID. The facade exports first-party public types. General execution imports no Repo owner, carries request input through existing `runOwnedCommand`, returns void, and does not synthesize a native receipt. Requests contain operation-specific selection fields without policy, environment, storage, or target-root fields. Exact-law targets are closed discriminated shapes. Declaration selection compares program/cwd/fixed argument prefix, requires exact length for `none`, rejects ambiguous matches, and binds a test tail into the request only when its declared arguments are empty. Runtime execution compares the selected request with the backend request.

Execution forwards original cancellation and intersects the child bound with remaining original time. Environment propagation uses the original invocation. Actual endpoint admission, source snapshot rechecks, source grant identity/reservation, and Actor/build issuer binding cannot be established from the present General contract files and remain integration review work. Schema draft-07 with `$defs` is the repository's existing canonical contract requirement and is not reported as a defect.

## Capability Issuance Follow-Up

Subsequent static snapshot hashes: General execution `402612e04a9b8e5a3284f1ef2f426837852f60966a67157c8d09def25dced0c1`; Repository execution `9857879c0becce4bdc9767084b9b1b0612eee0fe2f4ea558b49d81b75c49162f`; build caller `9ce037bd0fc901bcb479b38c574142e5d26735b62e7206ad75bd32873ca65654`. These files remain under concurrent implementation.

The original import mismatch and mutable backend across publication have been addressed in the new General snapshot: imports name defining modules; capability/request snapshots and comparisons bracket publication and child execution.

### Original Policy and Control Identity Gap

General execution lines 34–46 captures capabilities/request but not original policy, control, signal, or callback identity. A publication/continuation callback can replace `invocation.control` with another valid closed control object using a fresh un-aborted signal and remaining-time callback, or replace policy with another valid policy. Existing `checkScriptInvocation` validates the current values and does not establish identity with entry values. Line 44 consequently supplies replacement signal and replacement policy/deadline environment to the child port. This can bypass an original cancellation during publication before the first port execution. The Repository input reader line 59 and runner line 74 likewise preserve capabilities/request only across their awaited operations. Sent precise locations to HighScope, HighRuntime, and Root. This is a static branch finding; no runtime proof was attempted.

### Issuance and Admission Observations

Build caller lines 52–59 appends effective task arguments once to the declared command, issues a backend from that complete selection, validates the closed native capabilities, and publishes them through the existing invocation environment. The native capabilities reader registers the canonical General schema as a sibling document, closing backend shape through its external schema reference.

Repository issuer lines 41–47 reads actual manifest bytes and the manifest-named declaration using bounded reads and captures their file identities, ancestry, hashes, and lengths. Selection matches complete command program, working directory, and argument grammar; request manifest must resolve to the same manifest. Admission lines 51–54 recaptures sources and compares the full backend and selected request with the originally transmitted backend rather than replacing the received snapshot. The input reader rechecks sources/capabilities before and after each awaited chunk, enforces byte and line limits, and checks exact request equality before orchestration. Runtime source admission and genuine Source grant reservation were not executed or established by this audit.

## Registered Source Mounting and Canonical Schema Audit

Static snapshot: Process script SHA-256 `3383cd20d02b4dfd20a030ce4b6fa85f5cecd2f2400a52e8f9104ad2fe54c175`; General execution schema SHA-256 `3d2b7db1fb3965d1b2b12528121dfe7b4cbfa89e3cc7bd9214defab264f8d2d6`. No commands were launched through the inspected routes.

### General Source Test Imports Specific Store

General `📦️artifacts/🏗️native-build/📥️source/🧪️tests/🟦️.ts:78` imports Repo `withRepositoryCompilerStoreV1`; its physical rustc corpus at line 82 uses that Specific Store. The existing Specific registered owner is Repo library `⚡️caching/📜️script.ts:142–143`, where `rust-units` mounts this General Source corpus and the General unit tests. HighScope should move only the Repo-backed physical oracle to a Specific Cargo Source test owner and mount it in that existing Specific `rust-units` suite. Neutral General closure tests and their reusable fixture should remain General-owned. Sent exact locations to HighScope and Root.

### Execution Test Registration

General Process `📜️script.ts:76` already mounts General execution contract tests within `test exact-cargo-storage`; this adds no Repo test dependency. The new Repo endpoint test path is not currently mounted by a registered script. Exact existing Specific owner: Repo library `⚡️caching/📜️script.ts:142–143`, entry `repository-exact`; corresponding `⚡️caching/📋️project.json:697` target `test-repository-exact-cargo` calls `bun ./📜️script.ts test repository-exact` with Source execution metadata. Extend that Specific suite with `../🏃️process/🦀️cargo/🏃️execution/🧪️tests/🟦️.ts`, or register a parallel `repository-cargo-execution` Source target in that same owner. General Process must not import or mount Repo endpoint tests. Sent this integration gap to HighScope.

### Canonical Export and Catalog Gap

General execution schema now has the canonical address `https://json.schemas.assets.semio-tech.com/framework/process/testing/cargo/execution/component.json` and required draft-07 dialect. However, all its `$defs` keys are lowercase (`text`, `strings`, `target`, `group`, `request`, `command`, `declaration`, `identity`, `source`, `backend`). Repo discovery `🔍️discovery/🟦️.ts:3515–3517` rejects every non-PascalCase export key using taxonomy `schemaExportResolution.exportIdPattern` at line 28960 (`^[A-Z][A-Za-z0-9]*$`). HighScope owns renaming these to domain PascalCase names and updating all local and native-capability external refs. This is a direct current repository rule, not a generic JSON Schema style preference.

The new execution scope/address is absent from Repo library `🔣️schema-catalog.json` and `📓️schema-catalog.md`. Taxonomy lines 28958–28961 identify these canonical catalog outputs; catalog generator metadata names `bun ./📜️script.ts schema generate`. HighScope should include the new schema through the canonical catalog publication after export corrections. The Actor `🧪️execution/🔣️.json` is declaration data referenced by Cargo metadata, not a separate schema document; it should consume the General declaration schema rather than define a fixture schema. No catalog generation, schema probe, or runtime import was performed by this audit.

## Current Correction Review

Later static hashes: General execution `3db36987b1895c18b82e5f4d6b4e81f76144c143323911f910c020ca77cbbcee`; Repo execution `7554814081f4c1c7dbf885085ec5503badda817aeb7bcca631169093aeb71d1a`.

General execution lines 36–47 now captures and retains original control, signal, callback identities, policy value, capabilities, and request across publication, child progress, and execution. The child receives the captured original signal. Repo execution lines 21–23 retains original policy/control/capability object identities plus callback/signal identities and serialized policy/capability images; the input reader uses the captured signal for its interruption race. These changes address the reported original-control substitution paths by static inspection. No test or runtime acceptance is claimed.

General schema `$defs` now uses PascalCase domain names, and the reader maps public request/backend/declaration selectors to those names. The earlier lowercase finding is resolved in current source. Catalog and Specific test mount integration are owned by HighScope and remain in progress at this audit boundary.

Root took the Source oracle relocation. Exact original boundary was General Source test lines 74–83 (five physical-only imports and the final `genuine rustc dep-info independently confirms neutral Source closure membership` test). Preserve neutral tests lines 1–72, including `corpus.closures` membership cases lines 67–72. The relocated Specific oracle should consume that same General corpus and closure/dep-info APIs and be appended to the existing Specific caching `rust-units` mount at line 143. This preserves the independent compiler oracle without a General test importing Specific Store authority. Precise boundary and caller path were sent to Root.

## Relocation and Final Port Static Review

Current verified path/hash pairs:

- Specific Repo library `⚡️caching/🦀️cargo/📦️units/🧪️tests/📥️source/🟦️.ts`: `edd74aceb89f1a3697fe891128770b3d4f5377609d1395cca02ba6e6fe8307d8`.
- General Process `📦️artifacts/🏗️native-build/📥️source/🧪️tests/🟦️.ts`: `0141660537880406a1c42bcc91860574d28fde47da37b601aadc35f2fc85bcc5`.
- General Cargo execution: `3db36987b1895c18b82e5f4d6b4e81f76144c143323911f910c020ca77cbbcee`.
- Repo Cargo execution: `7e2e23f45c9c611a14f3d617dc94b40d6a5ded9b3c30b9294602078cd41cd9fa`.

All seven relative imports in the relocated Specific oracle resolve to the intended existing files. The physical test logically preserves the previously inspected body: original invocation and compiler admission, actual rustup discovery, Repo compiler Store, unchanged closure corpus, rustc dep-info membership comparisons, General closure comparison, and Store rechecks around cleanup. The neutral General file retains closure corpus membership cases and no longer imports Repo. Shared fixture ownership remains General; Specific consumption creates no reverse import. The existing Specific `rust-units` branch includes the new oracle and both neutral General Source/unit test files. The Specific `repository-exact` branch now also includes Repo execution tests; no General test mount of Repo tests was observed.

Final inspected Repo runner refuses native mode/offline/target environment differences before imports, resolves selected test policy and checks manifest/mode/target, retains environment bytes across imports/execution, and rechecks original invocation and source descriptor before/after awaited work. General retains original cancellation/control and exact selected request across publication/progress/execution. Build metadata injection still derives one backend from declared command plus effective task argv and validates the closed native owner capabilities. No additional confirmed contract hole was found in these bounded static checks.

This reviewer executed only static file reads, lexical path resolution/existence checks, and hashing. No test, import, factory, compiler, Source oracle, native process, or registered command was executed by this reviewer. Other agents' test reports are not evidence asserted by this report.

## Source Storage Argument Correction Audit

Static hashes: General exact Cargo `7c47f6200912f4887b29c964bba7509ff7268cca61408a28a519728c55bc878b`; Repo Cargo admission `a61d58e9af9a974fbf8dd8aa8a41fd90f7466d470c8f77e3b49f13c431e0ae65`; Repo cache budget opening `4c65ac3fe802aaec5eb4aa2203626f8052e403cff6684f9f962499679d75969e`; General Source admission `f9fb00cbfb8fac3733cf975b412c879ccdff28830502c5edc05c7cbb8ef5aeb6`.

General exact Cargo line 214 admits `options.storage` through `requireNativeStorageReservationV1` under original-derived compiler control and requires compiler-workspace allocation; line 266 passes that exact reservation as the new third actual Source port argument. The bounded phase wrapper retains the original signal. Repo admission line 37 passes `store.storage`; compiler Store obtains it through actual acquire/recover reservation issuance and keeps the backing open through the body. Native storage `originalReservations` is a private WeakMap keyed by the actual reservation object and original signal; issuance freezes/registers the reservation, and admission refuses copied/manufactured objects. Reservation recheck verifies durable held allocation identity and enforcing backing custody. No cast, JSON copy, default storage, or manufactured witness was observed in these corrections.

All relative imports in General exact, Repo admission, Repo cache budget opening, and General Source admission resolve to existing files. In particular, the opening module's corrected deep General imports resolve to the intended platform, lease, retention and custody owners.

One existing callback asymmetry remains in General Source admission line 15: `cfg` requires and rechecks storage before awaited rustc capture but returns its result without a post-capture `storage.recheck()`. The `read` callback checks both sides. Both corrected exact/admission callers recheck after the complete Source admission before compiler dispatch, so this audit does not establish a compiler-dispatch bypass. HighScope received the precise recommendation to recheck after cfg capture and before returning observed cfg. No confirmed regression in the new third storage argument or opening import correction was found. No runtime/test/native execution was performed.
