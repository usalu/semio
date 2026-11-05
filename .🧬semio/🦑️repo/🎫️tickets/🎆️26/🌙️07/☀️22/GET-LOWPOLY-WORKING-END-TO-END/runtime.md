# Lowpoly Runtime

## Confirmed Defect And Fix

The duplicateObject command required nonempty literal mesh source and constructed copies without managed topology. Current primitives store geometry in mesh_state and keep literal source empty, so duplication rejected primitives; copies of literal-source objects also lacked the topology needed for rendering and editing.

The command now requires and clones managed topology, derives the copied child handle from that topology and the new object identity, and preserves independent literal source and compact paint layers. Object-command regression assertions read managed topology rather than stale source JSON.

## Regression Contract

The language-neutral managed-duplicate JSON contract covers empty literal source, managed geometry presence, a distinct copied identity, an X offset, and undo/redo object counts. The Rust object-command test checks exact managed topology and uses the third-party serde_json parser to compare paint serialized by the repository JSON implementation. It verifies exact copied object recovery after redo.

## Verification

- git diff --check passes for the object command owner. New and edited object-command paths are at most 167 characters long.
- Requested repo cargo test for semio-s-artifact-lowpoly-lowpoly-editor, target lowpoly_commands_object.
- Original object-suite tool session 95828 became unavailable after the initial agent turn. A replacement exact suite request is queued as session 25166. No Rust result is claimed until execution completes.
- An earlier request was cancelled after selecting the incorrect integration target; it did not run tests.
- Source inspection of paint commands and session confirms streamed brush/fill gestures retain one provisional transaction, commit on release, and abort on cancellation. Existing paint command tests already check exact undo/redo pixels.

## Files

- Object command Rust implementation.
- Object command Rust unit integration test.
- New language-neutral managed-duplicate JSON fixture.
- This report.

## Engine Regression Inspection

The engine unit suite no longer parses literal source as live geometry. Its sync regression still had a stale JSON-oriented name and only checked the transient cache. The strengthened managed-sync JSON contract and Rust regression now verify persisted mesh_state, content-addressed child identity, unchanged literal source, and edited geometry reconstructed from the snapshot with an empty session cache. No engine production change was needed.

## Canonical Verification Entry

Direct repo cargo did not provide the compiler admission needed by the repository test infrastructure. Replacement direct request 25166 was cancelled before execution. The retained runtime-tests.ts input was also unable to receive a Script envelope when launched directly and is unused. A temporary declared target pointing at that ticket script was removed before execution.

The reusable composition target is now repo run @semio-tech/lowpoly-plugin:test-runtime --parallel=1 (session 93032; supersedes the cancelled pre-start 36901 run), backed by the composition Rust package script. It selects the fixed composition, editor, engine and IO crate list through runRepositoryCargoTests. This covers all editor integration binaries, including mounted, media, object and paint, plus engine unit tests and IO tests. The target hashes default/^default and the full lowpoly owner tree.

## Live Infrastructure Observation

Integrated native session 93032 was admitted with one job, completed repository source listing/rechecking (about 525k operations), and is waiting for another process to build shared dependencies. A read-only repo leases status snapshot reported 79 held leases, zero waiting leases, zero deadlock cycles and zero recent order violations. Machine admission still had cores fully booked by running jobs. No native success is claimed at this stage.

A later fleet snapshot identified shared dependency builder process 30392 in dependencies:store-input-replay at age 37 minutes, owned by a separate agent. Capacity at that snapshot had zero waiters and about 24 GB spare above its floor, so ongoing shared dependency preparation, rather than initial memory admission, was the remaining visible boundary.

## Exact Runtime Source Inventory

- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🦀️.rs
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/🗑️object/🧪️tests/🧬️managed-duplicate/🔣️.json
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🔬️unit/🦀️.rs
- ✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🧪️tests/🧬️managed-sync/🔣️.json
- 🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/📜️script.ts
- 🌎️hub/🧩️compositions/💠️lowpoly/📦️packages/🦀️rust/📋️project.json

Retained ticket inputs: runtime-tests.ts (unused experimental entry) and this runtime.md report.

## Shared Dependency Owner Diagnosis

The coordinator reported a read-only status from the shared Draw dependency owner: preparation encountered Exact current Store Source fence changed before graph/compiler, involving node_modules endpoint/parent/ancestor comparisons. The dependency owner is actively diagnosing that guard mismatch. Native 93032 remains running under the original gate; no duplicate suite or infrastructure modification is being attempted.

## Verification Cancellation Before Compilation

At the coordinator's direction, only native session 93032 was cancelled while it still waited for shared dependency preparation. It had not started Rust compilation or any test body; no native success/failure result is claimed. The prior shared Draw preparation exited at the Source-preservation fence and a fresh diagnostic was queued; another older Puzzle producer then owned preparation. Cancellation releases the superseded loaded context for a fresh canonical run after shared repair. Other agents' processes were left running. Lowpoly runtime source and test fixtures remain intact.

## Continuation 2 Read-Only Compile Review

Static resolution found a concrete compile failure in the two newly changed unit tests: include_str paths used ../../🧬️managed-duplicate/🔣️.json and ../../🧬️managed-sync/🔣️.json. From their unit-test directories those resolve at the object/engine owner, but the fixtures are under each owner's 🧪️tests directory. Both should use ../🧬️managed-…/🔣️.json. This was reported to the coordinator while maintaining the requested read-only source review. Permanent composition script imports all resolve to existing public entry points; the Cargo helper is publicly reexported and takes the current four-argument call. Selected package manifests declare the fixed names. Existing editor dev-dependencies supply the new serde_json and repository value/pack uses. The engine parent imports HashMap and the existing mesh traits used by its nested test module. Scoped git diff --check passed. These are source observations, not a native compiler/test result.

## Continuation 2 Compile Correction

At the coordinator's direction, corrected both managed fixture include_str paths from ../../ to ../. An independent Node filesystem resolver now resolves all six literal includes in the changed object, engine, IO and surface test files to existing files and parses all six as JSON. The corrected targets are the existing object/🧪️tests/🧬️managed-duplicate/🔣️.json and engine/🧪️tests/🧬️managed-sync/🔣️.json; IO managed-export.json and gltf-scene.json, surface activation.json and the component descriptor also resolve unchanged. All five permanent runtime-script imports resolve. Scoped git diff --check passed again. No build was started and no native pass is claimed.

## Continuation 3 Gesture Terminal Correction

The retained editor boundary lowpoly_paint_step returned immediately on an eyedropper tick before driving the gesture lifecycle. An open brush stroke followed by a utility switch and a UV-bearing abort/commit therefore left its original transient open. Abort could leave dabs eligible for a later release to commit. The fix skips tick computation on abort and drives every other phase even when its tick only samples configuration; any sample mutation is merged after lifecycle settlement. No framework code changed.

The new paint terminal.json contract and Rust regression call the actual retained editor paint boundary with captured context: open a brush stroke, switch to eyedropper, then abort or commit with UV coordinates. Both must clear the transient; abort emits neither paint nor sample and commit emits the original one-dab stroke plus its sample. Existing full-host paint tests cover transaction history/undo/redo and cancellation. An independent XState oracle replayed the same neutral contract and passed both cases. That oracle validates contract outputs, not execution of the Rust regression. The fixture include resolves exactly and JSON parses; scoped whitespace check passes. No native build/test was started while shared repair is pending.

Additional source review found private retained-command symbols used across extracted modules and by an external integration test. The coordinator assigned that separate compile correction to the retained_visibility agent. The current AppOperationContext retained grant field was also flagged to that agent; the new paint test uses the mounted interactive maintenance grant.

Remaining expensive-command limitation observed: LowpolyRetainedCommandWork yields one preparing progress turn and honors host cancellation between turns, but its subsequent reducer invokes complete mesh/fill algorithms synchronously. Checkpoint resume replays the command preparation rather than resuming inside an algorithm. No broad framework change or speculative time-limit weakening was attempted.

## Paint Regression Cargo Ownership

The terminal_paint_phase_settles_after_switching_to_eyedropper regression is in the semio-s-artifact-lowpoly-lowpoly-editor package, explicitly mounted as integration test lowpoly_commands_paint by its Cargo.toml. Its path is editor/🎮️commands/🖌️paint/🧪️tests/🔬️unit/🦀️.rs. It invokes the view/session pipeline through the editor retained boundary but is not a view-crate lib unit test. The existing editor selection in test-runtime executes it; adding the view package is not required for this regression. This ownership was checked against the current manifest and source without starting a native build.

## Progress And Cancellation Presentation

Lowpoly now consumes doc.operations() in lowpoly_render. Following the Raster generic-control composition pattern, live operation controls wrap the existing Model and UV surfaces, while empty status removes that wrapper and other panels retain their existing content. The operation_progress_scope explicitly dirties only those two window bodies. Existing locale-resolved failure labels and renderer signatures were preserved. No descriptor execution flags were changed.

The control scope is derived with surface_app_id from LowpolyPlayApp::DIALECT and ROLE; it does not use the kit's legacy lowpoly-play alias. The regression independently compares emitted binding scope to create_lowpoly_app().controller_id, verifies its id/controller agree with the neutral canonical identity s.lowpoly.lowpoly@1/*#editor, and checks EditorApp<LowpolyPlayApp>::surface_id(). This matters because tree-windows actionBindingToActionDescriptor maps semantic ActionId.scope directly to controllerId, and the native Shell session-and-topology dispatch requires session.app.controller_id == action.controller_id. After that owner is selected, dispatch_operation_cancellation checks operationId/generation, instance, actor, nonterminal status and the live cancellation lease. No source evidence establishes a legacy alias admission.

The new editor integration target lowpoly_editor_progress renders real ArtifactView.with_operations inputs through LowpolyPlayApp::render. Its independent serde_json projection oracle checks both window surfaces remain, explicit English/German captions and cancel labels, polite live status, cancellation disabled after request, exact action identity and hexadecimal operation/generation payloads, empty-status removal, excluded catalogue panel, and precise refresh scope. The neutral contract is editor/🧪️tests/📶️progress/status.json. Its include path and declared Cargo mount resolve; both new paths are below 140 characters. Scoped whitespace checks passed.

Cargo corrections accompanying this work: semio-framework-ui-contract moved from dev to normal dependencies for the runtime column builder. Existing semio-s-artifact-lowpoly-lowpoly-kit also moved from dev to normal because play-app already uses reset_document_effect. Kit's normal dependencies are framework/model/config/view, with no editor backedge; no duplicate kit dependency remains. pack::snapshot::ArtifactPack is valid because the semio-framework-pack manifest explicitly declares [lib] name = "pack"; no alias was added.

Native tests and actual UI acceptance remain unexecuted while shared dependency repair is unfinished. Required acceptance still includes observing progress during a real retained command, activating Cancel against its exact live identity before publication, and confirming terminal cleanup and no cancelled document edit/history row. The existing synchronous algorithm turn limitation remains separate from this presentation correction.

The independent progress fixture audit additionally located the final host app-address gate: plugin/🧵️instance/🔌️plugin-app-impl/📨️action-dispatch/🦀️.rs rejects address.app_id unequal to self.app.instance_id() before classifying even a reserved cancellation action. EditorApp::instance_id returns its dialect/role-derived surface id. The new oracle now also asserts cancellation action version 1.

## Mounted Retained-Operation Cancellation Regression

Existing Lowpoly mounted tests exercised command persistence and paint gesture abort, but no test dispatched cancelTypedOperation against a live retained command. The Raster png_progress_control_cancels_its_exact_visible_operation test is the authoritative peer pattern; Lowpoly had no corresponding coverage.

Added visible_operation_cancel_rejects_stale_generation_and_preserves_document_history in editor/🧪️tests/🔌️mounted/cancellation.rs, included by the existing mounted/🦀️.rs and therefore Cargo integration target lowpoly_editor_mounted in the editor package. The neutral cancellation.json contract selects a face and starts shell-shape extrude without settling it to completion. The test advances actual host maintenance/publication until its rendered progress exposes the cancellation binding, independently reads that JSON through serde_json, checks assembled controller identity, refuses a stale generation, submits the exact original cancellation identity, ACKs actual pages until normal Terminal, and compares the full snapshot and committed history count with the baseline. It also verifies progress removal and advances sixteen additional maintenance/publication turns, asserting no late result/effect/completion or mutation.

This is focused pre-publication cancellation coverage, not a claim that a running mesh algorithm is interruptible internally. The retained command yields its preparing progress turn; its following reducer still computes a whole mesh/fill operation synchronously, and checkpoint replay restores preparation rather than an internal algorithm cursor. Runtime latency and the mounted test body remain unverified until the coordinator's single native run executes. No duplicate build was started.

## Admission Deadline Handoff Repair

See deadline-projection.md for schema-first reproduction, precise correction and limits. Focused registered admission suite passes 7 tests / 41 assertions, including independent Python and SQLite oracles. Existing actual child test refuses light-route setup without admitted parent; no native/compiler pass claimed. Source retains transitive issuing-control cancellation/retirement and cached finite narrowing, with measured wait credited once. Mounted cancellation fixture sequencing now inspects progress before ticking; after stale rejection it re-renders the same live control and compares full serialized history upserts plus snapshot before exact cancellation.

Final deadline projection scope: identical-byte wire records merge transitive sources without rebasing their earliest clock, including ordinary/unbounded projections. Deterministic fixed-clock finite+unbounded collision regression passes. Final registered admission suite terminal exit0: 8pass/45assertions; scoped whitespace check exit0. See deadline-projection.md. Native remains unexecuted by this agent.

Final post-materialization cancellation audit added finite+ordinary environment-getter regression. Current admission suite terminal exit0: 9pass/47assertions; exact current/source cancellations checked after environment spread. deadline-projection.md updated. No native acceptance claimed.

## Superseding Deadline Authority Review

Earlier source-lifetime/provenance-union claims are superseded: serialized envelopes are deadline DATA; CURRENT actual control plus parents/follows own cancellation. Root requested positive sequential ordinary regression, which exposed poisoning after a retired ordinary scope under wire-lifetime unions. Removed that accidental semantics while retaining numeric nonrenewing deadline/wait provenance. Final suite verifies sequential original/newroot data, identical finite/ordinary sibling wires with independent lifetimes, current cancellation/retirement and getter cancellation. Registered admission command terminal exit0: 10pass/58assertions. See rewritten deadline-projection.md for final semantics and remaining admitted/native limitations.

## Large Import Payload Boundary

Schema-first public mounted chunk/staging/typed/LoadDocument regression added for raw77,867B and dataURL103,846B; third-party Three+Ajv oracle passes both. Import-only concrete factory, static key/publication partition, proof type and producer ceiling aligned; ordinarytools16KiB unchanged. No native run by this agent and no failing/passing Rust execution claimed. See import-boundary.md for precise source scope, framing audit and monolithic parser/performance limits.

Large import final audit found independent65,536B generic original producer cap before factory. Added command-owned captured maxselector seam; framework/default runtime stays65,536 while Lowpoly public action producer now16KiB ordinary/import2,101,280B. Wire max derives2MiB payload+4096name+32Pack framing; native OpBinary extent law added. New generic producer law belongs semio-framework-plugin-command --lib and remains native-unexecuted. See import-boundary.md for exact ownership and monolithic encoding/parser limitations.

Import producer bounds final audit rejects unrepresentable/zero selected wire maxima before encoding demand, preserves originalcommand for funded close on invalidselection; neutral negativecases and frameworkunitlaw added (unexecuted). Final Three/Ajv/JSONextent oracle exit0, scoped whitespace exit0. No native/heavyduplicate was started.

## Bounded Flood Kernel And Native Prerequisite

See flood-kernel.md (actualTSred/green10pass5674assertions; Rustmirrorauthorednotexecuted), flood-parity.md (commongrant/outcome/sourceidentitycontract), fill-runtime-plan.md (integrationandcheckpoint/close). Native29416 isoneexplicitlyauthorizedfocusedpaidfillRED attempt, stillbeforebodylastpoll1261s; logsandOSidentity in fill-native.md. No Rustred/passclaimed. IndependentGraphlibcomb oracle passes32896changed/32641sparseruns/32768mathbound; Verifyadoptedcombprivateunit. No Lowpoly paidfill productionintegrationstarted pendinggenuineNativeevidence.

## Canonical Texture Source In Progress
Canonical texture schema and independent Ajv/Buffer fixture tests passed1test45assertions after initial missing-module RED. Rust model helpers now return typed errors, composite validates hidden layers, engine validates the entire affected snapshot before materialization, and pack/text decode plus retained snapshot admission reject invalid extents. Production call-site propagation is being audited; no Native pass. Own Native29416 terminated before observed test bodies; see fill-native.md.

## Native Hold After Freshness Regression
Owned57181 and replacement29725 were normally canceled on Root instruction; every recorded own process was verified gone before subsequent action. No Native remains owned by this agent. Neither cancellation is a test failure. See fill-native.md for exact identities, reason and preserved generated logs. Kernel/canonical source is authored and source-reviewed, with actual TS/Ajv/NumPy evidence; Rust tests and Lowpoly paid-fill integration remain outstanding.
