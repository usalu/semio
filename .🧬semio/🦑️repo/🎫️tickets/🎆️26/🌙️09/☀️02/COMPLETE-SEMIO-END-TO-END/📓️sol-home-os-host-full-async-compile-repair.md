# Home OS Host Full Async Compile Repair

## Scope

The registered Home identity/native packet reaches `semio-framework-os --lib --features os-host-full`. Receipt `exact-cargo-laws-C22uJD` built and ran the plugin-host inference effect law (`GREEN1`, executable SHA-256 `dc4dd506699e6bd6720be7b58b7546c018c104707e546986ea847d6635fc9cb8`), then stopped while compiling the OS host test binary. No Home identity law ran.

## Recovered diagnostic

`01/build.json` records Cargo status `101`. Its JSON diagnostics contain 149 errors:

- 28 missing `semio_framework_async_macros` resolutions from the included workflow test module.
- 55 type mismatches, 27 future-field accesses, 22 future trait-bound cascades, 12 missing result/vector methods on futures, three obsolete `dyn Backbone` uses, and two comparisons against unresolved futures.
- The concrete owned boundary was the OS host's synchronous facade and its tests after kernel APIs became async: `ConfigSpec`, `CommandGrammar`, `AppIo`, workflow constructors/planning, Store constructors/dispatch/parsers, backbone constructors/storage, and format registration.
- Two extension fixtures still constructed `serde_json::Value` after the extension manifest switched to the first-party JSON value.

## Repair

- Added the existing first-party async test macro crate as an OS-host dev dependency.
- Kept the already-published synchronous OS-host facade on its documented immediate-future boundary, including enum-dispatched `store::Backbones`; no compatibility wrapper or runtime API redesign was introduced.
- Resolved the affected pure/in-memory async constructors and calls at the existing immediate boundary in native host tests and synchronous storage ports.
- Updated extension fixtures to the first-party `semio_framework_os_kernel::json::Value`.
- The first warm retry, `exact-cargo-laws-Tj3Gs6`, kept the plugin-host group GREEN and reduced the OS-host compile frontier from 149 errors to 11 diagnostics at seven exact call sites. Those were repaired at their typed boundary: async `AppIo::with_ports`, fixed edit-message ledger comparison, borrowed cursor comparison, synchronous workflow validation/event-log construction/I/O registration.

## Qualification

Registered warm retry `exact-cargo-laws-60QtKH` stopped in group 00 before any Home law: the concurrently evolved durable-group composition omitted newly required `OpBinary + OpText` mutation bounds at its three `ArtifactStore::begin_apply_one` calls. Its owner repaired and source-qualified those exact bounds.

Immediate retry `exact-cargo-laws-v4J1FZ` produced two positive receipts: current plugin-host `GREEN1`, then `semio-framework-os --features os-host-full` compiled and its exact workflow law ran `GREEN1`. Group 02 completed its fresh Space dependency build but failed before test listing because the crate root mounted a removed logical fixture path. The actual bounded physical fixture is `📇️bumps-the-36f82f`; its stale `📇️bumps-the-catalog-generation-to-7` mount was repaired. A TDD source assertion first reproduced the stale mount and now proves both the physical fixture and exact mount; the language-agnostic/source packet is GREEN (`checks=14`).

The public-member cache was handed back for one coordinated warm retry, `exact-cargo-laws-52dKES`. That process did not survive the executor continuation and has no terminal verdict. The `C22uJD`, `Tj3Gs6`, `60QtKH`, `v4J1FZ`, and `52dKES` receipts therefore remain build failures or interrupted attempts, not complete Home runtime verdicts; `v4J1FZ` nevertheless provides valid scoped GREEN receipts for groups 00 and 01.

## Continued Space test-binary migration

The resumed warm attempts consistently compiled and ran groups 00 and 01, then exposed the next current-API fixture cohort while building the Space group 02 test binary. The bounded receipts were `f5UhF6`, `m3bJg5`, `6euyP6`, `BF2VzI`, `tR7rg8`, `GCMckJ`, `0e33mf`, `9WD6Ca`, `mHWvhW`, and `ty0rrr`. They repaired, without compatibility facades:

- dynamic first-party JSON array construction and all retained Home/Space snapshot, mutation, and diff fixture codecs;
- Home config's third-party JSON oracle by parsing and re-emitting the first-party canonical wire before domain `FromValue` decoding;
- async Home/Space app, config, catalog, document, Store, dispatch, backbone-ref, and workflow fixture boundaries;
- the authoritative `OsBackbonePorts` catalog/sync path and a test-only canonical hash dependency used to seal directory-page fixtures;
- fallible fixed-capacity Home editor/viewer row assembly, including explicit admission, typed fixture projection/retirement, exact client identity, and async catalog/row helper ownership;
- all physical `#[path]` mounts in the Space crate, guarded by the neutral mount-existence census.

After every repair the registered source/census target returned `home-directory-identity-rows-check: checks=33 clean`. The canonical actor-import fixture call was also re-read after the public builder control contraction and now uses `buildClosedBrowserActorArtifactV1(component, {})`; it passes no removed `repoRoot` or `evidenceRoot` authority.

Receipt `exact-cargo-laws-SscNyl` ended BUILD RED in group 02 after groups 00 and 01 were GREEN: the last textual Home row assertions attempted to serialize `BuiltNode` directly. Those assertions now use the existing first-party test projection and retire the projected component tree.

Receipt `exact-cargo-laws-wJyQYU` again kept groups 00 and 01 GREEN, then ended BUILD RED in group 02 because that test-only projection helper accepted `UiNode` while `render_rows` now returns `BuiltNode`. Both editor and viewer helpers now accept the exact `BuiltNode` boundary, and the full neutral source/path census remains GREEN at 33 checks.

Receipt `exact-cargo-laws-ToE61X` kept groups 00 and 01 GREEN, then ended BUILD RED in group 02 because five adjacent structural Home editor/viewer assertions still matched the retired `UiNode` variants. The complete cohort now inspects the fixed `BuiltNode` tree directly and retires every observed tree even when an assertion panics. It retains exact checks for the stable row key, author action count and action scope/name/space id, restricted/local open-only policy, create-space binding, and viewer absence of action buttons.

Receipt `exact-cargo-laws-D5dv6s` kept groups 00 and 01 GREEN, then ended BUILD RED in group 02 at three adjacent dialog assertions that still applied `?` to the now-infallible `ActionRef::new`. The exact cohort now calls the current constructor directly, and a neutral source guard rejects reintroduction. The source/path census is GREEN at 34 checks.

Receipt `exact-cargo-laws-TwAqPd` kept groups 00 and 01 GREEN, then ended BUILD RED in group 02 at three adjacent Space-index/members assertions that attempted to encode fallible render results or fixed `BuiltNode` owners as Pack values. Completed `ComponentTree` values now use the first-party projection/retirement path. The binding-sensitive members tests use a third-party serde wire oracle and then explicitly retire their exact fixed tree. Two neutral source guards protect those boundaries, and the source/path census is GREEN at 36 checks.

Receipt `exact-cargo-laws-FlfDcB` kept groups 00 and 01 GREEN, then ended BUILD RED in group 02 at the final mounted Space editor row cohort: one fallible `BuiltNode` serialization and two retired `UiNode` structural matches. Space editor row tests now admit and project or directly inspect the fixed tree, prove exact action bindings, and retire every owner. The adjacent Space viewer default/row tests were migrated in the same pass. A bounded census over the mounted artifacts now finds no `let UiNode::` or Pack-node serialization pattern; the source/path corpus is GREEN at 38 checks.

Receipt `exact-cargo-laws-ZZbbvg` kept groups 00 and 01 GREEN, then ended BUILD RED in group 02 at the enclosing Home/Space viewer fixture cohort: two async definitions were not awaited and one fallible component tree was passed to Pack encoding. Both definitions are now awaited; every render tree is admitted, projected, and retired. The proactive mounted-artifact census now also finds no raw artifact render encoding or unawaited `create_*` definition pattern. The source/path corpus is GREEN at 40 checks.

Receipt `exact-cargo-laws-JWcQj4` kept groups 00 and 01 GREEN, then ended BUILD RED in group 02 at two adjacent current-codec seams. `create_space_index_viewer` is intentionally synchronous, so its fixture no longer awaits it. `SpaceConfig` is a first-party `ToValue`/`ArtifactDsl` type rather than `serde::Serialize`; its retained preparation law now projects through the first-party value codec, crosses a third-party `serde_json` decode, and compares the exact post-state JSON oracle. The neutral source/path census is GREEN at 41 checks. The public-member target was verified process-free before the next warm retry; no Home row runtime verdict is claimed until that retry reaches a terminal group 02 receipt.

Receipt `exact-cargo-laws-A5n5Vp` again ran groups 00 and 01 GREEN, then ended BUILD RED at three adjacent Space-engine fixtures in group 02. The empty-initial-snapshot assertion now awaits the async app snapshot. All three checkpoint/checkout arguments now convert the first-party scoped JSON value into the exact `DslValue` expected by `handle_action`; no borrowed JSON value crosses that boundary. The source/path census is GREEN at 42 checks before the next warm retry.

Receipt `exact-cargo-laws-mVLRHi` again ran groups 00 and 01 GREEN, then ended BUILD RED at the next three emitted Space-engine sites in group 02. Both checkpoint apps now use `VcsArtifactApp::<SpaceApp>` so the current default `NoMembers` type is selected rather than left ambiguous. The complete adjacent locale/body cohort now awaits all four fallible `SpaceApp::render` calls, projects their admitted component trees through the test boundary, and retires every owner. A proactive census rejects every former direct render serialization, and the source/path packet is GREEN at 43 checks before the next warm retry.

Receipt `exact-cargo-laws-x7mmLx` again ran groups 00 and 01 GREEN, then ended BUILD RED at one Space context-menu fixture whose three vector operations all targeted the unresolved future. The sole direct test invocation now awaits `space_workflow_context_menu_items`, matching the already-current production call. The source/path packet is GREEN at 44 checks before the next warm retry.

Receipt `exact-cargo-laws-Vdf8jp` again ran groups 00 and 01 GREEN, then ended BUILD RED at two config round-trip future dereferences and one unawaited format registration in group 02. Both config laws now await the current helper, and the neutral Home-I/O format descriptor is registered through the async registry boundary before its handlers are installed. A broader current-source census also found two catalogue tests holding unresolved fallible builder futures and one unawaited studio-port registration. Both catalogue owners are now awaited, admitted, projected, and retired; the port registration is awaited before opening the document. The same-file async helper census has no genuine remaining unawaited test call, and the guarded source/path packet is GREEN at 46 checks before the next warm retry.

Receipt `exact-cargo-laws-pCWWNn` again kept groups 00 and 01 GREEN, then ended BUILD RED at three independent current-boundary sites in group 02. The node-graph fixture now edits the `serde_json::Value` returned by `os_workflow_to_flow_fixture` with `serde_json::Map`/`Value`, while the command envelope remains on the first-party Pack value family. The created catalog studio now passes the exact `Arc<OsBackbonePorts::Store(..)>` required by `create_os_space` and coerces the same owner into the test registration interface. A full targeted census of the current Home/Space test entrypoints found five unresolved `empty_workflow_snapshot` calls across the example and catalog tests; every call now awaits the snapshot before constructing `ArtifactView` or invoking `studio_emit`. The permanent source/mount packet is GREEN at 48 checks; this is source evidence only until the next registered group-02 runtime receipt.

Receipt `exact-cargo-laws-KXSc1L` retained current GREEN runtime receipts for plugin-host group 00 (SHA-256 `97274e8e7486a7bcde07782612a90f8ad275cd55a669bd1ef8386a9d5afcbb96`) and OS-host-full group 01, then ended BUILD RED in group 02. Its three diagnostics were the remaining async UI-fixture cohort: two `load_document_snapshot` calls and one `VcsArtifactApp::render` call. The source-wide call-site census found two additional identical render sites that the compiler did not report before aborting. Both document loads and all three app renders are now awaited; every returned `ComponentTree` is projected and retired through the first-party test boundary instead of being serialized as a retained owner. The permanent packet is GREEN at 49 checks before the next warm retry.

Receipt `exact-cargo-laws-LDXj5H` preserved current GREEN runtime receipts for groups 00 and 01, then stopped before compiling the Space test binary on a concurrent directory-client execution-target migration: `DocumentExecutionTargetLeaseFieldsV1` was missing the new `browser_actor` member, and `lease_fields_from_plan_v1` had not yet been migrated to its fourth optional byte-length argument and fallible return. These are not Home/Space fixture failures and remain with the catalog/plan owner; no Home row law ran. The public-member target was released pending that owner’s coherence signal.

After that owner completed the execution-target constructor census, receipt `exact-cargo-laws-kkCwUN` again preserved current GREEN groups 00 and 01 and reached the Space test binary. It ended BUILD RED on three `VcsArtifactApp<SpaceApp, _>` fixtures whose current `SpaceMember` parameter could not be inferred. The broader source census found a fourth identical constructor in `spawn-app` that rustc had not emitted before aborting. All four fixtures now select the intended default explicitly as `VcsArtifactApp::<SpaceApp>`, and the permanent source/path/async packet is GREEN at 50 checks before the next warm retry.

Receipt `exact-cargo-laws-d57P4X` again preserved current GREEN groups 00 and 01, then ended BUILD RED on the final three direct `BuiltNode` serialization sites in the inspector and parameters panels. A complete Space-tree search found exactly those three sites. They now cross the existing first-party fixture projection and retirement boundary, preserving the original JSON assertions without requiring `BuiltNode: ToValue` or leaking retained component owners. The permanent packet is GREEN at 51 checks before the next warm retry.

Receipt `exact-cargo-laws-dJCJv0` preserved current GREEN groups 00 and 01, then proved the projection helper's stricter ownership boundary: each of the three repaired panel sites supplied the `BuiltNode` root while the helper accepts only a complete `ComponentTree`. All three now wrap that root explicitly before projection and retirement. The source oracle was updated to require the complete-tree handoff.

Receipt `exact-cargo-laws-DkUijX` compiled the complete current Space test binary without errors after groups 00 and 01 were GREEN. Its exact inventory fence then rejected three stale row-law paths because the canonical test modules are now nested below `main::component::tests`. The emitted inventory supplied the three authoritative names, which replaced the stale selectors without relaxing exact selection. The same fresh build exposed nine unpolled future warnings across the current async cohort: six surface testkit assertions and three synchronous command-to-async-registry seams. All six assertions now await their testkit future; the three synchronous handlers use the existing explicit `resolve_ready` boundary. The permanent oracle requires all nine and the current selectors, and is GREEN at 54 checks.

## Terminal native receipt

Registered warm receipt `exact-cargo-laws-E3jNek` is terminal GREEN with five exact runtime laws:

- `semio-framework-plugin-host` executable SHA-256 `f5a5e351ef7422cf8e789140ae215e37aeee2783bbf935889263cfe5d2b01d7e`: `request_inference_proposal_preserves_the_closed_kind` (`GREEN1`).
- `semio-framework-os --features os-host-full` executable SHA-256 `bc64997eb644d098a2776424da600d0263593d95f812c4f13132ff484e669784`: transformed SVG path extraction (`GREEN1`).
- `semio-s-plugin-space` executable SHA-256 `32375eabe77ce4847d170a05768077d4eb1ac319642f2e3ebc4d893a3bb3a1b4`: the author Hub row's stable id and dispatchable actions, spectator/unbound Hub open-only policy, and viewer row stable id (`GREEN3`).

The exact artifact directories are `home-directory-identity-rows-exact/exact-cargo-laws-E3jNek/{00,01,02}` under the ticket-generated root. The public-member Cargo target was process-free and released after the terminal receipt. This qualifies the retained Home native row packet; it does not by itself qualify the still-separate full mounted Chromium Shell journey.
